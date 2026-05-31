# 实验报告

## 语法变量表

AST 各节点类型如下：

### 顶层结构

| 类型         | 字段      | 说明             |
| ------------ | --------- | ---------------- |
| `Program`    | `stms`    | 语句序列的向量   |

### 语句（Statement）

| 类型              | 字段                            | 说明             |
| ----------------- | ------------------------------- | ---------------- |
| `DeclStatement`   | `name: String`, `init: Expr`    | 声明语句         |
| `AssignStatement` | `name: String`, `value: Expr`   | 赋值语句         |
| `IfStatement`     | `cond: Expr`, `then`, `else?`   | if 分支          |
| `WhileStatement`  | `cond: Expr`, `body: Block`     | while 循环       |

### 表达式（Expression）

| 类型                    | 字段                                    | 说明             |
| ----------------------- | --------------------------------------- | ---------------- |
| `Identifier`            | `name: String`                          | 标识符引用       |
| `Integer`               | `value: i64`                            | 整数字面量       |
| `Binary`                | `oper: BinaryOperator`, `left`, `right` | 二元运算         |

### 二元运算符（BinaryOperator）

| 运算符 | Token | 说明     |
| ------ | ----- | -------- |
| `Equal` | `==` | 相等比较 |
| `Less`  | `<`  | 小于比较 |
| `Add`   | `+`  | 加法     |
| `Mul`   | `*`  | 乘法     |

### 辅助结构

| 类型   | 字段       | 说明           |
| ------ | ---------- | -------------- |
| `Block` | `stms`     | `{ StmList }`  |

## 文法规则

### LL(1) 文法

开始符号为 `Program`，文法采用递归下降可解析的 LL(1) 形式。

#### 终结符约定

以下终结符来自 Lexer 产出的 Token：

| 记号    | 含义       |
| ------- | ---------- |
| `i32`   | 类型关键字 |
| `if`    | 条件分支   |
| `else`  | 否则分支   |
| `while` | 循环       |
| `id`    | 标识符     |
| `num`   | 非负整数   |
| `=`     | 赋值       |
| `==`    | 相等比较   |
| `<`     | 小于比较   |
| `+`     | 加法       |
| `*`     | 乘法       |
| `(` `)` | 圆括号     |
| `{` `}` | 语句块     |
| `;`     | 语句结束   |
| `$`     | 输入结束符 |

空白符和 `#` 单行注释在词法分析阶段已被消去，不进入语法分析。

#### 产生式表

| 编号 | 产生式                                    |
| ---- | ----------------------------------------- |
| (1)  | `Program -> StmList`                      |
| (2)  | `StmList -> Stm StmList`                  |
| (3)  | `StmList -> ε`                            |
| (4)  | `Stm -> DeclStm`                          |
| (5)  | `Stm -> AssignStm`                        |
| (6)  | `Stm -> IfStm`                            |
| (7)  | `Stm -> WhileStm`                         |
| (8)  | `DeclStm -> i32 id = ArithExpr ;`         |
| (9)  | `AssignStm -> id = ArithExpr ;`           |
| (10) | `IfStm -> if ( BoolExpr ) Block ElsePart` |
| (11) | `ElsePart -> else Block`                  |
| (12) | `ElsePart -> ε`                           |
| (13) | `WhileStm -> while ( BoolExpr ) Block`    |
| (14) | `Block -> { StmList }`                    |
| (15) | `BoolExpr -> ArithExpr RelOp ArithExpr`   |
| (16) | `RelOp -> ==`                             |
| (17) | `RelOp -> <`                              |
| (18) | `ArithExpr -> Term ArithTail`             |
| (19) | `ArithTail -> + Term ArithTail`           |
| (20) | `ArithTail -> ε`                          |
| (21) | `Term -> Factor TermTail`                 |
| (22) | `TermTail -> * Factor TermTail`           |
| (23) | `TermTail -> ε`                           |
| (24) | `Factor -> id`                            |
| (25) | `Factor -> num`                           |
| (26) | `Factor -> ( ArithExpr )`                 |

每个 `(非终结符, 向前看)` 组合至多对应一条产生式，满足 LL(1) 要求。

#### 核心设计决策

1. **`BoolExpr` 限定为关系表达式**：`BoolExpr -> ArithExpr RelOp ArithExpr`，不涉及短路逻辑，由此简化了整个文法。
2. **`Block` 强制使用花括号**：`Block -> { StmList }`，因此不存在悬空 `else` 问题——`ElsePart` 的 `else` 总是匹配最近的 `if`。
3. **运算符优先级**：文法将 `ArithExpr` 拆分为 `Term`（含 `*`）和 `ArithTail`（含 `+`），天然编码了 `*` 高于 `+` 的优先级。

## 设计过程

### Parser

Parser 的核心是将 Lexer 产出的 Token 流转化为 AST。与 Lexer 流式 `next_token()` 的接口配合，Parser 在工程上采用**递归下降 + 单 token 前瞻**的方案是最自然的选择。

核心接口设计：

```rust
pub struct Parser<I: Input> {
    lexer: Lexer<I>,
    ahead: Option<Token>,  // 单 token 前瞻缓冲区
    cursor: Cursor,
}

impl<I: Input> Parser<I> {
    pub fn new(input: I) -> Self { /* ... */ }
    pub fn parse_program(&mut self) -> ParserResult<Program> { /* ... */ }
}
```

### 递归下降函数编排

每个非终结符对应一个解析函数，向前看一个 token 决定分发方向。

#### 入口：`parse_program`

```
parse_program():
    stms = parse_stm_list()
    if peek() is not None:
        error("end of input")
    return Program { stms }
```

#### 语句层派发：`parse_stm`

```
parse_stm():
    match peek():
        Some(Keyword(I32))  → parse_decl_stm()    // 声明语句
        Some(Identifier(_)) → parse_assign_stm()  // 赋值语句
        Some(Keyword(If))   → parse_if_stm()      // if 分支
        Some(Keyword(While))→ parse_while_stm()   // while 循环
        _                   → error("start of statement")
```

#### 声明语句：`parse_decl_stm`

```
parse_decl_stm():
    expect_keyword(I32)
    name = expect_identifier()
    expect_operator(Assign)
    init = parse_arith_expr()
    expect_punctuation(Semicolon)
    return DeclStatement { name, init }
```

#### 赋值语句：`parse_assign_stm`

```
parse_assign_stm():
    name = expect_identifier()
    expect_operator(Assign)
    value = parse_arith_expr()
    expect_punctuation(Semicolon)
    return AssignStatement { name, value }
```

#### if 分支：`parse_if_stm` / `parse_else_part`

```
parse_if_stm():
    expect_keyword(If)
    expect_punctuation(ParenLeft)
    cond = parse_bool_expr()
    expect_punctuation(ParenRight)
    then_block = parse_block()
    else_block = parse_else_part()
    return IfStatement { cond, then_block, else_block }

parse_else_part():
    match peek():
        Some(Keyword(Else)):
            expect_keyword(Else)
            block = parse_block()
            return Some(block)
        _:
            return None     // ε：无 else 子句
```

#### while 循环：`parse_while_stm`

```
parse_while_stm():
    expect_keyword(While)
    expect_punctuation(ParenLeft)
    cond = parse_bool_expr()
    expect_punctuation(ParenRight)
    body = parse_block()
    return WhileStatement { cond, body }
```

#### 块：`parse_block`

```
parse_block():
    expect_punctuation(BraceLeft)
    stms = parse_stm_list()
    expect_punctuation(BraceRight)
    return Block { stms }
```

#### 布尔表达式：`parse_bool_expr`

```
parse_bool_expr():
    left = parse_arith_expr()
    oper = parse_rel_oper()
    right = parse_arith_expr()
    return Binary { oper, left, right }
```

#### 算术表达式（带优先级）

```
parse_arith_expr():
    left = parse_term()
    while peek() == Operator(Add):
        expect_operator(Add)
        right = parse_term()
        left = Binary { oper: Add, left, right }
    return left

parse_term():
    left = parse_factor()
    while peek() == Operator(Mul):
        expect_operator(Mul)
        right = parse_factor()
        left = Binary { oper: Mul, left, right }
    return left

parse_factor():
    match take_token():
        Some(Identifier(name)) → Identifier { name }
        Some(Integer(value))   → Integer { value }
        Some(Punctuation(ParenLeft)):
            expr = parse_arith_expr()
            expect_punctuation(ParenRight)
            return expr
        Some(found) → error("identifier, integer, or `(`")
        None        → error("identifier, integer, or `(`")
```

运算符优先级通过两层递归实现：`parse_arith_expr` 调用 `parse_term` 后循环消费 `+`，而 `parse_term` 调用 `parse_factor` 后循环消费 `*`。这保证了 `*` 先于 `+` 结合，例如 `a + b * c` 解析为 `Add(a, Mul(b, c))`。

### Token 前瞻机制

Parser 维护一个 `ahead: Option<Token>` 单 token 前瞻缓冲区：

```
peek_token():
    fill_ahead()          // Lazy：仅当 ahead 为空时才从 Lexer 拉取
    return ahead.as_ref()

take_token():
    fill_ahead()
    cursor = self.cursor
    return (ahead.take(), cursor)  // 消费后缓冲区置空

fill_ahead():
    if ahead.is_some(): return     // 已有前瞻值，跳过
    match lexer.next_token():
        Ok(Some(tok)):
            cursor = lexer.cursor()
            ahead = Some(tok)
        Ok(None):
            cursor = lexer.cursor()
            ahead = None
        Err(err) → ParserError::Lex { err, cursor }
```

每个 `expect_*` 函数调用 `take_token()` 消费一个 token 并验证其类型。如果类型不匹配，立即报告错误并附带光标位置。

### AST 前序输出

AST 提供了 `preorder_string()` 方法，将语法树以前序树形格式输出：

```text
Program
├── DeclStatement(a)
│   └── Integer(1)
└── IfStatement
    ├── Condition
    │   └── Equal
    │       ├── Identifier(a)
    │       └── Integer(1)
    ├── ThenBlock
    │   └── ...
    └── ElseBlock
        └── ...
```

实现要点：
- 每个节点调用 `push_tree_line` 写入一行带缩进和连接符（`├── ` 或 `└── `）的文本
- 非终节点递归展开其子节点，通过 `prefix` 参数传递缩进延续字符（`│   ` 或 `    `）

### 错误处理体系

| 错误类型 | 触发位置 | 含义 |
|---------|---------|------|
| `ParserError::Lex` | `fill_ahead()` | Lexer 产生词法错误，Parser 透传并附带 cursor |
| `ParserError::UnexpectedToken` | 各 `expect_*` / `parse_*` | 期望某类 token，但遇到其他类型的 token |
| `ParserError::UnexpectedEndOfInput` | 各 `expect_*` / `parse_*` | 期望 token，但输入流已耗尽（EOF） |

错误输出包含 `(line, col)` 光标位置信息，格式示例：

- `(line 1, col 3): expected ;, found Keyword(i32)` —— 遗漏分号
- `(line 2, col 0): expected }, found EOF` —— 遗漏右花括号
- `(line 1, col 8): InvalidChar('@')` —— 词法阶段捕获的非法字符

## 测试用例设计思路

核心是自底向上，先测单个语法结构，再测组合与控制流嵌套，最后测错误路径。

### 单元测试

| 维度 | 测试内容 | 例 |
|------|---------|------|
| 声明语句 | `i32` + 标识符 + `=` + 表达式 + `;` | `"i32 a = 1;"` |
| 运算符优先级 | `*` 优先于 `+` 的结合次序验证 | `"a = b + c * 2;"` —— 得 `Add(b, Mul(c, 2))` |
| 嵌套控制流 | if / while / else 完整嵌套 | `"if (a == 1) { while (b < 10) { ... } } else { ... }"` |
| 缺失分号 | 遗漏 `;` 的错误检测 | `"a = 1"` → error |
| 缺失右括号 | `if` 条件缺少 `)` | `"if (a == 1 { ... }"` → error |
| 缺失右花括号 | while body 缺少 `}` | `"while (a < 1) { a = a + 1;"` → error |
| 缺失左括号 | `if` 后没有 `(` | `"if a == 1) { ... }"` → error |
| 空赋值值 | `=` 后无表达式 | `"a = ;"` → error |
| BoolExpr 无关系运算符 | while 条件中缺 `==` 或 `<` | `"while (a) { ... }"` → error |
| 词法错误穿透 | 非法字符被 Lexer 捕获并由 Parser 透传 | `"a = @;"` → `InvalidChar('@')` |

### 集成测试

按 `tests/example/` 下的 5 对 `input-N.txt` / `output-N.txt` 文件验证：

| 用例 | 输入 | 预期输出 | 验证要点 |
|------|------|---------|---------|
| input-1 | 含注释、声明、if-else、while 嵌套的完整程序 | 完整前序语法树 | 复杂控制流 + 注释跳过 + 嵌套 AST 结构正确 |
| input-2 | 含非法字符 `@` 的程序 | 位置为 `(line 1, col 8)` 的 `InvalidChar('@')` | 词法错误穿透 Parser 并准确定位 |
| input-3 | 带前导零的整数 `0123` | 位置为 `(line 0, col 9)` 的 `UnexpectedChar('1')` | Lexer 前导零拒绝策略 → Parser 错误透传 |
| input-4 | 第一行遗漏分号 | `(line 1, col 3): expected ;, found Keyword(i32)` | 语义错误定位（分号缺失后，parser 在下一行开头发现 `i32`） |
| input-5 | 遗漏右花括号直到 EOF | `(line 2, col 0): expected }, found EOF` | EOF 错误检测，告知缺失的 `}` |

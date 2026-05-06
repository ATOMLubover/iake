# 词法分析算法

## 总体流程

词法分析器（Lexer）以流式方式工作，每次调用 `next_token()` 返回一个 token 或错误。

```
next_token():
    1. sanitize()               // 跳过空白与注释
    2. c = input.peek()         // 查看当前字符
    3. if c is None → EndOfInput
    4. if c is Some → dispatch(c)  // 按优先级分发到子 DFA
```

## 预处理算法

### left_trim — 跳过空白

```
left_trim():
    while input.peek() is whitespace:
        input.advance()
```

### skip_comment — 跳过注释

```
skip_comment():
    if input.peek() == '#':
        while input.peek() is not '\n' and not EOF:
            input.advance()
        return true
    return false
```

### sanitize

```
sanitize():
    left_trim()
    while skip_comment():
        left_trim()
```

空白和注释被完全透明地跳过，不影响后续 token 的识别。

## 分发算法

Lexer 作为主 DFA，根据首字符向子 DFA 分发，**按固定优先级**依次判断：

```
dispatch(c):
    if integer_automaton.acceptable(c)   → IntegerAutomaton.try_accept()
    else if identifier_automaton.acceptable(c) → IdentifierAutomaton.try_accept()
    else if operator_automaton.acceptable(c)   → OperatorAutomaton.try_accept()
    else if punctuation_automaton.acceptable(c) → PunctuationAutomaton.try_accept()
    else → InvalidChar(c)
```

优先级说明：数字优先于标识符（`123abc` 先产出 `Integer(123)` 再产出 `Identifier("abc")`）；运算符 `=` 优先于分隔符（`=` 不会误入 punctuation）。

## 子 DFA 算法

每个子 DFA 维护一个状态机，循环 peek/advance 直到终态停机。

### Integer DFA

正则：`0 | [1-9][0-9]*`

```
状态转移：
    S —'0'→ A（终态）
    S —'1'..'9'→ B
    B —'0'..'9'→ B
    B —other→ 停机（终态）

try_accept():
    state = S
    loop peek(c):
        S:  if acceptable(c) → advance, 0→A, 1-9→B
            else → UnexpectedChar(c)
        A:  数字 → UnexpectedChar(c)    // 前导零拒绝
            非数字 → 停机
        B:  数字 → advance, 留在 B
            非数字 → 停机（终态）
    state 为 A 或 B → Ok(Integer(buf))
```

### Identifier & Keyword DFA

正则：`[a-zA-Z_][a-zA-Z0-9_]*`

```
状态转移：
    S —[a-zA-Z_]→ A
    A —[a-zA-Z0-9_]→ A
    A —other→ 停机（终态）

try_accept():
    state = S
    loop peek(c):
        S:  if acceptable(c) → advance, state = A
            else → UnexpectedChar(c)
        A:  if is_alphanumeric_or_underscore(c) → advance
            else → 停机（终态）
    产出后查关键字表（HashSet {"i32","if","else"}）：
        命中 → Ok(Keyword(buf))
        未命中 → Ok(Identifier(buf))
```

### Operator DFA

正则：`== | = | \*`

```
状态转移：
    S —'='→ A（终态）
    S —'*'→ C（终态）
    A —'='→ B（终态）
    A —other→ 停机（终态）

try_accept():
    state = S
    loop peek(c):
        S:  if c == '=' → advance, state = A
            if c == '*' → advance, state = C
            else → UnexpectedChar(c)
        A:  if c == '=' → advance, state = B
            else → 停机（终态，返回 "="）
        B:  停机（终态，返回 "=="）
        C:  停机（终态，返回 "*"）
```

`=` 的贪心匹配：`===` 产出 `==` 后指针停在第三个 `=`。

### Punctuation DFA

正则：`\( | \) | \{ | \} | ;`

```
try_accept():
    peek(c):
        if c in {'(', ')', '{', '}', ';'} → advance, Ok(Punctuation(c))
        else if EOF → EndOfInput
        else → UnexpectedChar(c)
```

单字符匹配，无循环，读到即返回。

## 错误处理策略

| 错误类型 | 触发位置 | 含义 |
|---------|---------|------|
| `InvalidChar(c)` | `dispatch()` | 所有子 DFA 的 `acceptable()` 均返回 false，无匹配 |
| `UnexpectedChar(c)` | 子 DFA 内部 | dispatch 已选中该 DFA，但在非终态遇到非法字符 |
| `TokenTooLong(s)` | 子 DFA 内部 | token 长度超过缓冲区上限（512 字符） |
| `InvalidInteger(s)` | Integer DFA | buf 转整数时溢出 |
| `EndOfInput` | `next_token()` | 输入流已耗尽 |

`InvalidChar` 与 `UnexpectedChar` 的分工：前者表示"根本不知道这是什么"，后者表示"知道是什么类型但内容不符合预期"。例如 `@` 触发 `InvalidChar`，而 `0123` 中 `0` 后的 `1` 触发 `UnexpectedChar`（已进入整数 DFA，前导零后不应再有数字）。

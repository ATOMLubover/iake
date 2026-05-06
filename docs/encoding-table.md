# 单词编码表

| 编码 | 名称 | Token 形式 | 类别 |
|------|------|-----------|------|
| 1 | `KeywordI32` | `i32` | 关键字 |
| 2 | `KeywordIf` | `if` | 关键字 |
| 3 | `KeywordElse` | `else` | 关键字 |
| 4 | `Identifier` | `[a-zA-Z_][a-zA-Z0-9_]*` | 标识符 |
| 5 | `Integer` | `0 \| [1-9][0-9]*` | 整数 |
| 6 | `OperatorEq` | `==` | 运算符 |
| 7 | `OperatorAssign` | `=` | 运算符 |
| 8 | `OperatorMul` | `*` | 运算符 |
| 9 | `ParenLeft` | `(` | 分隔符 |
| 10 | `ParenRight` | `)` | 分隔符 |
| 11 | `BraceLeft` | `{` | 分隔符 |
| 12 | `BraceRight` | `}` | 分隔符 |
| 13 | `Semicolon` | `;` | 分隔符 |

编码范围按类别分段：1-3 关键字、4 标识符、5 整数、6-8 运算符、9-13 分隔符。每个编码占 1 字节（`#[repr(u8)]`）。

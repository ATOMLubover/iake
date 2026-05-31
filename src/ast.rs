#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub stms: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Decl(DeclStatement),
    Assign(AssignStatement),
    If(IfStatement),
    While(WhileStatement),
}

#[derive(Debug, PartialEq, Eq)]
pub struct DeclStatement {
    pub name: String,
    pub init: Expression,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AssignStatement {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IfStatement {
    pub cond: Expression,
    pub then_block: Block,
    pub else_block: Option<Block>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WhileStatement {
    pub cond: Expression,
    pub body: Block,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    pub stms: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Identifier { name: String },
    Integer { value: i64 },
    Binary {
        oper: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Equal,
    Less,
    Add,
    Mul,
}

impl Program {
    pub fn preorder_string(&self) -> String {
        let mut lines = Vec::new();
        lines.push("Program".to_string());

        let n = self.stms.len();
        for (i, stm) in self.stms.iter().enumerate() {
            stm.push_preorder(&mut lines, "", i == n - 1);
        }

        lines.join("\n")
    }
}

impl Statement {
    fn push_preorder(&self, lines: &mut Vec<String>, prefix: &str, is_last: bool) {
        match self {
            Statement::Decl(stm) => {
                let p = push_tree_line(lines, prefix, is_last, &format!("DeclStatement({})", stm.name));
                stm.init.push_preorder(lines, &p, true);
            }
            Statement::Assign(stm) => {
                let p = push_tree_line(lines, prefix, is_last, &format!("AssignStatement({})", stm.name));
                stm.value.push_preorder(lines, &p, true);
            }
            Statement::If(stm) => {
                let p = push_tree_line(lines, prefix, is_last, "IfStatement");
                let has_else = stm.else_block.is_some();

                // Condition (never the last child — ThenBlock always follows)
                let cond_p = push_tree_line(lines, &p, false, "Condition");
                stm.cond.push_preorder(lines, &cond_p, true);

                // ThenBlock (last only if there's no ElseBlock)
                let then_p = push_tree_line(lines, &p, !has_else, "ThenBlock");
                stm.then_block.push_preorder(lines, &then_p);

                // ElseBlock (always last, if present)
                if let Some(block) = &stm.else_block {
                    let else_p = push_tree_line(lines, &p, true, "ElseBlock");
                    block.push_preorder(lines, &else_p);
                }
            }
            Statement::While(stm) => {
                let p = push_tree_line(lines, prefix, is_last, "WhileStatement");

                // Condition (not last — Body follows)
                let cond_p = push_tree_line(lines, &p, false, "Condition");
                stm.cond.push_preorder(lines, &cond_p, true);

                // Body (last within WhileStatement)
                let body_p = push_tree_line(lines, &p, true, "Body");
                stm.body.push_preorder(lines, &body_p);
            }
        }
    }
}

impl Block {
    fn push_preorder(&self, lines: &mut Vec<String>, prefix: &str) {
        if self.stms.is_empty() {
            lines.push(format!("{}└── Empty", prefix));
            return;
        }

        let n = self.stms.len();
        for (i, stm) in self.stms.iter().enumerate() {
            stm.push_preorder(lines, prefix, i == n - 1);
        }
    }
}

impl Expression {
    fn push_preorder(&self, lines: &mut Vec<String>, prefix: &str, is_last: bool) {
        match self {
            Expression::Identifier { name } => {
                push_tree_line(lines, prefix, is_last, &format!("Identifier({})", name));
            }
            Expression::Integer { value } => {
                push_tree_line(lines, prefix, is_last, &format!("Integer({})", value));
            }
            Expression::Binary { oper, left, right } => {
                let p = push_tree_line(lines, prefix, is_last, oper.label());
                left.push_preorder(lines, &p, false);
                right.push_preorder(lines, &p, true);
            }
        }
    }
}

impl BinaryOperator {
    fn label(self) -> &'static str {
        match self {
            BinaryOperator::Equal => "Equal",
            BinaryOperator::Less => "Less",
            BinaryOperator::Add => "Add",
            BinaryOperator::Mul => "Mul",
        }
    }
}

/// Writes a tree line with connector (`├── ` or `└── `) and returns the
/// continuation prefix for this node's children.
fn push_tree_line(lines: &mut Vec<String>, prefix: &str, is_last: bool, text: &str) -> String {
    let connector = if is_last { "└── " } else { "├── " };
    lines.push(format!("{}{}{}", prefix, connector, text));
    if is_last {
        format!("{}    ", prefix)
    } else {
        format!("{}│   ", prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_preorder_tree() {
        let program = Program {
            stms: vec![Statement::Decl(DeclStatement {
                name: "a".into(),
                init: Expression::Binary {
                    oper: BinaryOperator::Add,
                    left: Box::new(Expression::Integer { value: 1 }),
                    right: Box::new(Expression::Identifier { name: "b".into() }),
                },
            })],
        };

        assert_eq!(
            program.preorder_string(),
            "Program\n└── DeclStatement(a)\n    └── Add\n        ├── Integer(1)\n        └── Identifier(b)"
        );
    }
}

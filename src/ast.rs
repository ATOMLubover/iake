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
        push_line(&mut lines, 0, "Program");

        for stm in &self.stms {
            stm.push_preorder(&mut lines, 1);
        }

        lines.join("\n")
    }
}

impl Statement {
    fn push_preorder(&self, lines: &mut Vec<String>, depth: usize) {
        match self {
            Statement::Decl(stm) => {
                push_line(lines, depth, &format!("DeclStatement({})", stm.name));
                stm.init.push_preorder(lines, depth + 1);
            }
            Statement::Assign(stm) => {
                push_line(lines, depth, &format!("AssignStatement({})", stm.name));
                stm.value.push_preorder(lines, depth + 1);
            }
            Statement::If(stm) => {
                push_line(lines, depth, "IfStatement");
                push_line(lines, depth + 1, "Condition");
                stm.cond.push_preorder(lines, depth + 2);
                push_line(lines, depth + 1, "ThenBlock");
                stm.then_block.push_preorder(lines, depth + 2);

                if let Some(block) = &stm.else_block {
                    push_line(lines, depth + 1, "ElseBlock");
                    block.push_preorder(lines, depth + 2);
                }
            }
            Statement::While(stm) => {
                push_line(lines, depth, "WhileStatement");
                push_line(lines, depth + 1, "Condition");
                stm.cond.push_preorder(lines, depth + 2);
                push_line(lines, depth + 1, "Body");
                stm.body.push_preorder(lines, depth + 2);
            }
        }
    }
}

impl Block {
    fn push_preorder(&self, lines: &mut Vec<String>, depth: usize) {
        if self.stms.is_empty() {
            push_line(lines, depth, "Empty");
            return;
        }

        for stm in &self.stms {
            stm.push_preorder(lines, depth);
        }
    }
}

impl Expression {
    fn push_preorder(&self, lines: &mut Vec<String>, depth: usize) {
        match self {
            Expression::Identifier { name } => {
                push_line(lines, depth, &format!("Identifier({})", name));
            }
            Expression::Integer { value } => {
                push_line(lines, depth, &format!("Integer({})", value));
            }
            Expression::Binary { oper, left, right } => {
                push_line(lines, depth, oper.label());
                left.push_preorder(lines, depth + 1);
                right.push_preorder(lines, depth + 1);
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

fn push_line(lines: &mut Vec<String>, depth: usize, text: &str) {
    lines.push(format!("{}{}", "  ".repeat(depth), text));
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
            "Program\n  DeclStatement(a)\n    Add\n      Integer(1)\n      Identifier(b)"
        );
    }
}

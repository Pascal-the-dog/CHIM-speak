use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum MathOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program { loop_stmt: Box<ASTNode> },
    Loop { limit: i64, body: Vec<ASTNode> },
    ConditionalBlock { branches: Vec<ConditionalBranch> },
    VariableAssign { name: String, value: i64 },
    VariableMath { name: String, op: MathOp, value: i64 },
    WhileLoop { var_name: String, body: Vec<ASTNode> },
}

#[derive(Debug, Clone)]
pub struct ConditionalBranch {
    pub condition: ConditionType,
    pub action: ActionType,
}

#[derive(Debug, Clone)]
pub enum ConditionType {
    DivisibleByBoth(i64, i64),
    DivisibleBy(i64),
    Otherwise,
}

#[derive(Debug, Clone)]
pub enum ActionType {
    PrintString(String),
    PrintRawNumber,
}


pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let tok = self.tokens[self.position].clone();
            self.position += 1;
            Some(tok)
        } else {
            None
        }
    }

    pub fn parse(&mut self) -> Result<ASTNode, String> {
        // Enforce script start semantics
        if self.advance() != Some(Token::WheelTurns) || self.advance() != Some(Token::FromFirstDawn) {
            return Err("Compilation Error: Script must start with 'The Wheel turns from the first dawn'".into());
        }

        let limit = match self.advance() {
            Some(Token::UntilMiddleDawn(val)) => val,
            _ => return Err("Compilation Error: Expected middle dawn loop ceiling definition".into()),
        };

        if self.advance() != Some(Token::LetChronicler) || self.advance() != Some(Token::WithinHouseCounter) {
            return Err("Compilation Error: Missing tracking configuration inside the house of the Counter".into());
        }

        let mut body = Vec::new();

        // Parse everything inside the main global execution block until ALMSIVI is met
        while self.position < self.tokens.len() {
            if let Some(Token::Almsivi) = self.peek() {
                self.advance(); // Consume ALMSIVI
                break;
            }

            // Route execution down to individual line statements
            if let Some(stmt) = self.parse_statement()? {
                body.push(stmt);
            }
        }

        Ok(ASTNode::Program {
            loop_stmt: Box::new(ASTNode::Loop { limit, body }),
        })
    }

    fn parse_statement(&mut self) -> Result<Option<ASTNode>, String> {
        match self.peek() {
            Some(Token::EachStep) => {
                self.advance();
                if self.advance() != Some(Token::MustBeWitnessed) {
                    return Err("Compilation Error: Steps must be witnessed by the sleeping deity".into());
                }
                let conditional_node = self.parse_conditionals()?;
                Ok(Some(conditional_node))
            }

            Some(Token::RebelUserps(name, val)) => {
                let n = name.clone();
                let v = *val;
                self.advance();
                Ok(Some(ASTNode::VariableAssign { name: n, value: v }))
            }

            Some(Token::AnticipationReflects(name, val)) => {
                let n = name.clone();
                let v = *val;
                self.advance();
                Ok(Some(ASTNode::VariableMath { name: n, op: MathOp::Add, value: v }))
            }
            Some(Token::WalkingBrassDenies(name, val)) => {
                let n = name.clone();
                let v = *val;
                self.advance();
                Ok(Some(ASTNode::VariableMath { name: n, op: MathOp::Subtract, value: v }))
            }
            Some(Token::HeartAmplifies(name, val)) => {
                let n = name.clone();
                let v = *val;
                self.advance();
                Ok(Some(ASTNode::VariableMath { name: n, op: MathOp::Multiply, value: v }))
            }
            Some(Token::ToolsSunder(name, val)) => {
                let n = name.clone();
                let v = *val;
                self.advance();
                Ok(Some(ASTNode::VariableMath { name: n, op: MathOp::Divide, value: v }))
            }
            Some(Token::WhileSharmatMaintains(var_name)) => {
                let control_variable = var_name.clone();
                self.advance();

                let mut loop_body = Vec::new();

                while self.position < self.tokens.len() {
                    if let Some(Token::EndHouse) = self.peek() {
                        break;
                    }
                    if let Some(stmt) = self.parse_statement()? {
                        loop_body.push(stmt);
                    }
                }

                if self.advance() != Some(Token::EndHouse) {
                    return Err(format!("Compilation Error: The House of {} was never closed!", control_variable));
                }

                Ok(Some(ASTNode::WhileLoop {
                    var_name: control_variable,
                    body: loop_body,
                }))
            }

            _ => {
                // Safely advance over unrecognized tokens
                self.advance();
                Ok(None)
            }
        }
    }

    fn parse_conditionals(&mut self) -> Result<ASTNode, String> {
        let mut branches = Vec::new();

        while self.position < self.tokens.len() {
            match self.peek() {
                Some(Token::IfStarsAlignGeneric(x, y)) => {
                    let val_x = *x;
                    let val_y = *y;
                    self.advance();
                    if let Some(Token::PrintCommand(text)) = self.advance() {
                        branches.push(ConditionalBranch {
                            condition: ConditionType::DivisibleByBoth(val_x, val_y),
                                      action: ActionType::PrintString(text),
                        });
                    }
                }
                Some(Token::ButIfDivisibleGeneric(num)) => {
                    let val = *num;
                    self.advance();
                    if let Some(Token::PrintCommand(text)) = self.advance() {
                        branches.push(ConditionalBranch {
                            condition: ConditionType::DivisibleBy(val),
                                      action: ActionType::PrintString(text),
                        });
                    }
                }
                Some(Token::YetIfDivisibleGeneric(num)) => {
                    let val = *num;
                    self.advance();
                    if let Some(Token::PrintCommand(text)) = self.advance() {
                        branches.push(ConditionalBranch {
                            condition: ConditionType::DivisibleBy(val),
                                      action: ActionType::PrintString(text),
                        });
                    }
                }
                Some(Token::Otherwise) => {
                    self.advance();
                    if self.advance() == Some(Token::WhisperRaw) {
                        branches.push(ConditionalBranch {
                            condition: ConditionType::Otherwise,
                            action: ActionType::PrintRawNumber,
                        });
                    }
                }
                _ => break, // Break when hitting another structural instruction or ALMSIVI
            }
        }

        Ok(ASTNode::ConditionalBlock { branches })
    }

}

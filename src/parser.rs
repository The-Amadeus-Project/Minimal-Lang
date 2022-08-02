use crate::lexer::{Token, TokenType};

pub fn token_type_to_variable_type(token_type: TokenType) -> VariableType {
    match token_type {
        TokenType::String => VariableType::String,
        TokenType::Integer => VariableType::Integer,
        TokenType::FloatingPoint => VariableType::FloatingPoint,
        TokenType::Boolean => VariableType::Boolean,
        _ => {
            unimplemented!()
        }
    }
}

pub fn string_to_variable_type(string: &str) -> VariableType {
    match string {
        "string" => VariableType::String,
        "int" => VariableType::Integer,
        "float" => VariableType::FloatingPoint,
        "bool" => VariableType::Boolean,
        _ => {
            unimplemented!()
        }
    }
}

pub fn token_as_constant_node(tok: Token) -> Node {
    let value;
    match tok.token_type {
        TokenType::String => {
            value = Node::Const {
                value_type: ConstValue::String {
                    value: tok.value.clone(),
                },
            }
        }
        TokenType::Integer => {
            value = Node::Const {
                value_type: ConstValue::Integer {
                    value: tok.value.parse::<i128>().unwrap(),
                },
            }
        }
        TokenType::Boolean => {
            value = Node::Const {
                value_type: ConstValue::Boolean {
                    value: tok.value.parse::<bool>().unwrap(),
                },
            }
        }
        TokenType::FloatingPoint => {
            value = Node::Const {
                value_type: ConstValue::FloatingPoint {
                    value: tok.value.parse::<f64>().unwrap(),
                },
            }
        }
        _ => {
            unreachable!("{:?}", tok)
        }
    }
    value
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstValue {
    String { value: String },
    Integer { value: i128 },
    Boolean { value: bool },
    FloatingPoint { value: f64 },
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum VariableType {
    String,
    Integer,
    Boolean,
    FloatingPoint,

    Void,
}
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Program {
        body: Vec<Box<Node>>,
    },
    Function {
        name: String,
        return_type: VariableType,
        args: Vec<Box<Node>>,
        body: Vec<Box<Node>>,
    },
    FunctionCall {
        name: String,
        args: Vec<Box<Node>>,
    },
    Expr {
        value: Box<Node>,
    },
    Assign {
        name: String,
        var_type: VariableType,
        value: Box<Node>,
    },
    VariableReference {
        name: String,
    },
    Const {
        value_type: ConstValue,
    },
    Return {
        value: Box<Node>,
    },
    BinaryOp {
        left: Box<Node>,
        op: BinaryOperation,
        right: Box<Node>,
    },
    Blank,
}

pub struct Parser {
    tokens: Vec<Token>,
    current_token: Token,
    program: Vec<Box<Node>>,
    scopes: u32,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_token: Token {
                token_type: TokenType::NullForParser,
                value: "".to_string(),
                x: 0,
                y: 0,
            },
            program: vec![],
            scopes: 0,
            index: 0,
        }
    }
    fn error(&mut self, error_title: &str, error_body: &str) -> ! {
        panic!(
            "'{} Error: {}' at line {}, char {}",
            error_title, error_body, self.current_token.x, self.current_token.y
        );
    }
    fn next_token(&mut self) -> bool {
        self.index += 1;
        if self.index == self.tokens.len() {
            false
        } else {
            self.current_token = self.tokens.get(self.index).unwrap().clone() as Token;
            true
        }
    }
    fn peek_next(&self) -> TokenType {
        let index = self.index + 1;
        if self.index == self.tokens.len() {
            TokenType::NullForParser
        } else {
            (self.tokens.get(index).unwrap().clone() as Token).token_type
        }
    }
    fn current_token_allowed_for_expr(&self) -> bool {
        self.current_token.token_type == TokenType::ComparisonOperation
            || self.current_token.token_type == TokenType::MathOperation
            || self.current_token.is_data_type()
    }
    fn push_top_program(&mut self, node: Node) {
        let mut current_scope = self.program.pop().unwrap();
        match *current_scope {
            Node::Function {
                name,
                return_type,
                args,
                mut body,
            } => {
                body.push(Box::new(node));
                self.program.push(Box::new(Node::Function {
                    name,
                    return_type,
                    args,
                    body,
                }))
            }
            _ => unimplemented!(),
        }
    }
    pub fn parse(&mut self) -> Node {
        let types = vec!["int", "string", "char", "bool", "float"];
        self.current_token = self.tokens.get(self.index).unwrap().clone() as Token;

        loop {
            match self.current_token.token_type {
                TokenType::Identifier => {
                    // type name <- value;
                    if types.contains(&&*self.current_token.value.clone()) {
                        if self.program.len() == 0 {
                            self.error("Semantics", "cannot Assign outside of function");
                        }
                        let var_type = &*self.current_token.value.clone();
                        if !self.next_token()
                            || self.current_token.token_type != TokenType::Identifier
                        {
                            self.error("Expectation", "Expected var name")
                        }
                        let var_name = self.current_token.value.clone();

                        if !self.next_token()
                            || self.current_token.token_type != TokenType::AssignmentArrow
                        {
                            self.error("Expectation", "Expected assignment arrow")
                        }

                        let mut values = vec![];
                        loop {
                            self.next_token();

                            if self.current_token.token_type == TokenType::EndLine {
                                break;
                            } else if self.current_token.token_type == TokenType::EndOfFile {
                                self.error("Expectation", "Expected Variable Values or End of line")
                            } else {
                                values.push(self.current_token.clone());
                            }
                        }
                        if values.len() == 0 {
                            self.error("Expectation", "Expected Variable Values");
                        }

                        let value;
                        if values.len() == 1 && values.get(0).unwrap().is_data_type() {
                            value = Box::new(token_as_constant_node(values.pop().unwrap()));
                        } else {
                            unimplemented!()
                        }
                        self.push_top_program(Node::Assign {
                            name: var_name,
                            var_type: string_to_variable_type(var_type),
                            value: Box::new(Node::Expr { value }),
                        })
                    } else if self.peek_next() != TokenType::NullForParser {
                        // todo: syntax handling
                        let name = self.current_token.value.clone();
                        let peek = self.peek_next();
                        match peek {
                            TokenType::ParenthesisOpen => {
                                self.next_token();
                                let mut params = vec![];
                                let mut param = vec![];
                                loop {
                                    self.next_token();

                                    if self.current_token.token_type == TokenType::ParenthesisClose
                                    {
                                        if !param.is_empty() {
                                            if param.len() != 1 {
                                                unimplemented!()
                                            }
                                            params.push(Box::new(Node::Expr {
                                                value: Box::new(token_as_constant_node(
                                                    param.pop().unwrap(),
                                                )),
                                            }));
                                            param.clear()
                                        }
                                        break;
                                    } else if self.current_token.token_type
                                        == TokenType::SeparatorComma
                                    {
                                        if param.len() != 1 {
                                            unimplemented!()
                                        }
                                        params.push(Box::new(Node::Expr {
                                            value: Box::new(token_as_constant_node(
                                                param.pop().unwrap(),
                                            )),
                                        }));
                                        param.clear()
                                    } else if !self.current_token_allowed_for_expr() {
                                        self.error(
                                            "Unexpected",
                                            &*format!(
                                                "'{}'({:?}) is a non parse token",
                                                self.current_token.value.clone(),
                                                self.current_token.token_type
                                            ),
                                        )
                                    } else {
                                        param.push(self.current_token.clone())
                                    }
                                }
                                self.push_top_program(Node::FunctionCall { name, args: params });
                                if !self.next_token()
                                    || self.current_token.token_type != TokenType::EndLine
                                {
                                    self.error("Expectation", "Expected End Line")
                                }
                            }
                            _ => {
                                unimplemented!()
                            }
                        }
                    } else {
                        self.error(
                            "Undefined",
                            &*format!("Unknown '{}'", self.current_token.value),
                        )
                    }
                }
                TokenType::EndOfFile => break,
                TokenType::Fun => {
                    if !self.next_token() || self.current_token.token_type != TokenType::Identifier
                    {
                        self.error("Expectation", "Expected function")
                    }
                    let function_name = self.current_token.value.clone();

                    if !self.next_token()
                        || self.current_token.token_type != TokenType::ParenthesisOpen
                    {
                        self.error("Expectation", "Expected open parenthesis for argument")
                    }
                    let mut last_was_type = false;
                    let mut last_was_arg = false;
                    let mut arg_type = VariableType::String;
                    let mut args = vec![];
                    loop {
                        self.next_token();
                        match self.current_token.token_type {
                            TokenType::ParenthesisClose => break,
                            TokenType::Identifier => {
                                if types.contains(&&*self.current_token.value) {
                                    if last_was_type {
                                        self.error("Expectation", "Expected Variable Name")
                                    } else if last_was_arg {
                                        self.error("Expectation", "Expected Comma for separation")
                                    } else {
                                        last_was_type = true;
                                        arg_type = string_to_variable_type(
                                            &*self.current_token.value.clone(),
                                        )
                                    }
                                } else {
                                    if !last_was_type {
                                        self.error("Expectation", "Expected Parameter Type")
                                    } else {
                                        last_was_type = false;
                                        last_was_arg = true;
                                        args.push(Box::new(Node::Assign {
                                            name: self.current_token.value.clone(),
                                            var_type: arg_type,
                                            value: Box::new(Node::Blank),
                                        }))
                                    }
                                }
                            }
                            TokenType::SeparatorComma => {
                                if !last_was_arg {
                                    self.error("Expectation", "Expected Parameter Type")
                                }
                                last_was_arg = false;
                            }
                            _ => unimplemented!(),
                        }
                    }
                    if !self.next_token()
                        || self.current_token.token_type == TokenType::ReturnTypeArrow
                    {
                        if !self.next_token()
                            || self.current_token.token_type != TokenType::Identifier
                        {
                            self.error("Expectation", "Expected Return Type")
                        } else if !types.contains(&&*self.current_token.value) {
                            self.error("Type", "Invalid Return Type")
                        }
                        let return_type =
                            string_to_variable_type(&*self.current_token.value.clone());
                        if !self.next_token()
                            || self.current_token.token_type != TokenType::Identifier
                        {
                            self.error("Expectation", "Expected start of Function Body")
                        }
                        self.program.push(Box::new(Node::Function {
                            name: function_name,
                            return_type,
                            args,
                            body: vec![],
                        }));
                        self.scopes += 1
                    } else if self.current_token.token_type == TokenType::CurlyBracketOpen {
                        self.program.push(Box::new(Node::Function {
                            name: function_name,
                            return_type: VariableType::String,
                            args,
                            body: vec![],
                        }));
                        self.scopes += 1;
                    } else {
                        self.error(
                            "Expectation",
                            "Expected A ReturnTypeArrow or A Curly Bracket",
                        )
                    }
                }
                TokenType::CurlyBracketClose => {
                    self.scopes -= 1;
                }
                TokenType::Return => {
                    let mut values = vec![];
                    loop {
                        self.next_token();

                        if self.current_token.token_type == TokenType::EndOfFile {
                            self.error("Expectation", "Expected End Of Line")
                        } else if self.current_token.token_type == TokenType::EndLine {
                            break;
                        } else if self.current_token_allowed_for_expr() {
                            values.push(self.current_token.clone())
                        } else {
                            self.error(
                                "Unexpected",
                                &*format!(
                                    "'{}'({:?}) is a non parse token",
                                    self.current_token.value.clone(),
                                    self.current_token.token_type
                                ),
                            )
                        }
                    }
                    if values.len() != 1 {
                        unimplemented!()
                    }
                    let value;
                    value = Box::new(token_as_constant_node(values.pop().unwrap()));
                    self.push_top_program(Node::Return {
                        value: Box::new(Node::Expr { value }),
                    })
                }
                _ => {
                    unimplemented!("{:?}", self.current_token)
                }
            }

            self.next_token();
        }
        if self.scopes != 0 {
            self.error(
                "Syntax",
                &*format!("Unclosed Scopes, {} scopes unclosed", self.scopes),
            );
        }

        Node::Program {
            body: self.program.clone(),
        }
    }
    pub fn parse_tokens(tokens: Vec<Token>) -> Node {
        Parser::new(tokens).parse()
    }
}

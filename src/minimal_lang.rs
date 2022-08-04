use crate::compiler::Compiler;
use crate::lexer::{Lexer, Token};
use crate::parser::{BinaryOperation, ConstValue, Node, Parser, VariableType};

fn pre_compile(file_path: &str, debug: bool) -> Node {
    let file_content = std::fs::read_to_string(file_path).expect("couldnt open file");
    let lexed = Lexer::new().lex_text(file_content);
    if debug {
        for part in &lexed {
            println!("{:?}", part)
        }
        println!("--------------------------------------------------------");
    }
    Parser::parse_tokens(lexed)
    // Checker::check_instructions(returned_lexed.clone());
}
pub fn compile(file_path: &str, debug: bool) -> String {
    // let parsed = pre_compile(file_path, debug);
    // println!(
    //     "{:#?}\n--------------------------------------------------------\n",
    //     parsed
    // );
    // let program = Compiler::compile(parsed.clone());
    // println!("{}", program);
    let parsed = Node::Program { body: vec![
        Box::new(Node::Function {
            name: "main".to_string(),
            return_type: VariableType::Integer,
            args: vec![],
            body: vec![
                Box::new(Node::Assign {
                    name: "name".to_string(),
                    var_type: VariableType::Integer,
                    value: Box::new(
                        Node::Expr {
                            value: Box::new(
                                Node::BinaryOp {
                                    left: Box::new(
                                        Node::Const {
                                            value_type: ConstValue::Integer {
                                                value: 1
                                            }
                                        }
                                    ),
                                    op: BinaryOperation::Add,
                                    right: Box::new(Node::Const {
                                            value_type: ConstValue::Integer {
                                                value: 1
                                            }
                                        }
                                    )
                                }
                            )
                        }
                    )
                }),
                Box::new(
                    Node::FunctionCall {
                        name: "put_i".to_string(),
                        args: vec![
                            Box::new(
                            Node::Expr {
                                value: Box::new(
                                    Node::VariableReference {
                                        name: "name".to_string()
                                    }
                                )
                            }
                        )]
                    }
                )
            ]
        })
    ]
    };

    let mut program = Compiler::new(parsed);
    program.builder.new_string_literal("put_i_fmt_str", "%d\n");
    // program.builder.extern_add( "printf");

    program.builder.open_function("put_i");
    program.builder.mov("esi", "edi");
    program.builder.mov("eax", "0");
    program.builder.call_function("printf", vec!["put_i_fmt_str"]);
    program.builder.close_function();
    return program.run()
}

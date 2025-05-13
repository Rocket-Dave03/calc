#[allow(unused_imports)]
use parser::{Expression, Operator, OperatorType, util::print_tree};
use rustyline::{DefaultEditor, error::ReadlineError};

mod parser;

fn main() {
    // let a = Expression::Op(Box::new(Operator {
    //     typ: OperatorType::Add,
    //     lhs: Expression::Op(Box::new(Operator {
    //         typ: OperatorType::Add,
    //         lhs: Expression::Op(Box::new(Operator {
    //             typ: OperatorType::Add,
    //             lhs: Expression::Number(8),
    //             rhs: Expression::Number(2),
    //         })),
    //         rhs: Expression::Number(2),
    //     })),
    //     // lhs: Expression::Number(23),
    //     rhs: Expression::Op(Box::new(Operator {
    //         typ: OperatorType::Add,
    //         lhs: Expression::Number(1),
    //         rhs: Expression::Op(Box::new(Operator {
    //             typ: OperatorType::Add,
    //             lhs: Expression::Number(16),
    //             rhs: Expression::Number(0),
    //         })),
    //     })),
    // }));
    //
    // print_tree(&a);

    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                match line.as_str() {
                    "quit" | "exit" => break,
                    line => {
                        let tokens = match parser::lex(line) {
                            Ok(t) => t,
                            Err(e) => {
                                println!("{e}");
                                continue;
                            }
                        };
                        println!("{tokens:?}")
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    let inp = "1 + 2 * 3";
    let tokens = parser::lex(inp);
    println!("{tokens:?}");
}

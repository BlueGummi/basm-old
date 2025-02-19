use basm2::*;

fn main() {
    let input_string = r#"label: macro_rules! my_macro ( arg1 : reg, arg2 : imm, arg3 : mem, arg4 : ireg, arg5 : label ) {
    mov %arg1, %arg2 ; comment
    label_again: .asciiz "My text"
    %arg4:
}
"#;
    println!("{input_string}");
    let mut parser = Parser::new(input_string);
    match parser.parse() {
        Ok(tokens) => {
            println!("{}", serde_json::to_string_pretty(&tokens).unwrap());
        }
        Err(e) => {
            for error in e {
                println!("Error: {}", error);
            }
        }
    }
}

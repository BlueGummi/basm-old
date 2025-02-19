use basm2::*;

fn main() {
    let input_string = r#"label: macro_rules! my_macro ( arg1 : reg, arg2 : imm, arg3 : mem, arg4 : ireg ) {
    mov %arg1, %arg2 ; comment
}

macro_rules! boink (floop: imm, florp: ireg) {
    mov r0, %floop
    mov r1, %florp
}

0o333

"fnaf is real \" "

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

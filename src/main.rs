use basm2::*;

fn main() {
    let input_string = r#"


    label: macro_rules!  ( arg1 : reg, arg2 : imm, arg3 : mem, arg4 : ireg, arg5 : label ) { 
    mov %arg1, %arg2 ; comment
    lea r2, [0xff]
    mov r0, &[0x0]
    label_again: .asciiz "My text"
    .word 'm'
    nand r3, r3
    push 0x8
    pop 0o02
    %arg5:
}
"#;
    println!("{input_string}");
    let mut parser = Parser::new(input_string);
    match parser.parse() {
        Ok(tokens) => {
            println!("{}", serde_json::to_string(&tokens).unwrap());
        }
        Err(e) => {
            for error in e {
                println!("{error}");
            }
        }
    }
}

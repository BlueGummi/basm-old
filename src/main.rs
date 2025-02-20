use basm2::*;

fn main() {
    let input_string = r#"

const v = 3
label: macro_rules! fanf ( arg1 : reg, arg2 : imm, arg3 : mem, arg4 : ireg, arg5 : label ) { 
    mov %arg1, %arg2 ; comment
    lea r2, [0xff]
    mov r0, &[0x0]
    ld r2, [v]
    label_again: .asciiz "My text"
    .word 'm'
    label: nand r4, (2 * 2)
    nand r3, r3
    push 0x8
    pop 0o02
    %arg5:
}
    mov r0, (33 + 3)

    push (3 << 1)

    add r0, ((3 + 3) * 5)
"#;
    println!("{input_string}");
    let mut parser = match Parser::new(String::from("input.asm"), input_string) {
        Ok(v) => v,
        Err(e) => {
            for er in e {
                println!("{er}");
            }
            std::process::exit(1);
        }
    };
    match parser.parse() {
        Ok(tokens) => {
            //println!("{#:?}", serde_json::to_string_pretty(&tokens).unwrap());
            println!("{:#?}", tokens);
        }
        Err(e) => {
            for error in e {
                println!("{error}");
            }
            std::process::exit(1);
        }
    }
}

use basm2::*;

fn main() {
    let input_string = r#"

label: macro_rules! fanf ( arg1 : reg, arg2 : imm, arg3 : mem, arg4 : ireg, arg5 : label ) { 
const     v = (4 * 3)
    mov %arg1, %arg2 ; comment
    mov r0, &[0x0]
    label_again: .asciiz "My text"
}
    mov r0, (v + 3)
    [( 2 * (v + 3))]

    push (3 << 1)
    fanf!( ( 3 + 3 ))
    fnaf!(f)
    add r0, (3 & 4)
    hlt

"#;
    let input_string_2 = r#"
const v = (4 * 3)
    mov r0, (v + 2)
    const a = 5
    const b = 3
    ;add r0, (((( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33) + (( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33))
    const c = ((a * a) + (b * b))
    mov r0, (c)
    macro_rules! fnaf ( arg1 : imm ) {
        mov r0, %arg1
    }
    [( 2 * (c + 3))]
"#;
    println!("{input_string}");
    let my_macaroni = MacroContent {
        full_data: String::from("macro_rules! ka ( frank: reg ) {"),
        file: String::from("aw"),
        name: String::from("ka"),
        args: vec![(
            FullArgument {
                name: String::from("frank"),
                arg_type: ArgumentType::Reg,
            },
            18..23,
        )],
        tokens: Vec::new(),
    };
    if let Err(e) = my_macaroni.is_valid(String::from("ka!(2)"), vec![(TokenKind::IntLit(2), 4..5)])
    {
        // this is working
        println!("{e}");
    }
    let mut parser = match Parser::new(String::from("input.asm"), input_string) {
        Ok(v) => v,
        Err(e) => {
            for er in e {
                println!("{er}");
            }
            std::process::exit(1);
        }
    };
    let mut toks = match parser.parse() {
        Ok(tokens) => {
            //println!("{#:?}", serde_json::to_string_pretty(&tokens).unwrap());
            /*for (element, _) in &tokens {
                println!("{}", element);
            }*/
            tokens
        }
        Err(e) => {
            for error in e {
                println!("{error}");
            }
            std::process::exit(1);
        }
    };
    use crate::TokenKind::*;
    let mut mac_locs = Vec::new();
    for (index, (element, span)) in toks.iter().enumerate() {
        if let Macro(data) = element {
            let mut mac_map = MACRO_MAP.lock().unwrap();
            mac_map.insert(data.name.to_string(), (data.file.to_string(), data.clone()));
            mac_locs.push(index);
        }
    }
    for element in mac_locs {
        toks.remove(element);
    }
    for (index, (element, span)) in toks.iter().enumerate() {
        if let MacroCall(call) = element {
            let mut mac_map = MACRO_MAP.lock().unwrap();
            if let Some(v) = mac_map.get(call) {
                println!("Called macro {call} at {span:?}");
            } else {
                println!("couldn't find macro with name {call}");
            }
        }
    }
}

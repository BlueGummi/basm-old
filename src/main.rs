use basm2::*;
use colored::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
fn main()
{
    let input_string = r#"
    @include "my.asm"
label: macro_rules! silly ( arg1: reg, arg2: imm, arg3: reg, arg4: mem) { 
    mov %arg1, %arg2
    lea %arg2, %arg4
    .asciiz "Yap!"
}
    const memloc = 0xff
    lea r0, [(memloc + 3)]
    illy!(r3, 3, r2, [0xffff])
add r0, (((( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33) + (( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33))
"#;
    println!("{input_string}");
    let file = "input.asm";

    let mut error_count = 0;

    let mut parser = match Parser::new(String::from(file), input_string) {
        Ok(v) => v,
        Err(e) => {
            for er in e {
                error_count += 1;
                println!("{er}\n");
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
                error_count += 1;
                println!("{error}\n");
            }
            std::process::exit(1);
        }
    };

    use crate::TokenKind::*;

    'inc_check: loop {
        let mut included_toks = Vec::new();
        let mut index = 0;
        #[allow(clippy::explicit_counter_loop)]
        for (fname, element, loc) in &toks {
            if let IncludeFile(new_file) = element {
                if *new_file == *fname {
                    let problem = ParserError {
                        file: file.to_string(),
                        help: None,
                        input: input_string.to_string(),
                        message: "cannot include macro file itself".to_string(),
                        start_pos: loc.start,
                        last_pos: loc.end,
                    };
                    error_count += 1;
                    println!("{problem}\n");
                    break 'inc_check;
                }
                let mut file_data = match File::open(new_file) {
                    Ok(v) => v,
                    Err(e) => {
                        let problem = ParserError {
                            file: file.to_string(),
                            help: None,
                            input: input_string.to_string(),
                            message: format!(
                                "{}: with name `{}`: {}",
                                "cannot open file".bold(),
                                new_file.bold(),
                                e.to_string().bold()
                            ),
                            start_pos: loc.start,
                            last_pos: loc.end,
                        };
                        error_count += 1;
                        println!("{problem}\n");
                        break;
                    }
                };
                let mut contents = String::new();
                match file_data.read_to_string(&mut contents) {
                    Ok(_) => (),
                    Err(e) => {
                        let problem = ParserError {
                            file: file.to_string(),
                            help: None,
                            input: input_string.to_string(),
                            message: format!(
                                "{}: with name `{}`: {}",
                                "cannot open file".bold(),
                                new_file.bold(),
                                e.to_string().bold()
                            ),
                            start_pos: loc.start,
                            last_pos: loc.end,
                        };
                        error_count += 1;
                        println!("{problem}\n");
                        break;
                    }
                }
                let mut parser = match Parser::new(String::from(new_file), &contents) {
                    Ok(v) => v,
                    Err(e) => {
                        for er in e {
                            error_count += 1;
                            println!("{er}\n");
                        }
                        break 'inc_check;
                    }
                };

                match parser.parse() {
                    Ok(tokens) => {
                        //println!("{#:?}", serde_json::to_string_pretty(&tokens).unwrap());
                        /*for (element, _) in &tokens {
                            println!("{}", element);
                        }*/
                        for token in tokens.into_iter().rev() {
                            included_toks.insert(index, token);
                        }
                    }
                    Err(e) => {
                        for error in e {
                            error_count += 1;
                            println!("{error}\n");
                        }
                        break 'inc_check;
                    }
                };
            } else {
                included_toks.push((fname.to_string(), element.clone(), loc.clone()));
            }
            index += 1;
        }
        toks = included_toks.clone();
        let toks_has_include = toks
            .iter()
            .any(|(_, kind, _)| matches!(kind, basm2::TokenKind::IncludeFile(_)));
        if !toks_has_include {
            break;
        }
    }

    let mut mac_locs = Vec::new();

    for (index, (_fname, element, _)) in toks.iter().enumerate() {
        if let Macro(data) = element {
            let mut mac_map = MACRO_MAP.lock().unwrap();
            mac_map.insert(
                data.name.0.to_string(),
                (data.file.to_string(), data.clone()),
            );
            mac_locs.push(index);
        }
    }

    for element in mac_locs {
        toks.remove(element);
    }

    let mut mac_call_data = Vec::new();

    let mut in_call = false;
    let mut curr_mac = None;

    let mac_map = MACRO_MAP.lock().unwrap();

    let mut expanded_loc_map: HashMap<usize, Vec<(TokenKind, std::ops::Range<usize>)>> =
        HashMap::new();
    let mut expanded_indices = Vec::new();

    // collecting macro arguments upon macro calls
    let mut counter = 0;
    for (fname, element, span) in &toks {
        counter += 1;
        if let MacroCall(call) = element {
            in_call = true;
            mac_call_data = Vec::new();
            if let Some(v) = mac_map.get(call) {
                curr_mac = Some(v);
            } else {
                let problem = ParserError {
                    file: file.to_string(),
                    help: None,
                    input: input_string.to_string(),
                    message: format!(
                        "{}: with name `{}`",
                        "cannot find macro".bold(),
                        call.bold()
                    ),
                    start_pos: span.start,
                    last_pos: span.end,
                };
                error_count += 1;
                println!("{problem}\n");
                curr_mac = None;
            }

            continue;
        }
        if let RightParen = element {
            in_call = false;
            if let Some((_, m)) = curr_mac {
                match m.is_valid(
                    fname.to_string(),
                    input_string.to_string(),
                    mac_call_data.clone(),
                ) {
                    Ok(v) => {
                        expanded_loc_map.insert(counter, v.clone());
                        expanded_indices.push(counter);
                    }
                    Err(e) => {
                        for e in e {
                            error_count += 1;
                            println!("{e}\n");
                        }
                    }
                }
            }
            continue;
        }
        if in_call {
            mac_call_data.push((element.clone(), span.clone()));
        }
    }

    let size = toks.len();
    for i in 0..size {
        if expanded_indices.contains(&i) {
            let expanded = expanded_loc_map.get(&i).unwrap(); // this never fails as all pairs match
            for element in expanded.iter().rev() {
                let (x, y) = element;
                toks.insert(
                    i,
                    (
                        String::from("NULL: EXPANDED FROM MACRO"),
                        x.clone(),
                        y.clone(),
                    ),
                );
            }
        }
    }

    let mut new_tokens = Vec::new();
    let mut tokerator = toks.clone().into_iter();
    while let Some((f, v, s)) = tokerator.next() {
        match v {
            MacroCall(_) => {
                for (_, val, _) in tokerator.by_ref() {
                    if val == RightParen {
                        break;
                    }
                }
            }
            _ => new_tokens.push((f, v, s)),
        }
    }
    toks = new_tokens;
    if error_count > 0 {
        let msg = if error_count == 1 {
            "error generated "
        } else {
            "errors generated"
        };
        println!(
            "{}\n{} {}",
            "compilation unsuccessful".bold(),
            error_count.to_string().bright_red(),
            msg.bold()
        );
        std::process::exit(1);
    }
    for (_, f, _) in &toks {
        println!("{f}");
    }
}

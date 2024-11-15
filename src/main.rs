use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut replace: bool = false;
    let mut final_endline: bool = true;
    let mut count : u32 = 0;

    for arg in args.clone().into_iter(){
        match arg.as_str() {
            "-e" =>{
                replace = true;
                count+=1;
            },
            "-E" => {
                replace = false;
                count+=1;
            },
            "-n" => {
                final_endline = false;
                count+=1;
            },
            "_" => continue,
            &_ => continue,
        }
    }
    reverb(args, count+1, replace,final_endline);
}

fn reverb(inputs: Vec<String>, count_to_skip:u32,  replace: bool, final_endline : bool) {
    for input in inputs.into_iter().skip(count_to_skip.try_into().unwrap()){
        if replace {
            let char_vec: Vec<char> = input.chars().collect();
            let mut char_iterator = char_vec.iter();
            while let Some(c) = char_iterator.next() {
                if *c == '\u{5c}' {
                    match char_iterator.next().unwrap(){
                        'b' => print!("{}", '\u{8}'),
                        _ => print!("{}",*c),
                    }
                }
                else {
                    print!("{}", c);
                }
            }
            print!(" ");
        }
        else{
        print!("{} ",input);
        }
    }
    if final_endline {

        println!("");
    }
}

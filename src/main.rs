use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut replace: bool = false;
    let mut final_endline: bool = true;
    let mut count : u32 = 1; //start at 1 to skip over the function name call in args

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
            &_ => continue,//not really sure why I need this but rustc is convinced that I do
        }
    }
    reverb(args, count, replace,final_endline);
}

fn reverb(inputs: Vec<String>, count_to_skip:u32,  replace: bool, final_endline : bool) {
    for input in inputs.into_iter().skip(count_to_skip.try_into().unwrap()){
        if replace {
            let char_vec: Vec<char> = input.chars().collect();
            let mut char_iterator = char_vec.iter();
            while let Some(c) = char_iterator.next() {
                if *c == '\u{5c}' {//if the current character you're looking at is \
                    match char_iterator.next().unwrap(){
                        'a' => print!("{}", '\u{7}'),//replaces a with bell
                        'b' => print!("{}", '\u{8}'),//replaces b with backspace
                        'c' => std::process::exit(0),//replaces c with no further output
                        'e' => print!("{}", '\u{1B}'),//replaces with escape character
                        'f' => print!("{}", '\u{C}'), //form feed
                        //TODO: add other escape sequence patterns
                        _ => print!("{}",*c),
                    }
                }
                else {
                    print!("{}", c);
                }
            }
            print!(" "); //it's only a dumb idea if it doesn't work
        }
        else{
        print!("{} ",input);//this hopefully runs faster than going char by char if we don't 'char'
                            //about escape sequence patterns
        }
    }
    if final_endline {

        println!("");
    }
}

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

fn reverb(inputs: Vec<String>, count:u32,  _replace: bool, final_endline : bool) {
    for input in inputs.into_iter().skip(count.try_into().unwrap()){
        print!("{} ",input);
    }
    if final_endline {

        println!("");
    }
}


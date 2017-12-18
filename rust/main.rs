use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const MEM_SIZE: usize = 3000;

fn main() {
    // Read the ARGV
    // env::args()はIteratorで遅延評価されるのでcollect()で全部引き抜く
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("required argments: file");

    }
    let file_path = Path::new(&args[1]);

    // file input
    let mut file = match File::open(&file_path) {
        Err(why) => panic!("coludn't open {}: {}", file_path.display(),
                                                   Error::description(&why)),
        Ok(file) => file,
    };
    let mut script = String::new();
    match file.read_to_string(&mut script) {
        Err(why) => panic!("coludn't open {}: {}", file_path.display(),
                                                   Error::description(&why)),
        Ok(_) => () // Success
    }

    // do scripts
    let script: Vec<char> = script.chars().collect();
    let mut memory: [u8; MEM_SIZE] = [0; MEM_SIZE];
    let mut pointer = 0;
    let mut idx = 0;
    while idx < script.len() {
        match script[idx] {
            '>' => {
                pointer += 1;
                if pointer > MEM_SIZE {
                    panic!("ぬるぽ");
                }
            },
            '<' => {
                pointer -= 1;
                if pointer < 0 {
                    panic!("ぬるぽ");
                }
            },
            '+' => memory[pointer] += 1,
            '-' => memory[pointer] -= 1,
            '.' => print!("{}", memory[pointer] as char),
            ',' => (), // pass
            '[' => if memory[pointer] == 0 {
                let mut roop = 1;
                while roop > 0 {
                    idx += 1;
                    if script[idx] == ']' {
                        roop -= 1;
                    } else if script[idx] == '[' {
                        roop += 1;
                    }
                }
            },
            ']' => if memory[pointer] != 0 {
                let mut roop = 1;
                while roop > 0 {
                    idx -= 1;
                    if script[idx] == '[' {
                        roop -= 1;
                    } else if script[idx] == ']' {
                        roop += 1;
                    }
                }
            },
            _ => (),
        }
        idx += 1;
    }
}

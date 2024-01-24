// Interpreter for the Brainfuck language, following specs. 
// Note that this is not a compiler: it will strictly run through the original 
// code, without building an AST or anything of the sort. 

const TAPE_SIZE: usize = 30000; // from the spec
use std::io::{stdin, Read};

fn interpret(code: &str) {
    // main steps will be populating a jump table for any `/[]/` we might come 
    // across, and defining a tape to run through. 
    let mut needle: usize = 0; 
    let mut tape: [u8; 30000] = [0; 30000];
    let mut j_table: Vec<usize> = vec!(0; code.len());
    let code_len = code.len(); 

    // populate jump table

    // println!("initialized"); 

    let mut parity_stack: Vec<usize> = vec!();
    for (i, c) in code.chars().enumerate() {
        match c {
            '[' => {
                parity_stack.push(i); 
            },
            ']' => {
                if parity_stack.len() == 0 {
                    panic!("unbalanced loops in input !");
                }
                let open: usize = parity_stack.pop().unwrap(); 
                j_table[open] = i; 
                j_table[i] = open; 
            }
            _ => ()
        }
    }

    // println!("constructed"); 

    // run through the code!
    
    let mut PC: usize = 0; // instruction counter
    let tokens = code.as_bytes(); 
    while PC != code_len {
        // println!("{}", tokens[PC]);
        match tokens[PC] as char {
            '>' => {
                needle = if needle == TAPE_SIZE - 1 { 0 } else { needle + 1 };
            },
            '<' => {
                needle = if needle == 0 { TAPE_SIZE - 1 } else { needle - 1 };
            },
            '+' => {
                tape[needle] = tape[needle].wrapping_add(1); 
            },
            '-' => {
                tape[needle] = tape[needle].wrapping_sub(1); // avoid u8 *-flow
            }, 
            '.' => {
                print!("{}", tape[needle] as char);
            }, 
            ',' => {
                let mut input: [u8; 1] = [0];
                stdin().read(&mut input).unwrap(); 
                tape[needle] = input[0];
            },
            '[' => {
                if tape[needle] == 0 {
                    PC = j_table[PC];
                }
            },
            ']' => {
                if tape[needle] != 0 {
                    PC = j_table[PC];
                }
            },
            _ => () 
        }
        PC += 1; 
    }
    println!(); // flush stdout
}

fn main() {
   interpret("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
}


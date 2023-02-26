use std::fs;

fn main() {
    let mut program_file_string = fs::read_to_string("program.s")
        .expect("Can't read file into string.");
    
    println!("{program_file_string}");
    filter_program_file_string(&mut program_file_string)
}

fn filter_program_file_string(string:&String){
    let mut temp: String = String::new();
    for line in string.split('\n'){
        let mut i = 0;
        for token in line.split(' '){
            println!("{i} {token} {:?}", token.as_bytes());
            i += 1;
        }
    }
}
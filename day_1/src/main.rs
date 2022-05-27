use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];

    let input = fs::read_to_string(input_filename)
        .expect("Something went wrong reading the file");

    let input: Vec<usize> = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.parse::<usize>()
                .expect("Something went wrong parsing a number")
        })
        .collect();

    let windows = input.windows(2);
    
    let n_increasing = windows.filter(|x| x[1] > x[0]).count();
        
    println!("There are {} increasing measurements.", n_increasing)
}

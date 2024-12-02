use std::fs;

const FILE_PATH: &str = "data/input.txt";

fn read_input(file_path: &str) -> Result<String, String>{
    fs::read_to_string(file_path).map_err(|e| format!("Unable to read the input file: {}", e))
}

fn main() {
    let file_contents = match read_input(FILE_PATH) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Couldn't read the input file: {}", e);
            std::process::exit(1)
        }
    };

    let mut list_1: Vec<usize> = Vec::new();
    let mut list_2: Vec<usize> = Vec::new();

    // seperate into new line,
    // for each line, seperate by whitespace
    // trim each element, parse it
    // add both elements to their respective list
    
    file_contents
        .lines()
        .for_each(|line| {
            let mut numbers = line
                .split_whitespace()
                .filter_map(|x| x.trim().parse::<usize>().ok());
            if let (Some(num1), Some(num2)) = (numbers.next(), numbers.next()) {
                list_1.push(num1);
                list_2.push(num2);
            } 
            });

    println!("Length of List 1 and List 2 is {} and {}", list_1.len(), list_2.len());

    // sort the lists in ascending order
    list_1.sort();
    list_2.sort();

    // subtract list 1 from list 2
    let result = list_1.iter().zip(list_2.iter()).map(|(a,b)| a.abs_diff(*b)).collect::<Vec<usize>>();

    // sum the absolute difference
    let list_difference: usize = result.iter().sum();

    println!("The difference in 2 lists is: {}", list_difference);
}

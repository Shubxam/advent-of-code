use std::fs;

const FILE_PATH: &str = "data/input.txt";

/// takes the file path and returns the string read from file.
fn read_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).ok();
    if contents.is_some() {
        contents.unwrap()
    } else {
        String::from("Error Occured")
    }
}

/// takes a string seperated by multiple \n and splits and parses into usize and returns the sum.
fn parse_str(contents: String) -> Vec<usize> {
    // one vector where each element has calories (&str) for one elf separated by \n.
    let elf_lists: Vec<&str> = contents.split("\n\n").collect();

    let elf_calories = elf_lists
        .iter()
        .map(|x| {
            x.split("\n")
                .map(|x| x.trim().parse::<usize>().ok())
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .reduce(|acc, e| acc + e)
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<usize>>();

    elf_calories
}

fn main() {
    // read file contents
    let file_contents = match read_file(FILE_PATH) {
        contents if contents == "Error Occured" => std::process::exit(1),
        contents => contents,
    };

    // parses the string as usize and returns sum of numbers seperated by \n\n
    let mut elf_calories = parse_str(file_contents);
    elf_calories.sort();

    // find the largest value from elf_calories vector
    println!(
        "The largest calorie value from all the elves is {}",
        elf_calories[elf_calories.len() - 1]
    );
}
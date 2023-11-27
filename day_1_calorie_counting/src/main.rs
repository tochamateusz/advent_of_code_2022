use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    u32,
};

fn main() {
    let mut elves: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        lines.for_each(|line| match line {
            Ok(possible_value) => {
                let str = possible_value.as_str();
                match str.parse::<u32>() {
                    Ok(v) => {
                        let last = elves.pop().unwrap_or(0);
                        let current = v + last;
                        elves.push(current);
                    }
                    Err(_) => {
                        let last = elves.pop().unwrap_or(0);
                        let pos = elves.binary_search(&last).unwrap_or_else(|e| e);
                        elves.insert(pos, last);
                        elves.push(0);
                    }
                }
            }
            Err(e) => println!("Line Error {e}"),
        });
    }
    println!("{elves:#?}")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

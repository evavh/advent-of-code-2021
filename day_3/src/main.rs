use std::fs;

struct Data {
    columns: Vec<Vec<usize>>,
}

impl Data {
    fn from_string(input: String) -> Self {
        let lines = input.lines();
        let line_length = lines.clone().collect::<Vec<&str>>()[0].len();
        let mut columns = Vec::<Vec<usize>>::with_capacity(line_length);

        for i in 0..line_length {
            columns.push(
                lines
                    .clone()
                    .map(|line| {
                        line[i..i + 1]
                            .parse()
                            .expect("Couldn't parse character into digit")
                    })
                    .collect::<Vec<usize>>(),
            );
        }
        Data { columns }
    }

    fn column_averages(&self) -> Vec<f32> {
        self.columns
            .iter()
            .map(|column| {
                column.iter().sum::<usize>() as f32 / column.len() as f32
            })
            .collect()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let data = Data::from_string(input);

    let gamma = data
        .column_averages()
        .iter()
        .map(|avg| if avg > &0.5 { 1 } else { 0 })
        .fold(String::from(""), |acc, x| acc + &x.to_string());

    let epsilon = data
        .column_averages()
        .iter()
        .map(|avg| if avg < &0.5 { 1 } else { 0 })
        .fold(String::from(""), |acc, x| acc + &x.to_string());

    let gamma = usize::from_str_radix(&gamma, 2)
        .expect("Couldn't parse string into binary number");
    let epsilon = usize::from_str_radix(&epsilon, 2)
        .expect("Couldn't parse string into binary number");

    println!(
        "Part 1: gamma is {}, epsilon is {}, multiplied: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

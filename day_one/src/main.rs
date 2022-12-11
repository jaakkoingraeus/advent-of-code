use std::fs;

fn main() {
    let path = "data.txt";

    read_from_file_and_compare_calories(path);
}

fn return_elf(elf: i32, list_of_cals: &str) -> (i32, i64) {
    let mut sum = 0;

    let parsed_list = list_of_cals.split("\n");

    for cal in parsed_list {
        sum += cal.parse::<i64>().unwrap()
    }

    (elf, sum)
}

fn read_from_file_and_compare_calories(path: &str) {
    println!("Reading contents of: {:?}", path);

    let contents = fs::read_to_string(path).expect("Error reading the file");

    //Create an iterator through which we can loop through or collect() them to into a vector
    let iterator_of_elves = contents.split("\n\n");

    let mut max = 0;
    let mut max_index = 0;

    let mut number_of_times = 0;
    let mut elves = iterator_of_elves.map(|x| { number_of_times += 1; return_elf(number_of_times, x) }).collect::<Vec<_>>();

    //Sort elves based on number of calories
    elves.sort_by(|a, b| b.1.cmp(&a.1));

    //Take top three
    let top_three = elves.iter().take(3).collect::<Vec<_>>();

    println!("Top three elves: {:?}", top_three.iter().map(|x| x.0).collect::<Vec<_>>());

    let total_of_top = top_three.iter().fold(0, |acc, x| acc + x.1);

    println!("Their total calories: {:?}", total_of_top)

}

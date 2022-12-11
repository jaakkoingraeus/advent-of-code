use std::fs;

fn main() {
    let path = "data.txt";

    read_from_file_and_compare_calories(path);
}

fn read_from_file_and_compare_calories(path: &str) {
    println!("Reading contents of: {:?}", path);

    let contents = fs::read_to_string(path).expect("Error reading the file");

    //Create an iterator through which we can loop through or collect() them to into a vector

    let iterator_of_elves = contents.split("\n\n");

    let mut max = 0;
    let mut max_index = 0;

    let mut current_index = 0;

    for elf in iterator_of_elves {
        //Loop through elf's calories and count them

        let list_of_calories = elf.split("\n");

        //Sum up calories
        let mut sum_of_calories = 0;
        for calories in list_of_calories {
            sum_of_calories += calories.parse::<i64>().unwrap(); 
        }

        //Compare to the current max
        if sum_of_calories > max {
            max = sum_of_calories;
            max_index = current_index;
        } 

        current_index += 1;
    }

    println!("Elf number {:?} has the most calories. Number of calories is: {:?}", max_index, max)

}
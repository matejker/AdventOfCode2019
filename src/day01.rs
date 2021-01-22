use std::fs;

fn fuel_counter_upper(x: i32) -> i32 {
    let y = x as f32;
    ((y / 3.).floor() - 2.) as i32
}

fn main() {
    let filename: &str = "data/day01.dat";
    let mut sum: i32 = 0;
    let mut total_sum: i32 = 0;
    let rows = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    for line in rows.split("\n") {
        let x: i32 = line.parse::<i32>().unwrap();
        let mut fuel: i32 = fuel_counter_upper(x);
        sum = sum + fuel;

        while fuel > 0{
            total_sum = total_sum + fuel;
            fuel = fuel_counter_upper(fuel);
        }

    }

    println!("Part 1 sum: {}", sum);  // Part 1 sum: 3395944
    println!("Part 2 total sum: {}", total_sum);  // Part 2 total sum: 5091036

}
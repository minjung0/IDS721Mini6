use std::io;

fn main() {
    println!("Find the day of the week of given a date!");
    
    println!("Please input a day.");
    let day = input_type();

    println!("Please input a month.");
    let month = input_type();

    println!("Please input a year.");
    let year = input_type();
        
    let day_week = day_of_the_week(day, month, year);

    println!("The day of the week is {}.", day_week);
}

fn input_type() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = input.trim().parse().expect("Error!");

    input
}

fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
    let weekdays = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    let months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let is_leap_year = |x: i32| -> bool { x % 4 == 0 && (x % 100 != 0 || x % 400 == 0) };

    let num_days = (1971..year)
        .map(|y| 365 + is_leap_year(y) as i32)
        .sum::<i32>()
        + (0..month - 1)
            .map(|m| months[m as usize] + (m == 1 && is_leap_year(year)) as i32)
            .sum::<i32>()
        + day;

    // 01/01/1971 - Friday
    let res_day = (4 + (num_days - 1)) % 7;
    weekdays[res_day as usize].to_string()
}
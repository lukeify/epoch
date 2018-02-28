extern crate regex;

use std::env;
use regex::Regex;

/// Begins the application, and prints out the returned result as a floating point number on screen.
fn main() {
    let arg : Option<String> = env::args().skip(1).last();

    match arg {
        Some(_i)    => println!("{}", calculate(_i)),
        None        => println!("No range of dates was found")
    }
}

/// Calculates the number of hours worked by splitting the input string on a comma and a space,
/// and mapping them across a custom function before summing them.
///
/// # Arguments
///
/// * `range` {String} - The user input consisting of a range of times delimited by a comma and a space.
///
/// # Returns
///
/// A float representing the number of hours worked.
fn calculate(range: String) -> f32 {
    range.split(", ")
        .map(|d| calculate_single_duration(d))
        .sum()
}

/// Calculates a string duration by splitting the two times on either a hyphen or en-dash.
///
/// # Arguments
///
/// * `duration` {&str} - A single duration of hours worked, such as '11:30-1:45'.
///
/// # Returns
///
/// A float representing the number of hours worked in that timespan.
fn calculate_single_duration(duration: &str) -> f32 {
    let re = Regex::new(r"[\-–]+").unwrap();
    let mut time = re.split(duration).take(2).map(|d| {
        convert_time_to_decimal(d)
    });

    let start = time.next();
    let finish = time.next();

    match (start, finish) {
        (Some(_i), Some(_j))    => calculate_difference_in_times(_i, _j),
        _                       => 0_f32
    }
}

/// Converts a single time to a decimal. For example, it will transform 8:30 into 8.5, and 12:45 into 12.75.
///
/// This is done by splitting the time on a colon, and parsing and adding the two values on either side together.
///
/// # Arguments
///
/// * `time` {&str} - A single time instance, such as '11:30'.
///
/// # Returns
///
/// The parsed time into a decimal, i.e. 11.5.
fn convert_time_to_decimal(time: &str) -> f32 {
    let parts_of_time : Vec<&str> = time.split(":").collect();
    let mut time : f32 = 0_f32;

    match parts_of_time.get(0) {
        Some(_i)    => time = _i.parse::<f32>().unwrap(),
        None        => println!("Something went wrong calculating hours!")
    }

    match parts_of_time.get(1) {
        Some(&"00")  => time = time,
        Some(&"15")  => time = time + 0.25_f32,
        Some(&"30")  => time = time + 0.5_f32,
        Some(&"45")  => time = time + 0.75_f32,
        None | _     => println!("Something went wrong calculating minutes!")
    }

    time
}

/// Helper function to calculate the difference in times, given a start and finish float. For example,
/// a start of 8.5 and end of 10.75 will return 2.25. Note that if the start time is greater than the end
/// time, it is presumed we have crossed noon, and 12 hours is added to the finish, i.e. 11:30 start and 2:45
/// finish becomes 11:30 start and 14:45 finish.
///
/// # Arguments
///
/// * `start` {f32} - A float representing the start time.
/// * `finish` {f32} - A float representing the end time.
///
/// # Returns
///
/// The number of hours worked in that duration.
fn calculate_difference_in_times(start: f32, finish: f32) -> f32 {
    let mut finish = finish;
    if start > finish {
        finish += 12_f32
    }
    finish - start
}
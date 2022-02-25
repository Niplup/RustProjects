use std::io;
use std::io::Write;

struct Date {
    day: u64,
    date: u64,
    month: u64,
    year: u64,
}

impl Date {
    fn date_in_text(&self) -> String {
        let day;
        let month;
        match self.day {
            1 => day = String::from("Monday"),
            2 => day = String::from("Tuesday"),
            3 => day = String::from("Wednesday"),
            4 => day = String::from("Thursday"),
            5 => day = String::from("Friday"),
            6 => day = String::from("Saturday"),
            7 => day = String::from("Sunday"),
            _ => panic!(),
        };
        //Don't need match statement for date as date remains a number
        match self.month {
            1 => month = String::from("January"),
            2 => month = String::from("February"),
            3 => month = String::from("March"),
            4 => month = String::from("April"),
            5 => month = String::from("May"),
            6 => month = String::from("June"),
            7 => month = String::from("July"),
            8 => month = String::from("August"),
            9 => month = String::from("September"),
            10 => month = String::from("October"),
            11 => month = String::from("November"),
            12 => month = String::from("December"),
            _ => panic!(),
        };
        //Don't need match for year as it remains a number
       format!("{}, {} {}, {}", day, month, self.date, self.year)
    }
    
    fn parser() -> Date {
        let mut datestr = String::new();
        print!("Enter the date in the form d/DD/MM/YYYY: ");
        io::stdout().flush().expect("Unable to flush");
        io::stdin().read_line(&mut datestr).expect("Failed to read input");

        //FIX: Find a proper way / Fix the conversion of &str to u64

        let day: u64 = match *&datestr[0..0].trim().parse() {
            Ok(num) => num,
            Err(_)=> panic!(),
        };
        let date: u64 = match *&datestr[2..3].trim().parse() {
            Ok(num) => num,
            Err(_)=> panic!(),
        };
        let month: u64 = match *&datestr[5..6].trim().parse() {
            Ok(num) => num,
            Err(_)=> panic!(),
        };
        let year: u64 = match *&datestr[8..].trim().parse() {
            Ok(num) => num,
            Err(_)=> panic!(),
        };
        Date {
            day,
            date,
            month,
            year,
        }
    }
    
}

fn main() {
    let date1 = Date {
        day: 2,
        date: 3,
        month: 11,
        year: 2022,
    };
    let date2 = Date::parser();
    println!("The date is {}", date1.date_in_text());
    println!("The date is {}", date2.date_in_text());
}

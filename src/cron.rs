use std::num::ParseIntError;
use nom;

#[derive(Debug,PartialEq)]
pub struct CronItem {
    minute: Option<u8>,
    hour: Option<u8>,
    day_of_month: Option<u8>,
    month: Option<u8>,
    day_of_week: Option<u8>,
    command: String
}

fn is_time_item(c: char) -> bool {
    c == '*' || c.is_digit(10)
}

named!(alpha<&str, &str>, take_while!(is_time_item));

named!(time_item<&str, Vec<&str>>,
       separated_list!(tag!(" "), alpha)
);

// named!(cron_item<&str, CronItem>,
//        do_parse!(
//                minute:          time_item >>

//                hour:            time_item >>
//                day_of_month:    time_item >>
//                month:           time_item >>
//                day_of_week:     time_item >>
//                (CronItem {
//                    minute: minute,
//                    hour: hour,
//                    day_of_month: day_of_month,
//                    month: month,
//                    day_of_week: day_of_week,
//                    command: String::from("ls")
//                })
//        )
// );

// #[test]
// fn parse_cron_item() {
//     assert_eq!(cron_item("* * 1 1 1 1 ls"), Ok(("", CronItem {
//         minute: None,
//         hour: None,
//         day_of_month: Some(1),
//         month: Some(1),
//         day_of_week: Some(1),
//         command: String::from("ls")
//     })));
// }

// #[test]
// fn parse_numeric_time_item() {
//     assert_eq!(time_item("1  "), Ok(("  ", Some(1))));
// }

#[test]
fn parse_numeric_time_item() {
    assert_eq!(alpha("1 "), Ok((" ", "1")));
}

#[test]
fn parse_empty_time_item() {
    assert_eq!(alpha("* "), Ok((" ", "*")));
}

#[test]
fn parse_list_time_items() {
    println!("{:?}", alpha("* 1 2 *  "));
    assert_eq!(time_item("* 1 2 *  "), Ok(("  ", vec!("*", "1", "2", "*"))));
}

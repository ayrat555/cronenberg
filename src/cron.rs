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

fn is_time_char(c: char) -> bool {
    c == '*' || c.is_digit(10)
}

fn to_time_item(c: &str) -> Result<Option<u8>, ParseIntError> {
    if c == "*" { return Ok(None) }

    match u8::from_str_radix(c, 10) {
        Ok(num) => Ok(Some(num)),
        Err(er) => Err(er)
    }
}

fn is_new_line(c: char) -> bool {
    c == '\n'
}


named!(time_item<&str, Option<u8>>, map_res!(take_while!(is_time_char), to_time_item));

named!(time_items<&str, Vec<Option<u8>>>,
       separated_list!(tag!(" "), time_item)
);

named!(command<&str, &str>, take_till!(is_new_line));

named!(cron_item<&str, CronItem>,
       do_parse!(
               minute:          time_item >>
               tag!(" ") >>
               hour:            time_item >>
               tag!(" ") >>
               day_of_month:    time_item >>
               tag!(" ") >>
               month:           time_item >>
               tag!(" ") >>
               day_of_week:     time_item >>
               tag!(" ") >>
               command:         command >>
               (CronItem {
                   minute: minute,
                   hour: hour,
                   day_of_month: day_of_month,
                   month: month,
                   day_of_week: day_of_week,
                   command: String::from(command)
               })
       )
);

#[test]
fn parse_numeric_time_item() {
    assert_eq!(time_item("1 "), Ok((" ", Some(1))));
}

#[test]
fn parse_empty_time_item() {
    assert_eq!(time_item("* "), Ok((" ", None)));
}

#[test]
fn parse_list_time_items() {
    assert_eq!(time_items("* 1 2 *  "), Ok(("  ", vec!(None, Some(1), Some(2), None))));
}

#[test]
fn parse_cron_item() {
    assert_eq!(cron_item("* 1 * 2 * ls\n"),
               Ok(("\n",
                   CronItem {
                       minute: None,
                       hour: Some(1),
                       day_of_month: None,
                       month:  Some(2),
                       day_of_week: None,
                       command: String::from("ls")
                   }
               ))
    );
}

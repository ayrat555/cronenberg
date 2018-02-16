use std::num::ParseIntError;
use nom::ErrorKind;

#[derive(Debug,PartialEq)]
pub struct CronItem {
    minute: Option<u8>,
    hour: Option<u8>,
    day_of_month: Option<u8>,
    month: Option<u8>,
    day_of_week: Option<u8>,
    command: String
}

fn from_char(input: &str) -> Result<Option<u8>, ParseIntError> {

    match input {
        "*" => Ok(None),
        _   => match u8::from_str_radix(input, 10) {
                Ok(num) => Ok(Some(num)),
                Err(er) => Err(er)
            }
    }
}

fn is_time_item(c: char) -> bool {
    c.is_digit(10) || c == '*'
}

named!(time_item<&str, Option<u8>>,
       map_res!(take_while_m_n!(2, 2, is_time_item), from_char)
);

named!(cron_item<&str, CronItem>,
       do_parse!(
               minute:          time_item >>
               hour:            time_item >>
               day_of_month:    time_item >>
               month:           time_item >>
               day_of_week:     time_item >>
               (CronItem {
                   minute: minute,
                   hour: hour,
                   day_of_month: day_of_month,
                   month: month,
                   day_of_week: day_of_week,
                   command: String::from("ls")
               })
       )
);

#[test]
fn parse_cron_item() {
    assert_eq!(cron_item("* * 1 1 1 1 ls"), Ok(("", CronItem {
        minute: None,
        hour: None,
        day_of_month: Some(1),
        month: Some(1),
        day_of_week: Some(1),
        command: String::from("ls")
    })));
}

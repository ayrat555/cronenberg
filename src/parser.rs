use std::num::ParseIntError;
use cron_item::TimeItem;
use cron_item::TimeItem::*;
use cron_item::CronItem;

fn is_time_char(c: char) -> bool {
    c == '*' || c.is_digit(10)
}

fn to_time_item(c: &str) -> Result<TimeItem, ParseIntError> {
    if c == "*" { return Ok(AllValues) }

    match u8::from_str_radix(c, 10) {
        Ok(num) => Ok(SingleValue(num)),
        Err(er) => Err(er)
    }
}

fn is_new_line(c: char) -> bool {
    c == '\n'
}


named!(time_item<&str, TimeItem>, map_res!(take_while!(is_time_char), to_time_item));

named!(time_items<&str, Vec<TimeItem>>,
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
    assert_eq!(time_item("1 "), Ok((" ", SingleValue(1))));
}

#[test]
fn parse_empty_time_item() {
    assert_eq!(time_item("* "), Ok((" ", AllValues)));
}

#[test]
fn parse_list_time_items() {
    assert_eq!(time_items("* 1 2 *  "), Ok(("  ", vec!(AllValues, SingleValue(1), SingleValue(2), AllValues))));
}

#[test]
fn parse_cron_item() {
    assert_eq!(cron_item("* 1 * 2 * ls\n"),
               Ok(("\n",
                   CronItem {
                       minute: AllValues,
                       hour: SingleValue(1),
                       day_of_month: AllValues,
                       month:  SingleValue(2),
                       day_of_week: AllValues,
                       command: String::from("ls")
                   }
               ))
    );
}

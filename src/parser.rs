use std::num::ParseIntError;
use cron_item::TimeItem;
use cron_item::TimeItem::*;
use cron_item::CronItem;

fn is_new_line(c: char) -> bool {
    c == '\n'
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn to_number(c: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(c, 10)
}
named!(number<&str, u8>, map_res!(take_while!(is_digit), to_number));

named!(command<&str, &str>, take_till!(is_new_line));

named!(time_interval<&str, TimeItem>,
       do_parse!(
           start: number >>
           tag!("-")     >>
           end:   number >>
           is_a!(" ")    >>
           (Interval((start, end)))
       )
);

named!(multiple_time_values<&str, TimeItem>,
       do_parse!(
           values: separated_list!(tag!(","), number) >>
           is_a!(" ")                                 >>
           (MultipleValues(values))
       )
);

named!(single_time_value<&str, TimeItem>,
       do_parse!(
           number: number >>
           is_a!(" ")     >>
           (SingleValue(number))
       )
);

named!(all_time_values<&str, TimeItem>,
       do_parse!(
           tag!("*")  >>
           is_a!(" ") >>
           (AllValues)
       )
);

named!(time_item<&str, TimeItem>,
       alt!(time_interval | single_time_value | all_time_values | multiple_time_values)
);

named!(cron_item<&str, CronItem>,
       do_parse!(
               minute:          time_item >>
               hour:            time_item >>
               day_of_month:    time_item >>
               month:           time_item >>
               day_of_week:     time_item >>
               command:         command   >>
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
fn parse_time_iterval() {
    assert_eq!(time_interval("1-100 "), Ok(("", Interval((1, 100)))));
}

#[test]
fn parse_multiple_time_values() {
    assert_eq!(multiple_time_values("1,5,25 "), Ok(("", MultipleValues(vec!(1, 5, 25)))));
}

#[test]
fn parse_single_time_value() {
    assert_eq!(single_time_value("55 "), Ok(("", SingleValue(55))));
}

#[test]
fn parse_all_values() {
    assert_eq!(all_time_values("* "), Ok(("", AllValues)));
}

#[test]
fn parse_time_item_as_interval() {
    assert_eq!(time_item("1-100 "), Ok(("", Interval((1, 100)))));
}

#[test]
fn parse_time_item_as_multiple_values() {
    assert_eq!(time_item("1,5,25 "), Ok(("", MultipleValues(vec!(1, 5, 25)))));
}

#[test]
fn parse_time_item_as_single_value() {
    assert_eq!(time_item("1 "), Ok(("", SingleValue(1))));
}

#[test]
fn parse_time_item_as_all_values() {
    assert_eq!(time_item("* "), Ok(("", AllValues)));
}

#[test]
fn parse_cron_item() {
    assert_eq!(cron_item("* 1-5 * 2,5,6 5 ls\n"),
               Ok(("\n",
                   CronItem {
                       minute: AllValues,
                       hour: Interval((1, 5)),
                       day_of_month: AllValues,
                       month:  MultipleValues(vec!(2, 5, 6)),
                       day_of_week: SingleValue(5),
                       command: String::from("ls")
                   }
               ))
    );
}

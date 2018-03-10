use std::num::ParseIntError;
use cron_item::TimeItem;
use cron_item::TimeItem::*;
use cron_item::CronItem;
use nom;

static COMMAND_TERMINATOR: &'static str = "command_end";

#[derive(Debug, PartialEq)]
pub struct ParserError {
    pub message: String,
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn is_space(c: char) -> bool {
    nom::is_space(c as u8)
}

named!(spaces<&str, &str>, take_while!(is_space));

fn to_number(c: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(c, 10)
}

named!(number<&str, u8>, map_res!(take_while!(is_digit), to_number));

named!(command<&str, &str>,
       do_parse!(
           com: take_until!(COMMAND_TERMINATOR) >>
           tag!(COMMAND_TERMINATOR)             >>
           (com)
       )
);

named!(time_interval<&str, TimeItem>,
       do_parse!(
           start: number >>
           tag!("-")     >>
           end:   number >>
           spaces        >>
           (Interval((start, end)))
       )
);

named!(multiple_time_values<&str, TimeItem>,
       do_parse!(
           values: separated_nonempty_list!(tag!(","), number) >>
           spaces                                              >>
           (MultipleValues(values))
       )
);

named!(single_time_value<&str, TimeItem>,
       do_parse!(
           number: number >>
           is_a!(" ")     >>
           spaces         >>
           (SingleValue(number))
       )
);

named!(all_time_values<&str, TimeItem>,
       do_parse!(
           tag!("*")  >>
           spaces     >>
           (AllValues)
       )
);

named!(time_item<&str, TimeItem>,
       alt!(time_interval |  single_time_value | all_time_values | multiple_time_values)
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

pub fn parse_cron_item(s: &str) -> Result<CronItem, ParserError> {
    let s_with_term = &format!("{}{}", s, COMMAND_TERMINATOR);

    match cron_item(s_with_term) {
        Ok((_, cron_item)) => Ok(cron_item),
        Err(_) => Err(ParserError {
            message: String::from("Coundn't parse cron item"),
        }),
    }
}

mod test {
    use super::*;

    #[test]
    fn parse_time_iterval() {
        assert_eq!(time_interval("1-100 1"), Ok(("1", Interval((1, 100)))));
    }

    #[test]
    fn parse_multiple_time_values() {
        assert_eq!(
            multiple_time_values("1,5,25 1"),
            Ok(("1", MultipleValues(vec![1, 5, 25])))
        );
    }

    #[test]
    fn parse_single_time_value() {
        assert_eq!(single_time_value("55   1"), Ok(("1", SingleValue(55))));
    }

    #[test]
    fn parse_all_values() {
        assert_eq!(all_time_values("*     55"), Ok(("55", AllValues)));
    }

    #[test]
    fn parse_time_item_as_interval() {
        assert_eq!(time_item("1-100  0"), Ok(("0", Interval((1, 100)))));
    }

    #[test]
    fn parse_time_item_as_multiple_values() {
        assert_eq!(
            time_item("1,5,25   9"),
            Ok(("9", MultipleValues(vec![1, 5, 25])))
        );
    }

    #[test]
    fn parse_time_item_as_single_value() {
        assert_eq!(time_item("1  sad"), Ok(("sad", SingleValue(1))));
    }

    #[test]
    fn parse_time_item_as_all_values() {
        assert_eq!(time_item("*  wanna die"), Ok(("wanna die", AllValues)));
    }

    #[test]
    fn parse_cron_item() {
        assert_eq!(
            cron_item("* 1-5 * 2,5,6 5 lscommand_end"),
            Ok((
                "",
                CronItem {
                    minute: AllValues,
                    hour: Interval((1, 5)),
                    day_of_month: AllValues,
                    month: MultipleValues(vec![2, 5, 6]),
                    day_of_week: SingleValue(5),
                    command: String::from("ls"),
                }
            ))
        );
    }

    #[test]
    fn parse_cron_item_with_multiword_command() {
        assert_eq!(
            cron_item("* 1-5 * 2,5,6 5 /bin/kill_me_please --now command_end"),
            Ok((
                "",
                CronItem {
                    minute: AllValues,
                    hour: Interval((1, 5)),
                    day_of_month: AllValues,
                    month: MultipleValues(vec![2, 5, 6]),
                    day_of_week: SingleValue(5),
                    command: String::from("/bin/kill_me_please --now "),
                }
            ))
        );
    }
}

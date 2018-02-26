use std::str::FromStr;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str;
use parser::parse_cron_item;
use parser::ParserError;

#[derive(Debug,PartialEq)]
pub enum TimeItem {
    AllValues,
    SingleValue(u8),
    MultipleValues(Vec<u8>),
    Interval((u8, u8))
}

#[derive(Debug,PartialEq)]
pub struct CronItem {
    pub minute: TimeItem,
    pub hour: TimeItem,
    pub day_of_month: TimeItem,
    pub month: TimeItem,
    pub day_of_week: TimeItem,
    pub command: String
}

impl FromStr for CronItem {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, ParserError> {
        parse_cron_item(s)
    }
}

impl Display for TimeItem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &TimeItem::AllValues              => write!(f, "*"),
            &TimeItem::Interval((start, end)) => write!(f, "{}-{}", start, end),
            &TimeItem::MultipleValues(ref values) =>  {
                let result = values.iter()
                    .map(|val| val.to_string())
                    .fold(String::from(""), |mut acc, val| {
                        acc.push_str(&val);
                        acc.push_str(",");

                        acc
                    });

                write!(f, "{}", &result[0..result.len()-1])
            }
            &TimeItem::SingleValue(value) => write!(f, "{}", value)
        }
    }
}

impl Display for CronItem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {} {} {} {} {}", self.minute, self.hour, self.day_of_month, self.month, self.day_of_week, self.command)
    }
}

mod test {
    use super::CronItem;
    use super::TimeItem::*;
    use std::str::FromStr;
    use std::string::ToString;

    #[test]
    fn create_cron_item_from_str() {
        let s = "* * 5-7 1,2,5 8 sudo rm -rf /";

        assert_eq!(CronItem::from_str(s).unwrap(),
                   CronItem {
                       minute: AllValues,
                       hour:   AllValues,
                       day_of_month: Interval((5,7)),
                       month: MultipleValues(vec!(1,2,5)),
                       day_of_week: SingleValue(8),
                       command: String::from("sudo rm -rf /")
                   }
        );
    }

    #[test]
    fn convert_time_item_interval_to_string() {
        let interval = Interval((1, 10));

        assert_eq!("1-10", interval.to_string());
    }

    #[test]
    fn convert_time_item_single_value_to_string() {
        let value = SingleValue(5);

        assert_eq!("5", value.to_string());
    }

    #[test]
    fn convert_time_item_multiple_values_to_string() {
        let value = MultipleValues(vec!(1, 5, 7));

        assert_eq!("1,5,7", value.to_string());
    }

    #[test]
    fn convert_time_item_all_values_to_string() {
        let value = AllValues;

        assert_eq!("*", value.to_string());
    }

    #[test]
    fn convert_cron_item_to_string() {
        let cron_item = CronItem {
            minute: MultipleValues(vec!(1, 10)),
            hour:   Interval((1, 4)),
            day_of_month: Interval((1,11)),
            month: MultipleValues(vec!(1,2,5)),
            day_of_week: AllValues,
            command: String::from("sudo rm -rf /")
        };

        assert_eq!("1,10 1-4 1-11 1,2,5 * sudo rm -rf /", cron_item.to_string());
    }
}

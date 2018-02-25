use std::str::FromStr;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str;
use parser::parse_cron_item;

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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
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
                    .map(|val| val.to_string().as_str())
                    .collect::<Vec<&str>>().join(",");

                write!(f, "{}", result)
            }
            &TimeItem::SingleValue(value)     => write!(f, "{}", value)
        }
    }
}

mod test {
    use super::CronItem;
    use super::TimeItem::*;
    use std::str::FromStr;

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
}

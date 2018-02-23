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

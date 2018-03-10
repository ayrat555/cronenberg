extern crate cronenberg;

use cronenberg::CronItem;
use cronenberg::TimeItem::*;
use cronenberg::ParserError;
use std::str::FromStr;

#[test]
fn failes_with_error() {
    let input = "1 command";

    assert_eq!(
        Err(ParserError { message: String::from("Coundn\'t parse cron item") }),
        CronItem::from_str(input)
    );
}

# cronenberg [![Build Status](https://travis-ci.org/ayrat555/cronenberg.svg?branch=master)](https://travis-ci.org/ayrat555/cronenberg)

## simple cron command entry parser

`cronenberg` provides two core components

* `TimeItem`: An enum that represents cron command time or date field
* `CronItem`: A struct that represents cron command entry, for example, `* * 5-7 1,2,5 8 sudo rm -rf /`

## Example

```rust
extern crate cronenberg;

use cronenberg::CronItem;
use cronenberg::TimeItem::*;
use std::str::FromStr;
use std::string::ToString;

let s = "* * 5-7 1,2,5 8 sudo rm -rf /";
assert_eq!(
    CronItem::from_str(s).unwrap(),
    CronItem {
        minute: AllValues,
        hour: AllValues,
        day_of_month: Interval((5, 7)),
        month: MultipleValues(vec![1, 2, 5]),
        day_of_week: SingleValue(8),
        command: String::from("sudo rm -rf /"),
    }
);

let cron_item = CronItem {
    minute: MultipleValues(vec![1, 10]),
    hour: Interval((1, 4)),
    day_of_month: Interval((1, 11)),
    month: MultipleValues(vec![1, 2, 5]),
    day_of_week: AllValues,
    command: String::from("sudo rm -rf /"),
};
assert_eq!("1,10 1-4 1-11 1,2,5 * sudo rm -rf /", cron_item.to_string());
```

## Contributing

1. [Fork it!](http://github.com/ayrat555/cronenberg/fork)
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create new Pull Request

## Author

Ayrat Badykov (@ayrat555)

## License

cronenberg is released under the MIT License.

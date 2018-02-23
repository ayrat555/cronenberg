extern crate chrono;
#[macro_use]
extern crate nom;

mod cron_item;
mod parser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

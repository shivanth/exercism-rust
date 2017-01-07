extern crate chrono;
use chrono::*;

pub fn after(start: DateTime<UTC>) -> DateTime<UTC> {
    let end = start + Duration::seconds(1000000000);
    end
}

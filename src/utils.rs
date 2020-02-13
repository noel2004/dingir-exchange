use chrono::Utc;

use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_timestamp() -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap().as_micros() as f64 / 1_000_000_f64;
    since_the_epoch
    //current_native_date_time().timestamp_millis() as f64
}

pub fn timestamp_to_chrono(t: f64) -> chrono::NaiveDateTime {
    let sec_f = t; // / 1000_f64;
    let sec = sec_f as i64;
    let ns = ((sec_f - sec as f64) * 1e9) as u32;
    chrono::NaiveDateTime::from_timestamp(sec, ns)
}

/*
impl TryFrom<rust_decimal::Decimal> for bigdecimal::BigDecimal {
    type Error = ParseBigDecimalError;
    fn try_from(value: rust_decimal::Decimal) -> Result<Self, Self::Error> {
        bigdecimal::BigDecimal::from_str(&value.to_string())
    }
}
*/
/*
impl TryFrom<bigdecimal::BigDecimal> for rust_decimal::Decimal {
    type Error = rust_decimal::Error;
    fn try_from(value: rust_decimal::Decimal) -> Result<Self, Self::Error> {
        rust_decimal::Decimal::from_str(&value.to_string())
    }
}
*/
/*
struct DecimalWrapper {
    inner: String,
}
impl TryInto<bigdecimal::BigDecimal> for X {
    type Error = ParseBigDecimalError;

    fn try_into(self) -> Result<bigdecimal::BigDecimal, Self::Error> {
        bigdecimal::BigDecimal::from_str(&self.to_string())
    }
}
*/
pub fn decimal_r2b(d: &rust_decimal::Decimal) -> bigdecimal::BigDecimal {
    bigdecimal::BigDecimal::from_str(&d.to_string()).unwrap()
}

pub fn decimal_b2r(d: &bigdecimal::BigDecimal) -> rust_decimal::Decimal {
    rust_decimal::Decimal::from_str(&d.to_string()).unwrap()
}

pub fn current_native_date_time() -> chrono::NaiveDateTime {
    Utc::now().naive_utc()
}

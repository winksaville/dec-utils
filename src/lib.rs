use rust_decimal::prelude::*;
use rusty_money::{iso, Money};
use separator::Separatable;

/// Convert a decimal to string or an empty string if None
///
/// # Example
/// ```
/// use rust_decimal::prelude::*;
/// use rust_decimal_macros::dec;
///
/// use dec_utils::dec_to_string_or_empty;
///
/// let v = Some(dec!(123));
/// let v_str = &dec_to_string_or_empty(v);
/// assert_eq!(v_str, "123");
///
/// let v = None;
/// let v_str = &dec_to_string_or_empty(v);
/// assert_eq!(v_str, "");
/// ```
pub fn dec_to_string_or_empty(d: Option<Decimal>) -> String {
    if let Some(q) = d {
        format!("{}", q)
    } else {
        "".to_owned()
    }
}

/// Convert a decimal to a USD string using "Bankers Rounding"
///
/// # Example
/// ```
/// use rust_decimal::prelude::*;
/// use rust_decimal_macros::dec;
///
/// use dec_utils::dec_to_usd_string;
///
/// let v = dec!(123.124);
/// let v_str = &dec_to_usd_string(v);
/// assert_eq!(v_str, "$123.12");
///
/// let v = dec!(123.125);
/// let v_str = &dec_to_usd_string(v);
/// assert_eq!(v_str, "$123.12");
///
/// let v = dec!(123.126);
/// let v_str = &dec_to_usd_string(v);
/// assert_eq!(v_str, "$123.13");
/// ```
pub fn dec_to_usd_string(v: Decimal) -> String {
    let v_string = v.round_dp(2).to_string();
    let money_string: String = match Money::from_str(&v_string, iso::USD) {
        Ok(v) => format!("{}", v),
        Err(e) => format!("({} {})", v_string, e),
    };

    money_string
}

/// Convert a a string with comma separators at the 1,000 place
///
/// # Example
/// ```
/// use rust_decimal::prelude::*;
/// use rust_decimal_macros::dec;
///
/// use dec_utils::dec_to_separated_string;
///
/// let v = dec!(0);
/// let v_str = &dec_to_separated_string(v, 0);
/// assert_eq!(v_str, "0");
///
/// let v = dec!(1);
/// let v_str = &dec_to_separated_string(v, 0);
/// assert_eq!(v_str, "1");
///
/// let v = dec!(-1);
/// let v_str = &dec_to_separated_string(v, 0);
/// assert_eq!(v_str, "-1");
///
/// let v = dec!(123.125);
/// let v_str = &dec_to_separated_string(v, 2);
/// assert_eq!(v_str, "123.12");
///
/// let v = dec!(123456.126);
/// let v_str = &dec_to_separated_string(v, 2);
/// assert_eq!(v_str, "123,456.13");
///
/// let v = dec!(-123456.126);
/// let v_str = &dec_to_separated_string(v,2);
/// assert_eq!(v_str, "-123,456.13");
/// ```
pub fn dec_to_separated_string(v: Decimal, dp: u32) -> String {
    let negative = v.is_sign_negative();
    let rounded = v.abs().round_dp(dp);
    let integral_part = rounded.trunc();
    let fractional_part = rounded.fract();

    let fractional_part_string = fractional_part.to_string();
    let fractional_part_str = if dp == 0 {
        // No fractional part
        ""
    } else {
        // There is at least one value to right of decimal point
        // and the values are known to be "ascii" which is utf8.
        // Thus we'll skip the leading "0" and get everything else
        // as our str.
        let fractional_part_utf8 = &fractional_part_string.as_bytes()[1..];
        std::str::from_utf8(fractional_part_utf8).unwrap()
    };
    format!(
        "{}{}{}",
        if negative {
            "-".to_owned()
        } else {
            "".to_owned()
        },
        integral_part.to_u128().unwrap().separated_string(),
        if fractional_part_str.is_empty() {
            ""
        } else {
            fractional_part_str
        },
    )
}

#[cfg(test)]
mod tests {

    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_dec_to_string_or_empty() {
        assert_eq!(dec_to_string_or_empty(Some(dec!(1.024))), "1.024");
        assert_eq!(dec_to_string_or_empty(None), "");
    }

    #[test]
    fn test_dec_to_usd_string() {
        assert_eq!(dec_to_usd_string(dec!(1.024)), "$1.02");
        assert_eq!(dec_to_usd_string(dec!(1.026)), "$1.03");
        assert_eq!(dec_to_usd_string(dec!(1000.026)), "$1,000.03");
    }

    #[test]
    fn test_dec_to_separated_string() {
        assert_eq!(dec_to_separated_string(dec!(0), 0), "0");
        assert_eq!(dec_to_separated_string(dec!(-0), 0), "0");
        assert_eq!(dec_to_separated_string(dec!(1), 0), "1");
        assert_eq!(dec_to_separated_string(dec!(-1), 0), "-1");
        assert_eq!(dec_to_separated_string(dec!(1.1), 0), "1");
        assert_eq!(dec_to_separated_string(dec!(1.024), 2), "1.02");
        assert_eq!(dec_to_separated_string(dec!(1.026), 2), "1.03");
        assert_eq!(dec_to_separated_string(dec!(999), 0), "999");
        assert_eq!(dec_to_separated_string(dec!(-999), 0), "-999");
        assert_eq!(dec_to_separated_string(dec!(999.9), 0), "1,000");
        assert_eq!(dec_to_separated_string(dec!(-999.9), 0), "-1,000");
        assert_eq!(dec_to_separated_string(dec!(1000.026), 2), "1,000.03");
        assert_eq!(dec_to_separated_string(dec!(-1000.026), 2), "-1,000.03");
    }
}

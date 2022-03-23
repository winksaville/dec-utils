use rust_decimal::prelude::*;
use rusty_money::{iso, Money};
use separator::FixedPlaceSeparatable;

pub fn dec_to_string_or_empty(d: Option<Decimal>) -> String {
    if let Some(q) = d {
        format!("{}", q)
    } else {
        "".to_owned()
    }
}

pub fn dec_to_money_string(v: Decimal) -> String {
    let v_string = v.round_dp(2).to_string();
    let money_string: String = match Money::from_str(&v_string, iso::USD) {
        Ok(v) => format!("{}", v),
        Err(e) => format!("({} {})", v_string, e),
    };

    money_string
}

pub fn dec_to_separated_string(v: Decimal, dp: u32) -> String {
    let v_string = v.round_dp(dp).to_string();
    let v_f64: f64 = v_string.parse().unwrap();
    v_f64.separated_string_with_fixed_place(dp as usize)
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
    fn test_dec_to_money_string() {
        assert_eq!(dec_to_money_string(dec!(1.024)), "$1.02");
        assert_eq!(dec_to_money_string(dec!(1.026)), "$1.03");
        assert_eq!(dec_to_money_string(dec!(1000.026)), "$1,000.03");
    }

    #[test]
    fn test_dec_to_separated_string() {
        assert_eq!(dec_to_separated_string(dec!(1.024), 2), "1.02");
        assert_eq!(dec_to_separated_string(dec!(1.026), 2), "1.03");
        assert_eq!(dec_to_separated_string(dec!(1000.026), 2), "1,000.03");
    }
}

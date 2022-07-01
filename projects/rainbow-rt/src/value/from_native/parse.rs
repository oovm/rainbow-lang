use super::*;

///
pub fn parse_number(s: &str) -> Option<Value> {
    let exp_separator: &[_] = &['e', 'E', '*'];
    let (base_part, exponent_value) = match s.find(exp_separator) {
        None => (s, 0),
        Some(loc) => {
            let (base, exp) = (&s[..loc], &s[loc + 1..]);
            let exp = match exp.chars().next() {
                Some('+') => &exp[1..],
                _ => exp,
            };
            match i64::from_str(exp.trim_start_matches('*')) {
                Ok(o) => (base, o),
                Err(_) => return None,
            }
        }
    };
    if base_part == "" {
        return None;
    }
    let (digits, decimal_offset): (String, _) = match base_part.find('.') {
        None => {
            if exponent_value >= 0 {
                return BigInt::from_str_radix(&base_part, 10).map(|i| i * 10_u32.pow(exponent_value as u32)).map(|i| Value::from(i)).ok();
            }
            else {
                (base_part.to_string(), 0)
            }
        }
        Some(loc) => {
            let (lead, trail) = (&base_part[..loc], &base_part[loc + 1..]);
            let mut digits = String::from(lead);
            digits.push_str(trail);
            (digits, trail.len() as i64)
        }
    };
    BigInt::from_str_radix(&digits, 10).map(|big_int| BigDecimal::new(big_int, decimal_offset - exponent_value)).map(|i| Value::from(i)).ok()
}

#[test]
fn test() {
    println!("{:?}", parse_number("+2**2").unwrap());
    println!("{:?}", parse_number("+2.e2").unwrap());
    println!("{:?}", parse_number("+2.0e2").unwrap());
    println!("{:?}", parse_number("+200.0").unwrap());
    println!("{:?}", BigDecimal::from_str_radix("+0.31", 10).unwrap());
    println!("{:?}", BigDecimal::from_str_radix("+0.21", 10).unwrap());
    println!("{:?}", BigDecimal::from_str_radix("+0.11", 10).unwrap());
}

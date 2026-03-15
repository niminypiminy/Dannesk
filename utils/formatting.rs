/// Adds thousand separators (commas) to an i64 number for display.
/// Handles negatives by prefixing a minus sign.
/// Assumes positive balances typically, but works for negatives.
pub fn add_commas(num: i64) -> String {  
    if num < 0 {
        format!("-{}", add_commas(-num))
    } else {
        let mut s = String::new();
        let digits = num.to_string();
        // Dropped unused `len`—not needed for the loop logic
        for (i, c) in digits.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                s.push(',');
            }
            s.push(c);
        }
        s.chars().rev().collect()
    }
}

pub fn format_token_amount(val: f64, decimals: usize) -> String {
    let s = format!("{:.1$}", val, decimals);
    if let Some(dot_idx) = s.find('.') {
        let int = &s[..dot_idx];
        let frac = &s[dot_idx + 1..].trim_end_matches('0');
        if frac.is_empty() {
            format!("{}.00", int)
        } else if frac.len() == 1 {
            format!("{}.{}0", int, frac)
        } else {
            format!("{}.{}", int, frac)
        }
    } else {
        format!("{}.00", s)
    }
}

pub fn format_usd(val: f64) -> String {
    format_token_amount(val, 4)
}
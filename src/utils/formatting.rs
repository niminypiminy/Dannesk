/// Adds thousand separators (commas) to an i64 number for display.
/// Handles negatives by prefixing a minus sign.
/// Assumes positive balances typically, but works for negatives.
pub fn add_commas(num: i64) -> String {  // Removed unnecessary `mut`
    if num < 0 {
        // Handle negative: recurse or adjust, but assuming positive balances
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
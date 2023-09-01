pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let ones = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen",
        "eighteen", "nineteen",
    ];

    let tens = vec![
        "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    if n < 20 {
        return ones[(n - 1) as usize].to_string();
    } else if n < 100 {
        let ten = tens[((n / 10) - 2) as usize];
        let one = if n % 10 != 0 {
            ones[(n % 10 - 1) as usize]
        } else {
            ""
        };
        return format!("{}{}", ten, if one != "" { format!("-{}", one) } else { "".to_string() });
    } else if n < 1000 {
        let hundred = ones[((n / 100) - 1) as usize];
        let remainder = n % 100;
        if remainder == 0 {
            return format!("{} hundred", hundred);
        } else {
            return format!("{} hundred {}", hundred, spell(remainder));
        }
    } else if n < 1_000_000 {
        let thousand = spell(n / 1000);
        let remainder = n % 1000;
        if remainder == 0 {
            return format!("{} thousand", thousand);
        } else {
            return format!("{} thousand {}", thousand, spell(remainder));
        }
    } else {
        let million = spell(n / 1_000_000);
        let remainder = n % 1_000_000;
        if remainder == 0 {
            return format!("{} million", million);
        } else {
            return format!("{} million {}", million, spell(remainder));
        }
    }
}
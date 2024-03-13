fn main() {
    let mut credit_card_number = "79927398713"; // チェックするクレジットカード番号

    if is_valid_luhn(&credit_card_number) {
        println!("有効なクレジットカード番号です");
    } else {
        println!("無効なクレジットカード番号です");
    }
}

/// クレジットカード番号をチェックするLuhnのアルゴリズム
fn is_valid_luhn(credit_card_number: &str) -> bool {
    let mut sum = 0;
    let mut double_next = false;

    for digit in credit_card_number.chars().rev() {
        if let Some(mut val) = digit.to_digit(10) {
            if double_next {
                val *= 2;
                if val > 9 {
                    val -= 9;
                }
            }
            sum += val;
            double_next = !double_next;
        } else {
            // 数字以外の文字が含まれていた場合
            return false;
        }
    }

    sum % 10 == 0
}

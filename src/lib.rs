use core::num::NonZeroU64;
use unicode_segmentation::UnicodeSegmentation;


static CHARACTER_TUPLES: phf::Map<u32, (&str, &str, &str)> = phf::phf_map! {
    0u32 => ("I", "V", "X"),
    1u32 => ("X", "L", "C"),
    2u32 => ("C", "D", "I̅"),
    3u32 => ("I̅", "V̅", "X̅"),
    4u32 => ("X̅", "L̅", "C̅"),
    5u32 => ("C̅", "D̅", "M̅"),
    6u32 => ("M̅", "V̿", "X̿"),
    7u32 => ("X̿", "L̿", "C̿"),
    8u32 => ("C̿", "D̿", "M̿"),
    9u32 => ("I̲̿", "V̲̿", "X̲̿"),
    10u32 => ("X̲̿", "L̲̿", "C̲̿"),
    11u32 => ("C̲̿", "D̲̿", "M̲̿"),
    12u32 => ("I̳̿", "V̳̿", "X̳̿"),
    13u32 => ("X̳̿", "L̳̿", "C̳̿"),
    14u32 => ("C̳̿", "D̳̿", "M̳̿"),
    15u32 => ("I⃒̳̿", "V⃒̳̿", "X⃒̳̿"),
    16u32 => ("X⃒̳̿", "L⃒̳̿", "C⃒̳̿"),
    17u32 => ("C⃒̳̿", "D⃒̳̿", "M⃒̳̿"),
    18u32 => ("I⃦̳̿", "V⃦̳̿", "X⃦̳̿"),
    19u32 => ("X⃦̳̿", "L⃦̳̿", "C⃦̳̿"),
    20u32 => ("C⃦̳̿", "D⃦̳̿", "M⃦̳̿"),
};

// TODO me no like redundancy, merge this into the other ^ map somehow and get rid of either.
static GLYPH2INT: phf::Map<&str, u64> = phf::phf_map! {
    // normal numerals
    "I" => 1, // 1 * 10_u64.pow(0)
    "V" => 5, // 5 * 10_u64.pow(0)
    "X" => 10, // 1 * 10_u64.pow(1)
    "L" => 50, // 5 * 10_u64.pow(1)
    "C" => 100, // 1 * 10_u64.pow(2)
    "D" => 500, // 5 * 10_u64.pow(2)
    "M" => 1000, // 1 * 10_u64.pow(3)
    // simple vinculum
    "I̅" => 1000, // 1 * 10_u64.pow(3)
    "V̅" => 5000, // 5 * 10_u64.pow(3)
    "X̅" => 10000, // 1 * 10_u64.pow(4)
    "L̅" => 50000, // 5 * 10_u64.pow(4)
    "C̅" => 100000, // 1 * 10_u64.pow(5)
    "D̅" => 500000, // 5 * 10_u64.pow(5)
    "M̅" => 1000000, // 1 * 10_u64.pow(6)
    // double vinculum
    "I̿" => 1000000, // 1 * 10_u64.pow(6)
    "V̿" => 5000000, // 5 * 10_u64.pow(6)
    "X̿" => 10000000, // 1 * 10_u64.pow(7)
    "L̿" => 50000000, // 5 * 10_u64.pow(7)
    "C̿" => 100000000, // 1 * 10_u64.pow(8)
    "D̿" => 500000000, // 5 * 10_u64.pow(8)
    "M̿" => 1000000000, // 1 * 10_u64.pow(9)
    // Double vinculum with single underline
    "I̲̿" => 1000000000, // 1 * 10_u64.pow(9)
    "V̲̿" => 5000000000, // 5 * 10_u64.pow(9)
    "X̲̿" => 10000000000, // 1 * 10_u64.pow(10)
    "L̲̿" => 50000000000, // 5 * 10_u64.pow(10)
    "C̲̿" => 100000000000, // 1 * 10_u64.pow(11)
    "D̲̿" => 500000000000, // 5 * 10_u64.pow(11)
    "M̲̿" => 1000000000000, // 1 * 10_u64.pow(12)
    // Double vinculum with double underline
    "I̳̿" => 1000000000000, // 1 * 10_u64.pow(12)
    "V̳̿" => 5000000000000, // 5 * 10_u64.pow(12)
    "X̳̿" => 10000000000000, // 1 * 10_u64.pow(13)
    "L̳̿" => 50000000000000, // 5 * 10_u64.pow(13)
    "C̳̿" => 100000000000000, // 1 * 10_u64.pow(14)
    "D̳̿" => 500000000000000, // 5 * 10_u64.pow(14)
    "M̳̿"  => 1000000000000000, // 1 * 10_u64.pow(15)
    // Double vinculum with double underline and vertical middle bar
    "I⃒̳̿"  => 1000000000000000, // 1 * 10_u64.pow(15)
    "V⃒̳̿" => 5000000000000000, // 5 * 10_u64.pow(15)
    "X⃒̳̿" => 10000000000000000, // 1 * 10_u64.pow(16)
    "L⃒̳̿" => 50000000000000000, // 5 * 10_u64.pow(16)
    "C⃒̳̿" => 100000000000000000, // 1 * 10_u64.pow(17)
    "D⃒̳̿" => 500000000000000000, // 5 * 10_u64.pow(17)
    "M⃒̳̿"  => 1000000000000000000, // 1 * 10_u64.pow(18)
    // Double vinculum with double underline and double vertical middle bar
    "I⃦̳̿"  => 1000000000000000000, // 1 * 10_u64.pow(18)
    "V⃦̳̿" => 5000000000000000000, // 5 * 10_u64.pow(18)
    "X⃦̳̿" => 10000000000000000000, // 1 * 10_u64.pow(19)
    "L⃦̳̿" => 50000000000000000000, // 5 * 10_u64.pow(19)
    "C⃦̳̿" => 100000000000000000000, // 1 * 10_u64.pow(20)
    "D⃦̳̿" => 500000000000000000000, // 5 * 10_u64.pow(20)
    "M⃦̳̿" => 1000000000000000000000, // 5 * 10_u64.pow(21)
};



/// Returns a roman numeral in vinculum syntax for a given arabic number
///
/// # Arguments
///
/// * `input` - The arabic number to convert into a roman one.
///
/// # Examples
///
/// ```
/// let result = vinculum::arabic2vinculum(4711);
/// ```
pub fn arabic2vinculum(input: NonZeroU64) -> String {
    let mut result = String::new();
    let mut arabic = input.get();

    // From 1_000_000_000 to 10 in steps of powers of ten:
    for n in (1..=19).rev() {
        let divisor: u64 = 10_u64.pow(n);
        let divided: u64 = arabic / divisor;
        if divided > 0 {
            let appendix = make_vinculum_number(n, divided).unwrap();
            result.push_str(&appendix);
            arabic -= divisor * divided;
        }
    }
    if arabic > 0 {
        // arabic is a single digit number at this point
        let rest = make_vinculum_number(0, arabic).unwrap();
        result.push_str(&rest);
    }
    result
}

/// Returns an arabic number for a roman numeral in vinculum syntax
///
/// # Arguments
///
/// * `input` - The String or &str holding the vinculum numeral
///
/// # Examples
///
/// ```
/// let result = vinculum::vinculum2arabic("I̅I̅I̅CI̅XCIX");
/// ```
pub fn vinculum2arabic<S: AsRef<str>>(input: S) -> Result<NonZeroU64, String> {
    let values = input.as_ref().graphemes(true).map(value)
        .collect::<Result<Vec<_>, _>>()?;

    let result = values.iter().scan(None, |state, &next| {
        let prev = state.replace(next).unwrap_or(next);
        if prev < next {
            // We already added the previous value, so we need to subtract twice.
            next.checked_sub(prev)?.checked_sub(prev)
        } else {
            Some(next)
        }
    }).sum();

    NonZeroU64::new(result).ok_or_else(|| {
        format!("Vinculum value results in zero: {}", input.as_ref())
    })
}

fn value(grapheme: &str) -> Result<u64, String> {

    match GLYPH2INT.get(grapheme) {
        Some(value) => Ok(*value),
        None => Err(format!("Unknown grapheme {}", grapheme)),
    }
}

fn make_vinculum_number(power_ten: u32, times: u64) -> Result<String, String> {
    make_vinculum(times, *CHARACTER_TUPLES.get(&power_ten).unwrap())
}

fn make_vinculum(times: u64, chars: (&str, &str, &str)) -> Result<String, String> {
    macro_rules! vinc {
        [$($index:tt)*] => {
            Ok([$(chars.$index),*].concat())
        }
    }

    match times {
        1 => vinc![0],
        2 => vinc![0 0],
        3 => vinc![0 0 0],
        4 => vinc![0 1],
        5 => vinc![1],
        6 => vinc![1 0],
        7 => vinc![1 0 0],
        8 => vinc![1 0 0 0],
        9 => vinc![0 2],
        _ => Err(format!("Unsupported number: {}", times)),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn nz_a2v(num: u64) -> String {
        arabic2vinculum(NonZeroU64::new(num).unwrap())
    }

    fn nz_v2a(vinculum: &str) -> u64 {
        vinculum2arabic(vinculum).unwrap().get()
    }

    #[test]
    fn test_arabic2vinculum_single_digit() {
        assert_eq!(nz_a2v(1), "I");
        assert_eq!(nz_a2v(2), "II");
        assert_eq!(nz_a2v(3), "III");
        assert_eq!(nz_a2v(4), "IV");
        assert_eq!(nz_a2v(5), "V");
        assert_eq!(nz_a2v(6), "VI");
        assert_eq!(nz_a2v(7), "VII");
        assert_eq!(nz_a2v(8), "VIII");
        assert_eq!(nz_a2v(9), "IX");
    }

    #[test]
    fn test_arabic2vinculum_double_digit() {
        assert_eq!(nz_a2v(10), "X");
        assert_eq!(nz_a2v(11), "XI");
        assert_eq!(nz_a2v(12), "XII");
        assert_eq!(nz_a2v(13), "XIII");
        assert_eq!(nz_a2v(14), "XIV");
        assert_eq!(nz_a2v(15), "XV");
        assert_eq!(nz_a2v(19), "XIX");
        assert_eq!(nz_a2v(20), "XX");
        assert_eq!(nz_a2v(29), "XXIX");
        assert_eq!(nz_a2v(39), "XXXIX");
        assert_eq!(nz_a2v(40), "XL");
        assert_eq!(nz_a2v(50), "L");
        assert_eq!(nz_a2v(60), "LX");
    }

    #[test]
    fn test_arabic2vinculum_triple_digit() {
        assert_eq!(nz_a2v(100), "C");
        assert_eq!(nz_a2v(160), "CLX");
        assert_eq!(nz_a2v(200), "CC");
        assert_eq!(nz_a2v(246), "CCXLVI");
        assert_eq!(nz_a2v(207), "CCVII");
        assert_eq!(nz_a2v(300), "CCC");
        assert_eq!(nz_a2v(400), "CD");
        assert_eq!(nz_a2v(500), "D");
        assert_eq!(nz_a2v(600), "DC");
        assert_eq!(nz_a2v(800), "DCCC");
        assert_eq!(nz_a2v(900), "CI̅");
        assert_eq!(nz_a2v(789), "DCCLXXXIX");
    }

    #[test]
    fn test_arabic2vinculum_quadruple_digit() {
        assert_eq!(nz_a2v(1000), "I̅");
        assert_eq!(nz_a2v(1009), "I̅IX");
        assert_eq!(nz_a2v(1066), "I̅LXVI");
        assert_eq!(nz_a2v(1776), "I̅DCCLXXVI");
        assert_eq!(nz_a2v(1918), "I̅CI̅XVIII");
        assert_eq!(nz_a2v(1954), "I̅CI̅LIV");
        assert_eq!(nz_a2v(2014), "I̅I̅XIV");
        assert_eq!(nz_a2v(2421), "I̅I̅CDXXI");
        assert_eq!(nz_a2v(3999), "I̅I̅I̅CI̅XCIX");
        assert_eq!(nz_a2v(4000), "I̅V̅");
        assert_eq!(nz_a2v(4627), "I̅V̅DCXXVII");
        assert_eq!(nz_a2v(5000), "V̅");
        assert_eq!(nz_a2v(5015), "V̅XV");
        assert_eq!(nz_a2v(6000), "V̅I̅");
    }

    #[test]
    fn test_arabic2vinculum_quintuple_digit() {
        assert_eq!(nz_a2v(10000), "X̅");
        assert_eq!(nz_a2v(18034), "X̅V̅I̅I̅I̅XXXIV");
        assert_eq!(nz_a2v(20000), "X̅X̅");
        assert_eq!(nz_a2v(25000), "X̅X̅V̅");
        assert_eq!(nz_a2v(25459), "X̅X̅V̅CDLIX");
        assert_eq!(nz_a2v(50000), "L̅");
    }

    #[test]
    fn test_arabic2vinculum_chonky_bois() {
        assert_eq!(nz_a2v(100000), "C̅");
        assert_eq!(nz_a2v(500000), "D̅");
        assert_eq!(nz_a2v(500001), "D̅I");
        assert_eq!(nz_a2v(1000000), "M̅");
        assert_eq!(nz_a2v(1000001), "M̅I");
        assert_eq!(nz_a2v(2000000), "M̅M̅");
        assert_eq!(nz_a2v(3000000), "M̅M̅M̅");
    }

    #[test]
    fn test_arabic2vinculum_double_vinculum() {
        assert_eq!(nz_a2v(5000000), "V̿");
        assert_eq!(nz_a2v(10000000), "X̿");
        assert_eq!(nz_a2v(50000000), "L̿");
        assert_eq!(nz_a2v(100000000), "C̿");
        assert_eq!(nz_a2v(500000000), "D̿");
        // assert_eq!(nz_a2v(1000000000), "M̿"); TODO come up with a rule on
        // when to use M or the ^I in the class above
    }

    #[test]
    fn test_arabic2vinculum_irregular_numbers() {
        // for numbers which aren't actually valid roman numbers,
        // not even by vinculum's standards LOL
        // TODO add test cases for really large numbers
        assert_eq!(
            nz_a2v(18446744073709551615),
            "X⃦̳̿V⃦̳̿I⃦̳̿I⃦̳̿I⃦̳̿C⃒̳̿D⃒̳̿X⃒̳̿L⃒̳̿V⃒̳̿I⃒̳̿D̳̿C̳̿C̳̿X̳̿L̳̿I̳̿V̳̿L̲̿X̲̿X̲̿I̲̿I̲̿I̲̿D̿C̿C̿M̅X̿D̅L̅I̅DCXV"
        );
    }

    #[test]
    fn test_vinculum2arabic_single_digit() {
        assert_eq!(nz_v2a("I"), 1);
        assert_eq!(nz_v2a("II"), 2);
        assert_eq!(nz_v2a("III"), 3);
        assert_eq!(nz_v2a("IV"), 4);
        assert_eq!(nz_v2a("V"), 5);
        assert_eq!(nz_v2a("VI"), 6);
        assert_eq!(nz_v2a("VII"), 7);
        assert_eq!(nz_v2a("VIII"), 8);
        assert_eq!(nz_v2a("IX"), 9);
    }

    #[test]
    fn test_vinculum2arabic_double_digit() {
        assert_eq!(nz_v2a("X"), 10);
        assert_eq!(nz_v2a("XI"), 11);
        assert_eq!(nz_v2a("XII"), 12);
        assert_eq!(nz_v2a("XIII"), 13);
        assert_eq!(nz_v2a("XIV"), 14);
        assert_eq!(nz_v2a("XV"), 15);
        assert_eq!(nz_v2a("XIX"), 19);
        assert_eq!(nz_v2a("XX"), 20);
        assert_eq!(nz_v2a("XXIX"), 29);
        assert_eq!(nz_v2a("XXXIX"), 39);
        assert_eq!(nz_v2a("XL"), 40);
        assert_eq!(nz_v2a("L"), 50);
        assert_eq!(nz_v2a("LX"), 60);
    }

    #[test]
    fn test_vinculum2arabic_triple_digit() {
        assert_eq!(nz_v2a("C"), 100);
        assert_eq!(nz_v2a("CLX"), 160);
        assert_eq!(nz_v2a("CC"), 200);
        assert_eq!(nz_v2a("CCXLVI"), 246);
        assert_eq!(nz_v2a("CCVII"), 207);
        assert_eq!(nz_v2a("CCC"), 300);
        assert_eq!(nz_v2a("CD"), 400);
        assert_eq!(nz_v2a("D"), 500);
        assert_eq!(nz_v2a("DC"), 600);
        assert_eq!(nz_v2a("DCCC"), 800);
        assert_eq!(nz_v2a("CI̅"), 900);
        assert_eq!(nz_v2a("DCCLXXXIX"), 789);
    }

    #[test]
    fn test_vinculum2arabic_quadruple_digit() {
        assert_eq!(nz_v2a("I̅"), 1000);
        assert_eq!(nz_v2a("I̅IX"), 1009);
        assert_eq!(nz_v2a("I̅LXVI"), 1066);
        assert_eq!(nz_v2a("I̅DCCLXXVI"), 1776);
        assert_eq!(nz_v2a("I̅CI̅XVIII"), 1918);
        assert_eq!(nz_v2a("I̅CI̅LIV"), 1954);
        assert_eq!(nz_v2a("I̅I̅XIV"), 2014);
        assert_eq!(nz_v2a("I̅I̅CDXXI"), 2421);
        assert_eq!(nz_v2a("I̅I̅I̅CI̅XCIX"), 3999);
        assert_eq!(nz_v2a("I̅V̅"), 4000);
        assert_eq!(nz_v2a("I̅V̅DCXXVII"), 4627);
        assert_eq!(nz_v2a("V̅"), 5000);
        assert_eq!(nz_v2a("V̅XV"), 5015);
        assert_eq!(nz_v2a("V̅I̅"), 6000);
    }

    #[test]
    fn test_vinculum2arabic_quintuple_digit() {
        assert_eq!(nz_v2a("X̅"), 10000);
        assert_eq!(nz_v2a("X̅V̅I̅I̅I̅XXXIV"), 18034);
        assert_eq!(nz_v2a("X̅X̅"), 20000);
        assert_eq!(nz_v2a("X̅X̅V̅"), 25000);
        assert_eq!(nz_v2a("X̅X̅V̅CDLIX"), 25459);
        assert_eq!(nz_v2a("L̅"), 50000);
    }

    #[test]
    fn test_vinculum2arabic_chonky_bois() {
        assert_eq!(nz_v2a("C̅"), 100000);
        assert_eq!(nz_v2a("D̅"), 500000);
        assert_eq!(nz_v2a("D̅I"), 500001);
        assert_eq!(nz_v2a("M̅"), 1000000);
        assert_eq!(nz_v2a("M̅I"), 1000001);
        assert_eq!(nz_v2a("M̅M̅"), 2000000);
        assert_eq!(nz_v2a("M̅M̅M̅"), 3000000);
    }
}

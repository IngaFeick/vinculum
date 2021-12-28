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
pub fn arabic2vinculum(input: u64) -> Result<String, String>  {

    if input == 0 {
        return Ok(String::new());
    }

    let mut result = String::new();
    let mut arabic = input.clone();

    // From 1_000_000_000 to 10 in steps of powers of ten:
    for n in (1..=9).rev() {
        let divisor: u64 = 10_u64.pow(n);
        let divided: u64 = arabic / divisor;
        if divided > 0 {
            let appendix = make_vinculum_number(&divisor, &divided).unwrap();
            result.push_str(&appendix);
            arabic -= divisor as u64 * &divided;
        }
    }
    if arabic > 0 {
        // arabic is a single digit number at this point
        let rest = make_vinculum_number(&1, &arabic).unwrap();
        result.push_str(&rest);
    }
    Ok(result)
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
pub fn vinculum2arabic<S: AsRef<str>>(input: S) -> Result<u64, String> {
    // input.as_ref()
    Err("TODO implement".to_string())
}


fn make_vinculum_number(divisor: &u32, times: &u64) -> Result<String, String>
{
    match divisor {
        1000000000 => make_vinculum_billion(times),
        100000000 => make_vinculum(times, "C̿", "D̿", "M̿"),
        10000000 => make_vinculum(times, "X̿", "L̿", "C̿"),
        1000000 => make_vinculum(times, "M̅", "V̿", "X̿"),
        100000 => make_vinculum(times, "C̅", "D̅", "M̅"),
        10000 => make_vinculum(times, "X̅", "L̅", "C̅"),
        1000 => make_vinculum(times, "I̅", "V̅", "X̅"),
        100 => make_vinculum(times, "C", "D", "I̅"),
        10 => make_vinculum(times, "X", "L", "C"),
        1 => make_vinculum(times, "I", "V", "X"),
        _ => {
            Err(format!("Unsupported divisor: {}", times))
        }
    }
}

/*
This function is separate from the smaller powers of ten because
it is only partially compliant with the rules for roman numerals.
The largest supported number is 3.999.999 but we'll allow bigger numbers here.
*/
fn make_vinculum_billion(times: &u64) -> Result<String, String> {
    let mut result = String::new();
    for _ in 0..*times {
        result.push_str("M̿");
    }
    Ok(result)
}

fn make_vinculum(times: &u64, char1: &str, char5: &str, char10: &str) -> Result<String, String> {
    match times {
        1 => Ok(char1.to_string()),
        2 => {
            let mut result = String::from(char1.to_string());
            result.push_str(char1);
            Ok(result)
            },
        3 => {
            let mut result = String::from(char1.to_string());
            result.push_str(char1);
            result.push_str(char1);
            Ok(result)
            },
        4 => {
            let mut result = String::from(char1.to_string());
            result.push_str(char5);
            Ok(result)
            },
        5 => Ok(char5.to_string()),
        6 => { let mut result = String::from(char5.to_string());
            result.push_str(char1);
            Ok(result)
            },
        7 => { let mut result = String::from(char5.to_string());
            result.push_str(char1);
            result.push_str(char1);
            Ok(result)
            },
        8 => { let mut result = String::from(char5.to_string());
            result.push_str(char1);
            result.push_str(char1);
            result.push_str(char1);
            Ok(result)
            },
        9 => { let mut result = String::from(char1.to_string());
            result.push_str(char10);
            Ok(result)
            },
        _ => {
            let mut warning = String::from("Unsupported number: ");
            warning.push_str(&times.to_string());
            Err(warning)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_arabic2vinculum_single_digit() {

        assert_eq!(arabic2vinculum(0).unwrap(), "");
        assert_eq!(arabic2vinculum(1).unwrap(), "I");
        assert_eq!(arabic2vinculum(2).unwrap(), "II");
        assert_eq!(arabic2vinculum(3).unwrap(), "III");
        assert_eq!(arabic2vinculum(4).unwrap(), "IV");
        assert_eq!(arabic2vinculum(5).unwrap(), "V");
        assert_eq!(arabic2vinculum(6).unwrap(), "VI");
        assert_eq!(arabic2vinculum(7).unwrap(), "VII");
        assert_eq!(arabic2vinculum(8).unwrap(), "VIII");
        assert_eq!(arabic2vinculum(9).unwrap(), "IX");

    }

    #[test]
    fn test_arabic2vinculum_double_digit() {
        assert_eq!(arabic2vinculum(10).unwrap(), "X");
        assert_eq!(arabic2vinculum(11).unwrap(), "XI");
        assert_eq!(arabic2vinculum(12).unwrap(), "XII");
        assert_eq!(arabic2vinculum(13).unwrap(), "XIII");
        assert_eq!(arabic2vinculum(14).unwrap(), "XIV");
        assert_eq!(arabic2vinculum(15).unwrap(), "XV");
        assert_eq!(arabic2vinculum(19).unwrap(), "XIX");
        assert_eq!(arabic2vinculum(20).unwrap(), "XX");
        assert_eq!(arabic2vinculum(29).unwrap(), "XXIX");
        assert_eq!(arabic2vinculum(39).unwrap(), "XXXIX");
        assert_eq!(arabic2vinculum(40).unwrap(), "XL");
        assert_eq!(arabic2vinculum(50).unwrap(), "L");
        assert_eq!(arabic2vinculum(60).unwrap(), "LX");

    }

    #[test]
    fn test_arabic2vinculum_triple_digit() {
        assert_eq!(arabic2vinculum(100).unwrap(), "C");
        assert_eq!(arabic2vinculum(160).unwrap(), "CLX");
        assert_eq!(arabic2vinculum(200).unwrap(), "CC");
        assert_eq!(arabic2vinculum(246).unwrap(), "CCXLVI");
        assert_eq!(arabic2vinculum(207).unwrap(), "CCVII");
        assert_eq!(arabic2vinculum(300).unwrap(), "CCC");
        assert_eq!(arabic2vinculum(400).unwrap(), "CD");
        assert_eq!(arabic2vinculum(500).unwrap(), "D");
        assert_eq!(arabic2vinculum(600).unwrap(), "DC");
        assert_eq!(arabic2vinculum(800).unwrap(), "DCCC");
        assert_eq!(arabic2vinculum(900).unwrap(), "CI̅");
        assert_eq!(arabic2vinculum(789).unwrap(), "DCCLXXXIX");
    }

    #[test]
    fn test_arabic2vinculum_quadruple_digit() {
        assert_eq!(arabic2vinculum(1000).unwrap(), "I̅");
        assert_eq!(arabic2vinculum(1009).unwrap(), "I̅IX");
        assert_eq!(arabic2vinculum(1066).unwrap(), "I̅LXVI");
        assert_eq!(arabic2vinculum(1776).unwrap(), "I̅DCCLXXVI");
        assert_eq!(arabic2vinculum(1918).unwrap(), "I̅CI̅XVIII");
        assert_eq!(arabic2vinculum(1954).unwrap(), "I̅CI̅LIV");
        assert_eq!(arabic2vinculum(2014).unwrap(), "I̅I̅XIV");
        assert_eq!(arabic2vinculum(2421).unwrap(), "I̅I̅CDXXI");
        assert_eq!(arabic2vinculum(3999).unwrap(), "I̅I̅I̅CI̅XCIX");
        assert_eq!(arabic2vinculum(4000).unwrap(), "I̅V̅");
        assert_eq!(arabic2vinculum(4627).unwrap(), "I̅V̅DCXXVII");
        assert_eq!(arabic2vinculum(5000).unwrap(), "V̅");
        assert_eq!(arabic2vinculum(5015).unwrap(), "V̅XV");
        assert_eq!(arabic2vinculum(6000).unwrap(), "V̅I̅");
    }

    #[test]
    fn test_arabic2vinculum_quintuple_digit() {
        assert_eq!(arabic2vinculum(10000).unwrap(), "X̅");
        assert_eq!(arabic2vinculum(18034).unwrap(), "X̅V̅I̅I̅I̅XXXIV");
        assert_eq!(arabic2vinculum(20000).unwrap(), "X̅X̅");
        assert_eq!(arabic2vinculum(25000).unwrap(), "X̅X̅V̅");
        assert_eq!(arabic2vinculum(25459).unwrap(), "X̅X̅V̅CDLIX");
        assert_eq!(arabic2vinculum(50000).unwrap(), "L̅");
    }

    #[test]
    fn test_arabic2vinculum_chonky_bois() {
        assert_eq!(arabic2vinculum(100000).unwrap(), "C̅");
        assert_eq!(arabic2vinculum(500000).unwrap(), "D̅");
        assert_eq!(arabic2vinculum(500001).unwrap(), "D̅I");
        assert_eq!(arabic2vinculum(1000000).unwrap(), "M̅");
        assert_eq!(arabic2vinculum(1000001).unwrap(), "M̅I");
        assert_eq!(arabic2vinculum(2000000).unwrap(), "M̅M̅");
        assert_eq!(arabic2vinculum(3000000).unwrap(), "M̅M̅M̅");
    }

    #[test]
    fn test_arabic2vinculum_double_vinculum() {
        assert_eq!(arabic2vinculum(5000000).unwrap(), "V̿");
        assert_eq!(arabic2vinculum(10000000).unwrap(), "X̿");
        assert_eq!(arabic2vinculum(50000000).unwrap(), "L̿");
        assert_eq!(arabic2vinculum(100000000).unwrap(), "C̿");
        assert_eq!(arabic2vinculum(500000000).unwrap(), "D̿");
        assert_eq!(arabic2vinculum(1000000000).unwrap(), "M̿");
    }

    #[test]
    fn test_arabic2vinculum_irregular_numbers() {
        // for numbers which aren't actually valid roman numbers,
        // not even by vinculum's standards LOL
        assert_eq!(arabic2vinculum(4000000000).unwrap(), "M̿M̿M̿M̿");
    }

    // # TODO
    // #[test]
    // fn test_vinculum2arabic_single_digit() {
    //     assert_eq!(vinculum2arabic("I").unwrap(), 1);
    //     assert_eq!(vinculum2arabic("II").unwrap(), 2);
    //     assert_eq!(vinculum2arabic("III").unwrap(), 3);
    //     assert_eq!(vinculum2arabic("IV").unwrap(), 4);
    //     assert_eq!(vinculum2arabic("V").unwrap(), 5);
    //     assert_eq!(vinculum2arabic("VI").unwrap(), 6);
    //     assert_eq!(vinculum2arabic("VII").unwrap(), 7);
    //     assert_eq!(vinculum2arabic("VIII").unwrap(), 8);
    //     assert_eq!(vinculum2arabic("IX").unwrap(), 9);

    // }

    // #[test]
    // fn test_vinculum2arabic_double_digit() {
    //     assert_eq!(vinculum2arabic("X").unwrap(), 10);
    //     assert_eq!(vinculum2arabic("XI").unwrap(), 11);
    //     assert_eq!(vinculum2arabic("XII").unwrap(), 12);
    //     assert_eq!(vinculum2arabic("XIII").unwrap(), 13);
    //     assert_eq!(vinculum2arabic("XIV").unwrap(), 14);
    //     assert_eq!(vinculum2arabic("XV").unwrap(), 15);
    //     assert_eq!(vinculum2arabic("XIX").unwrap(), 19);
    //     assert_eq!(vinculum2arabic("XX").unwrap(), 20);
    //     assert_eq!(vinculum2arabic("XXIX").unwrap(), 29);
    //     assert_eq!(vinculum2arabic("XXXIX").unwrap(), 39);
    //     assert_eq!(vinculum2arabic("XL").unwrap(), 40);
    //     assert_eq!(vinculum2arabic("L").unwrap(), 50);
    //     assert_eq!(vinculum2arabic("LX").unwrap(), 60);

    // }

    // #[test]
    // fn test_vinculum2arabic_triple_digit() {
    //     assert_eq!(vinculum2arabic("C").unwrap(), 100);
    //     assert_eq!(vinculum2arabic("CLX").unwrap(), 160);
    //     assert_eq!(vinculum2arabic("CC").unwrap(), 200);
    //     assert_eq!(vinculum2arabic("CCXLVI").unwrap(), 246);
    //     assert_eq!(vinculum2arabic("CCVII").unwrap(), 207);
    //     assert_eq!(vinculum2arabic("CCC").unwrap(), 300);
    //     assert_eq!(vinculum2arabic("CD").unwrap(), 400);
    //     assert_eq!(vinculum2arabic("D").unwrap(), 500);
    //     assert_eq!(vinculum2arabic("DC").unwrap(), 600);
    //     assert_eq!(vinculum2arabic("DCCC").unwrap(), 800);
    //     assert_eq!(vinculum2arabic("CI̅").unwrap(), 900);
    //     assert_eq!(vinculum2arabic("DCCLXXXIX").unwrap(), 789);
    // }

    // #[test]
    // fn test_vinculum2arabic_quadruple_digit() {
    //     assert_eq!(vinculum2arabic("I̅").unwrap(), 1000);
    //     assert_eq!(vinculum2arabic("I̅IX").unwrap(), 1009);
    //     assert_eq!(vinculum2arabic("I̅LXVI").unwrap(), 1066);
    //     assert_eq!(vinculum2arabic("I̅DCCLXXVI").unwrap(), 1776);
    //     assert_eq!(vinculum2arabic("I̅CI̅XVIII").unwrap(), 1918);
    //     assert_eq!(vinculum2arabic("I̅CI̅LIV").unwrap(), 1954);
    //     assert_eq!(vinculum2arabic("I̅I̅XIV").unwrap(), 2014);
    //     assert_eq!(vinculum2arabic("I̅I̅CDXXI").unwrap(), 2421);
    //     assert_eq!(vinculum2arabic("I̅I̅I̅CI̅XCIX").unwrap(), 3999);
    //     assert_eq!(vinculum2arabic("I̅V̅").unwrap(), 4000);
    //     assert_eq!(vinculum2arabic("I̅V̅DCXXVII").unwrap(), 4627);
    //     assert_eq!(vinculum2arabic("V̅").unwrap(), 5000);
    //     assert_eq!(vinculum2arabic("V̅XV").unwrap(), 5015);
    //     assert_eq!(vinculum2arabic("V̅I̅").unwrap(), 6000);
    // }

    // #[test]
    // fn test_vinculum2arabic_quintuple_digit() {
    //     assert_eq!(vinculum2arabic("X̅").unwrap(), 10000);
    //     assert_eq!(vinculum2arabic("X̅V̅I̅I̅I̅XXXIV").unwrap(), 18034);
    //     assert_eq!(vinculum2arabic("X̅X̅").unwrap(), 20000);
    //     assert_eq!(vinculum2arabic("X̅X̅V̅").unwrap(), 25000);
    //     assert_eq!(vinculum2arabic("X̅X̅V̅CDLIX").unwrap(), 25459);
    //     assert_eq!(vinculum2arabic("L̅").unwrap(), 50000);
    // }

    // #[test]
    // fn test_vinculum2arabic_chonky_bois() {
    //     assert_eq!(vinculum2arabic("C̅").unwrap(), 100000);
    //     assert_eq!(vinculum2arabic("D̅").unwrap(), 500000);
    //     assert_eq!(vinculum2arabic("D̅I").unwrap(), 500001);
    //     assert_eq!(vinculum2arabic("M̅").unwrap(), 1000000);
    //     assert_eq!(vinculum2arabic("M̅I").unwrap(), 1000001);
    //     assert_eq!(vinculum2arabic("M̅M̅").unwrap(), 2000000);
    //     assert_eq!(vinculum2arabic("M̅M̅M̅").unwrap(), 3000000);
    // }

}
pub const FULL: &str = "█";
pub const SEVEN_EIGHTHS: &str = "▇";
pub const THREE_QUARTERS: &str = "▆";
pub const FIVE_EIGHTHS: &str = "▅";
pub const HALF: &str = "▄";
pub const THREE_EIGHTHS: &str = "▃";
pub const ONE_QUARTER: &str = "▂";
pub const ONE_EIGHTH: &str = "▁";

pub const UPPER_ONE_EIGHTH: &str = "▔";
pub const UPPER_ONE_QUARTER: &str = "🮂";
pub const UPPER_THREE_EIGHTHS: &str = "🮃";
pub const UPPER_HALF: &str = "▀";
pub const UPPER_FIVE_EIGHTHS: &str = "🮄";
pub const UPPER_THREE_QUARTERS: &str = "🮅";
pub const UPPER_SEVEN_EIGHTHS: &str = "🮆";

pub const HORIZONTAL_SEVEN_EIGHTHS: &str = "▉";
pub const HORIZONTAL_THREE_QUARTERS: &str = "▊";
pub const HORIZONTAL_FIVE_EIGHTHS: &str = "▋";
pub const HORIZONTAL_HALF: &str = "▌";
pub const HORIZONTAL_THREE_EIGHTHS: &str = "▍";
pub const HORIZONTAL_ONE_QUARTER: &str = "▎";
pub const HORIZONTAL_ONE_EIGHTH: &str = "▏";

pub const HORIZONTAL_RIGHT_ONE_EIGHTH: &str = "▕";
pub const HORIZONTAL_RIGHT_ONE_QUARTER: &str = "🮇";
pub const HORIZONTAL_RIGHT_THREE_EIGHTHS: &str = "🮈";
pub const HORIZONTAL_RIGHT_HALF: &str = "▐";
pub const HORIZONTAL_RIGHT_FIVE_EIGHTHS: &str = "🮉";
pub const HORIZONTAL_RIGHT_THREE_QUARTERS: &str = "🮊";
pub const HORIZONTAL_RIGHT_SEVEN_EIGHTHS: &str = "🮋";

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Set<'a> {
    pub full: &'a str,
    pub seven_eighths: &'a str,
    pub three_quarters: &'a str,
    pub five_eighths: &'a str,
    pub half: &'a str,
    pub three_eighths: &'a str,
    pub one_quarter: &'a str,
    pub one_eighth: &'a str,
    pub empty: &'a str,
}

impl Default for Set<'_> {
    fn default() -> Self {
        NINE_LEVELS
    }
}

#[allow(dead_code)]
pub const THREE_LEVELS: Set = Set {
    full: FULL,
    seven_eighths: FULL,
    three_quarters: HALF,
    five_eighths: HALF,
    half: HALF,
    three_eighths: HALF,
    one_quarter: HALF,
    one_eighth: " ",
    empty: " ",
};

pub const NINE_LEVELS: Set = Set {
    full: FULL,
    seven_eighths: SEVEN_EIGHTHS,
    three_quarters: THREE_QUARTERS,
    five_eighths: FIVE_EIGHTHS,
    half: HALF,
    three_eighths: THREE_EIGHTHS,
    one_quarter: ONE_QUARTER,
    one_eighth: ONE_EIGHTH,
    empty: " ",
};

pub const UPPER_NINE_LEVELS: Set = Set {
    full: FULL,
    seven_eighths: UPPER_SEVEN_EIGHTHS,
    three_quarters: UPPER_THREE_QUARTERS,
    five_eighths: UPPER_FIVE_EIGHTHS,
    half: UPPER_HALF,
    three_eighths: UPPER_THREE_EIGHTHS,
    one_quarter: UPPER_ONE_QUARTER,
    one_eighth: UPPER_ONE_EIGHTH,
    empty: " ",
};

pub const HORIZONTAL_NINE_LEVELS: Set = Set {
    full: FULL,
    seven_eighths: HORIZONTAL_SEVEN_EIGHTHS,
    three_quarters: HORIZONTAL_THREE_QUARTERS,
    five_eighths: HORIZONTAL_FIVE_EIGHTHS,
    half: HORIZONTAL_HALF,
    three_eighths: HORIZONTAL_THREE_EIGHTHS,
    one_quarter: HORIZONTAL_ONE_QUARTER,
    one_eighth: HORIZONTAL_ONE_EIGHTH,
    empty: " ",
};

pub const HORIZONTAL_RIGHT_NINE_LEVELS: Set = Set {
    full: FULL,
    seven_eighths: HORIZONTAL_RIGHT_SEVEN_EIGHTHS,
    three_quarters: HORIZONTAL_RIGHT_THREE_QUARTERS,
    five_eighths: HORIZONTAL_RIGHT_FIVE_EIGHTHS,
    half: HORIZONTAL_RIGHT_HALF,
    three_eighths: HORIZONTAL_RIGHT_THREE_EIGHTHS,
    one_quarter: HORIZONTAL_RIGHT_ONE_QUARTER,
    one_eighth: HORIZONTAL_RIGHT_ONE_EIGHTH,
    empty: " ",
};

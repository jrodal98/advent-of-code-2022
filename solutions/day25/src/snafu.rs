use std::{iter::Sum, ops::Add, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq)]
enum SnafuSymbol {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

impl From<SnafuSymbol> for isize {
    fn from(value: SnafuSymbol) -> Self {
        Self::from(&value)
    }
}

impl From<&SnafuSymbol> for isize {
    fn from(value: &SnafuSymbol) -> Self {
        match value {
            &SnafuSymbol::Two => 2,
            &SnafuSymbol::One => 1,
            &SnafuSymbol::Zero => 0,
            &SnafuSymbol::Minus => -1,
            &SnafuSymbol::DoubleMinus => -2,
        }
    }
}

impl From<char> for SnafuSymbol {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '1' => Self::One,
            '0' => Self::Zero,
            '-' => Self::Minus,
            '=' => Self::DoubleMinus,
            _ => unreachable!(),
        }
    }
}

impl From<SnafuSymbol> for char {
    fn from(value: SnafuSymbol) -> Self {
        Self::from(&value)
    }
}

impl From<&SnafuSymbol> for char {
    fn from(value: &SnafuSymbol) -> Self {
        match value {
            SnafuSymbol::Two => '2',
            SnafuSymbol::One => '1',
            SnafuSymbol::Zero => '0',
            SnafuSymbol::Minus => '-',
            SnafuSymbol::DoubleMinus => '=',
        }
    }
}

pub struct Snafu {
    /// most significant appears first
    symbols: Vec<SnafuSymbol>,
}

impl Add for Snafu {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(isize::from(self) + isize::from(rhs))
    }
}

impl Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, s| acc + s).unwrap()
    }
}

impl FromStr for Snafu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            symbols: s.chars().into_iter().map(|c| c.into()).collect(),
        })
    }
}

impl ToString for Snafu {
    fn to_string(&self) -> String {
        self.symbols.iter().map(|s| char::from(s)).collect()
    }
}

impl From<Snafu> for isize {
    fn from(value: Snafu) -> Self {
        value
            .symbols
            .iter()
            .rev()
            .enumerate()
            .map(|(i, s)| 5_isize.pow(i as u32) * isize::from(s))
            .sum()
    }
}

impl From<isize> for Snafu {
    fn from(value: isize) -> Self {
        let mut value_remaining = value;
        let mut symbols: Vec<SnafuSymbol> = Vec::new();

        while value_remaining > 0 {
            let r = value_remaining % 5;
            value_remaining = value_remaining / 5;
            let s = match r {
                0 => SnafuSymbol::Zero,
                1 => SnafuSymbol::One,
                2 => SnafuSymbol::Two,
                3 => {
                    value_remaining += 1;
                    SnafuSymbol::DoubleMinus
                }
                4 => {
                    value_remaining += 1;
                    SnafuSymbol::Minus
                }
                _ => unreachable!(),
            };
            symbols.push(s)
        }

        symbols.reverse();

        Self { symbols }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! conversion_test {
        ($snafu_str:expr, $dec:expr, $to_dec_name:ident, $to_snafu_name: ident) => {
            #[test]
            fn $to_dec_name() {
                let snafu: Snafu = $snafu_str.parse().unwrap();
                assert_eq!($dec, isize::from(snafu));
            }

            #[test]
            fn $to_snafu_name() {
                let snafu: Snafu = $dec.into();
                assert_eq!($snafu_str, &snafu.to_string());
            }
        };
    }

    conversion_test! {"1=-0-2", 1747, snafu_to_dec_1, dec_to_snafu_1}
    conversion_test! {"12111", 906, snafu_to_dec_2, dec_to_snafu_2}
    conversion_test! {"2=0=", 198, snafu_to_dec_3, dec_to_snafu_3}
    conversion_test! {"21", 11, snafu_to_dec_4, dec_to_snafu_4}
    conversion_test! {"2=01", 201, snafu_to_dec_5, dec_to_snafu_5}
    conversion_test! {"111", 31, snafu_to_dec_6, dec_to_snafu_6}
    conversion_test! {"20012", 1257, snafu_to_dec_7, dec_to_snafu_7}
    conversion_test! {"112", 32, snafu_to_dec_8, dec_to_snafu_8}
    conversion_test! {"1=-1=", 353, snafu_to_dec_9, dec_to_snafu_9}
    conversion_test! {"1-12", 107, snafu_to_dec_10, dec_to_snafu_10}
    conversion_test! {"12", 7, snafu_to_dec_11, dec_to_snafu_11}
    conversion_test! {"1=", 3, snafu_to_dec_12, dec_to_snafu_12}
    conversion_test! {"122", 37, snafu_to_dec_13, dec_to_snafu_13}
    conversion_test! {"1", 1, snafu_to_dec_14, dec_to_snafu_14}
    conversion_test! {"2", 2, snafu_to_dec_15, dec_to_snafu_15}
    conversion_test! {"1=", 3, snafu_to_dec_16, dec_to_snafu_16}
    conversion_test! {"1-", 4, snafu_to_dec_17, dec_to_snafu_17}
    conversion_test! {"10", 5, snafu_to_dec_18, dec_to_snafu_18}
    conversion_test! {"11", 6, snafu_to_dec_19, dec_to_snafu_19}
    conversion_test! {"12", 7, snafu_to_dec_20, dec_to_snafu_20}
    conversion_test! {"2=", 8, snafu_to_dec_21, dec_to_snafu_21}
    conversion_test! {"2-", 9, snafu_to_dec_22, dec_to_snafu_22}
    conversion_test! {"20", 10, snafu_to_dec_23, dec_to_snafu_23}
    conversion_test! {"1=0", 15, snafu_to_dec_24, dec_to_snafu_24}
    conversion_test! {"1-0", 20, snafu_to_dec_25, dec_to_snafu_25}
    conversion_test! {"1=11-2", 2022, snafu_to_dec_26, dec_to_snafu_26}
    conversion_test! {"1-0---0", 12345, snafu_to_dec_27, dec_to_snafu_27}
    conversion_test! {"1121-1110-1=0", 314159265, snafu_to_dec_28, dec_to_snafu_28}
}

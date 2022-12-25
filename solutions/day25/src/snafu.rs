use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SnafuSymbol {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

pub struct Snafu {
    /// most significant appears first
    symbols: Vec<SnafuSymbol>,
}

impl FromStr for Snafu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            symbols: s.chars().into_iter().map(|c| c.into()).collect(),
        })
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

// impl From<isize> for Snafu {
//     fn from(value: isize) -> Self {
//         let mut num_digits =
//         todo!()
//     }
// }

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
        match value {
            SnafuSymbol::Two => '2',
            SnafuSymbol::One => '1',
            SnafuSymbol::Zero => '0',
            SnafuSymbol::Minus => '-',
            SnafuSymbol::DoubleMinus => '=',
        }
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
                assert_eq!(isize::from(snafu), $dec);
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
}


use std::error::Error;
/// This code derived from
/// https://github.com/BurntSushi/xsv/blob/master/src/config.rs
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct Delimiter(pub u8);

impl Delimiter {
    pub fn as_byte(&self) -> u8 {
        self.0
    }

    pub fn as_string(&self) -> String {
        (self.0 as char).to_string()
    }
}

impl fmt::Display for Delimiter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

impl FromStr for Delimiter {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Delimiter, Self::Err> {
        match s {
            r"\t" => Ok(Delimiter(b'\t')),
            s => {
                if s.len() != 1 {
                    return Err(
                        format!("Could not convert '{}' to a single ASCII character.", s).into(),
                    );
                }
                let c = s.chars().next().unwrap();
                if c.is_ascii() {
                    Ok(Delimiter(c as u8))
                } else {
                    Err(format!("Could not convert '{}' to ASCII delimiter.", s).into())
                }
            }
        }
    }
}

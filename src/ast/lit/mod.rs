mod lit_bool;
mod lit_char;
mod lit_number;
mod lit_string;

pub use lit_bool::LitBool;
pub use lit_char::LitChar;
pub use lit_number::LitNumber;
pub use lit_string::LitString;

pub enum Lit<'input> {
    LitBool(LitBool),
    LitChar(LitChar<'input>),
    LitNumber(LitNumber<'input>),
    LitString(LitString<'input>),
}

impl<'input> Lit<'input> {
    pub fn number(text: &'input str, span: (usize, usize)) -> Self {
        Self::LitNumber(LitNumber::new(text, span))
    }
    pub fn char(text: &'input str, span: (usize, usize)) -> Self {
        Self::LitChar(LitChar::new(text, span))
    }
    pub fn string(text: &'input str, span: (usize, usize)) -> Self {
        Self::LitString(LitString::new(text, span))
    }
}

impl Lit<'static> {
    pub fn bool(value: bool, span: (usize, usize)) -> Self {
        Self::LitBool(LitBool::new(value, span))
    }
}

impl core::fmt::Debug for Lit<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::LitBool(v) => write!(f, "{:?}", v),
            Self::LitChar(v) => write!(f, "{:?}", v),
            Self::LitNumber(v) => write!(f, "{:?}", v),
            Self::LitString(v) => write!(f, "{:?}", v),
        }
    }
}

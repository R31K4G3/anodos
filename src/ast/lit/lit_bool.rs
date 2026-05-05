pub struct LitBool {
    pub value: bool,
}

impl LitBool {
    #[inline(always)]
    pub fn new(value: bool, _span: (usize, usize)) -> Self {
        Self { value }
    }
}

impl core::fmt::Debug for LitBool {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.value)
    }
}

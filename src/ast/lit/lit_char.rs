pub struct LitChar<'input> {
    pub text: &'input str,
    pub span: (usize, usize),
}

impl<'input> LitChar<'input> {
    #[inline(always)]
    pub fn new(text: &'input str, span: (usize, usize)) -> Self {
        Self { text, span }
    }
}

impl core::fmt::Debug for LitChar<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.text.get(self.span.0..self.span.1).unwrap_or("???"))
    }
}

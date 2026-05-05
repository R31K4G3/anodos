use super::Expr;

pub struct ExprArray<'input> {
    pub elements: Vec<Expr<'input>>,
}

impl core::fmt::Debug for ExprArray<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.elements.len() >= 1 {
            f.write_str("[")?;
            write!(f, "{:?}", self.elements[0])?;
            for i in &self.elements[1..] {
                write!(f, ", {:?}", i)?;
            }
            f.write_str("]")?;
        } else {
            f.write_str("[]")?;
        }
        Ok(())
    }
}

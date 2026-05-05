use super::Stmt;

#[repr(transparent)]
pub struct StmtBlock<'input> {
    pub items: Vec<Stmt<'input>>,
}

impl core::fmt::Debug for StmtBlock<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("{")?;
        for i in &self.items {
            write!(f, "{:?}", i)?;
        }
        f.write_str("}")?;
        Ok(())
    }
}

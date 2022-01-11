pub trait PrincipalT<P = Self>: Send + Sync {
    fn into_principal(self) -> P;
    fn get_principal(&self) -> &P;
    fn get_principal_mut(&mut self) -> &mut P;
}
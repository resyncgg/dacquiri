pub trait PrincipalT<P = Self>: Clone + Send {
    fn into_principal(self) -> P;
    fn get_principal(&self) -> &P;
    fn get_principal_mut(&mut self) -> &mut P;
}
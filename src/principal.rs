pub trait PrincipalT<P = Self> {
    fn into_principal(self) -> P;
    fn get_principal(&self) -> &P;
    fn get_principal_mut(&mut self) -> &mut P;
}

#[macro_export]
macro_rules! impl_principal {
    ($principal:ty) => {
        impl PrincipalT for $principal {
            fn into_principal(self) -> Self { self }
            fn get_principal(&self) -> &Self { self }
            fn get_principal_mut(&mut self) -> &mut Self { self }
        }
    }
}
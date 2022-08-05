use syn::parse::{Parse, ParseStream};
use super::{
    Constraint,
    DependentPolicy
};

pub enum Clause {
    Constraint(Constraint),
    Policy(DependentPolicy)
}

impl Parse for Clause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // todo: probably need to peek this stream since parse likely consumes part of the stream
        if let Ok(constraint) = input.parse::<Constraint>() {
            return Ok(Clause::Constraint(constraint));
        }

        let policy = input.parse::<DependentPolicy>()?;
        Ok(Clause::Policy(policy))
    }
}

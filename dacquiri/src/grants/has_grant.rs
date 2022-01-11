use crate::DEFAULT_GRANT_TAG;
use crate::grants::grant::BaseGrant;
use crate::principal::PrincipalT;

pub trait HasGrant<T, const ID: &'static str = DEFAULT_GRANT_TAG>: PrincipalT<T::Principal>
    where
        T: BaseGrant<ID>
{
    fn get_resource(&self) -> &T::Resource;
}
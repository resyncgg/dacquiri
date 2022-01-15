use crate::DEFAULT_ATTRIBUTE_TAG;
use crate::attributes::attribute::BaseAttribute;
use crate::subject::SubjectT;

pub trait HasAttribute<T, const ID: &'static str = DEFAULT_ATTRIBUTE_TAG>: SubjectT<T::Subject>
    where
        T: BaseAttribute<ID>
{
    fn _get_attribute_resource(&self) -> &T::Resource;
}
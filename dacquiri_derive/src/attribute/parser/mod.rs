pub mod attribute_fn;

// The name of the context lifetime.
// This should match the context lifetime defined on BaseAttribute
pub(crate) const CONTEXT_LIFETIME: &str = "'ctx";
pub(crate) const IGNORED_ARGUMENT_NAME: &str = "_";
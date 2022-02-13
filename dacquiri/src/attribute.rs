use async_trait::async_trait;

pub type AttributeResult<E> = Result<(), E>;

pub trait BaseAttribute: Send + Sync {
    type Subject = ();
    type Resource = ();
    type Context<'ctx> = ();
    type Error;
}

pub trait SyncAttribute: BaseAttribute {
    fn test<'ctx>(
        subject: &Self::Subject,
        resource: &Self::Resource,
        context: Self::Context<'ctx>
    ) -> AttributeResult<Self::Error>;
}

#[async_trait]
pub trait AsyncAttribute: BaseAttribute {
    async fn test_async<'ctx>(
        subject: &Self::Subject,
        resource: &Self::Resource,
        context: Self::Context<'ctx>
    ) -> AttributeResult<Self::Error>;
}
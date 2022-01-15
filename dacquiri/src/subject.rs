pub trait SubjectT<P = Self>: Send + Sync {
    fn into_subject(self) -> P;
    fn get_subject(&self) -> &P;
    fn get_subject_mut(&mut self) -> &mut P;
}
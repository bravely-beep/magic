use parsel::syn::punctuated::Punctuated;

pub trait PunctuatedExt<T, P> {
    fn with_element(self, segment: T) -> Self
    where
        P: Default;
}

impl<T, P> PunctuatedExt<T, P> for Punctuated<T, P> {
    fn with_element(mut self, element: T) -> Self
    where
        P: Default,
    {
        self.push(element);
        self
    }
}

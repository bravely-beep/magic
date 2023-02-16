use parsel::{syn::Lifetime, Span};

pub trait LifetimeExt {
    fn call_site_span<T: AsRef<str>>(symbol: T) -> Self;
}

impl LifetimeExt for Lifetime {
    fn call_site_span<T: AsRef<str>>(symbol: T) -> Self {
        Self::new(symbol.as_ref(), Span::call_site())
    }
}

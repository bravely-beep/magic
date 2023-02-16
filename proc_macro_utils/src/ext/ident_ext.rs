use parsel::{syn::Ident, Span};

pub trait IdentExt {
    fn call_site_span<T: AsRef<str>>(string: T) -> Self;
}

impl IdentExt for Ident {
    fn call_site_span<T: AsRef<str>>(string: T) -> Self {
        Self::new(string.as_ref(), Span::call_site())
    }
}

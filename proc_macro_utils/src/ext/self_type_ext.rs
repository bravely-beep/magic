use parsel::{syn::token::SelfType, Span};

pub trait SelfTypeExt {
    fn call_site_span() -> Self;
}

impl SelfTypeExt for SelfType {
    fn call_site_span() -> Self {
        Self {
            span: Span::call_site(),
        }
    }
}

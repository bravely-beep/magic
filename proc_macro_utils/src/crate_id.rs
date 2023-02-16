use parsel::{
    syn::{punctuated::Punctuated, Ident, Path, Token},
    Span, ToTokens,
};
use proc_macro_crate::{crate_name, FoundCrate};

use crate::CompileErrorTokenStream;

pub enum CrateId {
    ThisCrate(Token![crate]),
    OtherCrate(Ident),
}

impl CrateId {
    pub fn new<T: AsRef<str>>(name: T) -> Result<Self, CrateNotFoundError> {
        Self::new_spanned(name, Span::call_site())
    }

    pub fn new_spanned<T: AsRef<str>>(name: T, span: Span) -> Result<Self, CrateNotFoundError> {
        match crate_name(name.as_ref()) {
            Ok(v) => Ok(match v {
                FoundCrate::Itself => Self::ThisCrate(Token![crate](span)),
                FoundCrate::Name(v) => Self::OtherCrate(Ident::new(&v, span)),
            }),
            Err(e) => Err(CrateNotFoundError { inner: e, span }),
        }
    }

    pub fn to_path(&self) -> Path {
        match self {
            Self::ThisCrate(crate_token) => Path {
                leading_colon: None,
                segments: {
                    let mut segments = Punctuated::new();
                    segments.push((*crate_token).into());
                    segments
                },
            },
            Self::OtherCrate(ident) => Path {
                leading_colon: Some(<Token![::]>::default()),
                segments: {
                    let mut segments = Punctuated::new();
                    segments.push(ident.clone().into());
                    segments
                },
            },
        }
    }
}

impl ToTokens for CrateId {
    fn to_tokens(&self, tokens: &mut parsel::TokenStream) {
        match self {
            Self::ThisCrate(crate_token) => crate_token.to_tokens(tokens),
            Self::OtherCrate(ident) => ident.to_tokens(tokens),
        }
    }
}

pub struct CrateNotFoundError {
    pub inner: proc_macro_crate::Error,
    pub span: Span,
}

impl From<CrateNotFoundError> for CompileErrorTokenStream {
    fn from(value: CrateNotFoundError) -> Self {
        parsel::Error::new(value.span, value.inner.to_string()).into()
    }
}

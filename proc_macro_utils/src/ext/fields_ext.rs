use parsel::{
    syn::{Fields, Ident},
    Span, Spanned,
};

use crate::CompileErrorTokenStream;

pub trait FieldsExt {
    /// If all fields are named, return a `Vec` of their idents. Otherwise, return `Err`.
    fn fully_named_idents<'a, T>(&'a self) -> Result<T, NotNamedFieldsError>
    where
        T: FromIterator<&'a Ident>;
}

impl FieldsExt for Fields {
    fn fully_named_idents<'a, T>(&'a self) -> Result<T, NotNamedFieldsError>
    where
        T: FromIterator<&'a Ident>,
    {
        self.iter()
            .map(|field| {
                field
                    .ident
                    .as_ref()
                    .ok_or_else(|| NotNamedFieldsError { span: self.span() })
            })
            .collect()
    }
}

pub struct NotNamedFieldsError {
    span: Span,
}

impl From<NotNamedFieldsError> for CompileErrorTokenStream {
    fn from(value: NotNamedFieldsError) -> Self {
        parsel::Error::new(value.span, "Expected named fields").into()
    }
}

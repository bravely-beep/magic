use parsel::TokenStream;

pub struct CompileErrorTokenStream {
    inner: TokenStream,
}

impl From<parsel::Error> for CompileErrorTokenStream {
    fn from(value: parsel::Error) -> Self {
        Self {
            inner: value.into_compile_error(),
        }
    }
}

impl From<CompileErrorTokenStream> for TokenStream {
    fn from(value: CompileErrorTokenStream) -> Self {
        value.inner
    }
}

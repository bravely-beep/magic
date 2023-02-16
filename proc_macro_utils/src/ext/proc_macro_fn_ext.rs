use parsel::TokenStream;

use crate::CompileErrorTokenStream;

pub trait ProcMacroFnExt {
    /// Adapts a function that accepts `parsel::TokenStream` and returns `Result<TokenStream, CompileErrorTokenStream>`
    /// so that it accepts and returns a raw token stream type (usually `proc_macro::TokenStream`).
    fn adapt_for_proc_macro<T>(self, raw_token_stream: T) -> T
    where
        T: From<TokenStream>,
        TokenStream: From<T>;
}

impl<S> ProcMacroFnExt for S
where
    Self: Fn(TokenStream) -> Result<TokenStream, CompileErrorTokenStream>,
{
    fn adapt_for_proc_macro<T>(self, raw_token_stream: T) -> T
    where
        T: From<TokenStream>,
        TokenStream: From<T>,
    {
        let token_stream = raw_token_stream.into();
        match self(token_stream) {
            Ok(v) => v.into(),
            Err(e) => {
                let token_stream: TokenStream = e.into();
                token_stream.into()
            }
        }
    }
}

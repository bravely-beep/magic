use proc_macro::TokenStream;
use proc_macro_utils::ext::ProcMacroFnExt;

mod derive_explode;

#[proc_macro_derive(Explode)]
pub fn derive_explode(item: TokenStream) -> TokenStream {
    derive_explode::main.adapt_for_proc_macro(item)
}

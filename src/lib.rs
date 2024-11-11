pub use proc_macro2::{Ident,TokenStream as Token2};
pub use quote::quote;

/// Create derive presets inside a proc-macro crate!
///
/// ## Inside a Proc-Macro Crate:
/// ```rust
///     derive_preset::create!{
///         hashable "PartialEq,Eq,Hash,Clone,Debug"
///         serde "Serialize,Deserialize,Clone"
///     }
/// ```
///
/// ## Use it in another Crate:
/// ```rust
///     use my_proc_crate::*;
///
///     #[hashable(Clone,Default)]
///     struct Id(u32);
///
///     #[serde(Debug)]
///     struct Data(f32);
/// ```
#[macro_export]
macro_rules! create {
    ($($name:ident $doc:literal)*) => {$(
        #[doc = "derive(..) preset for:"]
        #[doc = $doc]
        #[proc_macro_attribute]
        pub fn $name(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let mut derives: Vec<derive_preset::Ident> = vec![];
            for a in attr.into_iter() {
                let str = a.to_string();
                if &str == "," {continue}
                derives.push(derive_preset::Ident::new(&str,proc_macro::Span::call_site().into()))
            }
            let pre:derive_preset::Token2 = $doc.parse().unwrap();
            let mut pre = proc_macro::TokenStream::from(derive_preset::quote!(#[derive(#pre,#(#derives),* )]));
            pre.extend([item].into_iter());
            pre
        }
    )*}
}

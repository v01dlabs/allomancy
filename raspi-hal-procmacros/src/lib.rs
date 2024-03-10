#[allow(unused)]
use proc_macro::TokenStream;

#[cfg(feature = "enum-dispatch")]
mod enum_dispatch;

/// Create an enum for erased GPIO pins, using the enum-dispatch pattern
///
/// Only used internally
#[cfg(feature = "enum-dispatch")]
#[proc_macro]
pub fn make_gpio_enum_dispatch_macro(input: TokenStream) -> TokenStream {
    use quote::{format_ident, quote};

    use self::enum_dispatch::{build_match_arms, MakeGpioEnumDispatchMacro};

    let input = syn::parse_macro_input!(input as MakeGpioEnumDispatchMacro);

    let macro_name = format_ident!("{}", input.name);
    let arms = build_match_arms(input);

    quote! {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! #macro_name {
            ($m:ident, $target:ident, $body:block) => {
                match $m {
                    #(#arms)*
                }
            }
        }

        pub(crate) use #macro_name;
    }
    .into()
}

use embedded_hal_ext::digital::*;

/*!
# Execute Command Macro Impl

See [`execute-command-macro`](https://crates.io/crates/execute-command-macro).
*/

use syn::parse_macro_input;

use quote::quote;

use proc_macro::TokenStream;
use syn::LitStr;

use execute_command_tokens::command_tokens;

#[proc_macro]
pub fn command(input: TokenStream) -> TokenStream {
    let s = parse_macro_input!(input as LitStr).value();

    let tokens = command_tokens(s);

    let tokens_length = tokens.len();

    let command = match tokens_length {
        0 => {
            quote! {
                ::std::process::Command::new("")
            }
        }
        1 => {
            let program = &tokens[0];

            quote! {
                ::std::process::Command::new(#program)
            }
        }
        _ => {
            let program = &tokens[0];
            let args = &tokens[1..];

            quote! {
                {
                    let mut command = ::std::process::Command::new(#program);

                    command.args(&[#(#args,)*]);

                    command
                }
            }
        }
    };

    command.into()
}

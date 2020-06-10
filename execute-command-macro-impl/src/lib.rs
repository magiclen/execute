/*!
# Execute Command Macro Impl

See [`execute-command-macro`](https://crates.io/crates/execute-command-macro).
*/

extern crate execute_command_tokens;
extern crate proc_macro_hack;

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use syn::LitStr;

use execute_command_tokens::command_tokens;

#[proc_macro_hack]
pub fn command(input: TokenStream) -> TokenStream {
    let s = parse_macro_input!(input as LitStr).value();

    let tokens = command_tokens(s);

    let tokens_length = tokens.len();

    let command = match tokens_length {
        0 => {
            quote! {
                std::process::Command::new("")
            }
        }
        1 => {
            let program = &tokens[0];

            quote! {
                std::process::Command::new(#program)
            }
        }
        _ => {
            let program = &tokens[0];
            let args = &tokens[1..];

            quote! {
                {
                    let mut command = std::process::Command::new(#program);

                    command.args(&[#(#args,)*]);

                    command
                }
            }
        }
    };

    command.into()
}

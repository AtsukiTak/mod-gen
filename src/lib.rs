use proc_macro::TokenStream as StdTokenStream;
use std::convert::TryFrom as _;

mod args;
mod config;
mod gen;
mod module;

macro_rules! try_ok {
    ($x: expr) => {
        match $x {
            Ok(ok) => ok,
            Err(e) => return e.to_compile_error().into(),
        }
    };
}

#[proc_macro_attribute]
pub fn expand(args: StdTokenStream, _input: StdTokenStream) -> StdTokenStream {
    let args = {
        let args = syn::parse_macro_input!(args as syn::AttributeArgs);
        try_ok!(args::Args::try_from(args))
    };

    let config = try_ok!(config::Config::try_from(args));

    let module = module::read_module(config.dir_path).unwrap();

    gen::gen_mod(&module).into()
}

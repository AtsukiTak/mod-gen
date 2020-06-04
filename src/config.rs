use crate::args::Args;
use proc_macro2::Span;
use std::convert::TryFrom;
use syn::Error;

#[derive(Debug)]
pub struct Config {
    pub dir_path: String,
}

impl TryFrom<Args> for Config {
    type Error = syn::Error;

    fn try_from(args: Args) -> syn::Result<Config> {
        let manifest_dir_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();

        let expand_dir_path = match args.dir_path {
            Some(path) => format!("{}/{}", manifest_dir_path, path),
            None => return Err(Error::new(Span::call_site(), "path must be set for now")),
        };

        Ok(Config {
            dir_path: expand_dir_path,
        })
    }
}

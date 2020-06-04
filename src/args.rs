use std::convert::TryFrom;
use syn::{Meta, MetaNameValue, NestedMeta};

pub struct Args {
    pub dir_path: Option<String>,
}

impl TryFrom<syn::AttributeArgs> for Args {
    type Error = syn::Error;

    fn try_from(args: syn::AttributeArgs) -> syn::Result<Args> {
        from_items(from_input_vec(args)?)
    }
}

fn from_items(items: Vec<ArgItem>) -> syn::Result<Args> {
    let dir_path = items
        .iter()
        .map(|item| match item {
            ArgItem::DirPath(path) => path.clone(),
        })
        .next();

    Ok(Args { dir_path })
}

/*
 * ==============
 * ArgItem
 * ==============
 */
enum ArgItem {
    DirPath(String),
}

macro_rules! unrecognized_item {
    ($item: expr) => {
        return Err(syn::Error::new_spanned($item, "Unrecognized arg"));
    };
}

fn from_input_vec(args: syn::AttributeArgs) -> syn::Result<Vec<ArgItem>> {
    args.into_iter().map(from_input).collect()
}

fn from_input(arg: NestedMeta) -> syn::Result<ArgItem> {
    match arg {
        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            path,
            lit: syn::Lit::Str(s),
            ..
        })) => {
            if path.is_ident("path") {
                Ok(ArgItem::DirPath(s.value()))
            } else {
                unrecognized_item!(path)
            }
        }
        _ => unrecognized_item!(arg),
    }
}

use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::quote;
use regex::Regex;
use std::env;

static YEAR_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"2\d{3}").unwrap());
static DAY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d{2}").unwrap());

/// Parses the year from the package name of the crate that is currently being compiled (i.e. the
/// crate that invoked the macro).
#[proc_macro]
pub fn year(_item: TokenStream) -> TokenStream {
    handle_errors(|| {
        let pkg_name = env_var("CARGO_PKG_NAME")?;
        let year = YEAR_REGEX
            .find(&pkg_name)
            .ok_or_else(|| format!("no year found in package name {pkg_name}"))?
            .as_str()
            .parse::<u32>()
            .unwrap();
        Ok(quote!(#year).into())
    })
}

#[proc_macro]
pub fn day(_item: TokenStream) -> TokenStream {
    handle_errors(|| {
        let bin_name = env_var("CARGO_BIN_NAME")?;
        let day = DAY_REGEX 
            .find(&bin_name)
            .ok_or_else(|| format!("no day found in binary name {bin_name}"))?
            .as_str()
            .parse::<u32>()
            .unwrap();
        Ok(quote!(#day).into())
    })
}

fn env_var(name: &str) -> Result<String, String> {
    env::var(name)
        .map_err(|_| format!("{name} not set in environment"))
}

fn handle_errors(mut f: impl FnMut() -> Result<TokenStream, String>) -> TokenStream {
    f().unwrap_or_else(|message| quote!(compile_error!(#message);).into())
}

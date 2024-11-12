use core::fmt;

use heck::*;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse, parse_macro_input, parse_quote, Attribute, ExprAssign, ItemStruct, LitStr};

#[proc_macro_attribute]
pub fn serde_infer(opt: TokenStream, i: TokenStream) -> TokenStream {
    let mut data = parse_macro_input!(i as ItemStruct);
    let case = parse::<ExprAssign>(opt)
        .ok()
        .map(|f| parse::<LitStr>(f.right.to_token_stream().into()).ok())
        .flatten();

    data.fields.iter_mut().for_each(|f| {
        f.attrs
            .extend_from_slice(&cases(f.ident.clone().unwrap().to_string()));

        case.clone().inspect(|c| {
            f.attrs.extend_from_slice({
                let out = outgoing(c.value(), f.ident.clone().unwrap().to_string());
                &[parse_quote!(#[serde(rename = #out)])]
            })
        });
    });

    data.to_token_stream().into()
}

fn outgoing(case: String, field: String) -> String {
    match case.as_str() {
        "kebab" => AsKebabCase(&field).to_string(),
        "lower_camel" => AsLowerCamelCase(&field).to_string(),
        "pascal" => AsPascalCase(&field).to_string(),
        "shouty_kebab" => AsShoutyKebabCase(&field).to_string(),
        "shouty_snake" => AsShoutySnakeCase(&field).to_string(),
        "snake" => AsSnakeCase(&field).to_string(),
        "title" => AsTitleCase(&field).to_string(),
        "train" => AsTrainCase(&field).to_string(),
        "upper_camel" => AsUpperCamelCase(&field).to_string(),
        "uppercase" | "upper" => field.to_uppercase(),
        "lowercase" | "lower" => field.to_lowercase(),
        _ => panic!("Invalid casing: {}", case),
    }
}

fn cases(field: String) -> Vec<Attribute> {
    vec![
        AsKebabCase(&field).to_string(),
        AsLowerCamelCase(&field).to_string(),
        AsPascalCase(&field).to_string(),
        AsShoutyKebabCase(&field).to_string(),
        AsShoutySnakeCase(&field).to_string(),
        AsShoutySnekCase(&field).to_string(),
        AsSnakeCase(&field).to_string(),
        AsSnekCase(&field).to_string(),
        AsTitleCase(&field).to_string(),
        AsTrainCase(&field).to_string(),
        AsUpperCamelCase(&field).to_string(),
        field.to_uppercase(),
        field.to_lowercase(),
    ]
    .iter()
    .map(|t| {
        let field = t.to_string();
        parse_quote!(#[serde(alias = #field)])
    })
    .collect::<Vec<Attribute>>()
}

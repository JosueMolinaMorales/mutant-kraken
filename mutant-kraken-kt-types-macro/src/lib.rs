extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[derive(serde::Deserialize)]
struct JsonItem {
    r#type: String,
    named: bool,
}

#[proc_macro]
pub fn generate_types(_input: TokenStream) -> TokenStream {
    const JSON_DATA: &str = tree_sitter_kotlin::NODE_TYPES;
    let json_items: Vec<JsonItem> = serde_json::from_str(JSON_DATA).expect("Failed to parse JSON");

    let mut named_types: Vec<_> = json_items
        .iter()
        .filter(|item| item.named)
        .map(|item| snake_to_camel(&item.r#type))
        .collect();
    // Add custom types
    named_types.push("ERROR".into());
    named_types.push("Remove".into());
    named_types.push("RemoveOperator".into());
    named_types.push("AnyParent".into());

    let non_named_types: Vec<_> = json_items
        .iter()
        .filter(|item| !item.named)
        .map(|item| &item.r#type)
        .collect::<Vec<_>>();

    let enum_variants = named_types.iter().map(|name| {
        let variant_name = syn::Ident::new(name, proc_macro2::Span::call_site());
        quote! {
            #variant_name,
        }
    });

    let display_match_arms = named_types.iter().map(|name| {
        let variant_name = syn::Ident::new(name, proc_macro2::Span::call_site());
        quote! {
            KotlinTypes::#variant_name => write!(f, "{}", #name),
        }
    });

    let convert_match_arms = named_types.iter().map(|name| {
        let variant_name = syn::Ident::new(name, proc_macro2::Span::call_site());
        quote! {
            #name => KotlinTypes::#variant_name,
        }
    });

    let expanded = quote! {
        pub const NON_NAMED_TYPES: &[&str] = &[#(#non_named_types),*];
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum KotlinTypes {
            #(#enum_variants)*
            NonNamedType(String)
        }

        impl std::fmt::Display for KotlinTypes {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    #(#display_match_arms)*
                    KotlinTypes::NonNamedType(s) => write!(f, "{}", s)
                }
            }
        }

        impl KotlinTypes {
            pub fn new(s: &str) -> std::result::Result<KotlinTypes, String> {
                if NON_NAMED_TYPES.contains(&s) {
                    return Ok(KotlinTypes::NonNamedType(s.to_string()))
                }
                let name: String = s.split('_')
                    .map(|p| {
                        if !p.is_empty() {
                            let mut v: Vec<char> = p.chars().collect();
                            v[0] = v[0].to_uppercase().next().unwrap();
                            let x: String = v.into_iter().collect();
                            x
                        } else {
                            p.to_string()
                        }
                    })
                    .collect();
                let res = match name.as_str() {
                    #(#convert_match_arms)*
                    _ => {
                        return Err(format!("could not convert {} to kotlin type", s))
                    }
                };

                Ok(res)
            }

            pub fn as_str(&self) -> String {
                let mut second_upper = 0;
                let mut x = format!("{}", *self);
                x.as_bytes().iter().enumerate().for_each(|(i, c)| {
                    if (*c as char).is_uppercase() && i != 0 {
                        second_upper = i
                    }
                });
                x = x.to_lowercase();
                if second_upper != 0 {
                    x.insert(second_upper, '_');
                }
                x
            }
        }
    };

    TokenStream::from(expanded)
}

fn snake_to_camel(name: &str) -> String {
    // convert snake_case to camelcase
    name.split('_')
        .map(|p| {
            if !p.is_empty() {
                let mut v: Vec<char> = p.chars().collect();
                v[0] = v[0].to_uppercase().next().unwrap();
                let x: String = v.into_iter().collect();
                x
            } else {
                p.to_string()
            }
        })
        .collect()
}

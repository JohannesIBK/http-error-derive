use darling::{ast, FromDeriveInput, FromVariant, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::DeriveInput;

#[proc_macro_derive(HttpError, attributes(http))]
pub fn parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let gen = BaseReceiver::from_derive_input(&ast).unwrap();

    quote!(#gen).into()
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(http), supports(enum_any))]
struct BaseReceiver {
    ident: syn::Ident,
    data: ast::Data<FieldReceiver, ()>,
    default_code: u16,
}

impl ToTokens for BaseReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BaseReceiver {
            ref ident,
            ref data,
            default_code,
        } = *self;

        let fields = data.as_ref().take_enum().expect("Should never be enum");
        let mut code_tokens = Vec::<TokenStream>::new();
        let mut message_tokens = Vec::<TokenStream>::new();

        fields.into_iter().for_each(|f| {
            let field_ident = &f.ident;

            let var = match &f.fields.style {
                ast::Style::Tuple => {
                    quote! { (_) }
                },
                _ => quote! {}
            };

            if let Some(code) = f.code {
                code_tokens.push(quote! {
                    Self::#field_ident #var => #code,
                });
            }

            if f.message.is_some() {
                let message = f.message.clone().unwrap();

                message_tokens.push(quote! {
                    Self::#field_ident #var => Some(#message),
                })
            }

        });

        tokens.extend(quote! {
            impl #ident {
                fn http_code(&self) -> u16 {
                    match &self {
                        #(#code_tokens)*
                        _ => #default_code
                    }
                }
                fn http_message(&self) -> Option<&'static str> {
                    match &self {
                        #(#message_tokens)*
                        _ => None
                    }
                }
            }
        })
    }
}

#[derive(Debug, FromVariant)]
#[darling(attributes(http))]
struct FieldReceiver {
    ident: syn::Ident,
    fields: darling::ast::Fields<FieldFieldReceiver>,

    code: Option<u16>,
    message: Option<String>,
}

#[derive(Debug, FromField)]
struct FieldFieldReceiver {}
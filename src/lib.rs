use darling::{ast, FromDeriveInput, FromField, FromVariant};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[proc_macro_derive(HttpError, attributes(http))]
pub fn parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input);
    let gen = BaseReceiver::from_derive_input(&ast).unwrap();

    quote!(#gen).into()
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(http), supports(enum_any))]
struct BaseReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<FieldReceiver, ()>,
}

impl ToTokens for BaseReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BaseReceiver {
            ref ident,
            ref generics,
            ref data,
        } = *self;

        let (imp, ty, wher) = generics.split_for_impl();
        let fields = data.as_ref().take_enum().expect("Should never be enum");
        let mut code_tokens = Vec::<TokenStream>::new();
        let mut message_tokens = Vec::<TokenStream>::new();
        let mut error_tokens = Vec::<TokenStream>::new();

        fields.into_iter().for_each(|f| {
            let field_ident = &f.ident;

            let field_variant = match &f.fields.style {
                ast::Style::Tuple => {
                    quote! { (_) }
                }
                ast::Style::Struct => {
                    quote! { { .. } }
                }
                _ => quote! {},
            };

            if let Some(code) = f.code {
                code_tokens.push(quote! {
                    Self::#field_ident #field_variant => Some(#code),
                });
            }

            if let Some(error) = f.error {
                error_tokens.push(quote! {
                    Self::#field_ident #field_variant => Some(#error),
                });
            }

            if f.message.is_some() {
                let message = f.message.clone().unwrap();

                message_tokens.push(quote! {
                    Self::#field_ident #field_variant => Some(#message),
                })
            }
        });

        tokens.extend(quote! {
            impl #imp #ident #ty #wher {
                pub fn http_code(&self) -> Option<u16> {
                    match &self {
                        #(#code_tokens)*
                        _ => None
                    }
                }
                pub fn http_message(&self) -> Option<&'static str> {
                    match &self {
                        #(#message_tokens)*
                        _ => None
                    }
                }
                pub fn http_error(&self) -> Option<u16> {
                    match &self {
                        #(#error_tokens)*
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
    fields: ast::Fields<FieldFieldReceiver>,
    code: Option<u16>,
    message: Option<String>,
    error: Option<u16>,
}

#[derive(Debug, FromField)]
struct FieldFieldReceiver {}

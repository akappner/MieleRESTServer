use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput, Expr, ExprLit, Lit, Type, token::Comma, ExprPath, TypePath};

#[proc_macro_derive(AssocTypes, attributes(dop2field))]
pub fn derive_assoc_types(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

//    let trait_ident = format_ident!("__AssocFor_{}", struct_name);
    let trait_ident = format_ident!("Dop2ParseTreeExpressible");
    let marker_prefix = format!("__AssocFor_{}_field_", struct_name);

    // Only support named-structs
    let fields = match input.data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(named) => named.named,
            _ => {
                return syn::Error::new_spanned(
                    struct_name,
                    "AssocTypes only supports structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(struct_name, "AssocTypes can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let mut marker_defs = Vec::new();
    let mut impls = Vec::new();
    let mut constructor_fragments = Vec::new();

    for field in fields.iter() {
        let field_ident = match &field.ident {
            Some(id) => id,
            None => continue,
        };
        
        // Find an attribute named "assoc_type"
        for attr in &field.attrs {
            if attr.path().is_ident("dop2field") { // only parse our attribute

     let args = match attr.parse_args_with(Punctuated::<Expr, Comma>::parse_terminated) {
                    Ok(args) => args,
                    Err(e) => {
                        return syn::Error::new_spanned(
                            attr,
                            format!("failed to parse assoc_types arguments: {}", e),
                        )
                        .to_compile_error()
                        .into();
                    }
                };

                if args.len() != 2 {
                    return syn::Error::new_spanned(
                        attr,
                        "expected exactly two arguments: #[dop2field(fieldId, payloadType)]",
                    )
                    .to_compile_error()
                    .into();
                }

                let enum_expr = match &args[1] {
                    Expr::Path(ExprPath { path, .. }) => path.clone(),
                    _ => {
                        return syn::Error::new_spanned(&args[2], 
                            "third argument must be a path expression like MyEnum::Variant")
                            .to_compile_error()
                            .into();
                    }
                };

                let number = match &args[0] {
                    Expr::Lit(ExprLit { lit: Lit::Int(litint), .. }) => litint,
                    _ => {
                        return syn::Error::new_spanned(
                            &args[0],
                            "third argument must be an integer literal",
                        )
                        .to_compile_error()
                        .into();
                    }
                };

let in_ty: Type = syn::parse2(args[1].to_token_stream()).unwrap();
//let out_ty: Type = syn::parse2(args[1].to_token_stream()).unwrap();

let is_option = if let Type::Path(TypePath { path, .. }) = &field.ty {
    path.segments.last().map(|seg| seg.ident == "Option").unwrap_or(false)
} else {
    false
};

                let marker_ident = format_ident!("{}{}", marker_prefix, field_ident);

                marker_defs.push(quote! {
                    pub struct #marker_ident;
                });
                // TODO: Remove this "clone" -- should be avoidable by restructuring this loop
                // let Dop2Payloads::E8(appliance_state) = x.fields[0].value 

                if (is_option)
                {
                    impls.push(quote! {
                        let #enum_expr(#field_ident) = x.fields.get(#number-1).map(|s| s.value) && 
                    });
                    if (constructor_fragments.len() > 0 )
                    {
                        constructor_fragments.push(quote!{,});
                    }
                    constructor_fragments.push(quote! {
                        #field_ident: #field_ident.try_into().unwrap()
                    })
                }
                else 
                {
                impls.push(quote! {
                    let #enum_expr(#field_ident) = x.fields.get(#number-1).unwrap().value.clone() && 
                });
                if (constructor_fragments.len() > 0 )
                {
                    constructor_fragments.push(quote!{,});
                }
                constructor_fragments.push(quote! {
                    #field_ident: #field_ident.try_into().unwrap()
                })
            }
        } }
    }

    // Emit the trait, markers and impls
    let expanded = quote! {
        // trait whose associated type is the "associated type for a (struct, marker)"
//        pub trait #trait_ident<Marker> {
//            type Ty;
 //       }
         impl Dop2ParseTreeExpressible for #struct_name 
{
         fn from_parse_tree (payload: Dop2Payloads) -> Result<Self, String> { 
        
         if let Dop2Payloads::MStruct(x)=payload && #(#impls)* 1==1 {
            let y = Self {#(#constructor_fragments)*  }; 
            return Ok(y);
            //return Err("success".to_string());
         }
         else
         {
            return Err("failed converting type".to_string());
         }
        }
}
        // generated marker structs and impls
//        #(#marker_defs)*

    };

    TokenStream::from(expanded)
}

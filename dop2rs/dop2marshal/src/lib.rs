use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput, Expr, ExprLit, Lit, Type, token::Comma, ExprPath, TypePath};

#[proc_macro_derive(AssocTypes, attributes(dop2field))]
pub fn derive_assoc_types(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

//    let trait_ident = format_ident!("Dop2ParseTreeExpressible");
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

    let mut marshalling_field_definitions = Vec::new();

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
                        return syn::Error::new_spanned(&args[1], 
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

//let in_ty: Type = syn::parse2(args[1].to_token_stream()).unwrap();
//let out_ty: Type = syn::parse2(args[1].to_token_stream()).unwrap();

let is_option = if let Type::Path(TypePath { path, .. }) = &field.ty {
    path.segments.last().map(|seg| seg.ident == "Option").unwrap_or(false)
} else {
    false
};

                let marker_ident = format_ident!("{}{}", marker_prefix, field_ident);
                let marshaling_payload_ident = format_ident!("{}{}", "_payload_", field_ident);
                let marshaling_field_ident = format_ident!("{}{}", "_field_", field_ident);
                marker_defs.push(quote! {
                    pub struct #marker_ident;
                });
                // TODO: Remove this "clone" -- should be avoidable by restructuring this loop
                // let Dop2Payloads::E8(appliance_state) = x.fields[0].value 

                if is_option
                {
                    
                    /*impls.push(quote! {
                        let #enum_expr(#field_ident) = x.get_payload(#number) && 
                    });*/
                    if constructor_fragments.len() > 0
                    {
                        constructor_fragments.push(quote!{,});
                    }
                    constructor_fragments.push(quote! {
                       #field_ident: match x.get_payload(#number)
                       {
                         Some(test) => match test {
                            #enum_expr(unwrapped) => Some(unwrapped.try_into().unwrap()),
                            _ => None
                         },
                         None => None
                       }
                    })

                }
                else 
                {
                    //println!("field_ident: {:?} is not an option", stringify!(#field_ident));
                impls.push(quote! {
                    let Some(#enum_expr(#field_ident)) = x.get_payload(#number) && 
                });
                if constructor_fragments.len() > 0
                {
                    constructor_fragments.push(quote!{,});
                }
                
                if is_option // skip fields that are empty optionals

                {
                    
                }
                else {
                    constructor_fragments.push(quote! {
                        #field_ident: #field_ident.try_into().unwrap()
                    });
                marshalling_field_definitions.push(quote!( { 
               //     let marshaling_payload_ident : Dop2Payloads = Dop2Payloads::U16(self.selection_parameter.clone().into());
                 //   let marshaling_payload_ident = (self.#field_ident).clone(); // this works
                    //if (let Some(unwrapped_value) = self.#field_ident) 
                    { // skip the field if it's None
                    let #marshaling_payload_ident : Dop2Payloads = #enum_expr (self.#field_ident.clone().try_into().unwrap()).clone().into(); // this works
                    let #marshaling_field_ident : TaggedDopField = TaggedDopField { field_index: #number, tag: Dop2PayloadsKind::from(#marshaling_payload_ident.clone()), value: #marshaling_payload_ident};
                    fields.push(#marshaling_field_ident);
                    }
                
                  //  let #marshaling_field_ident = 3;
                } ));

                }
            }
        } }
    }

    // Emit the trait, markers and impls
    let expanded = quote! {
        // trait whose associated type is the "associated type for a (struct, marker)"
//        pub trait #trait_ident<Marker> {
//            type Ty;
 //       }
 //payloads::impl_tryfrom_dop2struct!(#struct_name);
         impl #struct_name 
         {
            pub fn to_dop2_struct_auto (&self) -> Result<Dop2Struct, String>
            {
                let mut fields: Vec<TaggedDopField> = vec!();

                #( #marshalling_field_definitions )*
                
               /* let request_id_payload : Dop2Payloads = Dop2Payloads::E16(self.program_id.clone().into());
                let request_id_field : TaggedDopField = TaggedDopField{ field_index: 1, tag: Dop2PayloadsKind::from(request_id_payload.clone()), value: request_id_payload};
                let selection_parameter_payload : Dop2Payloads = Dop2Payloads::U16(self.selection_parameter.clone().into());
                let selection_parameter_field : TaggedDopField = TaggedDopField{ field_index: 2, tag: Dop2PayloadsKind::from(selection_parameter_payload.clone()), value: selection_parameter_payload};
                let selection_type_payload :  Dop2Payloads = Dop2Payloads::E8(self.selection_type.clone().into());
                let selection_type_field : TaggedDopField = TaggedDopField{ field_index: 3, tag: Dop2PayloadsKind::from(selection_type_payload.clone()), value: selection_type_payload};
               
                fields.push(request_id_field);
                fields.push(selection_parameter_field);
                fields.push(selection_type_field);
                Ok(Dop2Struct::from_fields (fields))
                 */

                 Ok(Dop2Struct::from_fields (fields))
            }
         }

         impl TryInto<Dop2Struct> for #struct_name
         { 
            type Error = String;
            fn try_into(self)-> Result<Dop2Struct, String>
            {
                return self.to_dop2_struct_auto();
            }
         }

         impl TryInto<DopArray<Dop2Struct>> for Vec<#struct_name>
         {
             type Error = String;
         
             fn try_into(self) -> Result<DopArray<Dop2Struct>, String> {
                //let elements : Vec<Dop2Struct> = self.into_iter().map(|e| e.try_into().unwrap()).collect(); // TODO: Figure out error handling
                    // value.elements.into_iter().map(|garbage|#struct_name::try_into(garbage)).collect()
                     
                    Ok(DopArray {
                        count: self.len() as u16,
                        elements: self.into_iter().map(|e| e.try_into().unwrap()).collect()
                    })
                    
                    //Err("garbage2".to_string())
                }
             
         }
       
         impl Dop2ParseTreeExpressible for #struct_name 
{
         fn from_parse_tree (payload: Dop2Payloads) -> Result<Self, String> { 
         if let Dop2Payloads::MStruct(x)=payload 
         {
           // println!("{:?}", &x.fields.map(|x| s.field));
            if #(#impls)* 1==1 {
            let y = Self {#(#constructor_fragments)*  }; 
            return Ok(y);
            //return Err("success".to_string());
            }
            else
            {
               
                return Err(format!("failed converting type: one or more field conversions failed in {}", stringify!(#struct_name)));
            }
         }
         else
         {
            //println!("{:?}", &payload);
            return Err(format!("failed converting type: one or more field conversions failed in {}", stringify!(#struct_name)));
         }
        }
}

    };

    TokenStream::from(expanded)
}

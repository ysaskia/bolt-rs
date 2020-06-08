extern crate syn;
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::export::{TokenStream2, Span};
use syn::{ItemStruct, ItemEnum, LitInt, Field, Fields, Ident};
use syn::{parse_macro_input};
use quote::quote;

const ALPHA: &'static str = "abcdefghijklmnopqrstuvwxyz";

#[proc_macro_attribute]
pub fn bolt_packstream(sign: TokenStream, input: TokenStream) -> TokenStream {
    let input: ItemStruct = parse_macro_input!(input as ItemStruct);
    let sign: LitInt = parse_macro_input!(sign as LitInt);
    let pack_calls = input.fields.iter().map(pack_call).collect::<Vec<_>>();
    let field_names = input.fields.iter().map(field_name).collect::<Vec<_>>();
    let field_values = input.fields.iter().map(field_value).collect::<Vec<_>>();
    let size = pack_calls.len();
    let name = &input.ident;

    TokenStream::from(quote! {
    #input

    impl #name {
        pub fn struct_size() -> usize { #size }
        pub fn struct_sign() -> u8 { #sign }
    }

    impl PackValue<#name> for Packer {
        fn pack(&mut self, s:#name) -> Result<(), BoltError> {
            self.pack_struct_header(#size, #sign)?;
            #(#pack_calls)*
            Ok(())
        }
    }

    impl UnpackValue<#name> for Unpacker {
        fn unpack(&mut self) -> Result<#name, BoltError> {
            self.unpack_struct_header()
                .and_then(|_| self.unpack_struct_signature())
                .and_then(|_| {
                    #(#field_values)*
                    Ok(#name {
                        #(#field_names)*
                    })
                })
        }
    }
  })
}

fn pack_call(f: &syn::Field) -> syn::export::TokenStream2 {
    let field = &f.ident;
    quote!(self.pack(s.#field)?;)
}

fn field_name(f: &syn::Field) -> syn::export::TokenStream2 {
    let field = &f.ident;
    quote!(#field,)
}

fn field_value(f: &syn::Field) -> syn::export::TokenStream2 {
    let field = &f.ident;
    quote!(let #field = self.unpack()?;)
}

struct EnumContext {
    variant_names: Vec<TokenStream2>,
    variant_fields_names: Vec<TokenStream2>,
    variant_fields_unpack: Vec<TokenStream2>,
    variant_fields_encode: Vec<Vec<TokenStream2>>,
    variant_fields_decode: Vec<Vec<TokenStream2>>,
}

impl EnumContext {
    fn new() -> EnumContext {
        EnumContext {
            variant_names: vec![],
            variant_fields_names: vec![],
            variant_fields_unpack: vec![],
            variant_fields_encode: vec![],
            variant_fields_decode: vec![],
        }
    }
}

struct VariantFieldsContext {
    unpack: Vec<TokenStream2>,
    encode: Vec<TokenStream2>,
    decode: Vec<TokenStream2>,
}

impl VariantFieldsContext {
    fn new() -> VariantFieldsContext {
        VariantFieldsContext {
            unpack: vec![],
            encode: vec![],
            decode: vec![],
        }
    }
}

#[proc_macro_attribute]
pub fn bolt_enum(_sign: TokenStream, input: TokenStream) -> TokenStream {
    let input: ItemEnum = parse_macro_input!(input as ItemEnum);
    let enum_name = &input.ident;
    let enum_generics = &input.generics;
    let enum_context = input.variants
        .iter()
        .fold(EnumContext::new(), |mut ctx, variant| {
            let variant_name = &variant.ident;
            let variant_fields = &variant.fields;
            let (
                unpack,
                encode,
                decode
            ) = match variant.fields {
                Fields::Named(_) => named_fields_context(variant_fields),
                Fields::Unnamed(_) => unnamed_fields_context(variant_fields),
                Fields::Unit => (quote!(), vec![], vec![])
            };

            ctx.variant_names.push(quote!(#variant_name));
            ctx.variant_fields_names.push(quote!(#variant_fields));
            ctx.variant_fields_unpack.push(unpack);
            ctx.variant_fields_encode.push(encode);
            ctx.variant_fields_decode.push(decode);
            ctx
        });

    let variant_names = enum_context.variant_names;
    let _fields_names = enum_context.variant_fields_names;
    let unpack_fields = enum_context.variant_fields_unpack;
    let encode_fields = enum_context.variant_fields_encode;
    let decode_fields = enum_context.variant_fields_decode;

    let stream = TokenStream::from(quote! {
    impl#enum_generics PackValue<#enum_name#enum_generics> for Packer {
      fn pack(&mut self, s:#enum_name#enum_generics) -> Result<(), BoltError> {
        match s {
          #(#enum_name::#variant_names #unpack_fields => {
            #(#encode_fields)*
            Ok(())
          }),*
        }
      }
    }

    impl#enum_generics UnpackValue<#enum_name#enum_generics> for Unpacker {
      fn unpack(&mut self) -> Result<#enum_name#enum_generics, BoltError> {
        self
          .unpack_struct_header()
          .and_then(|_| self.unpack_struct_signature())
          .and_then(|signature| {
            match signature {
              #(_ => {
                #(#decode_fields)*
                Ok(#enum_name::#variant_names #unpack_fields)
              }),*
              _ => Err(BoltError::UnknowEnumVariantSignature)
            }
            // TODO:
            //   - unpack each fields
            //   - create enum variant from signature
            //   - assign variant fields with unpacked values
            //   - returns enum variant
          })
      }
    }
  });

    println!("{}", stream);
    TokenStream::from(quote! {
    #input
  })
}

fn named_fields_context(variant_fields: &Fields)
                        -> (TokenStream2, Vec<TokenStream2>, Vec<TokenStream2>) {
    let VariantFieldsContext { unpack, encode, decode } = variant_fields
        .iter()
        .fold(VariantFieldsContext::new(), |mut ctx, field| {
            ctx.unpack.push(unpack_named_field(field));
            ctx.encode.push(encode_named_field(field));
            ctx.decode.push(decode_named_field(field));
            ctx
        });
    (quote!({#(#unpack),*}), encode, decode)
}

fn unnamed_fields_context(variant_fields: &Fields)
                          -> (TokenStream2, Vec<TokenStream2>, Vec<TokenStream2>) {
    let VariantFieldsContext { unpack, encode, decode } = variant_fields
        .iter()
        .enumerate()
        .fold(VariantFieldsContext::new(), |mut ctx, (i, _)| {
            ctx.unpack.push(unpack_unnamed_field(i));
            ctx.encode.push(encode_unnamed_field(i));
            ctx.decode.push(decode_unnamed_field(i));
            ctx
        });
    (quote!((#(#unpack),*)), encode, decode)
}

fn encode_named_field(f: &syn::Field) -> TokenStream2 {
    let field = &f.ident;
    quote!(self.pack(#field)?;)
}

fn encode_unnamed_field(i: usize) -> TokenStream2 {
    let ident = gen_ident(i);
    quote!(self.pack(#ident)?;)
}

fn decode_named_field(f: &Field) -> TokenStream2 {
    let field = &f.ident;
    quote!(let #field = self.unpack()?;)
}

fn decode_unnamed_field(i: usize) -> TokenStream2 {
    let ident = gen_ident(i);
    quote!(let #ident = self.unpack()?;)
}

fn unpack_named_field(f: &Field) -> TokenStream2 {
    let field = &f.ident;
    quote!(#field)
}

fn unpack_unnamed_field(i: usize) -> TokenStream2 {
    let ident = gen_ident(i);
    quote!(#ident)
}

fn gen_ident(i: usize) -> Ident {
    match ALPHA.get(i..i + 1) {
        Some(ref c) => Ident::new(c, Span::call_site()),
        _ => panic!("Unnamed variant field overflow")
    }
}

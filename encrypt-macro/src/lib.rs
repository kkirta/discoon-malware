use aes::cipher::{block_padding::Pkcs7, generic_array::GenericArray, BlockEncryptMut, KeyIvInit};
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, LitStr};
type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;

extern crate proc_macro;

#[proc_macro]
pub fn encrypt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let plain_text = parse_macro_input!(input as LitStr).value();

    let key_bytes = base64::decode("S2FQZFNnVWtYcDJzNXY4eS9CP0UoSCtNYlFlVGhXbVk=").unwrap();
    let key = GenericArray::from_slice(key_bytes.as_slice());

    let iv_bytes = base64::decode("S2FQZFNnVmtZcDNzNnY5eQ==").unwrap();
    let iv = GenericArray::from_slice(iv_bytes.as_slice());

    let cipher_text =
        Aes256CbcEnc::new(&key, &iv).encrypt_padded_vec_mut::<Pkcs7>(plain_text.as_bytes());

    let lit = syn::LitStr::new(base64::encode(&cipher_text).as_str(), Span::call_site());
    return quote! { #lit }.into();
}

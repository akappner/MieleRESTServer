use cbc;
//use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit, generic_array::GenericArray,};
use aes::{cipher::{BlockDecryptMut, KeyIvInit, BlockEncryptMut}, Aes256};
use rand::prelude::*;
use std::fmt;
use aes::cipher::{block_padding::NoPadding,KeyInit};

use sha2::{Sha256, Digest};
use hmac::{digest::Update, Hmac};

struct GroupId([u8; 8]);

impl GroupId {
    fn random () -> Self
    {
        let mut rng = rand::rng();
        return GroupId { 0: rng.random()}
    }
    fn from_hex (s: &str)->Self
    {
        GroupId {0: hex::decode(s).unwrap().try_into().unwrap()}
    }
}

impl fmt::Display for GroupId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}
struct MieleKey([u8; 64]); // first 32 bytes are used as AES key. ALl 64 bytes are used for signature.

impl MieleKey
{
    fn get_aes_key (&self)->[u8; 32]
    {
        return self.0[0..32].try_into().unwrap();
    }
}

struct MieleHmac ([u8; 32]);

pub struct AesIv ([u8; 16]);
impl AesIv {
    fn random () -> Self
    {
        let mut rng = rand::rng();
        return AesIv { 0: rng.random()}
    }
}
struct MieleSignature 
{
    hmac : MieleHmac,
}
impl MieleSignature {
    fn from_hex (s: &str)->MieleSignature
    {
        let b = hex::decode(s).unwrap();
        println!("b length {}", b.len());
        MieleSignature { hmac: MieleHmac{0: b[0..32].try_into().unwrap()} } 
    }
    
    fn get_aes_iv (&self)->AesIv
    {
        return AesIv{0: self.hmac.0[0..16].try_into().unwrap()}
    }
}
impl fmt::Display for MieleKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl MieleKey {
    fn random () -> Self
    {
        let mut rng = rand::rng();
        return MieleKey { 0: rng.random()}
    }
}

struct MieleCryptoContext
{
    group_id : GroupId,
    group_key : MieleKey,
}

type encryptor = cbc::Encryptor<aes::Aes256>;
type decryptor = cbc::Decryptor<aes::Aes256>;
type signer = Hmac<Sha256>;

impl MieleCryptoContext 
{
    fn default() -> Self
    {
        return MieleCryptoContext {group_id: GroupId{0: [0x31; 8]}, group_key: MieleKey{0: [0x31; 64]}};
    }
    fn random () -> Self
    {
        let mut rng = rand::rng();
        return MieleCryptoContext {group_id: GroupId::random(), group_key: MieleKey::random()};
    }
    fn signature (&self, buffer: &[u8])->[u8; 32]
    {
        let mut mac = signer::new_from_slice(&self.group_key.0).unwrap();
        //mac.update(buffer);
        mac.update(&buffer);
        //return mac.finalize().into_bytes();   
        let signature = hmac::Mac::finalize(mac).into_bytes();
        return signature.into()
    }
    fn decrypt (&self, buffer : Vec<u8>, iv : &AesIv)-> Vec<u8>
    {
        println!("buffer len: {:?}", buffer.len());
        let mut d: cbc::Decryptor<Aes256> = decryptor::new_from_slices(&self.group_key.get_aes_key(), &iv.0).unwrap();
        return d.decrypt_padded_vec_mut::<NoPadding>(buffer.as_slice())
.unwrap()    }
    fn encrypt (&self, buffer : Vec<u8>, iv : &AesIv)->Vec<u8>
    {
        let mut e: cbc::Encryptor<Aes256> = encryptor::new_from_slices(&self.group_key.get_aes_key(), &iv.0).unwrap();
        return e.encrypt_padded_vec_mut::<NoPadding>(buffer.as_slice());
    }
}
impl std::fmt::Display for MieleCryptoContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}:{})", self.group_id, self.group_key)
    }
}

struct MieleHeader{
    group_id : GroupId,
    signature : MieleSignature,
}
impl MieleHeader
{
    fn to_http_header(&self) -> String
    {
        return "X-Signature: MieleH256 1111111111111111:DD361380F1AB6BC95C3A42144DA458CB58A204A4A509E14B59B7690D1846AAE3".to_string();
    }
    fn from_http_header(s: String) -> Self
    {
        let parts : Vec<&str> = s.split(':').collect();
        return MieleHeader {group_id: GroupId::from_hex(parts[0]), signature: MieleSignature::from_hex(parts[1])}
    }
}
mod tests {
    use super::*;
    #[test]
    fn test_round_trip() {
       let plaintextString = "0123456789012345678901234567890123456789012345678901234567891234";
       let context : MieleCryptoContext = MieleCryptoContext::random(); 
       let iv = AesIv::random();
       let ciphertext = context.encrypt(plaintextString.as_bytes().into(), &iv);
       let plaintext = context.decrypt(ciphertext, &iv);
       //println!("{:}", context);
       assert_eq!(plaintextString.as_bytes(), plaintext);
       
    }
    #[test]
    fn test_decryption()
    {
        let testKey = hex::decode("123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE").unwrap();
        let header = MieleHeader::from_http_header("123456789ABCDEFE:9258984775FF6450CF8D943A946C36C850FAAB65DEFCD2EAC6E7262B58308B47".to_string());
        let context = MieleCryptoContext {group_id: GroupId::from_hex("123456789ABCDEFE"), group_key: MieleKey{0: testKey.try_into().unwrap()} } ;
        
//        let ciphertext = hex::decode("f6eebe5e2bf7c5064c4d61c0da55c7e80010f700bd8b5d5c958e8165ab025bd5f65a002044ef3e573d2bfd1ee3eef862cb96115100307c472b5c7389793a6d713249b056231f0040e865b7931033e679f46c6a97ba6f58840050d58d0dc367e557f675d4092fb3254cb60060e9c0e4ca99b5c0a34df73a8802004cf90070b7fca41d0cbc521792df8ae4a0fc3e0e0080fefbc1d6550a7a66c13334680de6066c").unwrap();
        let ciphertext = hex::decode("8dc821a1c9eced3fa98fd74e0d6629b9ee41543376ea08dec33acca7949f6b1f812e2b828dae8c72f7ae0fa7670fa38a0ec8fe10e42988df0f09fa0815c2e2ee").unwrap();
       
        let plaintext = context.decrypt(ciphertext, &header.signature.get_aes_iv());

        //assert_eq!(hex::encode(context.signature(&plaintext)), hex::encode(header.signature.hmac.0));
        println!("{:?}", plaintext);
        println!("{:?}", str::from_utf8(&plaintext).unwrap());
        assert_eq!(&plaintext, "{\"DeviceAction\": 2                                             }".to_string().as_bytes());

       
    }
}

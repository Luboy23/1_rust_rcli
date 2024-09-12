use anyhow::{Ok, Result};
use std::io::Read;
use std::fs;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use crate::{get_reader, TextSignFormat};

trait TextSign {
    // sign the data from the reader and return signature
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
}

struct Blake3 {
    key: [u8; 32],
}

// struct Ed25519Signer {
//     key: [u8; 32],
// }

// struct Ed25519Verifier {
//     key: [u8; 32],
// }

pub fn process_text_sign(input: &str, key: &str, format:TextSignFormat) -> Result<()> {
    let mut reader = get_reader(input)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let key = fs::read(key)?;
            let key = &key[..32];
            let key = key.try_into().unwrap();
            let signer = Blake3 { key };
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => todo!(),
        };
        let signed = URL_SAFE_NO_PAD.encode(&signed);
        println!("{}", signed);
        Ok(())
    }

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key,&buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::hash(&buf);
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}
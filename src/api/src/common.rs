use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};

pub fn decrypt_data(data: &str, key: &str) -> Result<String, &'static str> {
    let (encrypted_data, random_string) = data.split_at(data.len() - 8);
    let mut cipher = aes::Aes256::new(GenericArray::from_slice(key.as_bytes()));
    let mut buffer = hex::decode(encrypted_data).map_err(|_| "Failed to decode hex")?;
    cipher.decrypt_block(GenericArray::from_mut_slice(&mut buffer));
    let decrypted_data = String::from_utf8(buffer).map_err(|_| "Decryption result is not valid UTF-8")?;
    if !decrypted_data.ends_with(random_string) {
        return Err("Decryption failed: random string mismatch");
    }
    let decrypted_data = decrypted_data.trim_end_matches(random_string);
    Ok(decrypted_data.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_data() {

        let encrypted_data = "1545F33FDDD1E326254EC651FA98883FSxSslBca";
        let key = "8ea8593bb2e44ccda1ccbb1fa07db5b6";
        let expected_decrypted_data = "ud123";
        let result = decrypt_data(encrypted_data, key);
        assert_eq!(result, Ok(expected_decrypted_data.to_string()));
    }
}


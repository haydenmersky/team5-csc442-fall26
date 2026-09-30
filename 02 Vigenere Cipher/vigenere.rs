fn encode(text: &str, key: &str) -> String {
    let key: Vec<u8> = key
        .bytes()
        .filter(|b| b.is_ascii_alphabetic())
        .map(|b| b.to_ascii_uppercase())
        .collect();

    let mut result = String::new();
    let mut key_index = 0;

    for ch in text.chars() {
        if ch.is_ascii_alphabetic() {
            let base = if ch.is_ascii_uppercase() { b'A' } else { b'a' };

            let plaintext = ch as u8 - base;
            let key_value = key[key_index % key.len()] - b'A';

            let encrypted = (plaintext + key_value) % 26 + base;

            result.push(encrypted as char);
            key_index += 1;
        } else {
            result.push(ch);
        }
    }

    result
}


fn decode(text: &str, key: &str) -> String {
    let key: Vec<u8> = key
        .bytes()
        .filter(|b| b.is_ascii_alphabetic())
        .map(|b| b.to_ascii_uppercase())
        .collect();

    let mut result = String::new();
    let mut key_index = 0;

    for ch in text.chars() {
        if ch.is_ascii_alphabetic() {
            let base = if ch.is_ascii_uppercase() { b'A' } else { b'a' };

            let ciphertext = ch as u8 - base;
            let key_value = key[key_index % key.len()] - b'A';

            let decrypted = (ciphertext + 26 - key_value) % 26 + base;

            result.push(decrypted as char);
            key_index += 1;
        } else {
            result.push(ch);
        }
    }

    result
}

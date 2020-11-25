use std::io;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use rand::SeedableRng;

fn main() {
    let message: String = String::from("Hello Wordl!");
    let key: String = String::from("dgjuc");

    let alphabet: [char; 52] = [
        'a', 'b', 'c', 'd', 'e',
        'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't',
        'u', 'v', 'w', 'x', 'y',
        'z', 'd', 'g', 'b', 'a',
        'i', 'h', 'c', 'f', 'e',
        'n', 'm', 'q', 'k', 'j',
        's', 'r', 'l', 'p', 'o',
        'x','w', 'z', 'u', 't',
        'v', 'y',
    ];

    let mut expanded_key: String = shuffled(key, alphabet);
    expanded_key = prng(expanded_key, message.len() as usize);

    println!("{}", expanded_key);

    let ciphertext: String = encrypt(message, expanded_key);
    println!("Hi, here is the ciphertext: {} ", ciphertext);

     let key_two: String = enter_key();

     let message_decrypt: String = decrypt(ciphertext, key_two);
     println!("Hi, your message is: {} ", message_decrypt);
     println!("Thank you for use Stream Cipher simulator!");
}

fn prng(expanded_key:String, len_m:usize) -> String{

    let rand: String = rand_pcg::Pcg32::seed_from_u64(123)
        .sample_iter(&Alphanumeric)
        .take(len_m - expanded_key.len() as usize)
        .collect();

    expanded_key + rand.as_str()
}

fn shuffled(key: String, alphabet: [char; 52]) -> String {

    let key_char: Vec<char> = key.chars().collect();
    let mut key_expanded: String = String::new();

    let mut i = 0;
    while i != key_char.len() {
        let mut count = 0;
        let mut j = 0;
        while j != alphabet.len() {
            if key_char[i] != alphabet[count] {
                count = count + 1;
            }
            j = j + 1;
        }
        if key_char[i] == " ".parse().unwrap(){
            key_expanded.push("@".parse().unwrap());
        }
        key_expanded.push(alphabet[count+26].to_string().parse().unwrap());

        i = i + 1;
    }

    key_expanded
}


fn enter_key() -> String {
    println!("Hi, enter the key to decryp message");
    let mut opcion = String::new();
    io::stdin().read_line(&mut opcion);

    opcion
}

fn encrypt(message: String, key: String) -> String {
    let mut ciphertext = String::new();
    let message_char: Vec<char> = message.chars().collect();
    let key_char: Vec<char> = key.chars().collect();

    let mut xor_aux: u32;

    for i in 0..key.chars().count() {
        xor_aux = u32::from(message_char[i]) ^ u32::from(key_char[i]);
        let help_to_push = std::char::from_u32(xor_aux);
        ciphertext.push(*help_to_push.as_ref().unwrap());
    }
    return String::from(ciphertext.as_str());
}


fn decrypt(ciphertext: String, key_two: String) -> String {
    let mut message = String::new();
    let ciphertext_char: Vec<char> = ciphertext.chars().collect();
    let key_char: Vec<char> = key_two.chars().collect();
    let mut xor_aux: u32;

    for i in 0..ciphertext.chars().count() {
        xor_aux = u32::from(ciphertext_char[i]) ^ u32::from(key_char[i]);
        let help_to_push = std::char::from_u32(xor_aux);
        message.push(*help_to_push.as_ref().unwrap());
    }
    return String::from(message.as_str());
}

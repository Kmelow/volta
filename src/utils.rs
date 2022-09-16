use rand::Rng;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

/// clip is a wrapper arround the [clipboard](https://lib.rs/crates/clipboard) crate
/// that sets the passed word to the clipboard
pub fn clip(w: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(w).unwrap();
}

/// mask_pass returns the word in the argument displaying the first `unmasked` characters
/// the rest is replaced by 5 "*"
pub fn mask_pass(p: String, unmasked: Option<usize>) -> String {
    let un = unmasked.unwrap_or(3);
    if un > p.len() {
        p
    } else {
        p[0..un].to_string() + &vec!['*'; 5].iter().collect::<String>()
    }
}

/// random_pass is a very **unsecure** and **naive** way to generate a random passwod
pub fn random_pass() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 16;
    let mut rng = rand::thread_rng();

    (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

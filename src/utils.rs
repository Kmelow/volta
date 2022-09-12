use rand::Rng;
// use serde::Serialize;

// pub fn stringify<T: Serialize> (o: T) -> String {
//     let json = serde_json::to_string(&o);
//     match json {
//         Ok(j) => j,
//         _ => String::from("Error")
//     }
// }

// pub fn parse<T: Deserialize> (s: String, o: T) -> T {
//     let s = serde_json::from_str(&s);
//     match s {
//         Ok(obj) => obj,
//         _ => o
//     }
// }

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

const DICTIONARY: [(char, &'static str); 3] = [
    ('l', "w"), 
    ('n', "wn"),
    ('v', "ww"),
];

pub fn translate(english: &str) -> String {
    let mut uwu = english.to_owned();

    for (old, new) in DICTIONARY {
        uwu = uwu.replace(old, new);
    }

    uwu
}

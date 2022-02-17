use std::borrow::Cow;

// actual tranlation lookup table
const DICTIONARY: [(char, &'static str); 3] = [
    // FORM
    // (<old>, <new>),
    ('l', "w"), 
    ('n', "wn"),
    ('v', "ww"),
];

// translate any language to uwu
pub fn translate<'s, S>(normal: S) -> String
where S: Into<Cow<'s, str>> {
    let normal = normal.into();
    let mut uwu = normal.into_owned();

    // apply translation table `DICTIONARY`
    for (old, new) in DICTIONARY {
        uwu = uwu.replace(old, new);
    }

    uwu
}

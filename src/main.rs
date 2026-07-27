use std::io::{self, Read};

/// Russian letter → English key it shares on a standard keyboard layout.
const RU_TO_EN: &[(char, char)] = &[
    ('й', 'q'),
    ('ц', 'w'),
    ('у', 'e'),
    ('к', 'r'),
    ('е', 't'),
    ('н', 'y'),
    ('г', 'u'),
    ('ш', 'i'),
    ('щ', 'o'),
    ('з', 'p'),
    ('ф', 'a'),
    ('ы', 's'),
    ('в', 'd'),
    ('а', 'f'),
    ('п', 'g'),
    ('р', 'h'),
    ('о', 'j'),
    ('л', 'k'),
    ('д', 'l'),
    ('ж', ';'),
    ('э', '\''),
    ('я', 'z'),
    ('ч', 'x'),
    ('с', 'c'),
    ('м', 'v'),
    ('и', 'b'),
    ('т', 'n'),
    ('ь', 'm'),
    ('б', ','),
    ('ю', '.'),
];

/// English key → Russian letter.
const EN_TO_RU: &[(char, char)] = &[
    ('q', 'й'),
    ('w', 'ц'),
    ('e', 'у'),
    ('r', 'к'),
    ('t', 'е'),
    ('y', 'н'),
    ('u', 'г'),
    ('i', 'ш'),
    ('o', 'щ'),
    ('p', 'з'),
    ('a', 'ф'),
    ('s', 'ы'),
    ('d', 'в'),
    ('f', 'а'),
    ('g', 'п'),
    ('h', 'р'),
    ('j', 'о'),
    ('k', 'л'),
    ('l', 'д'),
    (';', 'ж'),
    ('\'', 'э'),
    ('z', 'я'),
    ('x', 'ч'),
    ('c', 'с'),
    ('v', 'м'),
    ('b', 'и'),
    ('n', 'т'),
    ('m', 'ь'),
    (',', 'б'),
    ('.', 'ю'),
];

const RU_LETTERS: &[char] = &[
    'й', 'ц', 'у', 'к', 'е', 'н', 'г', 'ш', 'щ', 'з',
    'ф', 'ы', 'в', 'а', 'п', 'р', 'о', 'л', 'д', 'ж',
    'э', 'я', 'ч', 'с', 'м', 'и', 'т', 'ь', 'б', 'ю',
];

fn translate(text: &str, table: &[(char, char)]) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        let lower = ch.to_ascii_lowercase();
        if let Some(&(src, dst)) = table.iter().find(|(k, _)| *k == lower) {
            debug_assert_eq!(src, lower);
            out.push(if ch.is_uppercase() { dst.to_ascii_uppercase() } else { dst });
        } else {
            out.push(ch);
        }
    }
    out
}

/// Count how many characters belong to the given letter set.
fn count_letters(text: &str, letters: &[char]) -> usize {
    text.chars().filter(|c| letters.contains(&c.to_ascii_lowercase())).count()
}

fn detect_and_translate(text: &str) -> String {
    let ru = count_letters(text, RU_LETTERS);
    let en = text.chars().filter(|c| c.is_ascii_alphabetic()).count();
    if ru > en {
        // typed on a Russian layout — convert to English letters
        translate(text, RU_TO_EN)
    } else {
        // English layout — convert to Russian
        translate(text, EN_TO_RU)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "--help" {
        println!(
            "rus2eng — detect keyboard layout and translate between RU ↔ EN\n\
             \n\
             Usage:\n\
             rus2eng <text>          translate a single string (join multiple args with spaces)\n\
             rus2eng -               read lines from stdin (Ctrl+D to finish)\n\
             rus2eng file.txt        translate every line of an existing file"
        );
        return;
    }

    let input = if args.len() > 1 {
        if args[1] == "-" {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf).expect("cannot read stdin");
            buf
        } else if std::path::Path::new(&args[1]).is_file() {
            // Looks like an existing file — read and translate its contents.
            std::fs::read_to_string(&args[1]).unwrap_or_else(|e| {
                eprintln!("error: {}", e);
                std::process::exit(1);
            })
        } else {
            // Not an existing file — treat all remaining args as the text itself.
            args[1..].join(" ")
        }
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).expect("cannot read stdin");
        buf
    };

    for line in input.lines() {
        println!("{}", detect_and_translate(line));
    }
}

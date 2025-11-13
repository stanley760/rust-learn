use crate::operation::GenPwdOpts;
use anyhow::Ok;
use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>/?";

pub fn parse_gen_pwd(opts: &GenPwdOpts) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut pwd = Vec::new();
    let mut chars = Vec::new();

    if opts.lowercase {
        chars.extend_from_slice(LOWERCASE);
        pwd.push(
            *LOWERCASE
                .choose(&mut rng)
                .expect("char won't be empty in lowercase."),
        );
    }
    if opts.uppercase {
        chars.extend_from_slice(UPPERCASE);
        pwd.push(
            *UPPERCASE
                .choose(&mut rng)
                .expect("char won't be empty in uppercase."),
        );
    }

    if opts.numbers {
        chars.extend_from_slice(NUMBERS);
        pwd.push(
            *NUMBERS
                .choose(&mut rng)
                .expect("char won't be empty in numbers."),
        );
    }

    if opts.symbols {
        chars.extend_from_slice(SYMBOLS);
        pwd.push(
            *SYMBOLS
                .choose(&mut rng)
                .expect("char won't be empty in symbol."),
        );
    }

    for _ in 0..(opts.length - pwd.len() as u8) {
        let c = *chars
            .choose(&mut rng)
            .expect("char won't be empty in context.");
        pwd.push(c);
    }

    pwd.shuffle(&mut rng);
    let password = String::from_utf8(pwd)?;
    println!("{}", password);

    let estimate = zxcvbn(&password, &[]);
    eprintln!("password level: {:?}", estimate.score() as u8);
    Ok(())
}

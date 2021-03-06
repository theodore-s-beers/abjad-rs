//! This library is meant to facilitate calculating the
//! [numerical _abjad_ value](https://en.wikipedia.org/wiki/Abjad_numerals)
//! of a string of text in Arabic or Persian (support for other Arabic-script
//! languages may be added over time).
//!
//! At the moment, this simply adds three methods for `&str`:
//!
//! - `abjad` returns a best-effort value, ignoring unrecognized characters.
//! - `abjad_collect_errors` also records unrecognized characters in a `Vec`.
//! - `abjad_strict` returns an error as soon as any character is not recognized.
//!

#![deny(missing_docs)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::fn_params_excessive_bools, clippy::struct_excessive_bools)]

use thiserror::Error;

/// The error type for this crate. Currently there is only one member:
/// `UnrecognizedCharacter`, which is returned by `abjad_strict` upon encountering
/// any character outside of the Arabic script.
#[derive(Error, Debug)]
pub enum AbjadError {
    /// This error is returned by `abjad_strict` upon encountering any character
    /// outside of the Arabic script. It reports the Unicode escape sequence for
    /// the character in question.
    #[error("Unrecognized character: {0}")]
    UnrecognizedCharacter(String),
}

/// We need to allow some options for _abjad_ calculation. At present there are
/// four. All are false by default. If you don't need to activate any of them,
/// when calling one of the methods, you can pass `AbjadPrefs::default()`.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AbjadPrefs {
    /// Count the [_shaddah_](https://en.wikipedia.org/wiki/Shadda) diacritic?
    /// This will have the effect of doubling the value of the preceding letter.
    pub count_shaddah: bool,

    /// Count [_alif maddah_](https://en.wiktionary.org/wiki/maddah) as a double
    /// _alif_ (with value 2 instead of 1)?
    pub double_alif_maddah: bool,

    /// Ignore the pseudo-letter [_hamzah_](https://en.wikipedia.org/wiki/Hamza)
    /// in its isolated state? (By default we assign it a value of 1.)
    pub ignore_lone_hamzah: bool,

    /// Use the Maghribi letter order? (Unless you're sure you need this, you
    /// don't.)
    pub maghribi_order: bool,
}

/// This is the trait that we implement for `&str`, allowing us to use the new
/// methods.
pub trait Abjad {
    /// This returns a best-effort value, ignoring unrecognized characters.
    fn abjad(self, prefs: AbjadPrefs) -> u32;

    /// This returns a tuple, with unrecognized characters (Unicode-escaped)
    /// in a `Vec`.
    fn abjad_collect_errors(self, prefs: AbjadPrefs) -> (u32, Vec<String>);

    /// # Errors
    /// This returns an error as soon as any unrecognized character is encountered.
    fn abjad_strict(self, prefs: AbjadPrefs) -> Result<u32, AbjadError>;
}

impl Abjad for &str {
    fn abjad(self, prefs: AbjadPrefs) -> u32 {
        let mut abjad_total: u32 = 0;
        let mut last_value: u32 = 0;

        for character in self.chars() {
            if let Ok(new_value) = get_letter_value(
                character,
                last_value,
                prefs.count_shaddah,
                prefs.double_alif_maddah,
                prefs.ignore_lone_hamzah,
                prefs.maghribi_order,
            ) {
                abjad_total += new_value;
                last_value = new_value;
            } else {
                last_value = 0;
            }
        }

        abjad_total
    }

    fn abjad_collect_errors(self, prefs: AbjadPrefs) -> (u32, Vec<String>) {
        let mut abjad_total: u32 = 0;
        let mut errors: Vec<String> = Vec::new();
        let mut last_value: u32 = 0;

        for character in self.chars() {
            if let Ok(new_value) = get_letter_value(
                character,
                last_value,
                prefs.count_shaddah,
                prefs.double_alif_maddah,
                prefs.ignore_lone_hamzah,
                prefs.maghribi_order,
            ) {
                abjad_total += new_value;
                last_value = new_value;
            } else {
                errors.push(character.escape_unicode().collect());
                last_value = 0;
            }
        }

        (abjad_total, errors)
    }

    fn abjad_strict(self, prefs: AbjadPrefs) -> Result<u32, AbjadError> {
        let mut abjad_total: u32 = 0;
        let mut last_value: u32 = 0;

        for character in self.chars() {
            let new_value = get_letter_value(
                character,
                last_value,
                prefs.count_shaddah,
                prefs.double_alif_maddah,
                prefs.ignore_lone_hamzah,
                prefs.maghribi_order,
            )?;

            abjad_total += new_value;
            last_value = new_value;
        }

        Ok(abjad_total)
    }
}

fn get_letter_value(
    character: char,
    last_value: u32,
    count_shaddah: bool,
    double_alif_maddah: bool,
    ignore_lone_hamzah: bool,
    maghribi_order: bool,
) -> Result<u32, AbjadError> {
    let mut letter_value: u32 = 0;

    match character {
        '??' | '??' | '??' | '??' => letter_value = 1,
        '??' => {
            if double_alif_maddah {
                letter_value = 2;
            } else {
                letter_value = 1;
            }
        }
        '??' => {
            if !ignore_lone_hamzah {
                letter_value = 1;
            }
        }
        '??' | '??' => letter_value = 2,
        '??' | '??' => letter_value = 3,
        '??' => letter_value = 4,
        '??' | '??' | '??' => letter_value = 5,
        '??' | '??' => letter_value = 6,
        '??' | '??' => letter_value = 7,
        '??' => letter_value = 8,
        '??' => letter_value = 9,
        '??' | '??' | '??' | '??' => letter_value = 10,
        '??' | '??' | '??' => letter_value = 20,
        '??' => letter_value = 30,
        '??' => letter_value = 40,
        '??' => letter_value = 50,
        '??' => {
            if maghribi_order {
                letter_value = 300;
            } else {
                letter_value = 60;
            }
        }
        '??' => letter_value = 70,
        '??' => letter_value = 80,
        '??' => {
            if maghribi_order {
                letter_value = 60;
            } else {
                letter_value = 90;
            }
        }
        '??' => letter_value = 100,
        '??' => letter_value = 200,
        '??' => {
            if maghribi_order {
                letter_value = 1000;
            } else {
                letter_value = 300;
            }
        }
        '??' => letter_value = 400,
        '??' => letter_value = 500,
        '??' => letter_value = 600,
        '??' => letter_value = 700,
        '??' => {
            if maghribi_order {
                letter_value = 90;
            } else {
                letter_value = 800;
            }
        }
        '??' => {
            if maghribi_order {
                letter_value = 800;
            } else {
                letter_value = 900;
            }
        }
        '??' => {
            if maghribi_order {
                letter_value = 900;
            } else {
                letter_value = 1000;
            }
        }
        // Shaddah diacritic
        '\u{0651}' => {
            if count_shaddah {
                letter_value = last_value;
            }
        }
        // Space or zwnj is ok
        ' ' | '\u{200C}' => {}
        // Otherwise return error
        _ => {
            let escaped: String = character.escape_unicode().collect();
            return Err(AbjadError::UnrecognizedCharacter(escaped));
        }
    }

    Ok(letter_value)
}

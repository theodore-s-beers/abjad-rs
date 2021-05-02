use anyhow::{anyhow, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        let input = "ابجد هوز حطي كلمن سعفص قرشت ثخذ ضظغ";

        let prefs_mashriqi: AbjadPrefs = Default::default();
        let prefs_maghribi = AbjadPrefs {
            maghribi_order: true,
            ..Default::default()
        };

        let total_mashriqi = input.abjad_strict(prefs_mashriqi).unwrap();
        let total_maghribi = input.abjad_strict(prefs_maghribi).unwrap();

        assert_eq!(total_mashriqi, 5995);
        assert_eq!(total_mashriqi, total_maghribi);
    }

    #[test]
    fn baha_count() {
        let input = "بهاء";
        let prefs: AbjadPrefs = Default::default();

        assert_eq!(input.abjad_strict(prefs).unwrap(), 9);
    }

    #[test]
    fn baha_ignore() {
        let input = "بهاء";
        let prefs = AbjadPrefs {
            ignore_lone_hamzah: true,
            ..Default::default()
        };

        assert_eq!(input.abjad_strict(prefs).unwrap(), 8);
    }

    #[test]
    fn basmala() {
        let input = "بسم الله الرحمن الرحيم";
        let prefs: AbjadPrefs = Default::default();

        assert_eq!(input.abjad_strict(prefs).unwrap(), 786);
    }

    #[test]
    fn humayun() {
        let input = "همایون پادشاه از بام افتاد";
        let prefs: AbjadPrefs = Default::default();

        assert_eq!(input.abjad_strict(prefs).unwrap(), 962);
    }

    #[test]
    fn latin() {
        let input = "the quick brown fox";
        let prefs: AbjadPrefs = Default::default();

        assert_eq!(input.abjad(prefs), 0);
    }

    #[test]
    fn latin_report() {
        let input = "the quick brown fox";
        let prefs: AbjadPrefs = Default::default();

        let (total, errors) = input.abjad_collect_errors(prefs);

        assert_eq!(total, 0);
        assert_eq!(errors.len(), 16);
    }

    #[test]
    fn mixture() {
        let input = "روح الله tapdancing خمینی";
        let prefs: AbjadPrefs = Default::default();

        assert_eq!(input.abjad(prefs), 990);
    }

    #[test]
    fn mixture_fail() {
        let input = "روح الله tapdancing خمینی";
        let prefs: AbjadPrefs = Default::default();

        assert!(input.abjad_strict(prefs).is_err());
    }

    #[test]
    fn mixture_report() {
        let input = "روح الله tapdancing خمینی";
        let prefs: AbjadPrefs = Default::default();

        let (total, errors) = input.abjad_collect_errors(prefs);

        assert_eq!(total, 990);
        assert_eq!(errors.len(), 10);
    }

    #[test]
    fn shaddah() {
        let input = "رئیس مؤسّس دانشگاه";
        let prefs = AbjadPrefs {
            count_shaddah: true,
            ..Default::default()
        };

        assert_eq!(input.abjad_strict(prefs).unwrap(), 887);
    }

    #[test]
    fn tammamtu() {
        let input = "قد تمّمته";
        let prefs: AbjadPrefs = Default::default();

        assert_eq!(input.abjad_strict(prefs).unwrap(), 989);
    }

    #[test]
    fn vahshi() {
        let input = "وفات وحشی مسکین";
        let prefs: AbjadPrefs = Default::default();

        assert_eq!(input.abjad_strict(prefs).unwrap(), 991);
    }

    #[test]
    fn zwnj() {
        let input = "عادت می‌کنیم";
        let prefs: AbjadPrefs = Default::default();

        assert_eq!(input.abjad_strict(prefs).unwrap(), 645);
    }
}

#[derive(Default)]
pub struct AbjadPrefs {
    count_shaddah: bool,
    double_alif_maddah: bool,
    ignore_lone_hamzah: bool,
    maghribi_order: bool,
}

pub trait AbjadExt {
    fn abjad(self, prefs: AbjadPrefs) -> u32;

    fn abjad_collect_errors(self, prefs: AbjadPrefs) -> (u32, Vec<String>);

    fn abjad_strict(self, prefs: AbjadPrefs) -> Result<u32>;
}

impl AbjadExt for &str {
    fn abjad(self, prefs: AbjadPrefs) -> u32 {
        let input = self;

        let count_shaddah = prefs.count_shaddah;
        let double_alif_maddah = prefs.double_alif_maddah;
        let ignore_lone_hamzah = prefs.ignore_lone_hamzah;
        let maghribi_order = prefs.maghribi_order;

        let mut abjad_total: u32 = 0;

        let mut last_value: u32 = 0;

        for character in input.chars() {
            if let Ok(new_value) = get_letter_value(
                character,
                last_value,
                count_shaddah,
                double_alif_maddah,
                ignore_lone_hamzah,
                maghribi_order,
            ) {
                abjad_total += new_value;

                last_value = new_value
            } else {
                last_value = 0
            }
        }

        abjad_total
    }

    fn abjad_collect_errors(self, prefs: AbjadPrefs) -> (u32, Vec<String>) {
        let input = self;

        let count_shaddah = prefs.count_shaddah;
        let double_alif_maddah = prefs.double_alif_maddah;
        let ignore_lone_hamzah = prefs.ignore_lone_hamzah;
        let maghribi_order = prefs.maghribi_order;

        let mut abjad_total: u32 = 0;

        let mut errors: Vec<String> = Vec::new();

        let mut last_value: u32 = 0;

        for character in input.chars() {
            if let Ok(new_value) = get_letter_value(
                character,
                last_value,
                count_shaddah,
                double_alif_maddah,
                ignore_lone_hamzah,
                maghribi_order,
            ) {
                abjad_total += new_value;

                last_value = new_value
            } else {
                errors.push(character.escape_unicode().collect());

                last_value = 0
            }
        }

        (abjad_total, errors)
    }

    fn abjad_strict(self, prefs: AbjadPrefs) -> Result<u32> {
        let input = self;

        let count_shaddah = prefs.count_shaddah;
        let double_alif_maddah = prefs.double_alif_maddah;
        let ignore_lone_hamzah = prefs.ignore_lone_hamzah;
        let maghribi_order = prefs.maghribi_order;

        let mut abjad_total: u32 = 0;

        let mut last_value: u32 = 0;

        for character in input.chars() {
            let new_value = get_letter_value(
                character,
                last_value,
                count_shaddah,
                double_alif_maddah,
                ignore_lone_hamzah,
                maghribi_order,
            )?;

            abjad_total += new_value;

            last_value = new_value
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
) -> Result<u32> {
    let mut letter_value: u32 = 0;

    match character {
        'ا' | 'أ' | 'إ' | 'ٱ' => letter_value = 1,
        'آ' => {
            if double_alif_maddah {
                letter_value = 2
            } else {
                letter_value = 1
            }
        }
        'ء' => {
            if !ignore_lone_hamzah {
                letter_value = 1
            }
        }
        'ب' | 'پ' => letter_value = 2,
        'ج' | 'چ' => letter_value = 3,
        'د' => letter_value = 4,
        'ه' | 'ة' | 'ۀ' => letter_value = 5,
        'و' | 'ؤ' => letter_value = 6,
        'ز' | 'ژ' => letter_value = 7,
        'ح' => letter_value = 8,
        'ط' => letter_value = 9,
        'ي' | 'ى' | 'ئ' | 'ی' => letter_value = 10,
        'ك' | 'ک' | 'گ' => letter_value = 20,
        'ل' => letter_value = 30,
        'م' => letter_value = 40,
        'ن' => letter_value = 50,
        'س' => {
            if maghribi_order {
                letter_value = 300;
            } else {
                letter_value = 60;
            }
        }
        'ع' => letter_value = 70,
        'ف' => letter_value = 80,
        'ص' => {
            if maghribi_order {
                letter_value = 60;
            } else {
                letter_value = 90;
            }
        }
        'ق' => letter_value = 100,
        'ر' => letter_value = 200,
        'ش' => {
            if maghribi_order {
                letter_value = 1000
            } else {
                letter_value = 300
            }
        }
        'ت' => letter_value = 400,
        'ث' => letter_value = 500,
        'خ' => letter_value = 600,
        'ذ' => letter_value = 700,
        'ض' => {
            if maghribi_order {
                letter_value = 90
            } else {
                letter_value = 800
            }
        }
        'ظ' => {
            if maghribi_order {
                letter_value = 800
            } else {
                letter_value = 900
            }
        }
        'غ' => {
            if maghribi_order {
                letter_value = 900
            } else {
                letter_value = 1000
            }
        }
        // Next is the shaddah diacritic; will probably look funny
        'ّ' => {
            if count_shaddah {
                letter_value = last_value;
            }
        }
        // Space or zwnj is ok
        ' ' | '‌' => {}
        // Otherwise return error
        _ => {
            let escaped: String = character.escape_unicode().collect();
            return Err(anyhow!("Unrecognized character: {}", escaped));
        }
    }

    Ok(letter_value)
}

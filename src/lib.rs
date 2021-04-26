#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baha_count() {
        let input = "بهاء".to_string();
        let prefs: AbjadPrefs = Default::default();

        assert_eq!(abjad(input, prefs), 9);
    }

    #[test]
    fn baha_ignore() {
        let input = "بهاء".to_string();
        let prefs = AbjadPrefs {
            ignore_lone_hamzah: true,
            ..Default::default()
        };

        assert_eq!(abjad(input, prefs), 8);
    }

    #[test]
    fn basmala() {
        let input = "بسم الله الرحمن الرحيم".to_string();
        let prefs: AbjadPrefs = Default::default();

        assert_eq!(abjad(input, prefs), 786);
    }

    #[test]
    fn humayun() {
        let input = "همایون پادشاه از بام افتاد".to_string();
        let prefs: AbjadPrefs = Default::default();

        assert_eq!(abjad(input, prefs), 962);
    }

    #[test]
    fn latin() {
        let input = "the quick brown fox".to_string();
        let prefs: AbjadPrefs = Default::default();

        assert_eq!(abjad(input, prefs), 0);
    }

    #[test]
    fn mixture() {
        let input = "روح الله tapdancing خمینی".to_string();
        let prefs: AbjadPrefs = Default::default();

        assert_eq!(abjad(input, prefs), 990);
    }

    #[test]
    fn shaddah() {
        let input = "رئیس مؤسّس دانشگاه".to_string();
        let prefs = AbjadPrefs {
            count_shaddah: true,
            ..Default::default()
        };

        assert_eq!(abjad(input, prefs), 887);
    }

    #[test]
    fn vahshi() {
        let input = "وفات وحشی مسکین".to_string();
        let prefs: AbjadPrefs = Default::default();

        assert_eq!(abjad(input, prefs), 991);
    }
}

#[derive(Default)]
pub struct AbjadPrefs {
    pub count_shaddah: bool,
    pub double_alif_maddah: bool,
    pub ignore_lone_hamzah: bool,
    pub maghribi_order: bool,
}

pub fn abjad(input: String, prefs: AbjadPrefs) -> u32 {
    let count_shaddah = prefs.count_shaddah;
    let double_alif_maddah = prefs.double_alif_maddah;
    let ignore_lone_hamzah = prefs.ignore_lone_hamzah;
    let maghribi_order = prefs.maghribi_order;

    let mut abjad_total: u32 = 0;
    let mut last_value: u32 = 0;

    for character in input.chars() {
        match character {
            'ا' | 'أ' | 'إ' | 'ٱ' => {
                abjad_total += 1;
                last_value = 1
            }
            'آ' => {
                if double_alif_maddah {
                    abjad_total += 2;
                    last_value = 2
                } else {
                    abjad_total += 1;
                    last_value = 1
                }
            }
            'ء' => {
                if ignore_lone_hamzah {
                    last_value = 0
                } else {
                    abjad_total += 1;
                    last_value = 1
                }
            }
            'ب' | 'پ' => {
                abjad_total += 2;
                last_value = 2
            }
            'ج' | 'چ' => {
                abjad_total += 3;
                last_value = 3
            }
            'د' => {
                abjad_total += 4;
                last_value = 4
            }
            'ه' | 'ة' | 'ۀ' => {
                abjad_total += 5;
                last_value = 5
            }
            'و' | 'ؤ' => {
                abjad_total += 6;
                last_value = 6
            }
            'ز' | 'ژ' => {
                abjad_total += 7;
                last_value = 7
            }
            'ح' => {
                abjad_total += 8;
                last_value = 8
            }
            'ط' => {
                abjad_total += 9;
                last_value = 9
            }
            'ي' | 'ى' | 'ئ' | 'ی' => {
                abjad_total += 10;
                last_value = 10
            }
            'ك' | 'ک' | 'گ' => {
                abjad_total += 20;
                last_value = 20
            }
            'ل' => {
                abjad_total += 30;
                last_value = 30
            }
            'م' => {
                abjad_total += 40;
                last_value = 40
            }
            'ن' => {
                abjad_total += 50;
                last_value = 50
            }
            'س' => {
                if maghribi_order {
                    abjad_total += 300;
                    last_value = 300
                } else {
                    abjad_total += 60;
                    last_value = 60
                }
            }
            'ع' => {
                abjad_total += 70;
                last_value = 70
            }
            'ف' => {
                abjad_total += 80;
                last_value = 80
            }
            'ص' => {
                if maghribi_order {
                    abjad_total += 60;
                    last_value = 60
                } else {
                    abjad_total += 90;
                    last_value = 90
                }
            }
            'ق' => {
                abjad_total += 100;
                last_value = 100
            }
            'ر' => {
                abjad_total += 200;
                last_value = 200
            }
            'ش' => {
                if maghribi_order {
                    abjad_total += 1000;
                    last_value = 1000
                } else {
                    abjad_total += 300;
                    last_value = 300
                }
            }
            'ت' => {
                abjad_total += 400;
                last_value = 400
            }
            'ث' => {
                abjad_total += 500;
                last_value = 500
            }
            'خ' => {
                abjad_total += 600;
                last_value = 600
            }
            'ذ' => {
                abjad_total += 700;
                last_value = 700
            }
            'ض' => {
                if maghribi_order {
                    abjad_total += 90;
                    last_value = 90
                } else {
                    abjad_total += 800;
                    last_value = 800
                }
            }
            'ظ' => {
                if maghribi_order {
                    abjad_total += 800;
                    last_value = 800
                } else {
                    abjad_total += 900;
                    last_value = 900
                }
            }
            'غ' => {
                if maghribi_order {
                    abjad_total += 900;
                    last_value = 900
                } else {
                    abjad_total += 1000;
                    last_value = 1000
                }
            }
            'ّ' => {
                if count_shaddah {
                    abjad_total += last_value;
                }
                last_value = 0
            }
            _ => last_value = 0,
        }
    }

    abjad_total
}

use abjad::{Abjad, AbjadPrefs, LetterOrder};

#[test]
fn all() {
    let input = "ابجد هوز حطي كلمن سعفص قرشت ثخذ ضظغ";

    let prefs_mashriqi = AbjadPrefs::default();
    let prefs_maghribi = AbjadPrefs {
        letter_order: LetterOrder::Maghribi,
        ..AbjadPrefs::default()
    };

    let total_mashriqi = input.abjad_strict(prefs_mashriqi).unwrap();
    let total_maghribi = input.abjad_strict(prefs_maghribi).unwrap();

    assert_eq!(total_mashriqi, 5_995);
    assert_eq!(total_mashriqi, total_maghribi);
}

#[test]
fn baha_count() {
    let input = "بهاء";
    let prefs = AbjadPrefs::default();

    assert_eq!(input.abjad_strict(prefs).unwrap(), 9);
}

#[test]
fn baha_ignore() {
    let input = "بهاء";
    let prefs = AbjadPrefs {
        ignore_lone_hamzah: true,
        ..AbjadPrefs::default()
    };

    assert_eq!(input.abjad_strict(prefs).unwrap(), 8);
}

#[test]
fn basmala() {
    let input = "بسم الله الرحمن الرحيم";
    let prefs = AbjadPrefs::default();

    assert_eq!(input.abjad_strict(prefs).unwrap(), 786);
}

#[test]
fn humayun() {
    let input = "همایون پادشاه از بام افتاد";
    let prefs = AbjadPrefs::default();

    assert_eq!(input.abjad_strict(prefs).unwrap(), 962);
}

#[test]
fn latin() {
    let input = "the quick brown fox";
    let prefs = AbjadPrefs::default();

    assert_eq!(input.abjad(prefs), 0);
}

#[test]
fn latin_report() {
    let input = "the quick brown fox";
    let prefs = AbjadPrefs::default();

    let (total, errors) = input.abjad_collect_errors(prefs);

    assert_eq!(total, 0);
    assert_eq!(errors.len(), 16);
}

#[test]
fn mixture() {
    let input = "روح الله tapdancing خمینی";
    let prefs = AbjadPrefs::default();

    assert_eq!(input.abjad(prefs), 990);
}

#[test]
fn mixture_fail() {
    let input = "روح الله tapdancing خمینی";
    let prefs = AbjadPrefs::default();

    assert!(input.abjad_strict(prefs).is_err());
}

#[test]
fn mixture_report() {
    let input = "روح الله tapdancing خمینی";
    let prefs = AbjadPrefs::default();

    let (total, errors) = input.abjad_collect_errors(prefs);

    assert_eq!(total, 990);
    assert_eq!(errors.len(), 10);
}

#[test]
fn shaddah() {
    let input = "رئیس مؤسّس دانشگاه";
    let prefs = AbjadPrefs {
        count_shaddah: true,
        ..AbjadPrefs::default()
    };

    assert_eq!(input.abjad_strict(prefs).unwrap(), 887);
}

#[test]
fn tammamtu() {
    let input = "قد تمّمته";
    let prefs = AbjadPrefs::default();

    assert_eq!(input.abjad_strict(prefs).unwrap(), 989);
}

#[test]
fn vahshi() {
    let input = "وفات وحشی مسکین";
    let prefs = AbjadPrefs::default();

    assert_eq!(input.abjad_strict(prefs).unwrap(), 991);
}

#[test]
fn zwnj() {
    let input = "عادت می‌کنیم";
    let prefs = AbjadPrefs::default();

    assert_eq!(input.abjad_strict(prefs).unwrap(), 645);
}

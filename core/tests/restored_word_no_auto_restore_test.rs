//! Words brought back from the screen (backspace into a committed word) must not be
//! rewritten to raw keystrokes by English auto-restore.
use gonhanh_core::engine::{Action, Engine};
use gonhanh_core::utils::char_to_key;

fn type_seq(e: &mut Engine, seq: &str) -> String {
    let mut screen: Vec<char> = Vec::new();
    for c in seq.chars() {
        let key = if c == '<' { 51 } else { char_to_key(c) };
        let r = e.on_key_ext(key, false, false, false);
        if r.action == Action::Send as u8 {
            for _ in 0..r.backspace {
                screen.pop();
            }
            for i in 0..r.count as usize {
                screen.push(char::from_u32(r.chars[i]).unwrap());
            }
        } else if c == '<' {
            screen.pop();
        } else {
            screen.push(c);
        }
    }
    screen.into_iter().collect()
}

#[test]
fn backspace_into_word_then_type_is_not_auto_restored() {
    let mut e = Engine::new();
    e.set_english_auto_restore(true);
    let out = type_seq(&mut e, "nghieemj <<uoiwf ");
    assert!(!out.contains("nghieuoiwf"), "got {out:?}");
    assert!(out.starts_with("nghi"), "got {out:?}");
}

#[test]
fn restore_word_from_app_is_not_auto_restored() {
    let mut e = Engine::new();
    e.set_english_auto_restore(true);
    e.restore_word("nghiệ");
    let out = type_seq(&mut e, "uoiwf ");
    assert!(!out.contains("uoiwf"), "got {out:?}");
}

#[test]
fn english_auto_restore_still_works_for_fresh_words() {
    let mut e = Engine::new();
    e.set_english_auto_restore(true);
    assert_eq!(type_seq(&mut e, "text "), "text ");
}

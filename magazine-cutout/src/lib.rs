use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words = HashMap::new();

    for word in magazine {
        match words.get_mut(word) {
            None => {
                words.insert(word, 1);
            }
            Some(w) => {
                *w += 1;
            }
        }
    }

    for word in note {
        if let Some(w) = words.get_mut(word) {
            if *w == 0 {
                return false;
            };
            *w -= 1;
        } else {
            return false;
        }
    }

    true
}

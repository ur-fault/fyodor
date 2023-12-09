use crossterm::event::KeyCode;

pub struct Keylist {
    case_insensitive: bool,
    all: bool,
    keys: Vec<KeyCode>,
}

impl Keylist {
    pub fn new(allow_case: bool) -> Self {
        Self {
            case_insensitive: allow_case,
            all: false,
            keys: Vec::new(),
        }
    }

    pub fn allow_case(mut self, allow_case: bool) -> Self {
        self.case_insensitive = allow_case;
        self
    }

    pub fn all(mut self, all: bool) -> Self {
        self.all = all;
        self.clear_keys();
        self
    }

    pub fn with_keys(mut self, keys: &[KeyCode]) -> Self {
        self.all = false;
        self.keys.extend_from_slice(keys);
        self
    }

    pub fn with_chars(mut self, keys: &[char]) -> Self {
        self.all = false;
        self.keys.extend(keys.iter().map(|c| KeyCode::Char(*c)));
        self
    }

    pub fn except_keys(mut self, keys: &[KeyCode]) -> Self {
        self.all = true;
        self.keys.extend_from_slice(keys);
        self
    }

    pub fn except_chars(mut self, keys: &[char]) -> Self {
        self.all = true;
        self.keys.extend(keys.iter().map(|c| KeyCode::Char(*c)));
        self
    }

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    pub fn keys(&self) -> &[KeyCode] {
        self.keys.as_ref()
    }
}

impl Keylist {
    pub fn contains(&self, key: KeyCode) -> bool {
        fn includes(keys: &[KeyCode], key: KeyCode, case_insensitive: bool) -> bool {
            keys.iter().any(|k| match (*k, key) {
                (KeyCode::Char(k1), KeyCode::Char(k2)) if case_insensitive => {
                    k1.eq_ignore_ascii_case(&k2)
                }
                (k1, k2) => k1 == k2,
            })
        }

        self.all ^ includes(&self.keys, key, self.case_insensitive)
    }
}

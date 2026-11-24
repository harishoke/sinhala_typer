pub mod legacy;
pub mod phonetic;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum Mode {
    English,
    #[default]
    Unicode,
    LegacyFm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditAction {
    pub backspaces: usize,
    pub insert: String,
}

#[cfg(test)]
pub struct ScreenSim {
    text: String,
}

#[cfg(test)]
impl ScreenSim {
    pub fn new() -> Self {
        Self { text: String::new() }
    }

    pub fn apply(&mut self, action: &EditAction) {
        for _ in 0..action.backspaces {
            self.text.pop();
        }
        self.text.push_str(&action.insert);
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screensim_insert_appends_text() {
        let mut sim = ScreenSim::new();
        sim.apply(&EditAction {
            backspaces: 0,
            insert: "abc".to_string(),
        });
        assert_eq!(sim.as_str(), "abc");
    }

    #[test]
    fn screensim_backspace_removes_chars_not_bytes() {
        let mut sim = ScreenSim::new();
        sim.apply(&EditAction {
            backspaces: 0,
            insert: "a\u{0D9A}b".to_string(),
        });
        sim.apply(&EditAction {
            backspaces: 2,
            insert: String::new(),
        });
        assert_eq!(sim.as_str(), "a");
    }

    #[test]
    fn screensim_backspace_then_insert() {
        let mut sim = ScreenSim::new();
        sim.apply(&EditAction {
            backspaces: 0,
            insert: "xy".to_string(),
        });
        sim.apply(&EditAction {
            backspaces: 1,
            insert: "z".to_string(),
        });
        assert_eq!(sim.as_str(), "xz");
    }
}

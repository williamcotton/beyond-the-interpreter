#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextSize(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextRange {
    start: TextSize,
    end: TextSize,
}

impl TextRange {
    pub fn new(start: TextSize, end: TextSize) -> Option<Self> {
        if start.0 > end.0 {
            return None;
        }

        Some(Self { start, end })
    }

    pub fn slice(self, source: &str) -> Option<&str> {
        source.get(self.start.0..self.end.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{TextRange, TextSize};

    #[test]
    fn selects_ascii_fragment() {
        let word = TextRange::new(TextSize(0), TextSize(5)).expect("ordered range");

        assert_eq!(word.slice("print 42;"), Some("print"));
    }

    #[test]
    fn rejects_reversed_range() {
        assert_eq!(TextRange::new(TextSize(5), TextSize(0)), None);
    }

    #[test]
    fn respects_utf8_boundaries() {
        let source = "aé!";
        let accent = TextRange::new(TextSize(1), TextSize(3)).expect("ordered range");
        let split = TextRange::new(TextSize(1), TextSize(2)).expect("ordered range");

        assert_eq!(accent.slice(source), Some("é"));
        assert_eq!(split.slice(source), None);
    }

    #[test]
    fn checks_bounds_and_empty_ranges() {
        let source = "aé!";
        let end = TextSize(source.len());
        let eof = TextRange::new(end, end).expect("ordered range");
        let beyond = TextRange::new(TextSize(0), TextSize(5)).expect("ordered range");
        let interior = TextRange::new(TextSize(2), TextSize(2)).expect("ordered range");

        assert_eq!(eof.slice(source), Some(""));
        assert_eq!(beyond.slice(source), None);
        assert_eq!(interior.slice(source), None);
    }
}

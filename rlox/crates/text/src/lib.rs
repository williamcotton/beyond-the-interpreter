use std::fmt;

pub use rowan::{TextRange, TextSize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceTooLarge;

impl fmt::Display for SourceTooLarge {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("source exceeds the 4,294,967,295-byte size limit")
    }
}

pub fn checked_size(bytes: usize) -> Result<TextSize, SourceTooLarge> {
    TextSize::try_from(bytes).map_err(|_| SourceTooLarge)
}

pub fn checked_range(start: TextSize, end: TextSize) -> Option<TextRange> {
    if start > end {
        return None;
    }
    Some(TextRange::new(start, end))
}

pub fn slice(source: &str, range: TextRange) -> Option<&str> {
    source.get(usize::from(range.start())..usize::from(range.end()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyntaxOrigin {
    pub file: FileId,
    pub range: TextRange,
}

#[cfg(test)]
mod tests {
    use super::{
        FileId, SourceTooLarge, SyntaxOrigin, TextSize, checked_range, checked_size, slice,
    };

    #[test]
    fn selects_ascii_fragment() {
        let word = checked_range(TextSize::new(0), TextSize::new(5)).expect("ordered range");

        assert_eq!(slice("print 42;", word), Some("print"));
    }

    #[test]
    fn rejects_reversed_range() {
        assert_eq!(checked_range(TextSize::new(5), TextSize::new(0)), None);
    }

    #[test]
    fn respects_utf8_boundaries() {
        let source = "aé!";
        let accent = checked_range(TextSize::new(1), TextSize::new(3)).expect("ordered range");
        let split = checked_range(TextSize::new(1), TextSize::new(2)).expect("ordered range");

        assert_eq!(slice(source, accent), Some("é"));
        assert_eq!(slice(source, split), None);
    }

    #[test]
    fn checks_bounds_and_empty_ranges() {
        let source = "aé!";
        let end = checked_size(source.len()).expect("representable offset");
        let eof = checked_range(end, end).expect("ordered range");
        let beyond = checked_range(TextSize::new(0), TextSize::new(5)).expect("ordered range");
        let interior = checked_range(TextSize::new(2), TextSize::new(2)).expect("ordered range");

        assert_eq!(slice(source, eof), Some(""));
        assert_eq!(slice(source, beyond), None);
        assert_eq!(slice(source, interior), None);
    }

    #[test]
    fn same_range_in_different_files_has_different_origins() {
        let range = checked_range(TextSize::new(0), TextSize::new(5)).expect("ordered range");
        let first = SyntaxOrigin {
            file: FileId(0),
            range,
        };
        let second = SyntaxOrigin {
            file: FileId(1),
            range,
        };

        assert_eq!(first.range, second.range);
        assert_ne!(first, second);
    }

    #[test]
    fn selects_text_from_identified_source() {
        let first_source = "print 42;";
        let second_source = "while (true) {}";
        let origin = SyntaxOrigin {
            file: FileId(1),
            range: checked_range(TextSize::new(0), TextSize::new(5)).expect("ordered range"),
        };

        let source = match origin.file {
            FileId(0) => first_source,
            FileId(1) => second_source,
            _ => panic!("file ID was not assigned in this example"),
        };

        assert_eq!(slice(source, origin.range), Some("while"));
    }

    #[test]
    fn checks_capacity_without_allocating_source() {
        let maximum = usize::try_from(u32::MAX).expect("u32 fits this target's usize");
        assert_eq!(checked_size(maximum), Ok(TextSize::new(u32::MAX)));
        if let Some(too_large) = maximum.checked_add(1) {
            assert_eq!(checked_size(too_large), Err(SourceTooLarge));
        }
    }
}

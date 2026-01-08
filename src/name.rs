/// Represents a person's name with its various components.
///
/// Provides structured storage for different parts of a person's name, including
/// first, middle, and last names, as well as optional prefixes and suffixes.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Name {
    /// Last name (surname or family name).
    #[cfg_attr(feature = "serde", serde(default))]
    pub last: String,
    /// First name (given name).
    #[cfg_attr(feature = "serde", serde(default))]
    pub first: String,
    /// Middle name or initial.
    #[cfg_attr(feature = "serde", serde(default))]
    pub middle: String,
    /// Name prefix or title (e.g., "Dr.", "Mr.", "Ms.").
    #[cfg_attr(feature = "serde", serde(default))]
    pub prefix: String,
    /// Name suffix (e.g., "Jr.", "Sr.", "III").
    #[cfg_attr(feature = "serde", serde(default))]
    pub suffix: String,
}

impl Name {
    pub fn from_dicom(s: &str) -> Self {
        let parts = s
            .trim()
            .split(|c| c == '^' || c == '\\')
            .collect::<Vec<_>>();
        let n = parts.len();
        Self {
            last: if n > 0 {
                parts[0].to_string()
            } else {
                Default::default()
            },
            first: if n > 1 {
                parts[1].to_string()
            } else {
                Default::default()
            },
            middle: if n > 2 {
                parts[2].to_string()
            } else {
                Default::default()
            },
            prefix: if n > 3 {
                parts[3].to_string()
            } else {
                Default::default()
            },
            suffix: if n > 4 {
                parts[4].to_string()
            } else {
                Default::default()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_dicom_full_name_with_caret() {
        let name = Name::from_dicom("Doe^John^Michael^Dr.^Jr.");
        assert_eq!(name.last, "Doe");
        assert_eq!(name.first, "John");
        assert_eq!(name.middle, "Michael");
        assert_eq!(name.prefix, "Dr.");
        assert_eq!(name.suffix, "Jr.");
    }

    #[test]
    fn test_from_dicom_full_name_with_backslash() {
        let name = Name::from_dicom("Doe\\John\\Michael\\Dr.\\Jr.");
        assert_eq!(name.last, "Doe");
        assert_eq!(name.first, "John");
        assert_eq!(name.middle, "Michael");
        assert_eq!(name.prefix, "Dr.");
        assert_eq!(name.suffix, "Jr.");
    }

    #[test]
    fn test_from_dicom_mixed_separators() {
        let name = Name::from_dicom("Doe^John\\Michael^Dr.\\Jr.");
        assert_eq!(name.last, "Doe");
        assert_eq!(name.first, "John");
        assert_eq!(name.middle, "Michael");
        assert_eq!(name.prefix, "Dr.");
        assert_eq!(name.suffix, "Jr.");
    }

    #[test]
    fn test_from_dicom_last_and_first_only() {
        let name = Name::from_dicom("Doe^John");
        assert_eq!(name.last, "Doe");
        assert_eq!(name.first, "John");
        assert_eq!(name.middle, "");
        assert_eq!(name.prefix, "");
        assert_eq!(name.suffix, "");
    }

    #[test]
    fn test_from_dicom_last_only() {
        let name = Name::from_dicom("Doe");
        assert_eq!(name.last, "Doe");
        assert_eq!(name.first, "");
        assert_eq!(name.middle, "");
        assert_eq!(name.prefix, "");
        assert_eq!(name.suffix, "");
    }

    #[test]
    fn test_from_dicom_empty_string() {
        let name = Name::from_dicom("");
        assert_eq!(name.last, "");
        assert_eq!(name.first, "");
        assert_eq!(name.middle, "");
        assert_eq!(name.prefix, "");
        assert_eq!(name.suffix, "");
    }

    #[test]
    fn test_from_dicom_with_whitespace() {
        let name = Name::from_dicom("  Doe^John^Michael  ");
        assert_eq!(name.last, "Doe");
        assert_eq!(name.first, "John");
        assert_eq!(name.middle, "Michael");
        assert_eq!(name.prefix, "");
        assert_eq!(name.suffix, "");
    }

    #[test]
    fn test_from_dicom_with_empty_components() {
        let name = Name::from_dicom("Doe^^Michael");
        assert_eq!(name.last, "Doe");
        assert_eq!(name.first, "");
        assert_eq!(name.middle, "Michael");
        assert_eq!(name.prefix, "");
        assert_eq!(name.suffix, "");
    }

    #[test]
    fn test_from_dicom_trailing_separators() {
        let name = Name::from_dicom("Doe^John^^^");
        assert_eq!(name.last, "Doe");
        assert_eq!(name.first, "John");
        assert_eq!(name.middle, "");
        assert_eq!(name.prefix, "");
        assert_eq!(name.suffix, "");
    }
}

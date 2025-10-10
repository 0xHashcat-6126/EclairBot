pub trait IgnoreCaseCmp {
    fn ignore_case_eq(&self, other: &str) -> bool;
    fn ignore_case_starts_with(&self, prefix: &str) -> bool;
    fn ignore_case_ends_with(&self, suffix: &str) -> bool;
    fn ignore_case_contains(&self, other: &str) -> bool;
}

impl IgnoreCaseCmp for &str {
    fn ignore_case_eq(&self, other: &str) -> bool {
        let mut self_chars = self.chars();
        let mut other_chars = other.chars();

        while let (Some(a), Some(b)) = (self_chars.next(), other_chars.next()) {
            if a.is_ascii() && b.is_ascii() {
                if a.to_ascii_lowercase() != b.to_ascii_lowercase() {
                    return false;
                }
            } else {
                if a != b {
                    return false;
                }
            }
        }
        
        match (self_chars.next(), other_chars.next()) {
            (None, None) => true,
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (Some(_), Some(_)) => unreachable!(),
        }
    }

    fn ignore_case_starts_with(&self, prefix: &str) -> bool {
        let mut self_chars = self.chars();
        let mut prefix_chars = prefix.chars();

        while let (Some(a), Some(b)) = (self_chars.next(), prefix_chars.next()) {
            if a.is_ascii() && b.is_ascii() {
                if a.to_ascii_lowercase() != b.to_ascii_lowercase() {
                    return false;
                }
            } else {
                if a != b {
                    return false;
                }
            }
        }
        
        match (self_chars.next(), prefix_chars.next()) {
            (None, None) => true,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (Some(_), Some(_)) => unreachable!(),
        }
    }

    fn ignore_case_ends_with(&self, suffix: &str) -> bool {
        // I think that for now there is no better way
        self.to_lowercase().ends_with(&suffix.to_lowercase())
    }

    fn ignore_case_contains(&self, other: &str) -> bool {
        // I think that for now there is no better way
        self.to_lowercase().contains(&other.to_lowercase())
    }
}

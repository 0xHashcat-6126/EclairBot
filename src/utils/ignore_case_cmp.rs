pub trait IgnoreCaseCmp {
    fn ignore_case_eq(&self, other: &str) -> bool;
    fn ignore_case_starts_with(&self, prefix: &str) -> bool;
    fn ignore_case_ends_with(&self, suffix: &str) -> bool;
    fn ignore_case_contains(&self, other: &str) -> bool;
}

impl<T: AsRef<str>> IgnoreCaseCmp for T {
    #[inline(always)]
    fn ignore_case_eq(&self, b_string: &str) -> bool {
        if self.as_ref().len() != b_string.len() {
            return false;
        }

        let a_bytes = self.as_ref().as_bytes();
        let b_bytes = b_string.as_bytes();

        let mut i = 0;
        while i + 8 <= a_bytes.len() {
            let a_chunk = u64::from_le_bytes(a_bytes[i..i + 8].try_into().unwrap());
            let b_chunk = u64::from_le_bytes(b_bytes[i..i + 8].try_into().unwrap());
            if a_chunk != b_chunk {
                for j in 0..8 {
                    let a_char = a_bytes[i + j] | 0x20;
                    let b_char = b_bytes[i + j] | 0x20;
                    if a_char != b_char {
                        return false;
                    }
                }
            }
            i += 8;
        }

        for k in i..a_bytes.len() {
            if (a_bytes[k] | 0x20) != (b_bytes[k] | 0x20) {
                return false;
            }
        }

        true
    }

    fn ignore_case_starts_with(&self, prefix: &str) -> bool {
        let mut self_chars = self.as_ref().chars();
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
        self.as_ref().to_lowercase().ends_with(&suffix.to_lowercase())
    }

    fn ignore_case_contains(&self, other: &str) -> bool {
        // I think that for now there is no better way
        self.as_ref().to_lowercase().contains(&other.to_lowercase())
    }
}

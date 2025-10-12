pub trait IgnoreCaseCmp {
    fn de_pl(string: &str) -> String;
    fn ignore_case_eq(&self, other: &str) -> bool;
    fn ignore_case_starts_with(&self, prefix: &str) -> bool;
    fn ignore_case_ends_with(&self, suffix: &str) -> bool;
    fn ignore_case_contains(&self, other: &str) -> bool;
}

impl<T: AsRef<str>> IgnoreCaseCmp for T {
    #[inline(always)]
    fn de_pl(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let bytes = s.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            let b = bytes[i];

            if b < 0x80 {
                out.push(b as char);
                i += 1;
                continue;
            }

            if i + 1 >= bytes.len() {
                out.push(b as char);
                break;
            }

            let b2 = bytes[i + 1];

            match (b, b2) {
                (0xC4, 0x85) => out.push('a'),
                (0xC4, 0x87) => out.push('c'),
                (0xC4, 0x99) => out.push('e'),
                (0xC5, 0x82) => out.push('l'),
                (0xC5, 0x84) => out.push('n'),
                (0xC3, 0xB3) => out.push('o'),
                (0xC5, 0x9B) => out.push('s'),
                (0xC5, 0xBA) => out.push('z'),
                (0xC5, 0xBC) => out.push('z'),

                (0xC4, 0x84) => out.push('A'),
                (0xC4, 0x86) => out.push('C'),
                (0xC4, 0x98) => out.push('E'),
                (0xC5, 0x81) => out.push('L'),
                (0xC5, 0x83) => out.push('N'),
                (0xC3, 0x93) => out.push('O'),
                (0xC5, 0x9A) => out.push('S'),
                (0xC5, 0xB9) => out.push('Z'),
                (0xC5, 0xBB) => out.push('Z'),

                _ => {
                    out.push(b as char);
                    i += 1;
                    continue;
                }
            }

            i += 2;
        }

        out
    }

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

    #[inline(always)]
    fn ignore_case_starts_with(&self, prefix: &str) -> bool {
        if self.as_ref().len() < prefix.len() {
            return false;
        }

        let string_bytes = self.as_ref().as_bytes();
        let prefix_bytes = prefix.as_bytes();

        for i in 0..prefix_bytes.len() {
            if string_bytes[i] | 0x20 != prefix_bytes[i] {
                return false;
            }
        }

        true
    }

    #[inline(always)]
    fn ignore_case_ends_with(&self, suffix: &str) -> bool {
        if self.as_ref().len() < suffix.len() {
            return false;
        }

        let string_bytes = self.as_ref().as_bytes();
        let suffix_bytes = suffix.as_bytes();

        for i in 0..suffix_bytes.len() {
            if string_bytes[i + string_bytes.len() - suffix_bytes.len()] | 0x20 != suffix_bytes[i] {
                return false;
            }
        }

        true
    }

    fn ignore_case_contains(&self, other: &str) -> bool {
        // I think that for now there is no better way
        self.as_ref().to_lowercase().contains(&other.to_lowercase())
    }
}

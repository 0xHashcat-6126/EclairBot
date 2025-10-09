use regex::Regex;
use smallvec::SmallVec;

use crate::util::ignore_case_cmp::IgnoreCaseCmp as _;

pub enum AutoReplyRule {
    StartsWith(&'static str),
    EndsWith(&'static str),
    Contains(&'static str),
    EqualTo(&'static str),
    UnequalTo(&'static str),

    IgnoreCaseStartsWith(&'static str),
    IgnoreCaseEndsWith(&'static str),
    IgnoreCaseContains(&'static str),
    IgnoreCaseEqualTo(&'static str),

    MatchesRegex(Regex),
}

fn match_autoreply_rule(content: &str, rule: AutoReplyRule) -> bool {
    match rule {
        AutoReplyRule::StartsWith(pat) => content.starts_with(pat),
        AutoReplyRule::EndsWith(pat) => content.ends_with(pat),
        AutoReplyRule::Contains(pat) => content.contains(pat),
        AutoReplyRule::EqualTo(pat) => content.eq(pat),
        AutoReplyRule::UnequalTo(pat) => !content.eq(pat),

        AutoReplyRule::IgnoreCaseStartsWith(pat) => content.ignore_case_starts_with(pat),
        AutoReplyRule::IgnoreCaseEndsWith(pat) => content.ignore_case_ends_with(pat),
        AutoReplyRule::IgnoreCaseContains(pat) => content.ignore_case_contains(pat),
        AutoReplyRule::IgnoreCaseEqualTo(pat) => content.ignore_case_eq(pat),
    
        AutoReplyRule::MatchesRegex(reg) => false, // TODO
    }
}

pub type AutoReplyRules = SmallVec<[AutoReplyRule; 8]>;

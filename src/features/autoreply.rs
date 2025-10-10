use std::ops::{BitOr, BitAnd};

use rand::{Rng};
use regex::Regex;
use serenity::all::{CacheHttp, Message};
use smallvec::SmallVec;

use crate::utils::ignore_case_cmp::IgnoreCaseCmp as _;

#[allow(dead_code)]
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

    Or(Box<AutoReplyRule>, Box<AutoReplyRule>),
    And(Box<AutoReplyRule>, Box<AutoReplyRule>),
}

impl BitAnd for AutoReplyRule {
    type Output = AutoReplyRule;
    fn bitand(self, rhs: AutoReplyRule) -> Self::Output {
        AutoReplyRule::And(Box::new(self), Box::new(rhs))
    }
}

impl BitOr for AutoReplyRule {
    type Output = AutoReplyRule;
    fn bitor(self, rhs: AutoReplyRule) -> Self::Output {
        AutoReplyRule::Or(Box::new(self), Box::new(rhs))
    }
}

impl From<&'static str> for AutoReplyRule {
    fn from(value: &'static str) -> Self {
        return Self::IgnoreCaseEqualTo(value);
    }
}

pub type RepliesList = SmallVec<[&'static str; 8]>;

#[allow(dead_code)]
pub enum AutoReplyTarget {
    ReplyWith(&'static str),
    ReplyWithRandom(RepliesList),
}

impl From<&'static str> for AutoReplyTarget {
    fn from(value: &'static str) -> Self {
        return Self::ReplyWith(value);
    }
}

fn match_autoreply_rule(content: &str, rule: &AutoReplyRule) -> bool {
    match rule {
        AutoReplyRule::StartsWith(pat) => content.starts_with(pat),
        AutoReplyRule::EndsWith(pat) => content.ends_with(pat),
        AutoReplyRule::Contains(pat) => content.contains(pat),
        AutoReplyRule::EqualTo(pat) => (&content).eq(pat),
        AutoReplyRule::UnequalTo(pat) => !(&content).eq(pat),

        AutoReplyRule::IgnoreCaseStartsWith(pat) => content.ignore_case_starts_with(pat),
        AutoReplyRule::IgnoreCaseEndsWith(pat) => content.ignore_case_ends_with(pat),
        AutoReplyRule::IgnoreCaseContains(pat) => content.ignore_case_contains(pat),
        AutoReplyRule::IgnoreCaseEqualTo(pat) => content.ignore_case_eq(pat),
    
        AutoReplyRule::MatchesRegex(_reg) => false, // TODO
        
        AutoReplyRule::Or(a, b) => match_autoreply_rule(content, a.as_ref()) || match_autoreply_rule(content, b.as_ref()),
        AutoReplyRule::And(a, b) => match_autoreply_rule(content, a.as_ref()) && match_autoreply_rule(content, b.as_ref()),
    }
}

fn get_reply(target: &AutoReplyTarget) -> &'static str {
    match target {
        AutoReplyTarget::ReplyWith(str) => str,
        AutoReplyTarget::ReplyWithRandom(variants) => {
            let mut rng = rand::rng();
            variants[rng.random_range(0..variants.len()-1)]
        },
    }
}

pub struct AutoReplyDef {
    pub rule: AutoReplyRule,
    pub target: AutoReplyTarget,
}

#[macro_export]
macro_rules! autoreply {
    ($rule:expr, $target:expr) => {
        crate::features::autoreply::AutoReplyDef { rule: From::from($rule), target: From::from($target) }
    }
}

impl AutoReplyDef {
    pub async fn handle(&self, cache_http: impl CacheHttp, msg: &Message) -> Option<serenity::Result<Message>> {
        if match_autoreply_rule(&msg.content, &self.rule) {
            let reply = get_reply(&self.target);
            Some(msg.reply(cache_http, reply).await)
        } else {
            None
        }
    }
}


//! `titlecase` capitlizes English text according to [a style][style] defined by John
//! Gruber for post titles on his website [Daring Fireball].
//!
//! [Daring Fireball]: https://daringfireball.net/
//! [style]: https://daringfireball.net/2008/05/title_case
//!
//! ## Example
//!
//! ```
//! use titlecase::titlecase;
//!
//! let text = "a sample title to capitalize: an example";
//! assert_eq!(titlecase(text), "A Sample Title to Capitalize: An Example");
//! ```
//!
//! ## Style
//!
//! Instead of simply capitalizing each word it does the following ([amongst other
//! things][style]):
//!
//! * Lower case small words like an, of, or in.
//! * Don't capitalize words like iPhone.
//! * Don't interfere with file paths, URLs, domains, and email addresses.
//! * Always capitalize the first and last words, even if they are small words
//!   or surrounded by quotes.
//! * Don't interfere with terms like "Q&A", or "AT&T".
//! * Capitalize small words after a colon.

#[macro_use]
extern crate lazy_static;

use std::borrow::Cow;

use joinery::JoinableIterator;
use regex::{Captures, Regex};

#[rustfmt::skip]
const SMALL_WORDS: &[&str] = &[
    "a",
    "an",
    "and",
    "as",
    "at",
    "but",
    "by",
    "en",
    "for",
    "if",
    "in",
    "of",
    "on",
    "or",
    "the",
    "to",
    "v[.]?",
    "via",
    "vs[.]?",
];

lazy_static! {
    static ref SMALL_WORDS_PIPE: String = SMALL_WORDS.join("|");
}

/// Returns `input` in title case.
///
/// ### Example
///
/// ```
/// use titlecase::titlecase;
///
/// let text = "a sample title to capitalize: an example";
/// assert_eq!(titlecase(text), "A Sample Title to Capitalize: An Example");
/// ```
pub fn titlecase(input: &str) -> String {
    lazy_static! {
        static ref WORDS: Regex = Regex::new(
            r"(?x)
             (_*)
             ([\w'’.:/@\[\]/()&]+)
             (_*)",
        )
        .expect("unable to compile regex");
    }

    // If input is yelling (all uppercase) make lowercase
    let trimmed_input = input.trim();
    let trimmed_input = if trimmed_input.chars().any(|ch| ch.is_lowercase()) {
        Cow::from(trimmed_input)
    } else {
        Cow::from(trimmed_input.to_lowercase())
    };

    let result = WORDS.replace_all(&trimmed_input, |captures: &Captures| {
        let mut result = captures.get(1).map_or("", |cap| cap.as_str()).to_owned();
        let word = &captures[2];
        result.push_str(&process_word(word));
        result.push_str(captures.get(3).map_or("", |cap| cap.as_str()));
        result
    });

    // Now deal with small words at the start and end of the text
    fix_small_word_at_end(&fix_small_word_at_start(&result)).into_owned()
}

fn process_word(word: &str) -> Cow<'_, str> {
    lazy_static! {
        static ref SMALL_RE: Regex = Regex::new(&format!(r"\A(?:{})\z", *SMALL_WORDS_PIPE))
            .expect("unable to compile small words regex");
    }

    if is_digital_resource(word) {
        // pass through
        return Cow::from(word);
    }

    let lower_word = word.to_lowercase();
    if SMALL_RE.is_match(&lower_word) {
        Cow::from(lower_word)
    } else if starts_with_bracket(word) {
        let rest = titlecase(&word[1..]);
        Cow::from(format!("({}", rest))
    } else if has_internal_slashes(word) {
        Cow::from(word.split('/').map(titlecase).join_with('/').to_string())
    } else if has_internal_caps(word) {
        // Preserve internal caps like iPhone or DuBois
        Cow::from(word)
    } else {
        Cow::from(ucfirst(word))
    }
}

// https://stackoverflow.com/a/38406885
fn ucfirst(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(chars).collect(),
    }
}

fn is_digital_resource(word: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
            \A
            (?: [/\\] [[:alpha:]]+ [-_[:alpha:]/\\]+ |   # file path or
              [-_[:alpha:]]+ [@.:] [-_[:alpha:]@.:/]+ )  # URL, domain, or email",
        )
        .expect("unable to compile file/url regex");
    }
    RE.is_match(word)
}

// E.g. iPhone or DuBois
fn has_internal_caps(word: &str) -> bool {
    word.chars().skip(1).any(|chr| chr.is_uppercase())
}

fn has_internal_slashes(word: &str) -> bool {
    !word.is_empty() && word.chars().skip(1).any(|chr| chr == '/')
}

fn starts_with_bracket(word: &str) -> bool {
    word.starts_with('(')
}

fn fix_small_word_at_start(text: &str) -> Cow<'_, str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(&format!(
            r#"(?x)
            ( \A [[:punct:]]*        # start of title...
            |  [:.;?!]\x20+          # or of subsentence...
            |  \x20['"“‘(\[]\x20* )  # or of inserted subphrase...
            ( {small_re} ) \b        # ... followed by small word
            "#,
            small_re = *SMALL_WORDS_PIPE
        ))
        .expect("unable to compile fix_small_word_at_start regex");
    }

    RE.replace_all(text, |captures: &Captures| {
        let mut result = captures[1].to_owned();
        result.push_str(&ucfirst(&captures[2]));
        result
    })
}

fn fix_small_word_at_end(text: &str) -> Cow<'_, str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(&format!(
            r#"(?x)
            \b ( {small_re} )     # small word...
            ( [[:punct:]]* \z     # ... at the end of the title...
            |   ['"’”)\]] \x20 )  # ... or of an inserted subphrase?
            "#,
            small_re = *SMALL_WORDS_PIPE
        ))
        .expect("unable to compile fix_small_word_at_end regex");
    }

    RE.replace_all(text, |captures: &Captures| {
        let mut result = ucfirst(&captures[1]);
        result.push_str(&captures[2]);
        result
    })
}

#[cfg(test)]
mod tests {
    use super::titlecase;

    macro_rules! testcase {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(titlecase($input), $expected);
            }
        };
    }

    testcase!(
        email,
        "For step-by-step directions email someone@gmail.com",
        "For Step-by-Step Directions Email someone@gmail.com"
    );

    testcase!(
        subphrase_in_single_quotes,
        "2lmc Spool: 'Gruber on OmniFocus and Vapo(u)rware'",
        "2lmc Spool: 'Gruber on OmniFocus and Vapo(u)rware'"
    );

    testcase!(
        subphrase_in_double_quotes,
        r#"2lmc Spool: "Gruber on OmniFocus and Vapo(u)rware""#,
        r#"2lmc Spool: "Gruber on OmniFocus and Vapo(u)rware""#
    );

    testcase!(
        curly_double_quotes,
        "Have you read “the lottery”?",
        "Have You Read “The Lottery”?"
    );

    testcase!(
        brackets,
        "your hair[cut] looks (nice)",
        "Your Hair[cut] Looks (Nice)"
    );

    testcase!(
        multiple_brackets,
        "your hair[cut] looks ((Very Nice))",
        "Your Hair[cut] Looks ((Very Nice))"
    );

    testcase!(
        url,
        "People probably won't put http://foo.com/bar/ in titles",
        "People Probably Won't Put http://foo.com/bar/ in Titles"
    );

    testcase!(
        name_url,
        "Scott Moritz and TheStreet.com’s million iPhone la‑la land",
        "Scott Moritz and TheStreet.com’s Million iPhone La‑La Land"
    );

    testcase!(iphone, "BlackBerry vs. iPhone", "BlackBerry vs. iPhone");

    testcase!(
    curly_single_quotes,
    "Notes and observations regarding Apple’s announcements from ‘The Beat Goes On’ special event",
    "Notes and Observations Regarding Apple’s Announcements From ‘The Beat Goes On’ Special Event"
);

    testcase!(
        markdown,
        "Read markdown_rules.txt to find out how _underscores around words_ will be interpreted",
        "Read markdown_rules.txt to Find Out How _Underscores Around Words_ Will Be Interpreted"
    );

    testcase!(
        q_and_a,
        "Q&A with Steve Jobs: 'That's what happens in technology'",
        "Q&A With Steve Jobs: 'That's What Happens in Technology'"
    );

    testcase!(
        at_and_t,
        "What is AT&T's problem?",
        "What Is AT&T's Problem?"
    );

    testcase!(
        at_and_t2,
        "Apple deal with AT&T falls through",
        "Apple Deal With AT&T Falls Through"
    );

    testcase!(thisvthat, "this v that", "This v That");

    testcase!(thisvthat2, "this vs that", "This vs That");

    testcase!(thisvthat3, "this v. that", "This v. That");

    testcase!(thisvthat4, "this vs. that", "This vs. That");

    testcase!(
        sec,
        "The SEC's Apple probe: what you need to know",
        "The SEC's Apple Probe: What You Need to Know"
    );

    testcase!(
        small_word_at_start_in_quotes,
        "'by the way, small word at the start but within quotes.'",
        "'By the Way, Small Word at the Start but Within Quotes.'"
    );

    testcase!(
        small_word_at_end,
        "Small word at end is nothing to be afraid of",
        "Small Word at End Is Nothing to Be Afraid Of"
    );

    testcase!(
        subphrase_starting_with_small_word,
        "Starting sub-phrase with a small word: a trick, perhaps?",
        "Starting Sub-Phrase With a Small Word: A Trick, Perhaps?"
    );

    testcase!(
        subphrase_with_small_word_in_single_quotes,
        "Sub-phrase with a small word in quotes: 'a trick, perhaps?'",
        "Sub-Phrase With a Small Word in Quotes: 'A Trick, Perhaps?'"
    );

    testcase!(
        a_subphrase_with_small_word_in_single_quotes,
        "a Sub-phrase with a small word in quotes: 'a trick, perhaps?'",
        "A Sub-Phrase With a Small Word in Quotes: 'A Trick, Perhaps?'"
    );

    testcase!(
        subphrase_with_small_word_in_double_quotes,
        "Sub-phrase with a small word in quotes: \"a trick, perhaps?\"",
        "Sub-Phrase With a Small Word in Quotes: \"A Trick, Perhaps?\""
    );

    testcase!(
        all_in_double_quotes,
        "\"Nothing to Be Afraid of?\"",
        "\"Nothing to Be Afraid Of?\""
    );

    testcase!(a_thing, "a thing", "A Thing");

    testcase!(
        dr_strangelove,
        "Dr. Strangelove (or: how I Learned to Stop Worrying and Love the Bomb)",
        "Dr. Strangelove (Or: How I Learned to Stop Worrying and Love the Bomb)"
    );

    testcase!(trimming, "  this is trimming", "This Is Trimming");

    testcase!(trimming2, "this is trimming  ", "This Is Trimming");

    testcase!(trimming3, "  this is trimming  ", "This Is Trimming");

    testcase!(
        yelling,
        "IF IT’S ALL CAPS, FIX IT",
        "If It’s All Caps, Fix It"
    );

    testcase!(
        slashes,
        "What could/should be done about slashes?",
        "What Could/Should Be Done About Slashes?"
    );

    testcase!(
        paths,
        "Never touch paths like /var/run before/after /boot",
        "Never Touch Paths Like /var/run Before/After /boot"
    );

    // TODO: Implement these
    // testcase!(
    //     in_flight,
    //     "The in-flight entertainment was excellent",
    //     "The In-Flight Entertainment Was Excellent"
    // );

    // testcase!(
    //     stand_in,
    //     "The Stand-in teacher gave us homework",
    //     "The Stand-In Teacher Gave Us Homework"
    // );

    testcase!(
        man_in_the_middle,
        "They executed a man-in-the-middle attack",
        "They Executed a Man-in-the-Middle Attack"
    );

    testcase!(
        man_in_the_machine,
        "Jonathan Kim on Alex Gibney’s ‘Steve Jobs: The man in the machine’",
        "Jonathan Kim on Alex Gibney’s ‘Steve Jobs: The Man in the Machine’"
    );

    testcase!(
        lower_small_words,
        "Way Of The Dragon makes Of In An A lowercase",
        "Way of the Dragon Makes of in an a Lowercase"
    );

    testcase!(
        small_greek_letters,
        "μ",
        "Μ"
    );
}

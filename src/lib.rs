extern crate regex;

use regex::{Regex, Captures};

const SMALL_WORDS: [&str; 19] = [
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

// https://stackoverflow.com/a/38406885
fn ucfirst(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn is_digital_resource(word: &str) -> bool {
    let re = Regex::new(
        r"(?x)
        \A
        (?: (?: [/\\]) [[:alpha:]]+ [-_[:alpha:]/\\]+ | # file path or
          [-_[:alpha:]]+ [@.:] [-_[:alpha:]@.:/]+ )     # URL, domain, or email",
    ).expect("unable to compile file/url regex");
    re.is_match(word)
}

// E.g. iPhone or DuBois
fn has_internal_caps(word: &str) -> bool {
    word.chars().skip(1).any(|chr| chr.is_uppercase())
}

fn has_internal_slashes(word: &str) -> bool {
    word.chars().skip(1).any(|chr| chr == '/')
}

fn starts_with_bracket(word: &str) -> bool {
    match word.chars().next() {
        Some('(') => true,
        _ => false,
    }
}

fn fix_small_word_at_start(text: &str) -> String {
    let re = Regex::new(&format!(
        r#"(?x)
        ( \A [[:punct:]]*        # start of title...
        |  [:.;?!]\x20+          # or of subsentence...
        |  \x20['"“‘(\[]\x20* )  # or of inserted subphrase...
        ( {small_re} ) \b        # ... followed by small word
        "#,
        small_re = SMALL_WORDS.join("|")
    )).expect("unable to compile fix_small_word_at_start regex");

    re.replace_all(text, |captures: &Captures| {
        let mut result = captures[1].to_owned();
        result.push_str(&ucfirst(&captures[2]));
        result
    }).to_string()
}

fn fix_small_word_at_end(text: &str) -> String {
    let re = Regex::new(&format!(
        r#"(?x)
        \b ( {small_re} )     # small word...
        ( [[:punct:]]* \z     # ... at the end of the title...
        |   ['"’”)\]] \x20 )  # ... or of an inserted subphrase?
        "#,
        small_re = SMALL_WORDS.join("|")
    )).expect("unable to compile fix_small_word_at_end regex");

    re.replace_all(text, |captures: &Captures| {
        let mut result = ucfirst(&captures[1]);
        result.push_str(&captures[2]);
        result
    }).to_string()
}


pub fn titlecase(input: &str) -> String {
    let small_re = Regex::new(&format!(r"\A(?:{})\z", SMALL_WORDS.join("|")))
        .expect("unable to compile small words regex");
    let contains_lowercase = Regex::new(r"[[:lower:]]").expect("unable to compile lowercase regex");

    let trimmed_input = input.trim();

    // If input is yelling (all uppercase) make lowercase
    let result = if !contains_lowercase.is_match(trimmed_input) {
        trimmed_input.to_lowercase()
    } else {
        trimmed_input.to_string()
    };

    let words = Regex::new(
        r"(?x)
         (_*)
         ([\w'’.:/@\[\]/()]+)
         (_*)",
    ).expect("unable to compile regex");

    let result = words
        .replace_all(&result, |captures: &Captures| {
            let mut result = captures
                .get(1)
                .map(|cap| cap.as_str())
                .unwrap_or("")
                .to_owned();
            let word = &captures[2];

            result.push_str(&if is_digital_resource(word) {
                // pass through
                word.to_owned()
            } else if small_re.is_match(word) {
                word.to_lowercase()
            } else if starts_with_bracket(word) {
                let rest = titlecase(&word.chars().skip(1).collect::<String>());
                format!("({}", rest)
            } else if has_internal_slashes(word) {
                word.split("/")
                    .map(|sub_word| titlecase(sub_word))
                    .collect::<Vec<_>>()
                    .join("/")
            } else if has_internal_caps(word) {
                // Preserve internal caps like iPhone or DuBois
                word.to_owned()
            } else {
                ucfirst(word)
            });

            result.push_str(captures.get(3).map(|cap| cap.as_str()).unwrap_or(""));
            result
        })
        .to_string();

    // Now deal with small words at the start and end of the text
    fix_small_word_at_end(&fix_small_word_at_start(&result))
}

macro_rules! testcase {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!(titlecase($input), $expected);
        }
    }
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

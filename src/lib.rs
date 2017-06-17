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

// TODO: Deal with AT&T and Q&A

// https://stackoverflow.com/a/38406885
fn ucfirst(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn titlecase(input: &str) -> String {
    let apos = r"(?: ['’] [[:lower:]]* )?";
    let small_re = SMALL_WORDS.join("|");
    let contains_lowercase = Regex::new(r"[[:lower:]]").expect("unable to compile lowercase regex");

    let trimmed_input = input.trim();

    // If input is yelling (all uppercase) make lowercase
    let result = if !contains_lowercase.is_match(trimmed_input) {
        trimmed_input.to_lowercase()
    }
    else {
        trimmed_input.to_string()
    };

    // TODO: Extract each phase into its own method
    let re = Regex::new(&format!(r"(?x)
        \b _* (?:
            ( \x20[/\\] [[:alpha:]]+ [-_[:alpha:]/\\]+ |  # file path or
            [-_[:alpha:]]+ [@.:] [-_[:alpha:]@.:/]+ {apos} )   # URL, domain, or email
            |
            ( (?i) {small_re} {apos} )                         # or small word (case-insensitive)
            |
            ( [[:alpha:]] [[:lower:]'’()\[\]{{}}]* {apos} )    # or word w/o internal caps
            |
            ( [[:alpha:]] [[:alpha:]'’()\[\]{{}}]* {apos} )    # or some other word
        ) _* \b", apos = apos, small_re = small_re)).expect("unable to compile regex");

    let result = re.replace_all(&result, |captures: &Captures| {
        if let Some(file_url_domain_or_email) = captures.get(1) {
            file_url_domain_or_email.as_str().to_string()
        }
        else if let Some(small_word) = captures.get(2) {
            small_word.as_str().to_lowercase()
        }
        else if let Some(lower_word) = captures.get(3) {
            ucfirst(lower_word.as_str())
        }
        else {
            // preserve other kinds of words
            captures[4].to_string()
        }
    }).to_string();

    // exceptions for small words: capitalize at start and end of title
    let re = Regex::new(&format!(r#"(?xi)
        ( \A [[:punct:]]*        # start of title...
        |  [:.;?!]\x20+             # or of subsentence...
        |  \x20['"“‘(\[]\x20*     )  # or of inserted subphrase...
        ( {small_re} ) \b          # ... followed by small word
        "#, small_re = small_re)).expect("unable to complie regex 2");

    let result = re.replace_all(&result, |captures: &Captures| {
        let mut replacement = captures[1].to_string();
        replacement.push_str(&ucfirst(&captures[2]));
        replacement
    }).to_string();

    let re = Regex::new(&format!(r#"(?xi)
        \b ( {small_re} )      # small word...
        ( [[:punct:]]* \z    # ... at the end of the title...
        |   ['"’”)\]] \x20 )   # ... or of an inserted subphrase?
        "#, small_re = small_re)).expect("unable to complie regex 3");

    re.replace_all(&result, |captures: &Captures| {
        let mut replacement = ucfirst(&captures[1]);
        replacement.push_str(&captures[2]);
        replacement
    }).to_string()
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
    name,
    "2lmc Spool: 'Gruber on OmniFocus and Vapo(u)rware'",
    "2lmc Spool: 'Gruber on OmniFocus and Vapo(u)rware'"
);

testcase!(
    curly_double_quotes,
    "Have you read “The Lottery”?",
    "Have You Read “The Lottery”?"
);

testcase!(
    brackets,
    "your hair[cut] looks (nice)",
    "Your Hair[cut] Looks (Nice)"
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
    "Read markdown_rules.txt to find out how _underscores around words_ will be interpretted",
    "Read markdown_rules.txt to Find Out How _Underscores Around Words_ Will Be Interpretted"
);

testcase!(
    q_and_a,
    "Q&A with Steve Jobs: 'That's what happens in technology'",
    "Q&A With Steve Jobs: 'That's What Happens in Technology'"
);

testcase!(at_and_t, "What is AT&T's problem?", "What Is AT&T's Problem?");

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
    small_word_at_start,
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

fn file_paths(input: &str) -> Vec<String> {
    let re = Regex::new(r"(\x20[/\\][[:alpha:]]+[-_[:alpha:]/\\]+)|.").expect("file_paths regex");
    re.captures_iter(input).map(|cap| cap[1].to_string()).collect::<Vec<_>>()
}


#[test]
fn test_file_paths() {
    let a = file_paths("Never touch paths like /var/run before/after /boot");
    assert_eq!(a, vec![" /var/run".to_string(), " /boot".to_string()]);
}

const SMALL_WORDS: [&str; 19] = [
    "(?<!q&)a",
    "an",
    "and",
    "as",
    "at(?!&t)",
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

pub fn titlecase(input: &str) -> String {
    input.trim().to_string()
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
    single_quotes,
    "Q&A with Steve Jobs: 'That's what happens in technology'",
    "Q&A With Steve Jobs: 'That's What Happens in Technology'"
);

testcase!(atandt, "What is AT&T's problem?", "What Is AT&T's Problem?");

testcase!(
    atandt2,
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
    yalling,
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

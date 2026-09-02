//! The marker tables, kept apart from the matching so they stay greppable and auditable — the
//! same division [`crate::metadata`] draws between its extraction and its vendor lists.
//!
//! **Every entry here is transcribed from `reference/ai_writing_tells.md`.** Nothing is invented:
//! a marker with no line in that document does not belong in this file, and
//! [`super::tests::every_listed_marker_is_sourced`] enforces exactly that by searching the
//! bundled document for each one. When the document is revised, these tables follow it — never
//! the other way round.

/// §1.1 tier 1 — the largest measured frequency spikes in the PubMed corpus study. `delve`
/// alone rose ~28×. The one tier with peer-reviewed backing behind the individual words.
pub(super) const VOCAB_SIGNATURE: &[&str] =
    &["delve", "intricate", "meticulous", "underscore", "realm", "pivotal"];

/// §1.1 tier 2 — measured risers, smaller spikes.
pub(super) const VOCAB_STRONG: &[&str] = &[
    "robust", "facilitate", "leverage", "showcase", "foster", "garner", "notably",
    "comprehensive", "nuanced", "crucial", "significant", "enhancing",
];

/// §1.1 tier 3 — "common but weaker". Ordinary words with ordinary uses; only worth anything
/// as part of a cluster, which is why they are tiered apart rather than pooled.
pub(super) const VOCAB_WEAK: &[&str] = &[
    "tapestry", "testament", "landscape", "navigate", "harness", "embark", "unlock", "elevate",
    "seamless", "vibrant", "multifaceted", "holistic", "paramount", "myriad",
];

/// Suffixes a listed base word may carry and still count — the document lists
/// "delve / delves / delving" as one marker, so the forms are matched rather than the lemma
/// guessed. Deliberately short: no stemmer, nothing that could turn `foster` into `fostered`
/// into a false claim about a word the study never measured.
pub(super) const SUFFIXES: &[&str] = &["", "s", "es", "d", "ed", "ing", "ly"];

/// Suffixes that may follow a silent-`e` base with the `e` dropped: `delve` → `delving`,
/// `facilitate` → `facilitating`. Without this the document's own headline example is missed,
/// since `delving` does not contain `delve`. Restricted to the two vowel-initial endings where
/// the rule is regular, rather than a general stemmer.
pub(super) const E_DROP_SUFFIXES: &[&str] = &["ing", "ed"];

/// §1.2 — scene-setting openers, connective filler, inflated significance, closers. Practitioner
/// consensus, not measured frequency: each is ordinary English that clusters in default output.
pub(super) const STOCK_PHRASES: &[&str] = &[
    // Scene-setting openers
    "in today's fast-paced world",
    "in an increasingly digital world",
    "in the ever-evolving landscape",
    "in the realm of",
    "imagine a world where",
    "whether you're a beginner or a seasoned",
    "let's dive in",
    "let's explore",
    "let's take a deep dive",
    // Connective filler. The four bare connectives carry their comma, which is how the document
    // writes them and what keeps `in addition,` from firing inside `in additional`.
    "moreover,",
    "furthermore,",
    "additionally,",
    "in addition,",
    "it's worth noting that",
    "it is worth noting that",
    "it's important to note that",
    "it is important to note that",
    "that being said",
    "with that said",
    "at the end of the day",
    "when all is said and done",
    "it's important to remember that",
    // Inflated significance
    "plays a crucial role",
    "plays a vital role",
    "plays a pivotal role",
    "stands as a testament",
    "sheds light on",
    "paves the way for",
    "at the forefront of",
    "a game-changer",
    "transform the way we",
    "unlock the potential of",
    "harness the power of",
    "navigating the complexities of",
    "the intersection of",
    "a double-edged sword",
    "cutting-edge",
    "state-of-the-art",
    // Closers
    "in conclusion",
    "to sum up",
    "in summary",
    "i hope this helps",
    "the choice depends on your specific needs",
    "the possibilities are endless",
];

/// §1.5 — artifacts of chat-assistant RLHF rather than of writing: sycophantic openers,
/// self-identification, unsolicited disclaimers, hedging stacks, offers of follow-up.
/// Strong signal when text was pasted straight out of a chat window.
pub(super) const ASSISTANT_REGISTER: &[&str] = &[
    "great question",
    "what a fascinating topic",
    "i'd be happy to help",
    "i would be happy to help",
    "as an ai language model",
    "i don't have personal opinions",
    "i don't have personal feelings",
    "i don't have personal experiences",
    "as of my last update",
    "this is not professional advice",
    "consult a qualified professional",
    "individual results may vary",
    "there are several factors to consider",
    "may potentially",
    "can sometimes",
    "it's possible that in some cases",
    "would you like me to",
    "i apologize for the confusion",
    // From the document's Part 2, folded in HERE as generic rather than kept as per-model
    // fingerprints. Part 2 lists these as one model's surface habits, but its own evidence
    // warning says there is "essentially no peer-reviewed stylometry separating" the models and
    // marks the section impressionistic — so attributing a hit to a vendor would be inventing
    // precision the source explicitly disclaims. They are ordinary assistant register; that is
    // all this crate will say about them.
    "i should note",
    "that said",
    "to be clear",
    "here's the thing",
];

/// §1.3 "other recurring shapes" that are plain phrases rather than sentence patterns. The
/// pattern-shaped ones (range sweeps, rhetorical openers, disguised listicles) are regexes in
/// [`super::shape`], since no word list can express them.
pub(super) const RECURRING_SHAPES: &[&str] = &["think of it like"];

/// §1.8 — leakage: nobody's style, somebody's forgotten deletion. The document calls these
/// "the free wins", and they are the only family here that comes close to being conclusive.
pub(super) const LEAKAGE: &[&str] = &[
    "here is the essay you requested",
    "here's the essay you requested",
    "certainly! here is",
    "certainly! here's",
    "[your name]",
    "[insert company here]",
    "[insert",
    "[date]",
    "regenerate response",
];

/// §1.9 — visible typographic characters. Invisible carriers are [`crate::hidden`]'s job; what
/// is left here is the "correct typography a keyboard doesn't give you" set, every one of which
/// the document rates weak-to-very-weak ALONE. Hence `per_1000`: the document's own rule is that
/// "density, not existence, is the signal" — a human might use 2–3 em dashes in a piece where
/// generated text has 20+.
///
/// One entry per ROW of the document's §1.9 table, not one per character — the table groups
/// "Curly quotes / apostrophes" and "Arrow, multiplication, minus" as single signals with a
/// single strength, and splitting them would report one row three times over. Each name is the
/// document's own row label, which is what lets the sourcing test hold this table to it.
///
/// `(characters, row name, per_1000_words_threshold)`.
pub(super) const TYPOGRAPHY: &[(&[char], &str, f64)] = &[
    // "Weak — see false positives", and the subject of a 2025 meme that did real harm. The
    // threshold is the document's own "2–3 in a piece vs 20+" rule, normalised per 1000 words.
    (&['\u{2014}'], "em dash", 8.0),
    (&['\u{2013}'], "en dash in ranges", 6.0),
    // "Very weak — Word and Google Docs insert these automatically." Hence the high bar.
    (&['\u{2018}', '\u{2019}', '\u{201C}', '\u{201D}'], "curly quotes / apostrophes", 25.0),
    (&['\u{2026}'], "ellipsis character", 6.0),
    (&['\u{2022}'], "bullet character", 6.0),
    // "A cluster of 'correct' typographic signs where a keyboard would give `->`, `x`, `-`."
    (&['\u{2192}', '\u{00D7}', '\u{2212}'], "arrow, multiplication, minus", 4.0),
];

/// §1.4 — the emoji the document names as bullet/heading markers in professional contexts.
/// Exactly its list: a longer one would be guesswork, and the point is a *habit* of decoration,
/// which these six catch as well as sixty would.
pub(super) const BULLET_EMOJI: &[char] = &['✅', '🚀', '💡', '📊', '🔥', '🎯'];

/// Entries above that are not verbatim in the source document but *derive* from one that is —
/// a contraction expanded, or a slash-list ("crucial / vital / pivotal") written out. Each is
/// paired with the document text it comes from, so
/// [`super::tests::every_listed_marker_is_sourced`] can hold even these to the source rather
/// than waving them through.
/// (`#[cfg(test)]`: a ledger for the sourcing check, not data the detectors read.)
#[cfg(test)]
pub(super) const DERIVED: &[(&str, &str)] = &[
    ("it is worth noting that", "it's worth noting that"),
    ("it is important to note that", "it's important to note that"),
    ("plays a crucial role", "plays a crucial / vital / pivotal role in"),
    ("plays a vital role", "plays a crucial / vital / pivotal role in"),
    ("plays a pivotal role", "plays a crucial / vital / pivotal role in"),
    ("i would be happy to help", "i'd be happy to help"),
    ("i don't have personal feelings", "i don't have personal opinions / feelings / experiences"),
    ("i don't have personal experiences", "i don't have personal opinions / feelings / experiences"),
    ("here's the essay you requested", "here is the essay you requested"),
    ("certainly! here's", "certainly! here is"),
];

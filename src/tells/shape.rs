//! The detectors themselves: one function per family, each appending to the shared `Vec<Tell>`.
//!
//! Two conventions hold throughout. **A family reports once per distinct marker, with a count**,
//! not once per occurrence — the source document's point is that density matters, and a report
//! that lists "significant" forty times has buried its own signal. And **every threshold below
//! is named and justified against the document**, never tuned to taste: a number nobody can
//! trace is a number nobody can argue with.

use std::sync::OnceLock;

use regex::Regex;

use super::lexicon;
use super::{Doc, Family, Tell};

/// Bytes that continue a word. Non-ASCII counts: it keeps `delve` from matching inside a word
/// that merely ends in those letters before an accented character.
const fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte >= 0x80
}

/// Every whole-word occurrence of `base` in any of its listed forms, as `(offset, length)` into
/// the lowercased text.
///
/// Two stems are searched: the base itself with [`lexicon::SUFFIXES`], and — for a base ending in
/// silent `e` — the base without it with [`lexicon::E_DROP_SUFFIXES`], which is the only way
/// `delve` reaches `delving`. Hits are keyed by start offset so a form both stems can produce
/// (`delved`) is one hit, not two.
fn word_hits(lower: &str, base: &str) -> Vec<(usize, usize)> {
    let bytes = lower.as_bytes();
    let mut stems: Vec<(&str, &[&str])> = vec![(base, lexicon::SUFFIXES)];
    if let Some(dropped) = base.strip_suffix('e') {
        stems.push((dropped, lexicon::E_DROP_SUFFIXES));
    }

    let mut hits: Vec<(usize, usize)> = Vec::new();
    for (stem, suffixes) in stems {
        for (at, _) in lower.match_indices(stem) {
            if at > 0 && is_word_byte(bytes[at - 1]) {
                continue;
            }
            let after = at + stem.len();
            // The longest suffix that still lands on a word boundary — so `underscores` matches
            // once as the plural rather than twice, as itself and as a prefix of itself.
            let end = suffixes
                .iter()
                .filter_map(|suffix| {
                    let end = after + suffix.len();
                    let fits = end <= bytes.len() && &bytes[after..end] == suffix.as_bytes();
                    let bounded = end == bytes.len() || !is_word_byte(bytes[end]);
                    (fits && bounded).then_some(end)
                })
                .max();
            if let Some(end) = end {
                hits.push((at, end - at));
            }
        }
    }
    // One hit per position, the longest form winning: `delved` is reachable from both stems.
    hits.sort_unstable_by(|left, right| left.0.cmp(&right.0).then(right.1.cmp(&left.1)));
    hits.dedup_by_key(|(at, _)| *at);
    hits
}

/// One tell per distinct marker: the first line it appears on, and how often. `label` names the
/// marker, `detail_prefix` says which tier or list it came from.
fn report_counted(
    doc: &Doc<'_>,
    out: &mut Vec<Tell>,
    family: Family,
    label: &str,
    detail_prefix: &str,
    hits: &[(usize, usize)],
) {
    let Some(&(first, len)) = hits.first() else { return };
    let times = match hits.len() {
        1 => "once".to_string(),
        count => format!("{count} times"),
    };
    out.push(Tell {
        family,
        line: Some(doc.line_of(first)),
        excerpt: doc.excerpt(first, len),
        detail: format!("{detail_prefix}: `{label}`, {times}"),
    });
}

/// §1.1 — the excess-vocabulary set, by tier. The only family whose individual entries have
/// corpus backing, which is why the tier travels in the detail line.
pub(super) fn vocabulary(doc: &Doc<'_>, out: &mut Vec<Tell>) {
    for (tier, words) in [
        ("signature tier (largest measured spikes)", lexicon::VOCAB_SIGNATURE),
        ("strong tier", lexicon::VOCAB_STRONG),
        ("weaker tier", lexicon::VOCAB_WEAK),
    ] {
        for base in words {
            let hits = word_hits(&doc.lower, base);
            report_counted(doc, out, Family::ExcessVocabulary, base, tier, &hits);
        }
    }
}

/// §1.2, §1.5, §1.8 — the three phrase lists. Plain substring matching: every entry is a
/// multi-word formula, so word boundaries add nothing a phrase's own spaces don't already give.
pub(super) fn phrases(doc: &Doc<'_>, out: &mut Vec<Tell>) {
    for (family, list, tier) in [
        (Family::StockPhrase, lexicon::STOCK_PHRASES, "stock phrase"),
        (Family::AssistantRegister, lexicon::ASSISTANT_REGISTER, "assistant register"),
        (Family::Leakage, lexicon::LEAKAGE, "leakage"),
        (Family::RecurringShape, lexicon::RECURRING_SHAPES, "recurring shape"),
    ] {
        for phrase in list {
            let hits: Vec<(usize, usize)> =
                doc.lower.match_indices(phrase).map(|(at, found)| (at, found.len())).collect();
            report_counted(doc, out, family, phrase, tier, &hits);
        }
    }
}

/// The negative-parallelism shapes, as the document spells them: the comma pivot, the em-dash
/// pivot, the sentence-split version, and `not only … but also`.
fn parallelism_patterns() -> &'static [Regex] {
    static PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();
    PATTERNS.get_or_init(|| {
        [
            // "It's not X, it's Y" — the canonical form.
            r"(?i)\b(?:it'?s|it is|this is|that'?s|that is|we'?re|they'?re)\s+not\s+(?:just\s+)?(?:about\s+)?[^.!?;\n]{2,80}?[,;]\s*(?:it'?s|it is|this is|that'?s|that is|we'?re|they'?re)\b",
            // "It's not just about X — it's about Y": the em-dash pivot the document says
            // travels with this family.
            r"(?i)\bnot\s+just\s+[^.!?\n\u{2014}\u{2013}]{2,60}[\u{2014}\u{2013}]\s*(?:it'?s|it is|but)\b",
            // "This isn't X. It's Y." — split across a sentence boundary.
            r"(?i)\b(?:isn'?t|aren'?t|wasn'?t|weren'?t)\s+[^.!?\n]{2,80}[.!]\s+(?:it'?s|it is|this is|that'?s)\b",
            // "Not only … but also …".
            r"(?i)\bnot only\b[^.!?\n]{2,100}?\bbut also\b",
        ]
        .iter()
        .map(|pattern| Regex::new(pattern).expect("a literal pattern compiled at startup"))
        .collect()
    })
}

/// §1.3 — negative parallelism. Reported once per occurrence, not aggregated: the caveat is that
/// only *empty* uses are a tell, and that judgement needs the reader to see each one.
pub(super) fn negative_parallelism(doc: &Doc<'_>, out: &mut Vec<Tell>) {
    for pattern in parallelism_patterns() {
        for found in pattern.find_iter(doc.text) {
            out.push(Tell {
                family: Family::NegativeParallelism,
                line: Some(doc.line_of(found.start())),
                excerpt: doc.excerpt(found.start(), found.len()),
                detail: "contrastive construction — a tell only when both clauses say one thing"
                    .to_string(),
            });
        }
    }
}

/// §1.3 — the flattened rule-of-three: `A, B, and C` where the members are interchangeable.
///
/// Narrowed to single-word members on purpose, twice over. "The classical tricolon varies its
/// members; the AI version doesn't", and equal length is the only part of that a machine can
/// see — so the match requires the three to sit within [`SPREAD`] characters of each other.
/// Multi-word members are left out because a greedy pattern that allows them swallows the
/// preceding words ("It **is efficient**, effective, and reliable") and hides the very list it
/// was looking for; longer members also drift in length for ordinary reasons, so flagging them
/// would report good writing. Precision over recall, which is this crate's posture throughout.
pub(super) fn flat_tricolon(doc: &Doc<'_>, out: &mut Vec<Tell>) {
    /// Character spread across the three members that still counts as "equal in length".
    const SPREAD: usize = 3;
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    let pattern = PATTERN.get_or_init(|| {
        Regex::new(r"(?i)\b([a-z][a-z'\-]{1,20}),\s+([a-z][a-z'\-]{1,20}),\s+and\s+([a-z][a-z'\-]{1,20})\b")
            .expect("a literal pattern compiled at startup")
    });
    for found in pattern.captures_iter(doc.text) {
        let members: Vec<&str> = (1..=3).filter_map(|group| found.get(group)).map(|m| m.as_str()).collect();
        if members.len() != 3 {
            continue;
        }
        let lengths: Vec<usize> = members.iter().map(|member| member.chars().count()).collect();
        let spread = lengths.iter().max().unwrap_or(&0) - lengths.iter().min().unwrap_or(&0);
        if spread > SPREAD {
            continue;
        }
        let whole = found.get(0).expect("group 0 always exists");
        out.push(Tell {
            family: Family::FlatTricolon,
            line: Some(doc.line_of(whole.start())),
            excerpt: doc.excerpt(whole.start(), whole.len()),
            detail: format!("three members of {:?} characters — interchangeable, not varied", lengths),
        });
    }
}

/// Sentences, by a deliberately naive split: a `.`, `!` or `?` followed by whitespace or the end
/// of the text. Abbreviations and initials therefore over-split. That is acceptable *here*
/// because the only consumer measures the spread of lengths, where a little uniform noise moves
/// the statistic toward "varied" — the direction that reports nothing.
fn sentences(text: &str) -> Vec<&str> {
    let mut found = Vec::new();
    let mut start = 0;
    let bytes = text.as_bytes();
    for (at, ch) in text.char_indices() {
        if !matches!(ch, '.' | '!' | '?') {
            continue;
        }
        let next = bytes.get(at + 1);
        if next.is_none_or(|byte| byte.is_ascii_whitespace()) {
            let sentence = text[start..=at].trim();
            if !sentence.is_empty() {
                found.push(sentence);
            }
            start = at + 1;
        }
    }
    let tail = text[start..].trim();
    if !tail.is_empty() {
        found.push(tail);
    }
    found
}

/// §1.3 — "a rhythm of near-identical clauses that no one writes by hand". Returns the sentence
/// count so the caller can report it either way.
///
/// **This is not burstiness.** Burstiness is the variance of per-sentence *perplexity*, which
/// needs a reference language model; this is the variance of sentence *length*, which is the
/// visible shadow of it and nothing more. The name says which.
pub(super) fn sentence_uniformity(doc: &Doc<'_>, out: &mut Vec<Tell>) -> usize {
    /// Below this many sentences there is no rhythm to speak of, and short human text would
    /// trip the check constantly.
    const ENOUGH: usize = 8;
    /// Coefficient of variation under which the lengths count as flat. Human long-form runs
    /// well above this; the document's own contrast is Claude's reported 5-to-40-word swing
    /// (a CV near 0.6) against "more uniform output elsewhere".
    const FLAT: f64 = 0.35;

    let lengths: Vec<f64> = sentences(doc.text)
        .iter()
        .map(|sentence| sentence.split_whitespace().count() as f64)
        .filter(|count| *count > 0.0)
        .collect();
    if lengths.len() < ENOUGH {
        return lengths.len();
    }
    let mean = lengths.iter().sum::<f64>() / lengths.len() as f64;
    if mean <= 0.0 {
        return lengths.len();
    }
    let variance =
        lengths.iter().map(|length| (length - mean).powi(2)).sum::<f64>() / lengths.len() as f64;
    let spread = variance.sqrt() / mean;
    if spread < FLAT {
        out.push(Tell {
            family: Family::SentenceUniformity,
            line: None,
            excerpt: String::new(),
            detail: format!(
                "{} sentences averaging {mean:.0} words, variation {:.0}% (flat below {:.0}%)",
                lengths.len(),
                spread * 100.0,
                FLAT * 100.0
            ),
        });
    }
    lengths.len()
}

/// §1.4 — the formatting habits, each aggregated into one tell with a count.
///
/// Every threshold here is "enough that it is a habit rather than an instance": the document's
/// complaint is about *every* bullet leading with bold, not about one that does.
pub(super) fn formatting(doc: &Doc<'_>, out: &mut Vec<Tell>, options: super::Options) {
    /// How many instances make a habit.
    const HABIT: usize = 3;
    static BOLD_BULLET: OnceLock<Regex> = OnceLock::new();
    static HEADING: OnceLock<Regex> = OnceLock::new();
    static RULE: OnceLock<Regex> = OnceLock::new();
    static INLINE_BOLD: OnceLock<Regex> = OnceLock::new();
    let bold_bullet = BOLD_BULLET
        .get_or_init(|| Regex::new(r"(?m)^\s*[-*+]\s+\*\*[^*\n]{1,60}\*\*\s*:?").expect("literal"));
    let heading = HEADING.get_or_init(|| Regex::new(r"(?m)^\s*#{1,6}\s+(.+)$").expect("literal"));
    let rule = RULE.get_or_init(|| Regex::new(r"(?m)^\s*(?:-{3,}|\*{3,}|_{3,})\s*$").expect("literal"));
    let inline_bold =
        INLINE_BOLD.get_or_init(|| Regex::new(r"\*\*[^*\n]{1,60}\*\*").expect("literal"));

    let mut note = |count: usize, first: Option<usize>, what: &str, why: &str| {
        if count >= HABIT {
            out.push(Tell {
                family: Family::Formatting,
                line: first.map(|at| doc.line_of(at)),
                excerpt: first.map(|at| doc.excerpt(at, 0)).unwrap_or_default(),
                detail: format!("{what} ×{count} — {why}"),
            });
        }
    };

    let bullets: Vec<_> = bold_bullet.find_iter(doc.text).collect();
    note(
        bullets.len(),
        bullets.first().map(regex::Match::start),
        "bold-lead-in bullets",
        "named the single most recognisable structural tell, and almost nobody writes them by hand",
    );

    // Bold that isn't a bullet's lead-in and isn't the whole line: emphasis where it isn't
    // doing work. Counted by subtraction so the two never double-report the same run.
    let inline = inline_bold.find_iter(doc.text).count().saturating_sub(bullets.len());
    note(
        inline,
        inline_bold.find_iter(doc.text).nth(bullets.len()).map(|m| m.start()),
        "mid-sentence bold",
        "emphasis scattered where it isn't doing work",
    );

    // Title Case headings: three or more words with nearly all of them capitalised.
    let title_case: Vec<_> = heading
        .captures_iter(doc.text)
        .filter(|found| {
            let text = found.get(1).map_or("", |m| m.as_str());
            let words: Vec<&str> = text.split_whitespace().collect();
            let capitalised = words
                .iter()
                .filter(|word| word.chars().next().is_some_and(char::is_uppercase))
                .count();
            words.len() >= 3 && capitalised * 5 >= words.len() * 4
        })
        .filter_map(|found| found.get(0).map(|m| m.start()))
        .collect();
    note(title_case.len(), title_case.first().copied(), "Title Case headings", "§1.4");

    let rules: Vec<_> = rule.find_iter(doc.text).map(|m| m.start()).collect();
    note(rules.len(), rules.first().copied(), "horizontal rules", "a rule between every section");

    // Emoji as bullet markers or in headings.
    let emoji: Vec<usize> = doc
        .text
        .char_indices()
        .filter(|(_, ch)| lexicon::BULLET_EMOJI.contains(ch))
        .map(|(at, _)| at)
        .collect();
    note(
        emoji.len(),
        emoji.first().copied(),
        "decorative emoji",
        "as bullet markers or in headings, in a professional context",
    );

    // Markdown residue — visible `**` or `##` on a surface that will not render them. Only the
    // caller knows whether that is the case, so this is the one tell gated on [`super::Options`]:
    // in a `.md` file the same characters are simply markup, and reporting them would be noise
    // in every well-formed document in the repository.
    if options.surface_is_plain {
        let residue = inline_bold.find_iter(doc.text).count() + heading.find_iter(doc.text).count();
        let first = inline_bold
            .find(doc.text)
            .or_else(|| heading.find(doc.text))
            .map(|found| found.start());
        note(
            residue,
            first,
            "markdown residue",
            "`**` or `##` pasted into a surface that does not render markdown",
        );
    }
}

/// §1.3 "other recurring shapes" that need a pattern rather than a phrase: the range-sweep
/// opener, the rhetorical question opening a section, and the "disguised listicle" — what you get
/// after telling a model to stop using lists, where the skeleton stays and is wrapped in prose as
/// "The first… The second… The third…".
///
/// Each is an ordinary rhetorical move on its own, so each needs a *habit* to report: two range
/// sweeps or two rhetorical openers, and a disguised listicle only when three ordinals march in
/// order. One of anything here is just writing.
pub(super) fn recurring_shapes(doc: &Doc<'_>, out: &mut Vec<Tell>) {
    /// Two of the same move is a habit; one is a sentence.
    const HABIT: usize = 2;
    static SWEEP: OnceLock<Regex> = OnceLock::new();
    static RHETORICAL: OnceLock<Regex> = OnceLock::new();
    static ORDINAL: OnceLock<Regex> = OnceLock::new();

    // "From X to Y to Z, …" — two hops, which is what makes it a sweep rather than a range.
    let sweep = SWEEP.get_or_init(|| {
        Regex::new(r"(?i)\bfrom\s+[^,.!?\n]{2,40}?\s+to\s+[^,.!?\n]{2,40}?\s+to\s+")
            .expect("literal")
    });
    let sweeps: Vec<usize> = sweep.find_iter(doc.text).map(|m| m.start()).collect();
    if sweeps.len() >= HABIT {
        out.push(Tell {
            family: Family::RecurringShape,
            line: sweeps.first().map(|at| doc.line_of(*at)),
            excerpt: sweeps.first().map_or_else(String::new, |at| doc.excerpt(*at, 40)),
            detail: format!("range-sweep openers (\"from X to Y to Z\") ×{}", sweeps.len()),
        });
    }

    // A question opening a paragraph — the section-opener move, which is only visible at the
    // start of a block rather than mid-flow.
    let rhetorical = RHETORICAL
        .get_or_init(|| Regex::new(r"(?im)^\s*(?:but|so|why|what|how)\b[^.!?\n]{0,90}\?").expect("literal"));
    let openers: Vec<usize> = rhetorical.find_iter(doc.text).map(|m| m.start()).collect();
    if openers.len() >= HABIT {
        out.push(Tell {
            family: Family::RecurringShape,
            line: openers.first().map(|at| doc.line_of(*at)),
            excerpt: openers.first().map_or_else(String::new, |at| doc.excerpt(*at, 40)),
            detail: format!("rhetorical questions opening a section ×{}", openers.len()),
        });
    }

    // The listicle in a trenchcoat: ordinals in order, in prose.
    let ordinal = ORDINAL
        .get_or_init(|| Regex::new(r"(?i)\bthe\s+(first|second|third|fourth|fifth)\b").expect("literal"));
    let mut seen: Vec<String> = ordinal
        .captures_iter(doc.text)
        .filter_map(|found| found.get(1).map(|m| m.as_str().to_lowercase()))
        .collect();
    let first_at = ordinal.find(doc.text).map(|m| m.start());
    seen.dedup();
    if seen.len() >= 3 {
        out.push(Tell {
            family: Family::RecurringShape,
            line: first_at.map(|at| doc.line_of(at)),
            excerpt: first_at.map_or_else(String::new, |at| doc.excerpt(at, 30)),
            detail: format!(
                "a listicle in a trenchcoat — {} ordinals marching in prose",
                seen.len()
            ),
        });
    }
}

/// §1.4 — symmetry and over-structuring. "Human documents are lumpy; they dwell on what the
/// writer actually cares about."
///
/// Two measurements, both of spread rather than of any absolute shape: paragraphs that are all
/// one length, and a header for every couple of sentences.
pub(super) fn structural_symmetry(doc: &Doc<'_>, out: &mut Vec<Tell>) {
    /// Fewer paragraphs than this and "uniform" means nothing.
    const ENOUGH_PARAGRAPHS: usize = 4;
    /// Coefficient of variation below which paragraph lengths count as uniform. Stricter than
    /// the sentence-level bar: paragraphs vary more freely, so evenness is more telling.
    const UNIFORM: f64 = 0.25;
    /// A header more often than this many words is the "header for every two-sentence section"
    /// the document names.
    const CROWDED_WORDS: usize = 40;
    /// And it takes this many headers before the rate means anything.
    const ENOUGH_HEADERS: usize = 4;

    let paragraphs: Vec<f64> = doc
        .text
        .split("\n\n")
        .map(|block| block.split_whitespace().count() as f64)
        .filter(|count| *count > 0.0)
        .collect();
    if paragraphs.len() >= ENOUGH_PARAGRAPHS {
        let mean = paragraphs.iter().sum::<f64>() / paragraphs.len() as f64;
        if mean > 0.0 {
            let variance = paragraphs.iter().map(|len| (len - mean).powi(2)).sum::<f64>()
                / paragraphs.len() as f64;
            let spread = variance.sqrt() / mean;
            if spread < UNIFORM {
                out.push(Tell {
                    family: Family::StructuralSymmetry,
                    line: None,
                    excerpt: String::new(),
                    detail: format!(
                        "{} paragraphs averaging {mean:.0} words, variation {:.0}% \
                         (uniform below {:.0}%) — human documents are lumpy",
                        paragraphs.len(),
                        spread * 100.0,
                        UNIFORM * 100.0
                    ),
                });
            }
        }
    }

    static HEADING: OnceLock<Regex> = OnceLock::new();
    let heading = HEADING.get_or_init(|| Regex::new(r"(?m)^\s*#{1,6}\s+\S").expect("literal"));
    let headers = heading.find_iter(doc.text).count();
    if headers >= ENOUGH_HEADERS {
        let per_header = doc.words / headers;
        if per_header < CROWDED_WORDS {
            out.push(Tell {
                family: Family::StructuralSymmetry,
                line: None,
                excerpt: String::new(),
                detail: format!(
                    "{headers} headings for {} words — one every {per_header}, where the \
                     document's complaint is a header per two-sentence section",
                    doc.words
                ),
            });
        }
    }
}

/// §1.6 — what is *absent*: "Generic examples (\"a company might…\") where a human would name the
/// company, the year, the number."
///
/// Counts the concrete: any token carrying a digit, and any capitalised word that is not starting
/// a sentence (a stand-in for proper nouns). A rate this low over this much text means the prose
/// names nothing. **The weakest signal in the module** — abstract argument and introductory
/// material score identically — which is why the bar is set where almost nothing human reaches it.
pub(super) fn missing_specifics(doc: &Doc<'_>, out: &mut Vec<Tell>) {
    /// Below this, the rate is noise.
    const ENOUGH_WORDS: usize = 150;
    /// Concrete tokens per 100 words under which the text names nothing at all.
    const BARREN: f64 = 1.0;
    if doc.words < ENOUGH_WORDS {
        return;
    }
    let mut concrete = 0_usize;
    let mut sentence_start = true;
    for token in doc.text.split_whitespace() {
        let bare = token.trim_matches(|ch: char| !ch.is_alphanumeric());
        if bare.is_empty() {
            continue;
        }
        let capitalised = bare.chars().next().is_some_and(char::is_uppercase);
        if bare.chars().any(|ch| ch.is_ascii_digit()) || (capitalised && !sentence_start) {
            concrete += 1;
        }
        sentence_start = token.ends_with(['.', '!', '?', ':']);
    }
    let rate = concrete as f64 * 100.0 / doc.words as f64;
    if rate < BARREN {
        out.push(Tell {
            family: Family::MissingSpecifics,
            line: None,
            excerpt: String::new(),
            detail: format!(
                "{concrete} names, numbers or dates in {} words ({rate:.1} per 100, barren \
                 below {BARREN:.1}) — no company, no year, no figure",
                doc.words
            ),
        });
    }
}

/// §1.6 — citation-shaped strings, listed **for verification, not as an accusation**.
///
/// "Confident fabrication … the formatting is right and the content is invented — the single most
/// consequential tell." A scanner cannot know whether a DOI resolves, so it does the one useful
/// thing it can: surface the checkable items so a reader goes and checks them.
///
/// Restricted to DOIs and arXiv identifiers, which are almost always citations. Bare URLs are
/// deliberately excluded: every ordinary document has them, and reporting them would drown the
/// two shapes that actually mean "a source is being claimed here".
pub(super) fn checkable_claims(doc: &Doc<'_>, out: &mut Vec<Tell>) {
    static CITATION: OnceLock<Vec<(Regex, &'static str)>> = OnceLock::new();
    let patterns = CITATION.get_or_init(|| {
        vec![
            (Regex::new(r"\b10\.\d{4,9}/[^\s,;)\]]+").expect("literal"), "DOI"),
            (Regex::new(r"(?i)\barxiv:\s*\d{4}\.\d{4,5}(v\d+)?").expect("literal"), "arXiv id"),
        ]
    });
    for (pattern, kind) in patterns {
        let hits: Vec<regex::Match<'_>> = pattern.find_iter(doc.text).collect();
        let Some(first) = hits.first() else { continue };
        out.push(Tell {
            family: Family::CheckableClaim,
            line: Some(doc.line_of(first.start())),
            excerpt: doc.excerpt(first.start(), first.len()),
            detail: format!("{} {kind}(s) cited — verify each resolves", hits.len()),
        });
    }
}

/// §1.9 — typographic characters, by density. Never by presence: the document is unusually
/// insistent here, because this is the family whose careless use "caused real harm" — teachers
/// flagging essays and writers self-censoring punctuation they had used for decades.
pub(super) fn typography(doc: &Doc<'_>, out: &mut Vec<Tell>) {
    /// Below this many words a rate per thousand is meaningless arithmetic.
    const ENOUGH_WORDS: usize = 120;
    if doc.words < ENOUGH_WORDS {
        return;
    }
    for (chars, name, threshold) in lexicon::TYPOGRAPHY {
        let hits: Vec<(usize, char)> =
            doc.text.char_indices().filter(|(_, found)| chars.contains(found)).collect();
        if hits.is_empty() {
            continue;
        }
        let rate = hits.len() as f64 * 1000.0 / doc.words as f64;
        if rate < *threshold {
            continue;
        }
        let codepoints: Vec<String> =
            chars.iter().map(|ch| format!("U+{:04X}", *ch as u32)).collect();
        let (at, ch) = hits[0];
        out.push(Tell {
            family: Family::Typography,
            line: Some(doc.line_of(at)),
            excerpt: doc.excerpt(at, ch.len_utf8()),
            detail: format!(
                "{name} ({}) ×{}, {rate:.1} per 1000 words (flagged above {threshold:.0})",
                codepoints.join("/"),
                hits.len()
            ),
        });
    }
}

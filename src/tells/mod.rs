//! Finding the *writing* tells of AI-generated prose: vocabulary, stock phrases, sentence
//! shapes, formatting habits, assistant register, leakage, and typographic density. The third
//! engine of this crate, beside [`crate::hidden`]'s carrier characters and [`crate::metadata`]'s
//! provenance labels — and by far the least conclusive of the three, deliberately so.
//!
//! # Read this before using the output
//!
//! Everything here is transcribed from `reference/ai_writing_tells.md`, bundled with the crate
//! and reproduced in [`SOURCE_DOCUMENT`]. That document's own warning governs this module, and
//! it is not boilerplate:
//!
//! > **No item here is proof of AI authorship.** Every marker has a legitimate human use, and
//! > the strongest ones are statistical — they show up in *corpora*, not in single documents.
//!
//! Three consequences the API is shaped around:
//!
//! - **Nothing returns a verdict.** [`scan`] returns [`Tell`]s and counts. There is no
//!   `is_ai() -> bool`, no percentage, and no score, because the underlying instrument is a
//!   corpus-level one being borrowed for a document-level question — "which is exactly where it
//!   breaks". A caller that wants to accuse someone has to write that judgement itself.
//! - **Every tell carries its evidence tier** ([`Evidence`]) and its own caveat
//!   ([`Family::caveat`]), so a report can never present community folklore as if it were the
//!   PubMed corpus study.
//! - **Clusters are the unit of meaning.** The document is emphatic: "Never on a single marker."
//!   [`Report::families`] exists so a caller can weigh how many independent families fired,
//!   rather than counting hits.
//!
//! # What is not implemented
//!
//! Every detector the document describes and this module lacks has a documented stub in
//! [`pending`], and appears in [`pending::GAPS`] so a report can state what it did not look for.
//! The short version: perplexity and burstiness (§1.7) need a reference language model;
//! resolving the citations that [`Family::CheckableClaim`] surfaces needs a lookup this crate
//! deliberately cannot perform; and the remaining §1.6 absence tells need judgement.
//!
//! One item there is a refusal rather than a gap: **per-model attribution** (Part 2). The
//! document's own evidence warning rules it out, and markers it lists per-model are folded into
//! the generic tables instead. This crate reports "AI-ish writing habits", never a vendor.

mod lexicon;
pub mod pending;
mod shape;

/// The bundled source document, so a consumer always has the provenance and the caveats beside
/// the code — and so [`tests::every_listed_marker_is_sourced`] can hold the tables to it.
pub const SOURCE_DOCUMENT: &str = include_str!("../../reference/ai_writing_tells.md");

/// How well evidenced a family is, mirroring the source document's own ✅ / ⚠️ / ❓ marking.
/// Carried on every [`Tell`] so a report cannot flatten peer-reviewed corpus data and community
/// folklore into one undifferentiated list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Evidence {
    /// ❓ Impressionistic — practitioner guides and community observation, not measured.
    Community,
    /// ⚠️ A vendor statement or journalism.
    Vendor,
    /// ✅ Corpus study or peer-reviewed work.
    Corpus,
}

impl Evidence {
    /// The document's own mark for this tier.
    #[must_use]
    pub const fn mark(self) -> &'static str {
        match self {
            Self::Corpus => "✅",
            Self::Vendor => "⚠️",
            Self::Community => "❓",
        }
    }
}

/// A class of tell, one per section of the source document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Family {
    /// §1.1 The "excess vocabulary" set — the strongest single finding in the area.
    ExcessVocabulary,
    /// §1.2 Stock phrases: scene-setting openers, connective filler, inflated significance.
    StockPhrase,
    /// §1.3 "It's not X, it's Y" — and the tell that has *not* faded the way "delve" did.
    NegativeParallelism,
    /// §1.3 Rule-of-three whose members are interchangeable and equal in length.
    FlatTricolon,
    /// §1.3 A rhythm of near-identical sentence lengths that no one writes by hand.
    SentenceUniformity,
    /// §1.4 Bold-lead-in bullets, emoji markers, Title Case headings, rule-separated sections.
    Formatting,
    /// §1.3 The other recurring shapes: range sweeps, rhetorical section openers, the
    /// disguised listicle, "think of it like" analogies.
    RecurringShape,
    /// §1.4 Symmetry and over-structuring — sections of one length, a header per two sentences.
    StructuralSymmetry,
    /// §1.5 Chat-assistant register: sycophancy, self-identification, hedging stacks.
    AssistantRegister,
    /// §1.6 What is *absent* — generic prose with no names, numbers or dates in it.
    MissingSpecifics,
    /// §1.6 Citation-shaped strings, offered for verification rather than judged.
    CheckableClaim,
    /// §1.8 Leakage — a preamble or placeholder somebody forgot to delete.
    Leakage,
    /// §1.9 Typographic characters a keyboard doesn't produce, by density.
    Typography,
}

impl Family {
    /// Every family, in document-section order. The count is what a caller compares a cluster
    /// against, so it lives here rather than being written out at each call site.
    pub const ALL: &'static [Self] = &[
        Self::ExcessVocabulary,
        Self::StockPhrase,
        Self::NegativeParallelism,
        Self::FlatTricolon,
        Self::SentenceUniformity,
        Self::RecurringShape,
        Self::Formatting,
        Self::StructuralSymmetry,
        Self::AssistantRegister,
        Self::MissingSpecifics,
        Self::CheckableClaim,
        Self::Leakage,
        Self::Typography,
    ];

    /// The section of the source document this family comes from.
    #[must_use]
    pub const fn section(self) -> &'static str {
        match self {
            Self::ExcessVocabulary => "1.1",
            Self::StockPhrase => "1.2",
            Self::NegativeParallelism
            | Self::FlatTricolon
            | Self::SentenceUniformity
            | Self::RecurringShape => "1.3",
            Self::Formatting | Self::StructuralSymmetry => "1.4",
            Self::AssistantRegister => "1.5",
            Self::MissingSpecifics | Self::CheckableClaim => "1.6",
            Self::Leakage => "1.8",
            Self::Typography => "1.9",
        }
    }

    /// How well evidenced this family is.
    #[must_use]
    pub const fn evidence(self) -> Evidence {
        match self {
            Self::ExcessVocabulary => Evidence::Corpus,
            Self::NegativeParallelism | Self::FlatTricolon | Self::Typography => Evidence::Vendor,
            Self::StockPhrase
            | Self::SentenceUniformity
            | Self::RecurringShape
            | Self::Formatting
            | Self::StructuralSymmetry
            | Self::AssistantRegister
            | Self::MissingSpecifics
            | Self::CheckableClaim
            | Self::Leakage => Evidence::Community,
        }
    }

    /// A short human name.
    #[must_use]
    pub const fn title(self) -> &'static str {
        match self {
            Self::ExcessVocabulary => "excess vocabulary",
            Self::StockPhrase => "stock phrase",
            Self::NegativeParallelism => "negative parallelism",
            Self::FlatTricolon => "flattened rule-of-three",
            Self::SentenceUniformity => "uniform sentence length",
            Self::Formatting => "formatting habit",
            Self::RecurringShape => "recurring sentence shape",
            Self::StructuralSymmetry => "structural symmetry",
            Self::AssistantRegister => "assistant register",
            Self::MissingSpecifics => "no specifics",
            Self::CheckableClaim => "checkable claim",
            Self::Leakage => "leakage artifact",
            Self::Typography => "typographic density",
        }
    }

    /// This family's own reason for doubt — the counter-case a reader needs in order not to
    /// misread a hit. Carried in the type so no report can print a finding without one.
    #[must_use]
    pub const fn caveat(self) -> &'static str {
        match self {
            Self::ExcessVocabulary => {
                "measured across corpora, not documents; ordinary vocabulary, and some of it \
                 more common in certain varieties of English — accusing a writer on one word is \
                 unsound and carries a dialect-bias risk"
            }
            Self::StockPhrase => "ordinary English; practitioner consensus, not measured frequency",
            Self::NegativeParallelism => {
                "the device is not inherently bad — JFK's \"ask not what your country can do for \
                 you\" is the same move; what marks AI use is emptiness, both clauses saying one \
                 thing, which only a reader can judge"
            }
            Self::FlatTricolon => {
                "the classical tricolon is good writing; only the flattened kind, with \
                 interchangeable equal-length members, is a tell"
            }
            Self::SentenceUniformity => {
                "a shape heuristic over sentence lengths, NOT perplexity or burstiness — those \
                 need a reference language model; short or formulaic human text scores the same"
            }
            Self::Formatting => {
                "headers, bullets and bold are what technical writing should look like; this \
                 tell is fading as custom formatting instructions spread"
            }
            Self::RecurringShape => {
                "every one of these is a normal rhetorical move; only the habit of reaching for \
                 the same one in every section is a tell"
            }
            Self::StructuralSymmetry => {
                "well-edited documents are also even; this measures length spread, which good \
                 reference material and generated filler share"
            }
            Self::AssistantRegister => "strongest when text was pasted straight out of a chat window",
            Self::MissingSpecifics => {
                "counts names, numbers and dates, so abstract or introductory prose scores the \
                 same as generated filler — the WEAKEST signal here, and never usable alone"
            }
            Self::CheckableClaim => {
                "NOT an accusation: these are citation-shaped strings, listed so a reader can \
                 verify they exist. Confident fabrication is the most consequential tell there \
                 is, and the only way to settle it is to go and look"
            }
            Self::Leakage => {
                "the closest thing here to conclusive — but it says someone pasted without \
                 reading, not who wrote what surrounds it"
            }
            Self::Typography => {
                "density, not existence, is the signal: Word, Google Docs and phone autocorrect \
                 all produce these, and the em-dash meme did real harm to real writers"
            }
        }
    }
}

/// One marker found in the text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tell {
    /// Which class of tell this is — carries the evidence tier and the caveat.
    pub family: Family,
    /// 1-based line, or `None` for a whole-document observation (uniformity, density).
    pub line: Option<usize>,
    /// What matched, trimmed to one readable line.
    pub excerpt: String,
    /// Why it matched, when the excerpt alone doesn't say — a tier, a rate, a count.
    pub detail: String,
}

/// Everything [`scan`] found, plus the measurements a reader needs to weigh it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    /// Every marker found, in the order the detectors ran (roughly document-section order).
    pub tells: Vec<Tell>,
    /// Words in the text — the denominator behind every density figure.
    pub words: usize,
    /// Sentences, by the naive split described on [`Report::sentences`]'s producer.
    pub sentences: usize,
}

impl Report {
    /// The distinct families that fired, in section order. **This is the number that matters.**
    /// The source document's central instruction is "Never on a single marker. Look for
    /// clusters" — one family firing is noise; several independent ones is the actual signal.
    #[must_use]
    pub fn families(&self) -> Vec<Family> {
        let mut seen: Vec<Family> = self.tells.iter().map(|tell| tell.family).collect();
        seen.sort_unstable();
        seen.dedup();
        seen
    }

    /// How many tells come from the one family with corpus backing (§1.1). Separated because
    /// the document is explicit that the rigorous evidence is generic and lexical, while
    /// everything else is vendor statement or folklore.
    #[must_use]
    pub fn corpus_backed(&self) -> usize {
        self.tells.iter().filter(|tell| tell.family.evidence() == Evidence::Corpus).count()
    }
}

/// The text under examination, prepared once: an ASCII-lowercased twin for case-insensitive
/// matching (`to_ascii_lowercase` preserves byte length, so offsets index both interchangeably —
/// `to_lowercase` would not, and every marker here is ASCII), and the line table for reporting.
pub(super) struct Doc<'a> {
    pub(super) text: &'a str,
    pub(super) lower: String,
    line_starts: Vec<usize>,
    pub(super) words: usize,
}

impl<'a> Doc<'a> {
    fn new(text: &'a str) -> Self {
        let mut line_starts = vec![0];
        line_starts.extend(text.match_indices('\n').map(|(at, _)| at + 1));
        Self {
            lower: text.to_ascii_lowercase(),
            line_starts,
            words: text.split_whitespace().count(),
            text,
        }
    }

    /// The 1-based line holding `offset`.
    pub(super) fn line_of(&self, offset: usize) -> usize {
        self.line_starts.partition_point(|start| *start <= offset)
    }

    /// A readable one-line excerpt of `text[at..at + len]`, with enough around it to be
    /// recognisable and nothing that would break a report's line discipline.
    pub(super) fn excerpt(&self, at: usize, len: usize) -> String {
        const CONTEXT: usize = 24;
        const LIMIT: usize = 110;
        let start = self.text[..at.min(self.text.len())]
            .char_indices()
            .rev()
            .take(CONTEXT)
            .last()
            .map_or(at, |(index, _)| index);
        let end = {
            let from = (at + len).min(self.text.len());
            self.text[from..].char_indices().take(CONTEXT).last().map_or(from, |(i, ch)| from + i + ch.len_utf8())
        };
        let mut shown: String = self.text[start..end].split_whitespace().collect::<Vec<_>>().join(" ");
        if shown.chars().count() > LIMIT {
            shown = shown.chars().take(LIMIT).collect::<String>() + "…";
        }
        shown
    }
}

/// What the scanner can only learn from its caller.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Options {
    /// Set when the text lands somewhere that does **not** render Markdown — a plain-text file,
    /// a commit message, a form field. Only then is a visible `**bold**` or `## heading`
    /// reported as §1.4's "markdown residue"; in a `.md` file it is simply markup.
    ///
    /// Defaults to `false`, the quiet choice: a scanner that doesn't know the surface should not
    /// invent a finding about it.
    pub surface_is_plain: bool,
}

/// Scan `text` with default [`Options`] — the conservative reading, where the surface is assumed
/// to render Markdown and no residue is reported. See [`scan_with`].
#[must_use]
pub fn scan(text: &str) -> Report {
    scan_with(text, Options::default())
}

/// Scan `text` for every writing tell this module implements.
///
/// Pure: no I/O, no allocation the caller can't drop, and no judgement. Read [`Report::families`]
/// before reading [`Report::tells`] — the count of independent families is the signal, and a
/// single family firing is explicitly noise.
#[must_use]
pub fn scan_with(text: &str, options: Options) -> Report {
    let doc = Doc::new(text);
    let mut tells = Vec::new();
    shape::vocabulary(&doc, &mut tells);
    shape::phrases(&doc, &mut tells);
    shape::negative_parallelism(&doc, &mut tells);
    shape::flat_tricolon(&doc, &mut tells);
    let sentences = shape::sentence_uniformity(&doc, &mut tells);
    shape::recurring_shapes(&doc, &mut tells);
    shape::formatting(&doc, &mut tells, options);
    shape::structural_symmetry(&doc, &mut tells);
    shape::missing_specifics(&doc, &mut tells);
    shape::checkable_claims(&doc, &mut tells);
    shape::typography(&doc, &mut tells);
    pending::run(&doc, &mut tells);
    Report { tells, words: doc.words, sentences }
}

#[cfg(test)]
mod tests;

//! Detectors the source document describes that this module does **not** implement yet — one
//! stub apiece, with what is missing and what it would take.
//!
//! They are wired into [`super::scan_with`] and simply return, so filling one in means writing a
//! body rather than finding the call site. [`GAPS`] carries the same list as data, so a report
//! can *tell the reader what was not looked for* — an honest detector's silence has to be
//! distinguishable from a clean result, and §1.6's absence tells are the most diagnostic ones
//! there are, which makes their absence here worth stating out loud.
//!
//! # One thing that is not a gap, and will not become one
//!
//! **Per-model attribution** (the document's Part 2) is refused rather than deferred. Its own
//! evidence warning says there is "essentially no peer-reviewed stylometry separating Claude from
//! Gemini from Grok", that the per-model sections rest on vendor-adjacent SEO blogs, and that
//! Meta AI has no usable characterization at all. Naming a vendor from prose would invent
//! precision the source explicitly disclaims. Markers the document does list per-model are folded
//! into the generic tables instead — see [`super::lexicon::ASSISTANT_REGISTER`].
//!
//! (Vendor names in file *metadata* are a different question entirely, and answered:
//! [`crate::metadata`] matches them there, where a bare "AI" really is suspicious.)

use super::{Doc, Tell};

/// A detector the document describes and this module doesn't implement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gap {
    /// The source document's section.
    pub section: &'static str,
    /// What it would detect.
    pub name: &'static str,
    /// What implementing it would require.
    pub needs: &'static str,
}

/// Every unimplemented detector, for a report that wants to say what it did not look for.
pub const GAPS: &[Gap] = &[
    Gap {
        section: "1.7",
        name: "perplexity",
        needs: "a reference language model — the measure is how surprised one is by each token, \
                which cannot be computed without running one",
    },
    Gap {
        section: "1.7",
        name: "burstiness",
        needs: "the same language model: burstiness is the variance of per-sentence PERPLEXITY. \
                `SentenceUniformity` measures the variance of sentence LENGTH, which is its \
                visible shadow and not the same statistic",
    },
    Gap {
        section: "1.6",
        name: "confident fabrication",
        needs: "resolving the citations `CheckableClaim` surfaces — a network lookup or an \
                offline corpus. The document calls this the single most consequential tell, and \
                listing the claims is as far as a text scanner can honestly go",
    },
    Gap {
        section: "1.6",
        name: "knowledge-cutoff seams",
        needs: "world knowledge plus a sense of now: confident present tense about a superseded \
                state of the world reads exactly like confident present tense about a current one",
    },
    Gap {
        section: "1.6",
        name: "no lived detail / no idiosyncratic taste",
        needs: "judgement. An anecdote with no texture is anecdote-shaped either way, and \
                'nothing the writer disproportionately loves or hates' is not a countable thing",
    },
    Gap {
        section: "1.6",
        name: "suspicious cleanliness",
        needs: "a decision this crate is not willing to make on its own. Consistent serial \
                commas and an absence of typos are measurable — and they are also what careful \
                human editing produces, so the detector would mostly flag good writing",
    },
    Gap {
        section: "1.3",
        name: "second-person pivot",
        needs: "tracking person across a document and finding the swing into 'you' mid-piece; \
                doable, but the false-positive rate on instructional writing (which is second \
                person throughout, legitimately) needs measuring first",
    },
    Gap {
        section: "1.4",
        name: "tables for non-tabular content",
        needs: "judging whether a table's content is tabular, which is the whole question. \
                Counting tables is easy and says nothing",
    },
];

/// Run every stub. Each returns without pushing; implementing one means filling in its body.
pub(super) fn run(doc: &Doc<'_>, out: &mut Vec<Tell>) {
    perplexity(doc, out);
    burstiness(doc, out);
    confident_fabrication(doc, out);
    knowledge_cutoff_seams(doc, out);
    lived_detail(doc, out);
    suspicious_cleanliness(doc, out);
    second_person_pivot(doc, out);
    non_tabular_tables(doc, out);
}

/// §1.7 — "how *surprised* a reference model is by the next token. Lower = more predictable =
/// more machine-like. Computed from summed log probabilities per token."
///
/// TODO: needs a reference language model. Note the document's own warning about the original
/// implementations: they scored text with GPT-2 regardless of which model actually wrote it,
/// "a substantial methodological weakness", with documented false positives in academic-integrity
/// settings and poor performance on short, edited or mixed text. Any implementation here should
/// carry that caveat in its `Family::caveat`, not just in a comment.
fn perplexity(_doc: &Doc<'_>, _out: &mut Vec<Tell>) {}

/// §1.7 — "the *variance* of perplexity across the document, in practice the standard deviation
/// of per-sentence perplexity … Low burstiness ⇒ machine-like."
///
/// TODO: needs [`perplexity`] first, then the per-sentence spread of it. The sentence splitting
/// already exists in [`super::shape`]; only the per-sentence score is missing.
fn burstiness(_doc: &Doc<'_>, _out: &mut Vec<Tell>) {}

/// §1.6 — "Plausible-looking citations, DOIs, author names, version numbers, API flags and quotes
/// that don't exist. The formatting is right and the content is invented."
///
/// TODO: `super::shape::checkable_claims` already extracts the DOIs and arXiv ids; what is
/// missing is *resolving* them. That means network access, which this crate does not have and
/// arguably should not take — a likelier shape is a `verify` feature behind a caller-supplied
/// resolver closure, keeping the I/O on the caller's side as the walk already is.
fn confident_fabrication(_doc: &Doc<'_>, _out: &mut Vec<Tell>) {}

/// §1.6 — "Confident present tense about a superseded state of the world; silence on recent
/// events a human would mention."
///
/// TODO: needs world knowledge and a reference date. Even with both, "silence on recent events"
/// is unfalsifiable from the text alone.
fn knowledge_cutoff_seams(_doc: &Doc<'_>, _out: &mut Vec<Tell>) {}

/// §1.6 — "Anecdotes that are structurally anecdote-shaped but carry no texture — no names, no
/// sensory particulars, no irrelevant asides", and "No idiosyncratic taste."
///
/// TODO: `super::shape::missing_specifics` covers the countable corner of this (names, numbers,
/// dates). Texture and taste are not countable; a serious attempt would need a model, at which
/// point it is the §1.7 problem wearing a different hat.
fn lived_detail(_doc: &Doc<'_>, _out: &mut Vec<Tell>) {}

/// §1.6 — "No typos, uniform formatting, consistent serial commas throughout — human long-form
/// drifts."
///
/// TODO: serial-comma consistency is measurable today (count `A, B, and C` against `A, B and C`
/// and look for a document that never mixes them), and so is a spell-check pass. Both were left
/// out on purpose: they fire hardest on carefully edited human prose, which is the exact
/// false-positive class the document says did real harm. Implementing this should come with a
/// measured false-positive rate, not just a threshold.
fn suspicious_cleanliness(_doc: &Doc<'_>, _out: &mut Vec<Tell>) {}

/// §1.3 — "an explanatory passage that swings into 'you'".
///
/// TODO: track first/second/third person per paragraph and report a document that is
/// third-person throughout with a second-person island. The trap is instructional writing, which
/// is legitimately second person from start to finish; the detector must key on the *swing*, not
/// on the presence of "you".
fn second_person_pivot(_doc: &Doc<'_>, _out: &mut Vec<Tell>) {}

/// §1.4 — "tables for non-tabular content".
///
/// TODO: finding a Markdown table is trivial; deciding whether its content wanted to be a table
/// is the entire question, and needs the judgement this crate keeps handing back to the reader.
fn non_tabular_tables(_doc: &Doc<'_>, _out: &mut Vec<Tell>) {}

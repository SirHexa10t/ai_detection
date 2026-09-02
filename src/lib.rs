//! Detecting AI-generated content by what it verifiably leaves behind, in files you already
//! have. Two independent engines, one per kind of evidence:
//!
//! - [`hidden`] — *hidden characters* in text: the invisible codepoints that carry edit-based
//!   watermarks, and the same ones pasted in by accident that then break diffs, greps and search.
//! - [`metadata`] — *provenance labels* in file metadata: frontmatter and `<meta>` generator
//!   fields, XMP CreatorTool, C2PA containers, vendor names.
//! - [`tells`] — *writing habits*: excess vocabulary, stock phrases, sentence shapes, formatting
//!   tics, assistant register, leakage. The weakest of the three by design — read its module doc
//!   before using its output for anything.
//!
//! # What this crate is, and is not
//!
//! **Detection only.** Nothing here writes, and nothing here decodes media: a JPEG's pixels and
//! a PNG's IDAT stay unread. Reporting where something is is a different act from altering a
//! file, and only the first is safe to run over a whole tree.
//!
//! **No policy.** Both engines answer questions and return data — which codepoints are on this
//! line, which metadata fields this file carries. Whether that is worth reporting, how to walk a
//! tree, what to skip, and how any of it should look on a terminal belong to the caller. The one
//! place presentation could have leaked in, [`hidden::render_line`], takes the caller's own
//! marker function instead.
//!
//! **Honest about scope.** These are the two *verifiable* classes. Statistical (token-sampling)
//! watermarks live in word choice with nothing to scan for, and are not claimed here. A clean
//! result means "no carrier characters and no provenance labels", never "not AI".

pub mod hidden;
pub mod metadata;
pub mod tells;

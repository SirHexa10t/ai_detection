//! Finding *hidden characters* in text — the invisible codepoints that carry edit-based text
//! watermarks, and the same ones that get pasted in by accident and then break diffs, greps and
//! search. The other half of this crate's engine, beside [`crate::metadata`].
//!
//! Scope, stated so it can't be overclaimed: the *edit-based text* class only, the one that is
//! verifiable by looking. Statistical (token-sampling) watermarks live in word choice with
//! nothing to scan for, and provenance metadata lives in container headers rather than text —
//! the first is invisible to any character scan, the second is [`crate::metadata`]'s job.

/// One hidden character: where it sits on the line, what it is, and which class it belongs to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hit {
    /// 1-based character column, counted to read alongside an editor's own ruler.
    pub column: usize,
    /// The character itself — reported by codepoint, since it renders as nothing.
    pub ch: char,
    /// Its class, from [`classify`].
    pub kind: &'static str,
}

/// Every hidden character found on one line, with the line for context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineHits {
    /// 1-based line number.
    pub number: usize,
    /// The line as it stands, hidden characters and all.
    pub text: String,
    /// What was found on it, in column order. Never empty — a clean line yields no `LineHits`.
    pub hits: Vec<Hit>,
}

/// The class name for exotic spaces, the one class a caller may silence wholesale.
pub const SPACE: &str = "space";

/// Scan `text` line by line. `spaces` includes the exotic-space class, `emoji` includes the
/// codepoints that build emoji — both are the caller's policy, since both are legitimate in
/// ordinary text often enough that reporting them by default buries real carriers.
#[must_use]
pub fn scan(text: &str, spaces: bool, emoji: bool) -> Vec<LineHits> {
    let mut found = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let mut previous: Option<char> = None;
        let mut hits: Vec<Hit> = Vec::new();
        for (column, ch) in line.chars().enumerate() {
            let Some(kind) = classify(ch) else {
                previous = Some(ch);
                continue;
            };
            let reportable = match kind {
                SPACE => spaces,
                _ if !emoji && is_emoji_use(ch, previous) => false,
                _ => true,
            };
            if reportable {
                hits.push(Hit { column: column + 1, ch, kind });
            }
            previous = Some(ch);
        }
        if !hits.is_empty() {
            found.push(LineHits { number: index + 1, text: line.to_string(), hits });
        }
    }
    found
}

/// Whether this character is here to build an emoji rather than to hide a payload.
///
/// Variation selectors and tag characters are emoji machinery almost everywhere they
/// appear — `⚠️` is U+26A0 plus VS16, a flag is a base plus tag chars. A joiner counts as
/// emoji use only when it *follows a non-ASCII character*, which covers 👨‍👩‍👧 and Persian
/// `می‌روم` alike, while `a<ZWJ>b` between plain letters stays a carrier.
///
/// A heuristic, deliberately: the alternative is shipping Unicode's emoji tables. It errs
/// toward silence, which is why it is what an `--emoji` flag turns OFF rather than what it
/// turns on.
#[must_use]
pub fn is_emoji_use(ch: char, previous: Option<char>) -> bool {
    match ch as u32 {
        0xFE00..=0xFE0F | 0xE0100..=0xE01EF | 0xE0000..=0xE007F => true,
        0x200C | 0x200D => previous.is_some_and(|prev| !prev.is_ascii()),
        _ => false,
    }
}

/// What class of hidden character this is, or `None` if it is ordinary text.
///
/// An explicit table rather than a Unicode general-category lookup, which the standard
/// library does not expose and which would cost a dependency to get. The ranges below are
/// the format/invisible blocks actually used as carriers — everything here renders as
/// nothing (or as a plain space) while still occupying a codepoint.
#[must_use]
pub fn classify(ch: char) -> Option<&'static str> {
    match ch as u32 {
        0x200B | 0x200C | 0x200D | 0x2060 | 0xFEFF => Some("zero-width"),
        0x200E | 0x200F | 0x202A..=0x202E | 0x2066..=0x2069 | 0x061C => Some("bidi"),
        0xE0000..=0xE007F => Some("tag-char"),
        0xFE00..=0xFE0F | 0xE0100..=0xE01EF => Some("variation-selector"),
        0x00AD | 0x034F | 0x180B..=0x180E | 0x2061..=0x2064 | 0x206A..=0x206F => {
            Some("format-control")
        }
        0xFFF9..=0xFFFB => Some("annotation"),
        // Zl/Zp, not Zs — and never silenceable, not even by a spaces flag. `str::lines()` splits
        // on \n only, so one of these sits *inside* what this scan calls a line while many
        // editors and JS engines break there: the file displays with lines the report never
        // mentions. Nothing in ordinary prose needs them, so there is no legitimate use to
        // weigh against saying so.
        0x2028 | 0x2029 => Some("line-separator"),
        // The whole Zs category except U+0020 itself, which is the character the rest would
        // be normalised *to* — flagging it would report every space in every file.
        0x00A0 | 0x1680 | 0x2000..=0x200A | 0x202F | 0x205F | 0x3000 => Some(SPACE),
        _ => None,
    }
}

/// The line with every *reported* hidden character replaced by `mark`'s rendering of its
/// codepoint, so a reader can see where it sits relative to the visible text. Printing the raw
/// line would show exactly nothing, which is the whole problem being reported.
///
/// Driven by the hit list rather than by re-testing each character, so the markers and the list
/// beside them can never disagree: with spaces silenced, an exotic space is neither listed nor
/// marked, instead of being silently marked as something the report never mentioned.
///
/// `mark` receives the already-formatted `<U+XXXX>` and returns it however the caller wants it
/// to look — styling belongs to whoever is printing, not to the engine.
pub fn render_line(line: &LineHits, mark: impl Fn(&str) -> String) -> String {
    line.text
        .chars()
        .enumerate()
        .map(|(index, ch)| {
            if line.hits.iter().any(|hit| hit.column == index + 1) {
                mark(&format!("<U+{:04X}>", ch as u32))
            } else {
                ch.to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Rendering with no styling — what the engine's own tests assert against.
    fn plain(line: &LineHits) -> String {
        render_line(line, str::to_string)
    }

    #[test]
    fn the_invisible_carriers_are_recognised_and_ordinary_text_is_not() {
        for (ch, expected) in [
            ('\u{200B}', "zero-width"),
            ('\u{200D}', "zero-width"),
            ('\u{202E}', "bidi"),
            ('\u{E0041}', "tag-char"),
            ('\u{FE0F}', "variation-selector"),
            ('\u{00AD}', "format-control"),
        ] {
            assert_eq!(classify(ch), Some(expected), "U+{:04X}", ch as u32);
        }
        for ordinary in ['a', 'Z', ' ', '\t', 'é', '中', '🙂'] {
            assert_eq!(classify(ordinary), None, "{ordinary:?} is ordinary text");
        }
    }

    /// Exotic spaces report by default (they are a real carrier class) and a caller's
    /// spaces-off policy silences them for prose that uses them as typography.
    #[test]
    fn exotic_spaces_can_be_silenced_but_default_on() {
        let text = "a\u{00A0}b";
        assert!(scan(text, false, true).is_empty(), "silenced when the flag says so");
        let hits = scan(text, true, true);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].hits[0].kind, SPACE);
    }

    /// The space class means the whole Zs category, not a handful of it. Enumerated rather than
    /// sampled, because the gap that prompted this test (U+1680) was a member nobody
    /// thought of — a range plus a few literals reads complete without being complete.
    #[test]
    fn every_unicode_space_separator_is_covered_except_the_ordinary_one() {
        let zs = [
            0x00A0_u32, 0x1680, 0x2000, 0x2001, 0x2002, 0x2003, 0x2004, 0x2005, 0x2006, 0x2007,
            0x2008, 0x2009, 0x200A, 0x202F, 0x205F, 0x3000,
        ];
        for cp in zs {
            let ch = char::from_u32(cp).expect("a real codepoint");
            assert_eq!(classify(ch), Some(SPACE), "U+{cp:04X} is a space separator");
        }
        assert_eq!(classify(' '), None, "U+0020 is what the others normalise TO, not a carrier");
    }

    /// Line and paragraph separators are reported even with spaces silenced: `str::lines()` does
    /// not split on them, so they hide *inside* a reported line while an editor shows a
    /// break there — and unlike an em space, no ordinary prose needs one.
    #[test]
    fn line_separators_are_always_reported_and_do_not_split_our_lines() {
        for cp in ['\u{2028}', '\u{2029}'] {
            assert_eq!(classify(cp), Some("line-separator"), "{:04X}", cp as u32);
        }
        let text = "before\u{2028}after";
        assert_eq!(text.lines().count(), 1, "our line splitting does not see the break");
        let hits = scan(text, false, true);
        assert_eq!(hits.len(), 1, "yet it is reported, with spaces silenced");
        assert_eq!(hits[0].hits[0].kind, "line-separator");
    }

    #[test]
    fn a_hit_carries_its_line_number_and_column() {
        let text = "clean line\nhas a \u{200B}carrier\nalso clean";
        let hits = scan(text, false, true);
        assert_eq!(hits.len(), 1, "only the middle line");
        assert_eq!(hits[0].number, 2, "line numbers count from one");
        assert_eq!(hits[0].hits[0].column, 7, "and so do columns");
        assert_eq!(hits[0].hits[0].ch, '\u{200B}');
    }

    /// The reason the report renders lines at all: printing the raw line would show
    /// nothing where the carrier is, which is exactly what makes it hard to find.
    #[test]
    fn rendering_makes_the_invisible_visible_in_place() {
        let hits = scan("hi\u{200B}there", false, true);
        let shown = plain(&hits[0]);
        assert!(shown.contains("<U+200B>"), "{shown}");
        assert!(shown.contains("hi") && shown.contains("there"), "{shown}");
    }

    /// The markers and the listed hits are one decision, not two: a character the report
    /// chose not to list must not appear marked in the line beneath it.
    #[test]
    fn rendering_marks_exactly_what_was_reported() {
        let line = "a\u{00A0}b\u{200B}c";
        let quiet = scan(line, false, true);
        let shown = plain(&quiet[0]);
        assert!(shown.contains("<U+200B>"), "the carrier is marked: {shown}");
        assert!(!shown.contains("<U+00A0>"), "the unlisted space is not: {shown}");

        let loud = scan(line, true, true);
        assert!(plain(&loud[0]).contains("<U+00A0>"), "with spaces on it is both listed and marked");
    }

    /// Styling is the caller's: the engine hands out the `<U+XXXX>` and takes back whatever
    /// the caller made of it, so colour never leaks into the detection half.
    #[test]
    fn the_caller_decides_how_a_marker_looks() {
        let hits = scan("x\u{200B}y", false, true);
        let shouted = render_line(&hits[0], |marker| format!("[[{marker}]]"));
        assert_eq!(shouted, "x[[<U+200B>]]y");
    }

    /// Emoji are spelled with the same codepoints carriers use, and there are far more
    /// emoji in a normal repo than carriers — so reporting them by default buries the
    /// signal. On the origin project they were the majority of all hits.
    #[test]
    fn emoji_machinery_is_quiet_unless_asked_for() {
        for text in [
            "warn \u{26A0}\u{FE0F} here",
            "fam \u{1F468}\u{200D}\u{1F469}",
            "\u{1F3F4}\u{E0067}\u{E0062}",
        ] {
            assert!(scan(text, false, false).is_empty(), "quiet by default: {text:?}");
            assert!(!scan(text, false, true).is_empty(), "the emoji flag reports it: {text:?}");
        }
    }

    /// The other half: the same joiner between plain ASCII is a carrier, not emoji glue,
    /// and stays reported without the emoji flag. Persian `می‌روم` is the case the rule protects.
    #[test]
    fn a_joiner_between_ascii_is_still_a_carrier() {
        assert_eq!(scan("a\u{200D}b", false, false).len(), 1, "ASCII neighbours: a carrier");
        assert!(scan("\u{0645}\u{200C}\u{0631}", false, false).is_empty(), "Persian ZWNJ is not");
        // And the carriers that are never emoji stay on by default.
        assert_eq!(scan("a\u{200B}b", false, false).len(), 1, "ZWSP is always reported");
    }
}

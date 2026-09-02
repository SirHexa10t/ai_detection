//! Tests for the writing-tell detectors.
//!
//! The first one is the important one: it holds every marker in [`lexicon`] to the bundled
//! source document, so the tables can never drift into invention. The rest check that each
//! detector fires on the shape the document describes and — at least as importantly — stays
//! quiet on the human writing that resembles it.

use super::*;

/// Normalised for comparison: lowercase, whitespace collapsed. The document writes markers in
/// prose (capitalised, sometimes wrapped across lines); the tables write them for matching.
fn flatten(text: &str) -> String {
    text.to_lowercase().split_whitespace().collect::<Vec<_>>().join(" ")
}

/// **No unsourced markers.** Every phrase and word in the tables must appear in the bundled
/// document, either verbatim or through an entry in [`lexicon::DERIVED`] that names the document
/// text it expands. This is what makes the module doc's "nothing is invented" claim checkable
/// rather than a promise — and what stops a future edit from quietly adding a marker somebody
/// merely felt was AI-ish.
#[test]
fn every_listed_marker_is_sourced() {
    let document = flatten(SOURCE_DOCUMENT);
    let derived: std::collections::HashMap<&str, &str> = lexicon::DERIVED.iter().copied().collect();

    let mut listed: Vec<&str> = Vec::new();
    listed.extend(lexicon::VOCAB_SIGNATURE);
    listed.extend(lexicon::VOCAB_STRONG);
    listed.extend(lexicon::VOCAB_WEAK);
    listed.extend(lexicon::STOCK_PHRASES);
    listed.extend(lexicon::ASSISTANT_REGISTER);
    listed.extend(lexicon::LEAKAGE);
    listed.extend(lexicon::RECURRING_SHAPES);
    assert!(listed.len() > 80, "the tables should be substantial: {}", listed.len());

    for marker in listed {
        if document.contains(&flatten(marker)) {
            continue;
        }
        let source = derived.get(marker).unwrap_or_else(|| {
            panic!("`{marker}` is in no list in the source document, and claims no derivation")
        });
        assert!(
            document.contains(&flatten(source)),
            "`{marker}` derives from `{source}`, which is itself not in the document"
        );
    }

    // The character table too: one entry per §1.9 row, each carrying that row's own label.
    for (_, name, _) in lexicon::TYPOGRAPHY {
        assert!(
            document.contains(&flatten(name)),
            "the typography table has a row named {name:?}, which the document does not"
        );
    }
    for emoji in lexicon::BULLET_EMOJI {
        assert!(SOURCE_DOCUMENT.contains(*emoji), "{emoji} is not among the document's emoji");
    }
}

/// An exhaustive match, so adding a variant fails to compile here until it is also added to
/// [`Family::ALL`] — which is what keeps the "N of M families" a caller prints honest.
const fn is_listed_in_all(family: Family) -> bool {
    match family {
        Family::ExcessVocabulary
        | Family::StockPhrase
        | Family::NegativeParallelism
        | Family::FlatTricolon
        | Family::SentenceUniformity
        | Family::RecurringShape
        | Family::Formatting
        | Family::StructuralSymmetry
        | Family::AssistantRegister
        | Family::MissingSpecifics
        | Family::CheckableClaim
        | Family::Leakage
        | Family::Typography => true,
    }
}

/// Every family must carry a non-empty caveat and a real section number: the API's promise is
/// that a finding cannot be printed without the reason to doubt it.
#[test]
fn every_family_carries_its_evidence_and_its_doubt() {
    let families = Family::ALL;
    assert_eq!(families.len(), 13, "Family::ALL must list every variant");
    for family in families.iter().copied() {
        assert!(is_listed_in_all(family));
    }
    for family in families.iter().copied() {
        assert!(family.caveat().len() > 30, "{family:?} needs a real caveat");
        assert!(family.section().starts_with('1'), "{family:?} cites a Part 1 section");
        assert!(!family.title().is_empty());
        assert!(!family.evidence().mark().is_empty());
    }
    // Exactly one family has corpus backing — the document is explicit that the rigorous
    // evidence is the lexical corpus work and nothing else.
    let corpus: Vec<_> =
        families.iter().filter(|f| f.evidence() == Evidence::Corpus).copied().collect();
    assert_eq!(corpus, vec![Family::ExcessVocabulary]);
}

/// The vocabulary set matches whole words and their listed forms, reports each distinct word
/// once with a count, and names the tier — so a `delve` hit can never be read as equal in weight
/// to a `landscape` hit.
#[test]
fn excess_vocabulary_matches_word_forms_and_keeps_its_tiers() {
    let report = scan("We delve in. She delves. They are delving. Delved once more.");
    let vocab: Vec<&Tell> =
        report.tells.iter().filter(|t| t.family == Family::ExcessVocabulary).collect();
    assert_eq!(vocab.len(), 1, "one tell for the word, not one per occurrence: {vocab:?}");
    assert!(vocab[0].detail.contains("signature tier"), "{}", vocab[0].detail);
    assert!(vocab[0].detail.contains("4 times"), "{}", vocab[0].detail);

    // Word boundaries hold: no match inside a longer word.
    assert!(
        scan("The undelved realmscape of foldering.")
            .tells
            .iter()
            .all(|t| t.family != Family::ExcessVocabulary),
        "substrings of longer words are not hits"
    );
}

/// Phrase families fire on their own lists and nothing else's.
#[test]
fn phrase_families_are_kept_apart() {
    let report = scan(
        "Great question! In today's fast-paced world, it's worth noting that \
         [Your Name] should sign here.",
    );
    let families = report.families();
    assert!(families.contains(&Family::AssistantRegister), "{families:?}");
    assert!(families.contains(&Family::StockPhrase), "{families:?}");
    assert!(families.contains(&Family::Leakage), "{families:?}");
}

/// The four negative-parallelism shapes the document names, each reported per occurrence — the
/// caveat is that only *empty* uses are a tell, which needs the reader to see them.
#[test]
fn negative_parallelism_catches_the_documented_shapes() {
    for text in [
        "It's not a feature, it's a philosophy.",
        "This isn't about speed. It's about care.",
        "It's not just about performance — it's about trust.",
        "Not only does it compile, but also it runs.",
    ] {
        let report = scan(text);
        assert!(
            report.tells.iter().any(|t| t.family == Family::NegativeParallelism),
            "missed the construction in {text:?}"
        );
    }
    // A plain negation is not the construction.
    let plain = scan("It is not ready yet, and we said so in the notes.");
    assert!(plain.tells.iter().all(|t| t.family != Family::NegativeParallelism));
}

/// The flattened tricolon fires; the varied one — which is good writing — does not.
#[test]
fn only_the_flattened_tricolon_is_a_tell() {
    let flat = scan("It is efficient, effective, and reliable.");
    assert!(
        flat.tells.iter().any(|t| t.family == Family::FlatTricolon),
        "equal-length members are the tell: {:?}",
        flat.tells
    );

    let varied = scan("It is fast, remarkably well documented, and free.");
    assert!(
        varied.tells.iter().all(|t| t.family != Family::FlatTricolon),
        "a classical tricolon varies its members and must not be flagged"
    );
}

/// Uniform sentence length fires only on enough text, and varied writing is left alone. The
/// detail line must not claim to be perplexity or burstiness.
#[test]
fn sentence_uniformity_needs_enough_text_and_real_flatness() {
    let flat: String = std::iter::repeat_n("The system runs the job well. ", 12).collect();
    let report = scan(&flat);
    let uniform: Vec<&Tell> =
        report.tells.iter().filter(|t| t.family == Family::SentenceUniformity).collect();
    assert_eq!(uniform.len(), 1, "flat rhythm is reported: {:?}", report.tells);
    assert!(uniform[0].line.is_none(), "a whole-document observation has no line");
    for banned in ["perplexity", "burstiness"] {
        assert!(!uniform[0].detail.contains(banned), "must not claim to measure {banned}");
    }
    assert!(Family::SentenceUniformity.caveat().contains("NOT perplexity"));

    // Two sentences are not a rhythm.
    assert!(scan("Short. Also short.").tells.iter().all(|t| t.family != Family::SentenceUniformity));

    // Genuine variation is not flagged, however many sentences there are.
    let varied = "Yes. \
        The build failed again this morning, which is the third time this week and the reason \
        nobody trusts the pipeline any more. I rolled it back. \
        Then, after a long argument about whether the cache was at fault or the lockfile had \
        drifted underneath us without anyone noticing, we gave up and went to lunch. Fine. \
        It works now. Nobody knows why, and that is the part that worries me most of all. Done.";
    assert!(
        scan(varied).tells.iter().all(|t| t.family != Family::SentenceUniformity),
        "human variation must not trip it"
    );
}

/// Formatting habits report as habits — three or more — never as single instances, because one
/// bold bullet is how good technical writing looks.
#[test]
fn formatting_reports_habits_not_instances() {
    let once = scan("- **Security:** it matters.\nplain line\n");
    assert!(
        once.tells.iter().all(|t| t.family != Family::Formatting),
        "a single bold bullet is not a habit"
    );

    let habit = scan(
        "- **Security:** it matters.\n\
         - **Performance:** it also matters.\n\
         - **Cost:** this too.\n",
    );
    let found: Vec<&Tell> = habit.tells.iter().filter(|t| t.family == Family::Formatting).collect();
    assert!(!found.is_empty(), "three is a habit");
    assert!(found[0].detail.contains("bold-lead-in bullets ×3"), "{}", found[0].detail);
}

/// Typography is reported by density and never by presence — the family the document says did
/// real harm when used carelessly.
#[test]
fn typography_needs_density_not_presence() {
    let words = |count: usize| std::iter::repeat_n("word ", count).collect::<String>();

    // Two em dashes in a long piece is ordinary human punctuation.
    let sparse = format!("{} — {} — {}", words(100), words(100), words(100));
    assert!(
        scan(&sparse).tells.iter().all(|t| t.family != Family::Typography),
        "a couple of em dashes must never be a finding"
    );

    // Twenty in the same length is the document's own contrast.
    let dense = format!("{}{}", words(200), "a — b ".repeat(20));
    let report = scan(&dense);
    let dashes: Vec<&Tell> = report.tells.iter().filter(|t| t.family == Family::Typography).collect();
    assert_eq!(dashes.len(), 1, "{:?}", report.tells);
    assert!(dashes[0].detail.contains("em dash"), "{}", dashes[0].detail);
    assert!(dashes[0].detail.contains("per 1000 words"), "{}", dashes[0].detail);

    // Short text gets no density verdict at all: the arithmetic would be meaningless.
    assert!(scan("a — b — c — d").tells.iter().all(|t| t.family != Family::Typography));
}

/// Clustering is the unit of meaning, and the API says so: [`Report::families`] is what a caller
/// weighs. Ordinary human prose fires nothing; a document laying on every habit fires several.
#[test]
fn clusters_are_what_the_report_is_for() {
    let human = "I rewrote the parser on Tuesday because the old one choked on \
                 nested quotes, and Dave had been complaining about it since March.";
    let clean = scan(human);
    assert!(clean.families().is_empty(), "ordinary writing fires nothing: {:?}", clean.tells);
    assert_eq!(clean.corpus_backed(), 0);

    let slop = "Great question! In the realm of modern software, it's worth noting that \
                robust tooling plays a crucial role. It's not just about speed, it's about \
                quality. The result is efficient, effective, and reliable.\n\
                - **Security:** paramount.\n\
                - **Velocity:** pivotal.\n\
                - **Clarity:** crucial.\n\
                In conclusion, the possibilities are endless.";
    let report = scan(slop);
    let families = report.families();
    assert!(families.len() >= 5, "an obvious pile-on clusters: {families:?}");
    assert!(report.corpus_backed() > 0, "and some of it is the corpus-backed set");

    // Sorted and deduplicated, so a caller can compare cluster shapes.
    let mut sorted = families.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(families, sorted);
}

/// §1.3's other shapes each need a *habit* — one range sweep or one rhetorical question is
/// ordinary writing, and reporting it would be the false-positive class the document warns about.
#[test]
fn recurring_shapes_report_habits_not_single_moves() {
    let once = scan("From startups to enterprises to governments, adoption grew.");
    assert!(once.tells.iter().all(|t| t.family != Family::RecurringShape), "one sweep is writing");

    let twice = scan(
        "From startups to enterprises to governments, adoption grew. \
         From laptops to phones to watches, the same code runs.",
    );
    assert!(
        twice.tells.iter().any(|t| t.detail.contains("range-sweep")),
        "{:?}",
        twice.tells
    );

    let openers = scan("But what does this actually mean?\n\nSo why does any of it matter?\n");
    assert!(openers.tells.iter().any(|t| t.detail.contains("rhetorical questions")));

    // The listicle in a trenchcoat: ordinals marching through prose after the lists were removed.
    let disguised =
        scan("The first reason is speed. The second reason is cost. The third reason is trust.");
    assert!(
        disguised.tells.iter().any(|t| t.detail.contains("trenchcoat")),
        "{:?}",
        disguised.tells
    );
}

/// §1.4's symmetry: "Human documents are lumpy; they dwell on what the writer actually cares
/// about." Even paragraphs and a header every couple of sentences are the two measurable halves.
#[test]
fn structural_symmetry_measures_evenness_and_crowding() {
    let even = "alpha beta gamma delta epsilon zeta eta theta\n\n\
                one two three four five six seven eight\n\n\
                red orange yellow green blue indigo violet white\n\n\
                north south east west up down left right";
    let report = scan(even);
    assert!(
        report.tells.iter().any(|t| t.detail.contains("human documents are lumpy")),
        "{:?}",
        report.tells
    );

    // A lumpy document is not flagged, however many paragraphs it has.
    let lumpy = "one\n\n\
                 This paragraph carries most of the argument and runs considerably longer than \
                 its neighbours because that is where the writer actually had something to say \
                 about the subject at hand and kept going.\n\n\
                 two words\n\n\
                 Another short one.";
    assert!(
        scan(lumpy).tells.iter().all(|t| t.family != Family::StructuralSymmetry),
        "uneven writing must not be flagged"
    );

    let crowded = "# One\nA short section here.\n\n# Two\nAnother short one.\n\n\
                   # Three\nStill short.\n\n# Four\nAnd short again.\n";
    assert!(scan(crowded).tells.iter().any(|t| t.detail.contains("headings for")));
}

/// §1.6 — the absence tells. Barren prose is reported; prose that names things is not; and
/// citation shapes are surfaced for verification, never called fake.
#[test]
fn the_absence_tells_measure_what_is_missing_without_accusing() {
    let barren: String =
        std::iter::repeat_n("the system should handle the load without any of the usual trouble. ", 22)
            .collect();
    let report = scan(&barren);
    assert!(
        report.tells.iter().any(|t| t.family == Family::MissingSpecifics),
        "{:?}",
        report.tells
    );

    // The same length, but naming things: no finding.
    let specific: String = std::iter::repeat_n(
        "In 2019 Dave moved 3 services to Hetzner, cutting the bill by 40 percent that quarter. ",
        20,
    )
    .collect();
    assert!(
        scan(&specific).tells.iter().all(|t| t.family != Family::MissingSpecifics),
        "prose full of names and numbers is not barren"
    );

    // Citation shapes are listed for checking, and the wording must not assert fabrication.
    let cited = scan("See Kobak et al., 10.1126/sciadv.adt3813, and the preprint arXiv:2406.07016.");
    let claims: Vec<&Tell> =
        cited.tells.iter().filter(|t| t.family == Family::CheckableClaim).collect();
    assert_eq!(claims.len(), 2, "a DOI and an arXiv id: {:?}", cited.tells);
    for claim in &claims {
        assert!(claim.detail.contains("verify"), "{}", claim.detail);
        for accusation in ["fake", "fabricat", "invented"] {
            assert!(!claim.detail.contains(accusation), "must not accuse: {}", claim.detail);
        }
    }
    assert!(Family::CheckableClaim.caveat().contains("NOT an accusation"));
}

/// Markdown residue is the one tell that depends on knowledge only the caller has, so it is the
/// one tell behind [`Options`] — and silent by default.
#[test]
fn markdown_residue_needs_the_caller_to_say_the_surface_is_plain() {
    let text = "**Security:** it matters.\n## A Heading\n**Speed:** also.\n## Another\n";
    assert!(
        scan(text).tells.iter().all(|t| !t.detail.contains("markdown residue")),
        "in a markdown file this is just markup"
    );
    let plain = scan_with(text, Options { surface_is_plain: true });
    assert!(
        plain.tells.iter().any(|t| t.detail.contains("markdown residue")),
        "{:?}",
        plain.tells
    );
}

/// The scan reports positions a caller can act on, and counts a caller can weigh.
#[test]
fn tells_carry_position_and_the_report_carries_its_denominators() {
    let report = scan("first line is clean\nthe second delves into things\nthird is clean");
    let tell = report
        .tells
        .iter()
        .find(|t| t.family == Family::ExcessVocabulary)
        .expect("the marker is found");
    assert_eq!(tell.line, Some(2), "1-based line numbers");
    assert!(tell.excerpt.contains("delves"), "{}", tell.excerpt);
    assert_eq!(report.words, 12);
    assert!(report.sentences >= 1);
}

/// Empty and pathological inputs produce nothing, and never panic — the scanner runs over
/// whatever a caller hands it, including a file that is not prose at all.
#[test]
fn odd_input_is_survivable() {
    for text in ["", "\n\n\n", "🙂🙂🙂", "a", "\u{200B}", "— — —", "###", "- **"] {
        let report = scan(text);
        assert_eq!(report.words, text.split_whitespace().count(), "{text:?}");
    }
}

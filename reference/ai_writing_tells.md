# AI writing tells — characters, keywords and phrases by model

Compiled 2026-08-25. Sources at the end; evidence strength is marked throughout.

---

## Read this first: what this file can and cannot tell you

**No item here is proof of AI authorship.** Every marker below has a legitimate
human use, and the strongest ones are statistical — they show up in *corpora*,
not in single documents. The headline study on this topic says so explicitly:
its analysis works at the corpus level and **cannot identify which individual
documents were LLM-processed**.

Three structural problems with per-model attribution, which you should know
before trusting the tables:

1. **The rigorous evidence is generic, not per-model.** The peer-reviewed corpus
   work measures "LLM-assisted text" as a single category, dominated by
   ChatGPT-era output. There is essentially **no peer-reviewed stylometry
   separating Claude from Gemini from Grok**. The per-model sections below are
   therefore marked *impressionistic* and rest on vendor-adjacent blogs.
2. **Tells expire.** Vendors tune them away once they become memes. "Delve"
   peaked and receded; the em dash prompted a product change (below). A list
   like this decays within months.
3. **Prompting overrides style.** Any of these models will drop nearly all of
   these markers if asked to. Tells reflect *default* register, which is what
   people who don't customize get.

**Marked throughout:** ✅ = corpus/peer-reviewed evidence · ⚠️ = vendor statement
or journalism · ❓ = impressionistic, community-level only.

---

## Part 1 — Universal markers (shared across all major models)

This is where the real signal is. Cross-model markers are far better evidenced
than any per-model fingerprint.

### 1.1 Lexical — the "excess vocabulary" set ✅

The strongest single finding in this area. An analysis of **>15 million PubMed
biomedical abstracts (2010–2024)** found hundreds of words abruptly rising in
frequency after LLMs became available — and crucially, the risers were **not
content nouns but style-affecting verbs and adjectives**. The authors estimate
**at least 13.5% of 2024 abstracts** were LLM-processed, reaching ~40% in some
subcorpora. They describe the shift as exceeding the lexical impact of major
world events.

| Tier | Words |
|---|---|
| **Signature** (largest measured spikes) | **delve / delves / delving** (~28× increase), **intricate**, **meticulous**, **underscore(s)**, **realm**, **pivotal** |
| **Strong** | robust, facilitate, leverage, showcase, foster, garner, notably, comprehensive, nuanced, crucial, significant |
| **Common but weaker** | tapestry, testament (as in "a testament to"), landscape (figurative), navigate (figurative), harness, embark, unlock, elevate, seamless, vibrant, multifaceted, holistic, paramount, myriad |

A separate replication across **26,010 linguistics abstracts** in the top 100
journals found a **28% rise in 12 of 16 target stylistic words in 2024 alone**,
with delve, enhancing and pivotal the standouts. So the effect reproduces
outside biomedicine.

**On causation:** an FSU study (COLING 2025) tested whether RLHF explains it and
got a complicating result — human raters in their experiment reacted *more
negatively* to buzzword-laden abstracts. So the simple "humans rewarded these
words" story doesn't hold cleanly; the mechanism is unresolved.

### 1.2 Stock phrases and set openers ❓

Multi-word formulae, weaker evidence than the single-word corpus data above
(these come from practitioner "anti-slop" guides, not measured frequency), but
they cluster densely in default output.

**Scene-setting openers**
- "In today's fast-paced world…" / "In an increasingly digital world…"
- "In the ever-evolving landscape of…"
- "In the realm of…"
- "Imagine a world where…"
- "Whether you're a beginner or a seasoned professional…"
- "Let's dive in." / "Let's explore…" / "Let's take a deep dive into…"

**Connective filler**
- "It's worth noting that…" / "It's important to note that…"
- "That being said," / "With that said," / "That said,"
- "Moreover," / "Furthermore," / "Additionally," / "In addition,"
- "At the end of the day," / "When all is said and done,"
- "It's important to remember that…"

**Inflated significance**
- "plays a crucial / vital / pivotal role in"
- "stands as a testament to"
- "sheds light on" · "paves the way for" · "at the forefront of"
- "a game-changer" · "revolutionize" · "transform the way we…"
- "unlock the potential of" · "unleash" · "harness the power of"
- "navigating the complexities of"
- "the intersection of X and Y"
- "a double-edged sword"
- "cutting-edge" · "state-of-the-art" · "seamless" · "robust"

**Closers**
- "In conclusion," / "To sum up," / "In summary,"
- "I hope this helps!"
- "Ultimately, the choice depends on your specific needs."
- "The possibilities are endless."

### 1.3 Syntactic — sentence-shape tells

**Negative parallelism** ⚠️ — the "It's not X, it's Y" construction, also called
contrastive phrasing (the term an OpenAI model-behavior PM reportedly used).
Variants: *"It's not just about X — it's about Y"*, *"This isn't X. It's Y."*

The device isn't inherently bad; JFK's "ask not what your country can do for
you" is the same move. What marks AI use is **emptiness**: both clauses describe
the same thing, the first setting a modest baseline and the second restating it
in weightier language, delivering no new information. A human version's second
clause carries information the first withheld.

Notably, **this tell has not faded** the way "delve" did, and OpenAI has
acknowledged ChatGPT overuses it and says it is working on broadening the range.

**Rule-of-three / flattened tricolon** ⚠️ — three parallel items, *nearly always
equal in length and identically punctuated* ("Efficient, effective, and
reliable"). The classical tricolon varies its members; the AI version doesn't.

**These cluster.** Negative parallelism, rule-of-three lists and em-dash pivots
travel together — if you find one, look for the other two.

**Other recurring shapes** ❓
- "Not only… but also…"
- "From X to Y to Z, …" (range-sweep openers)
- Rhetorical question as a section opener: "But what does this actually mean?"
- Second-person pivot mid-piece: an explanatory passage that swings into "you"
- "Think of it like…" analogies, usually one per section
- **Uniform sentence length** — a rhythm of near-identical clauses that no one
  writes by hand

### 1.4 Structural and formatting ❓

The most-cited tells in practitioner guides, and the ones that survive a
vocabulary find-and-replace.

- **Bold-lead-in bullets** — every bullet opening with a bolded phrase then a
  colon ("**Security:** …"). Repeatedly named the single most recognizable
  structural tell, and something almost nobody does unprompted by hand.
- **Mid-sentence bold** with no heading function — emphasis scattered where it
  isn't doing work. If the point matters, the sentence should carry it.
- **Emoji as bullet markers or in headings** (✅ 🚀 💡 📊 🔥 🎯) in professional
  contexts — learned from a million LinkedIn posts, per one guide.
- **Over-structuring** — a header for every two-sentence section; bullets where
  two sentences of prose would flow better; tables for non-tabular content.
- **The disguised listicle** ("listicle in a trenchcoat") — what you get after
  telling a model to stop using lists: the skeleton stays, wrapped in prose as
  "The first… The second… The third…"
- **Title Case Headings** and colon-split titles ("Rust Performance: A Complete
  Guide").
- **Markdown residue** — visible `**`, `##`, or link syntax pasted into a
  surface that doesn't render markdown.
- **Symmetry** — every section roughly the same length, every list the same
  depth, paragraphs uniformly 3–4 sentences. Human documents are lumpy; they
  dwell on what the writer actually cares about.
- Horizontal rules between every section.
- A closing paragraph that restates the piece without advancing it.

> Caveat from the same guides: this tell is **fading**, because custom
> formatting instructions are now common — and a draft scrubbed of everything
> distinctive is its own kind of slop, "technically clean and completely
> anonymous."

### 1.5 Assistant-register tells ❓

Artifacts of chat-assistant RLHF rather than of writing as such. Strong signal
when text was pasted straight out of a chat window.

- **Sycophantic openers** — "Great question!", "Certainly!", "Absolutely!",
  "What a fascinating topic!", "I'd be happy to help!"
- **Self-identification** — "As an AI language model…", "I don't have personal
  opinions / feelings / experiences", "As of my last update…"
- **Unsolicited disclaimers** — "This is not professional advice", "Consult a
  qualified professional", "Individual results may vary"
- **Compulsive both-sidesing** — perfectly balanced pro/con lists, refusal to
  reach a conclusion, "There are several factors to consider"
- **Hedging stacks** — "may potentially", "can sometimes", "it's possible that
  in some cases"
- **Offering follow-ups** — "Would you like me to expand on any of these?"
- **Over-apologizing** — "I apologize for the confusion"

### 1.6 Content-level tells — what is *absent* ❓

Often more diagnostic than anything present, and immune to editing passes.

- **No specifics.** Generic examples ("a company might…") where a human would
  name the company, the year, the number.
- **No lived detail.** Anecdotes that are structurally anecdote-shaped but carry
  no texture — no names, no sensory particulars, no irrelevant asides.
- **No idiosyncratic taste.** Nothing the writer disproportionately loves or
  hates; even coverage of unevenly important things.
- **Confident fabrication.** Plausible-looking citations, DOIs, author names,
  version numbers, API flags and quotes that don't exist. The formatting is
  right and the content is invented — the single most consequential tell.
- **Suspicious cleanliness.** No typos, uniform formatting, consistent serial
  commas throughout — human long-form drifts.
- **Knowledge-cutoff seams.** Confident present tense about a superseded state
  of the world; silence on recent events a human would mention.

### 1.7 Statistical properties (what detectors actually measure) ⚠️

Worth knowing because it explains why surface edits don't defeat detection.

- **Perplexity** — how *surprised* a reference model is by the next token.
  Lower = more predictable = more machine-like. Computed from summed log
  probabilities per token.
- **Burstiness** — the *variance* of perplexity across the document, in practice
  the standard deviation of per-sentence perplexity. The core insight: a person
  might accidentally write one AI-like sentence, but people vary construction
  and diction across a document, while models apply the same next-token rule
  formulaically. Low burstiness ⇒ machine-like.

Two important qualifications. **GPTZero itself stopped relying on these in
autumn 2023**, moving to a deep-learning architecture, though they persist as
one of several indicators there and still underpin many cheaper detectors
(ZeroGPT, Copyleaks, Originality, Writer). And the original implementation
scored text with **GPT-2** regardless of which model actually wrote it — a
substantial methodological weakness. Documented failure modes: false positives
in academic-integrity settings, and poor performance on short, edited or mixed
text.

### 1.8 Leakage artifacts — the free wins ❓

Not stylistic at all; someone forgot to delete something.

- Preamble left in: "Certainly! Here is the essay you requested:"
- Unfilled placeholders: `[Your Name]`, `[Insert company here]`, `[Date]`
- A trailing offer of further help at the bottom of a "finished" document
- Chat-interface artifacts: "Regenerate response", copy-button text
- Meta-commentary about the request itself in the deliverable
- Mixed voice — a first-person assistant sentence stranded in third-person copy

### 1.9 Character-level ⚠️

**These are artifacts, not watermarks.** When narrow no-break spaces (U+202F)
were found in OpenAI's o3/o4-mini output in April 2025, OpenAI called it *"a
quirk of large-scale reinforcement learning"* — not deliberate marking. They'd
be pointless as watermarks since anyone can strip them.

| Character | Code point | Signal strength |
|---|---|---|
| Narrow no-break space | U+202F | **Strongest** — keyboards don't produce it |
| Zero-width space | U+200B | Strong |
| Non-breaking space | U+00A0 | Moderate — but Word/web copy-paste also produce it |
| Word joiner / BOM | U+2060 / U+FEFF | Strong |
| Directional marks | U+200E / U+200F | Strong |
| Thin / hair space | U+2009 / U+200A | Moderate |
| Em dash | U+2014 | **Weak** — see false positives |
| En dash in ranges | U+2013 | Weak — correct typography |
| Curly quotes / apostrophes | U+2018/19/1C/1D | **Very weak** — Word and Google Docs insert these automatically |
| Ellipsis character | U+2026 | Weak — autocorrect produces it |
| Bullet character | U+2022 (• not `-`) | Weak |
| Arrow, multiplication, minus | U+2192 → · U+00D7 × · U+2212 − | Weak — but a cluster of "correct" typographic signs where a keyboard would give `->`, `x`, `-` is worth noting |

**Frequency matters more than presence for the weak ones.** One guide's rule of
thumb: a human might use 2–3 em dashes in a piece, generated text 20+. Density,
not existence, is the signal.

Two important qualifications: **all popular LLMs inject hidden characters**, so
their presence doesn't discriminate *between* models; and stripping them does
**not** defeat detection, because statistical watermarks live in word patterns,
not code points. Clean them for practical reasons (broken URL slugs, corrupted
JSON, failed string comparisons, ATS rejections), not to hide provenance.

---

## Part 2 — Per-model notes

> **Evidence warning.** Everything in this part except where marked ✅ or ⚠️ is
> impressionistic and drawn from comparison blogs, many of them SEO-driven or
> vendor-adjacent. Treat as hypotheses to test, not findings.

### ChatGPT (OpenAI)

The best-documented model, largely because it was dominant during the corpus
studies — meaning **the Part 1 "universal" list is really the ChatGPT list**,
generalized.

- **Lexical** ✅ — the excess-vocabulary set above; a Tübingen analysis of 14M
  PubMed abstracts flags *robust, pivotal, facilitate, leverage, delve*.
- **Transitions** ❓ — heavy *Moreover, Furthermore, Additionally, That being
  said, In conclusion*.
- **Register** ❓ — "competent professional writer, every single time";
  described in blind tests as *too smooth* / *sounds like a corporate email*.
- **Em dash** ⚠️ — the "ChatGPT hyphen" meme, which took off in early 2025.
  Product consequence: in **November 2025 Sam Altman announced** users can
  control punctuation preferences via custom instructions, after complaints
  that the dashes couldn't be removed even by explicit instruction. That's a
  vendor acknowledgment the default was strong.
- **Characters** ⚠️ — U+202F in o3/o4-mini, per above.

### Claude (Anthropic)

❓ **and note a methodological problem: this file was written by Claude, so any
introspective characterization here is unreliable** — self-report is not
stylometry. What follows is from third-party comparisons only.

- Higher **burstiness** — sentence lengths reportedly swinging from ~5 to ~40
  words, versus more uniform output elsewhere.
- Broader vocabulary, less reliance on a fixed transition set, natural
  contractions.
- Its *own* tell, per one source: a **consistently thoughtful, measured
  quality that humans don't sustain** — real writing has lazy and abrupt
  moments.
- Detection data ❓: one test put Claude at ~78% average detectability with high
  variance (some samples in the 60s), versus Gemini ~86%. Single-vendor test,
  no methodology audit — low confidence.
- Common surface habits ❓ (community observation, unverified): "I should note",
  "That said", "To be clear", "Here's the thing", hedged self-correction.
- ⚠️ **Non-stylistic provenance marker:** Anthropic signed the EU AI Act Article
  50(2) Code of Practice and embeds an **imperceptible statistical watermark**
  in text from supported models (launched in the EU on/after 2026-08-02),
  worldwide. This is *not* a character or phrase — it lives in token-choice
  statistics and needs Anthropic's detector, which was not public as of this
  writing. Files get C2PA metadata instead.

### Gemini (Google)

❓ Only impressionistic sourcing found.

- The **flattest register** of the big three; reads like a briefing document or
  research summary — organized, slightly dry.
- Strong tendency toward **listicle structure**.
- Less stylistic range than Claude, less polish than ChatGPT.
- ⚠️ Google DeepMind's **SynthID** does statistical watermarking for text/image/
  audio — again a token-pattern watermark, not a visible marker.

### Grok (xAI)

❓ No corpus study found; characterizations are from comparison blogs and xAI's
own product framing.

- Designed with a **"rebellious streak"**, modeled partly on *The Hitchhiker's
  Guide to the Galaxy*; explicitly marketed as a "humorous AI assistant".
- Markers: punchy phrasing, jokes and playful asides, meme and
  trending-culture references, an "internet-native" voice.
- **Fun Mode vs Regular Mode** produce materially different registers — sarcastic
  and blunt versus straightforward — so "Grok style" isn't one thing.
- Failure mode: tone wanders into jokes when a direct answer was wanted.
- **Best available primary source:** xAI has published system prompts on GitHub.
  That's the place to look for real evidence; I did not audit them for this file.

### Meta AI (Llama-based)

**Explicit null result.** I could not find corpus studies, credible stylometric
comparisons, or vendor statements characterizing Meta AI's distinctive lexical
or punctuation habits. The comparison literature is overwhelmingly
ChatGPT/Claude/Gemini, with Grok a distant fourth and Meta AI largely absent.

I am **not going to invent** a marker list for it. What can be said safely: it is
an LLM assistant and the Part 1 universal markers should broadly apply. If you
need Meta-specific tells, they'd have to come from your own corpus comparison.

---

## Part 3 — False-positive traps

The markers most people cite are the least reliable ones.

- **Em dash** — used by Emily Dickinson and Nietzsche; defended by editors as
  "the most human punctuation mark there is". The 2025 meme caused real harm:
  teachers flagging essays, recruiters scrutinizing résumés, and writers
  **self-censoring punctuation they'd used for decades**. ChatGPT's own answer
  is that em dashes "by themselves are not a reliable sign that a text was
  AI-generated."
- **Curly quotes and ellipses** — inserted automatically by Word, Google Docs
  and phone autocorrect. Near-zero diagnostic value alone.
- **Non-breaking spaces** — produced by web copy-paste and word processors.
- **"Delve"** — genuinely spiked, but it's ordinary vocabulary, and notably more
  common in some varieties of English (a point raised in the RLHF-annotator
  discussion). Accusing an individual writer on this word is unsound and
  carries a dialect-bias risk.
- **Good structure** — headers, bullets and bold are what technical writing
  *should* look like.
- **AI detectors themselves** — unreliable, with documented false positives; no
  detector output should be treated as evidence about a person.

---

## Part 4 — How to actually use this

**Never on a single marker.** Look for **clusters**: excess vocabulary *plus*
negative parallelism *plus* flattened tricolons *plus* U+202F. Any one alone is
noise.

**Weight by keyboard difficulty.** U+202F and U+200B are stronger than em dashes
and curly quotes, because ordinary tools don't emit them by accident.

**Prefer the substantive tells.** Emptiness beats orthography: contrastive
clauses that restate rather than add, tricolons with interchangeable members,
conclusions that summarize without advancing. Those survive a find-and-replace;
character cleanup doesn't touch them.

**Remember the asymmetry.** Absence of markers means nothing (they're trivially
removable), and presence means little about a *specific* document. This is a
corpus-level instrument being borrowed for document-level questions, which is
exactly where it breaks.

---

## Sources

**Peer-reviewed / corpus (strongest):**

1. Kobak et al., ["Delving into LLM-assisted writing in biomedical publications
   through excess vocabulary"](https://www.science.org/doi/10.1126/sciadv.adt3813),
   *Science Advances* — 15M+ PubMed abstracts, the excess-vocabulary method, the
   ≥13.5% estimate, style-words-not-content-words finding, corpus-level caveat.
   [arXiv preprint](https://arxiv.org/html/2406.07016v1).
2. Juzek & Ward, "Why Does ChatGPT 'Delve' So Much?", COLING 2025 — the RLHF
   experiment and its complicating result. [FSU summary](https://news.fsu.edu/news/science-technology/2025/02/17/why-does-chatgpt-delve-so-much-fsu-researchers-begin-to-uncover-why-chatgpt-overuses-certain-words/).
3. Longitudinal PubMed word-overuse study (2020–2024) and the linguistics
   replication across 26,010 abstracts — the 28% rise in 12 of 16 target words.

**Vendor statements / journalism (moderate):**

4. [Washington Post](https://www.washingtonpost.com/technology/2025/04/09/ai-em-dash-writing-punctuation-chatgpt/),
   [Rolling Stone](https://www.rollingstone.com/culture/culture-features/chatgpt-hypen-em-dash-ai-writing-1235314945/),
   [The Ringer](https://www.theringer.com/2025/08/20/pop-culture/em-dash-use-ai-artificial-intelligence-chatgpt-google-gemini) —
   the em-dash controversy, real-world fallout, Altman's Nov 2025 custom-
   instructions change.
5. [Originality.AI on invisible characters](https://originality.ai/blog/invisible-text-detector-remover) —
   OpenAI's "quirk of large-scale reinforcement learning" statement on U+202F,
   the all-models-inject-them point, and that stripping them doesn't defeat
   detection.
6. [How Claude marks AI-generated content](https://support.claude.com/en/articles/16266773-how-claude-marks-ai-generated-content) —
   Anthropic's statistical text watermark and C2PA file metadata.

**Impressionistic / community (weak — flagged as such above):**

7. [Humanized Copy on negative parallelism](https://humanizedcopy.com/posts/the-it-s-not-just-x-it-s-y-tell-ai-negative-parallelism)
   and [Vollmer's field guide to AI tells](https://matthewvollmer.substack.com/p/i-asked-the-machine-to-tell-on-itself) —
   the construction, its clustering with tricolons and em dashes, OpenAI's
   acknowledgment, and the counterargument that the device isn't inherently bad.
8. Model-comparison blogs (tactiq.io, aitextdetector.ai, datastudios.org,
   coursera, datacamp) — the Claude/Gemini/Grok register characterizations and
   the detectability percentages. **SEO-driven and vendor-adjacent; not
   independent research.**
9. Practitioner "anti-slop" guides — [The Field Guide to AI Slop](https://www.ignorance.ai/p/the-field-guide-to-ai-slop),
   [tropes.fyi directory](https://tropes.fyi/directory),
   [Momentic: 34 types of AI slop](https://momenticmarketing.com/blog/avoid-ai-slop),
   [AIPromptIndex: 19 patterns](https://aipromptindex.io/guides/remove-ai-slop-from-writing/) —
   source for §1.2 stock phrases and §1.4 formatting tells: bold-lead-in
   bullets, mid-sentence bold, emoji bullets, the "listicle in a trenchcoat",
   Title Case headings, markdown residue, the 2–3 vs 20+ em-dash density rule,
   and the caveat that formatting tells are fading as custom instructions
   spread. **Practitioner consensus, not measured frequency data.**

**Detection methodology:**

10. [GPTZero: perplexity and burstiness](https://gptzero.me/news/perplexity-and-burstiness-what-is-it/)
    and [How AI detectors work](https://gptzero.me/news/how-ai-detectors-work/) —
    §1.7 definitions, the standard-deviation-of-per-sentence-perplexity
    implementation, the GPT-2 scoring weakness, GPTZero's autumn-2023 move away
    from these measures, which other detectors still rely on them, and the
    documented false-positive problems.

**Gaps I did not close:** no per-model stylometry exists in the peer-reviewed
literature; Meta AI has no usable characterization; xAI's published system
prompts were not audited; the Claude section is compromised by being written by
Claude.

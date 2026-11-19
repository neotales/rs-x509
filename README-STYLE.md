# README style for Neotales crates

Every crate README in the Neotales organisation follows this. The copy script
puts this file in each crate so the rule travels with the code.

READMEs are the one place where emoji and a picture are required. The
documentor persona rule of "no decorative emoji" was overridden for crate
READMEs by the owner. Do not strip emoji from a crate README during a tidy-up.
It applies to READMEs only. Every other document, comment and commit message
keeps the no-emoji rule.

## Required sections, in this order

1. `# crate-name` as an H1, then one sentence on what it does.
2. The brand image. `https://github.com/neotales/brand/raw/main/assets/logo.png`
   with alt text `Neotales`. Link the image to the repository. Do not vendor the
   image into the crate repository: it lives in `neotales/brand` and a copy
   would go stale.
3. A badge line: crates.io, docs.rs, CI. Add the badges when the crate name is
   known and the first version is on crates.io. The template cannot guess the
   name, so the line is a placeholder until then.
4. Install. `cargo add`, one line, plus the feature flags if the default feature
   set is not the interesting one.
5. "What you can do". The section that sells the crate. Bold lead-ins that name
   a capability and then real code that shows it. Short samples, each one
   something a caller would actually write. A GIF or an original captioned image
   belongs here or beside it. Do not hotlink a copyrighted meme host. Record the
   GIF yourself or draw the image, and store it under `docs/` in the crate.
6. Licence, linking to `LICENSE.md`.

No long "what this is not" section. If a caller could misuse the crate from the
README alone, one sentence of boundary is enough. Put it where the misuse would
happen, not in a section of its own.

## Shape

Emoji: at least one per README, in the "what you can do" section or its
headings. Keep it to a few. A wall of emoji reads as spam and a caller stops
reading.

Media: one GIF or one original captioned image, at most two. A GIF shows the
capability working. A captioned diagram shows the flow when there is nothing to
watch.

Samples: code blocks a reader can paste. Show the import, the call and the
result. Do not show a signature with `...` in it.

Length: a screen or two. The README sells the crate. The API docs explain it.

## Tone

Concrete and short. Name the function, the type and the error. Say what happens.
Cut every word that does no work.

Do not write "seamlessly", "powerful", "robust", "leverage" or "blazing fast".
If a claim needs an adjective, it needs a benchmark instead.

Before calling a README done, run it through:

- `/home/dev/work/atlas/agents/project-skills/unslop/SKILL.md`
- `/home/dev/work/atlas/agents/project-skills/technical-writing/SKILL.md`

The unslop rules that catch crate READMEs most often: em dashes, "not just X
but Y", forced rule-of-three lists, and headings in title case. Sentence case
for headings.

## Skeleton

````markdown
# neotales-example

Parses, validates and rewrites the example format.

[![Neotales](https://github.com/neotales/brand/raw/main/assets/logo.png)](https://github.com/neotales/neotales-example)

<!-- badges: crates.io, docs.rs, CI. Add once the crate name and first version exist. -->

## Install

```sh
cargo add neotales-example
```

## What you can do

**Parse without copying.** The parser borrows from the input buffer.

```rust
let doc = example::parse(input.as_bytes())?;
assert_eq!(doc.records().len(), 3);
```

**Rewrite in place.** ...

![Rewriting a document](docs/rewrite.gif)

## Licence

[MIT](LICENSE.md)
````

## Checklist

- [ ] H1 is the crate name, and only the crate name
- [ ] One sentence under the H1
- [ ] Brand image present, alt text `Neotales`, not vendored
- [ ] Badge line present, or the placeholder comment still there
- [ ] `cargo add` install line
- [ ] "What you can do" with bold lead-ins and pasteable samples
- [ ] At least one emoji
- [ ] A GIF or an original captioned image, no hotlinked memes
- [ ] Licence section links `LICENSE.md`
- [ ] No "what this is not" essay
- [ ] Read once for unslop and technical writing


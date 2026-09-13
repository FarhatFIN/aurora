## PART 6 — Work Breakdown Structure (WBS)

### §6.1 How to read and use the WBS

- The WBS is the project's **task ledger**. Every `- [ ]` item is a task with an
  acceptance bar: done means implemented, tested (§8), and reported (§11.4).
- Items are grouped by surface, not by crate: one item may touch several crates;
  the crate map (§4.3) tells you where the code lives.
- Milestones (Part 7) reference these sections; each milestone names the subset
  of items it claims. An item checked in a later milestone than its surface's
  "home" milestone is normal — the ledger records *when*, the WBS records *what*.
- Where a checklist row and the standard disagree, the standard wins (§0.4).
  Rows marked *verify* contain values you must confirm against the spec when you
  implement them — treat them as leads, not answers.
- Tick items **in the same session** that completes them (§12.1). A checked item
  without a passing-test reference in the session log is treated as unchecked by
  the final audit (§12.7).
- This section is generated from `tools/wbs_data_*.py`; regenerate with
  `python3 tools/generate_wbs.py` after editing the data files, and never
  hand-edit the generated text — edit the data, regenerate, review the diff.


### §6.2 HTML element checklist

One block per element of the supported surface (§1.3). Legacy elements are
parse-compatible only; `non-goal` surfaces stay stubs by design.

#### `<html>` — root
- Content model: One head, one body.
- Parser behavior: In body mode the start tag is merged into the existing element.
- UA defaults: display:block.
- Layout: Establishes the initial containing block; quirks mode holder.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<head>` — metadata
- Content model: Metadata content only.
- Parser behavior: In head insertion mode; most stray content pops it.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<title>` — metadata
- Content model: Text.
- Parser behavior: RCDATA; text becomes the document title.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<base>` — metadata
- Content model: Empty.
- Parser behavior: Sets the document base URL (first wins).
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<link>` — metadata
- Content model: Empty.
- Parser behavior: Triggers stylesheet/favicon/preload handling per rel.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<meta>` — metadata
- Content model: Empty.
- Parser behavior: charset form restarts encoding detection (§5.5.5).
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<style>` — metadata
- Content model: Raw text.
- Parser behavior: Contents parsed as CSS, appended to the document sheets.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<script>` — metadata
- Content model: Script data.
- Parser behavior: Execution queued per async/defer/type (§5.5.6).
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<noscript>` — metadata
- Content model: Depends on scripting flag.
- Parser behavior: When scripting: RAWTEXT; contents not rendered.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<template>` — metadata
- Content model: Its own content fragment.
- Parser behavior: Contents go to a DocumentFragment, invisible to the open-element stack.
- UA defaults: display:none.
- Layout: Contents lay out only when adopted into the tree.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<slot>` — metadata
- Content model: Transparent.
- Parser behavior: Shadow-DOM slotting target.
- UA defaults: display:contents.
- Layout: Distributes assigned nodes in the flat tree.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<body>` — section
- Content model: Flow content.
- Parser behavior: Implied when tokens hit it.
- UA defaults: display:block.
- Layout: Scrolling box owner in the default document structure.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<header>` — section
- Content model: Flow content.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<footer>` — section
- Content model: Flow content.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<main>` — section
- Content model: Flow content.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<section>` — section
- Content model: Flow content.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<nav>` — section
- Content model: Flow content.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<article>` — section
- Content model: Flow content.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<aside>` — section
- Content model: Flow content.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<address>` — section
- Content model: Flow content.
- Parser behavior: Generic.
- UA defaults: display:block; font-style:italic.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<h1>` — section
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:block; bold; size scale by level.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<h2>` — section
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:block; bold; scale.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<h3>` — section
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:block; bold; scale.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<h4>` — section
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:block; bold; scale.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<h5>` — section
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:block; bold; scale.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<h6>` — section
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:block; bold; scale.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<hgroup>` — section
- Content model: Heading content.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<p>` — grouping
- Content model: Phrasing.
- Parser behavior: Implied end tags; closes an open p.
- UA defaults: display:block; margins 1em.
- Layout: Block container; margin-collapsing showcase.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<hr>` — grouping
- Content model: Empty.
- Parser behavior: Generic.
- UA defaults: display:block; border:1px inset; margin.
- Layout: Atomic block; replaced-ish paint.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<pre>` — grouping
- Content model: Text.
- Parser behavior: Newline after open tag dropped; preserves whitespace.
- UA defaults: display:block; monospace; pre.
- Layout: Inline layout with pre white-space.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<blockquote>` — grouping
- Content model: Flow.
- Parser behavior: Generic.
- UA defaults: display:block; margins 1em 40px.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<ol>` — grouping
- Content model: Zero+ li.
- Parser behavior: Generic.
- UA defaults: display:block; padding-inline-start:40px; list-style-type:decimal.
- Layout: List-item container generating markers.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<ul>` — grouping
- Content model: Zero+ li.
- Parser behavior: Generic.
- UA defaults: display:block; padding-inline-start:40px; list-style-type:disc.
- Layout: List-item container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<menu>` — grouping
- Content model: Zero+ li.
- Parser behavior: Semantic alias of ul.
- UA defaults: display:block; list-style-type:disc.
- Layout: List-item container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<li>` — grouping
- Content model: Flow.
- Parser behavior: Implied end tags between siblings.
- UA defaults: display:list-item.
- Layout: Marker box + block/inline content.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<dl>` — grouping
- Content model: dt/dd groups.
- Parser behavior: Generic.
- UA defaults: display:block; margin:1em 0.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<dt>` — grouping
- Content model: Flow.
- Parser behavior: Implied end tags.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<dd>` — grouping
- Content model: Flow.
- Parser behavior: Implied end tags.
- UA defaults: display:block; margin-inline-start:40px.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<figure>` — grouping
- Content model: Flow + figcaption.
- Parser behavior: Generic.
- UA defaults: display:block; margin:1em 40px.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<figcaption>` — grouping
- Content model: Flow.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<div>` — grouping
- Content model: Flow.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container; the default case.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<a>` — text
- Content model: Transparent.
- Parser behavior: Implied end tags on block nesting.
- UA defaults: display:inline; color:-webkit-link; text-decoration:underline.
- Layout: Inline; activation behavior navigates (§5.6).
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<em>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; font-style:italic.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<strong>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; font-weight:bolder.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<small>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; font-size:smaller.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<s>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; text-decoration:line-through.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<cite>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; font-style:italic.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<q>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; quotes auto.
- Layout: Inline; generated quote marks per content/quotes.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<dfn>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; font-style:italic.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<abbr>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline.
- Layout: Inline; title tooltip source.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<ruby>` — text
- Content model: Phrasing.
- Parser behavior: Special in-body handling.
- UA defaults: display:ruby.
- Layout: Ruby annotation layout (MVP: inline fallback).
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<rt>` — text
- Content model: Phrasing.
- Parser behavior: Only inside ruby.
- UA defaults: display:ruby-text.
- Layout: MVP: inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<rp>` — text
- Content model: Phrasing.
- Parser behavior: Only inside ruby.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<code>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; monospace.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<kbd>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; monospace.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<samp>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; monospace.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<var>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; font-style:italic.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<time>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline.
- Layout: Inline; datetime attribute parsed.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<data>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline.
- Layout: Inline; value attribute.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<bdi>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; unicode-bidi:isolate.
- Layout: Inline; isolate in bidi pass.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<bdo>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; unicode-bidi:bidi-override.
- Layout: Inline; override direction from dir.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<span>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline.
- Layout: Inline; the inline default case.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<br>` — text
- Content model: Empty.
- Parser behavior: Generic.
- UA defaults: display:inline? (break).
- Layout: Forced line break in inline layout.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<wbr>` — text
- Content model: Empty.
- Parser behavior: Generic.
- UA defaults: display:inline.
- Layout: Soft wrap opportunity.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<ins>` — edits
- Content model: Transparent.
- Parser behavior: Generic.
- UA defaults: display:inline; text-decoration:underline.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<del>` — edits
- Content model: Transparent.
- Parser behavior: Generic.
- UA defaults: display:inline; text-decoration:line-through.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<img>` — embedded
- Content model: Empty.
- Parser behavior: Fires load/error async (§5.12).
- UA defaults: display:inline (replaced).
- Layout: Atomic inline; intrinsic size + object-fit.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<picture>` — embedded
- Content model: source+img.
- Parser behavior: Generic; sources pick the URL.
- UA defaults: display:inline.
- Layout: No box itself.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<source>` — embedded
- Content model: Empty.
- Parser behavior: Only inside picture/audio/video.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<iframe>` — embedded
- Content model: Fallback content.
- Parser behavior: Creates a nested browsing context placeholder.
- UA defaults: display:inline (replaced); border:2px inset.
- Layout: Atomic inline box, nested document area.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<embed>` — embedded
- Content model: Empty.
- Parser behavior: Plugin content placeholder.
- UA defaults: display:inline (replaced).
- Layout: Atomic inline placeholder box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<object>` — embedded
- Content model: Fallback content.
- Parser behavior: Placeholder; data URL fetch attempt.
- UA defaults: display:inline (replaced).
- Layout: Atomic inline placeholder.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<video>` — embedded
- Content model: Source elements + fallback.
- Parser behavior: Placeholder box, media element stub.
- UA defaults: display:inline (replaced).
- Layout: Atomic inline; intrinsic from attributes if any.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<audio>` — embedded
- Content model: Source elements.
- Parser behavior: No visual box; media stub with controls.
- UA defaults: display:none unless controls.
- Layout: Controls UI when controls attribute present.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<track>` — embedded
- Content model: Empty.
- Parser behavior: Only inside media elements.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<map>` — embedded
- Content model: Transparent.
- Parser behavior: Generic.
- UA defaults: display:inline.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<area>` — embedded
- Content model: Empty.
- Parser behavior: Only inside map.
- UA defaults: display:none.
- Layout: No box; hit regions parsed.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<table>` — tabular
- Content model: caption/colgroup/sections.
- Parser behavior: Foster parenting of stray content (§5.5).
- UA defaults: display:table; border-spacing 2px; border-collapse:separate.
- Layout: Table layout: anonymous boxes for stray rows/cells.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<caption>` — tabular
- Content model: Flow.
- Parser behavior: Only as first table child.
- UA defaults: display:table-caption.
- Layout: Caption box above/below per side.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<colgroup>` — tabular
- Content model: col elements.
- Parser behavior: Generic.
- UA defaults: display:table-column-group.
- Layout: Column grouping for spans.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<col>` — tabular
- Content model: Empty.
- Parser behavior: Generic.
- UA defaults: display:table-column.
- Layout: Column sizing contributor.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<tbody>` — tabular
- Content model: tr elements.
- Parser behavior: Implied when rows appear directly.
- UA defaults: display:table-row-group.
- Layout: Row group.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<thead>` — tabular
- Content model: tr elements.
- Parser behavior: Generic.
- UA defaults: display:table-header-group.
- Layout: Row group.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<tfoot>` — tabular
- Content model: tr elements.
- Parser behavior: Generic.
- UA defaults: display:table-footer-group.
- Layout: Row group.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<tr>` — tabular
- Content model: td/th.
- Parser behavior: Implied in row groups.
- UA defaults: display:table-row.
- Layout: Row box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<td>` — tabular
- Content model: Flow.
- Parser behavior: Implied end tags.
- UA defaults: display:table-cell; padding:1px.
- Layout: Cell box; anonymous row wrapping.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<th>` — tabular
- Content model: Flow.
- Parser behavior: Implied end tags.
- UA defaults: display:table-cell; bold; center.
- Layout: Cell box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<form>` — forms
- Content model: Flow, no nested form.
- Parser behavior: Form pointer management (§5.5).
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<label>` — forms
- Content model: Phrasing, no nested label.
- Parser behavior: Activation forwards to labeled control (§5.6).
- UA defaults: display:inline.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<input>` — forms
- Content model: Empty.
- Parser behavior: Type-driven control construction.
- UA defaults: varies by type (text: inline-block ~size).
- Layout: Replaced-ish inline-block controls.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<button>` — forms
- Content model: Phrasing.
- Parser behavior: Default button type; activation behavior.
- UA defaults: display:inline-block; UA chrome styling.
- Layout: Inline-block control with border/background defaults.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<select>` — forms
- Content model: option/optgroup.
- Parser behavior: Option list construction.
- UA defaults: display:inline-block.
- Layout: Control; popup list is shell UI.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<datalist>` — forms
- Content model: options.
- Parser behavior: Provides suggestions; renders nothing.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<optgroup>` — forms
- Content model: options.
- Parser behavior: Only inside select.
- UA defaults: display:block (in list UI).
- Layout: List UI group.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<option>` — forms
- Content model: Text.
- Parser behavior: Selectedness rules.
- UA defaults: display:block (in list UI).
- Layout: List UI entry.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<textarea>` — forms
- Content model: Text.
- Parser behavior: RCDATA default value.
- UA defaults: display:inline-block; monospace; resize both.
- Layout: Multi-line editable control.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<output>` — forms
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<progress>` — forms
- Content model: Phrasing.
- Parser behavior: Determinateness from value/max.
- UA defaults: display:inline-block.
- Layout: Replaced-ish; bar rendering.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<meter>` — forms
- Content model: Phrasing.
- Parser behavior: Gauge from value/min/max/low/high/optimum.
- UA defaults: display:inline-block.
- Layout: Replaced-ish; gauge rendering.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<fieldset>` — forms
- Content model: Flow + legend.
- Parser behavior: Generic.
- UA defaults: display:block; border groove; margins.
- Layout: Block container; legend special-placed.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<legend>` — forms
- Content model: Phrasing.
- Parser behavior: First-child special case.
- UA defaults: display:block; padding; float rules.
- Layout: Renders in the fieldset border gap.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<details>` — interactive
- Content model: summary + flow.
- Parser behavior: Toggle via name group (§5.6).
- UA defaults: display:block.
- Layout: Content hidden unless open.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<summary>` — interactive
- Content model: Phrasing + heading.
- Parser behavior: First summary is the widget.
- UA defaults: display:block; list-item marker.
- Layout: Marker + click target.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<dialog>` — interactive
- Content model: Flow.
- Parser behavior: open attribute; top layer when modal.
- UA defaults: display:none; block when open.
- Layout: Top-layer rendering with ::backdrop.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<canvas>` — embedded
- Content model: Fallback.
- Parser behavior: Bitmap backing store (§5.6 ctx2d).
- UA defaults: display:inline (replaced).
- Layout: Atomic inline; default 300x150.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<center>` — legacy
- Content model: Flow.
- Parser behavior: Treated as div with presentational hint.
- UA defaults: display:block; text-align:center.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<font>` — legacy
- Content model: Phrasing.
- Parser behavior: Presentational hints: face/size/color.
- UA defaults: display:inline.
- Layout: Inline; hints map to style.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<marquee>` — legacy
- Content model: Phrasing.
- Parser behavior: Parse; render as static block (non-goal animation).
- UA defaults: display:block; overflow:hidden.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<frameset>` — legacy
- Content model: frame/frameset.
- Parser behavior: Parse-only; content not rendered (non-goal).
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<frame>` — legacy
- Content model: Empty.
- Parser behavior: Parse-only.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<noframes>` — legacy
- Content model: Anything.
- Parser behavior: RAWTEXT in frameset docs.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<xmp>` — legacy
- Content model: Text.
- Parser behavior: RAWTEXT; literal text.
- UA defaults: display:block; monospace; pre.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<plaintext>` — legacy
- Content model: Text.
- Parser behavior: Rest of document becomes text tokens.
- UA defaults: display:block; monospace; pre.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

### §6.3 CSS property checklist

The engine's supported property surface. `Initial` values marked *verify*
must be confirmed against the property's CSSWG definition during
implementation; everything else follows §0.4 precedence. Shorthands are
expansion sugar — their longhands carry the real behavior.

#### Group `box` — §5.9/§5.10 — box generation, display, positioning; layout owns geometry
- 36 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `display`
- Inherited: No · Initial: `inline` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `box-sizing`
- Inherited: No · Initial: `content-box` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin`
- Inherited: No · Initial: `see longhands (0)` · Owner: box · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-top`
- Inherited: No · Initial: `0` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-right`
- Inherited: No · Initial: `0` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-bottom`
- Inherited: No · Initial: `0` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-left`
- Inherited: No · Initial: `0` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding`
- Inherited: No · Initial: `see longhands (0)` · Owner: box · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-top`
- Inherited: No · Initial: `0` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-right`
- Inherited: No · Initial: `0` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-bottom`
- Inherited: No · Initial: `0` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-left`
- Inherited: No · Initial: `0` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `width`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `height`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `min-width`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `min-height`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `max-width`
- Inherited: No · Initial: `none` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `max-height`
- Inherited: No · Initial: `none` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `inset`
- Inherited: No · Initial: `auto` · Owner: box · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `top`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `right`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `bottom`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `left`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `position`
- Inherited: No · Initial: `static` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `z-index`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `float`
- Inherited: No · Initial: `none` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `clear`
- Inherited: No · Initial: `none` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overflow`
- Inherited: No · Initial: `see longhands (visible)` · Owner: box · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overflow-x`
- Inherited: No · Initial: `visible` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overflow-y`
- Inherited: No · Initial: `visible` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `visibility`
- Inherited: Yes · Initial: `visible` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `object-fit`
- Inherited: No · Initial: `fill` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `object-position`
- Inherited: No · Initial: `50% 50%` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `aspect-ratio`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `container-type`
- Inherited: No · Initial: `normal` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `container-name`
- Inherited: No · Initial: `none` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `misc` — multiple — cross-cutting properties
- 6 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `all`
- Inherited: No · Initial: `see individual (reset)` · Owner: misc · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `image-rendering`
- Inherited: Yes · Initial: `auto` · Owner: misc · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `image-orientation`
- Inherited: No · Initial: `from-image` · Owner: misc · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `shape-outside`
- Inherited: No · Initial: `none` · Owner: misc · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `shape-margin`
- Inherited: No · Initial: `0` · Owner: misc · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `shape-image-threshold`
- Inherited: No · Initial: `0.5? verify` · Owner: misc · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `ui` — §5.19 — interaction surface: cursors, input, scrolling UX
- 28 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `appearance`
- Inherited: No · Initial: `none` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `cursor`
- Inherited: Yes · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `pointer-events`
- Inherited: Yes · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `user-select`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `resize`
- Inherited: No · Initial: `none` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `touch-action`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overscroll-behavior`
- Inherited: No · Initial: `see longhands (auto)` · Owner: ui · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overscroll-behavior-x`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overscroll-behavior-y`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-behavior`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-snap-type`
- Inherited: No · Initial: `none` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-snap-align`
- Inherited: No · Initial: `none` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-snap-stop`
- Inherited: No · Initial: `normal` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-margin`
- Inherited: No · Initial: `see longhands (0)` · Owner: ui · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-margin-top`
- Inherited: No · Initial: `0` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-margin-right`
- Inherited: No · Initial: `0` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-margin-bottom`
- Inherited: No · Initial: `0` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-margin-left`
- Inherited: No · Initial: `0` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-padding`
- Inherited: No · Initial: `see longhands (auto)` · Owner: ui · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-padding-top`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-padding-right`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-padding-bottom`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-padding-left`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `will-change`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `contain`
- Inherited: No · Initial: `none` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `content-visibility`
- Inherited: No · Initial: `visible` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `isolation`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mix-blend-mode`
- Inherited: No · Initial: `normal` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `flex` — §5.10.1 — flexbox layout module
- 20 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `flex`
- Inherited: No · Initial: `see longhands` · Owner: flex · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `flex-grow`
- Inherited: No · Initial: `0` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `flex-shrink`
- Inherited: No · Initial: `1` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `flex-basis`
- Inherited: No · Initial: `auto` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `flex-direction`
- Inherited: No · Initial: `row` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `flex-wrap`
- Inherited: No · Initial: `nowrap` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `flex-flow`
- Inherited: No · Initial: `see longhands` · Owner: flex · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `order`
- Inherited: No · Initial: `0` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `align-items`
- Inherited: No · Initial: `normal` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `align-self`
- Inherited: No · Initial: `auto` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `align-content`
- Inherited: No · Initial: `normal` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `justify-items`
- Inherited: No · Initial: `legacy` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `justify-self`
- Inherited: No · Initial: `auto` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `justify-content`
- Inherited: No · Initial: `normal` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `gap`
- Inherited: No · Initial: `normal` · Owner: flex · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `row-gap`
- Inherited: No · Initial: `normal` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-gap`
- Inherited: No · Initial: `normal` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `place-items`
- Inherited: No · Initial: `see longhands` · Owner: flex · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `place-content`
- Inherited: No · Initial: `see longhands` · Owner: flex · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `place-self`
- Inherited: No · Initial: `see longhands` · Owner: flex · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `grid` — §5.10.2 — grid layout module
- 15 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `grid`
- Inherited: No · Initial: `see longhands` · Owner: grid · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-template`
- Inherited: No · Initial: `see longhands` · Owner: grid · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-template-rows`
- Inherited: No · Initial: `none` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-template-columns`
- Inherited: No · Initial: `none` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-template-areas`
- Inherited: No · Initial: `none` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-auto-rows`
- Inherited: No · Initial: `auto` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-auto-columns`
- Inherited: No · Initial: `auto` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-auto-flow`
- Inherited: No · Initial: `row` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-row`
- Inherited: No · Initial: `see longhands (auto)` · Owner: grid · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-column`
- Inherited: No · Initial: `see longhands (auto)` · Owner: grid · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-area`
- Inherited: No · Initial: `see longhands (auto)` · Owner: grid · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-row-start`
- Inherited: No · Initial: `auto` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-row-end`
- Inherited: No · Initial: `auto` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-column-start`
- Inherited: No · Initial: `auto` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-column-end`
- Inherited: No · Initial: `auto` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `table` — §5.9 — table formatting (§5.9.2 box building + anonymous table boxes)
- 5 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `table-layout`
- Inherited: No · Initial: `auto` · Owner: table · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-collapse`
- Inherited: No · Initial: `separate` · Owner: table · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-spacing`
- Inherited: No · Initial: `0` · Owner: table · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `empty-cells`
- Inherited: No · Initial: `show` · Owner: table · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `caption-side`
- Inherited: No · Initial: `top` · Owner: table · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `color` — §5.16 — color and opacity resolve to paint values
- 7 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `color`
- Inherited: Yes · Initial: `canvastext` · Owner: color · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `opacity`
- Inherited: No · Initial: `1` · Owner: color · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `color-scheme`
- Inherited: Yes · Initial: `normal` · Owner: color · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `accent-color`
- Inherited: No · Initial: `auto` · Owner: color · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `caret-color`
- Inherited: No · Initial: `auto` · Owner: color · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `print-color-adjust`
- Inherited: Yes · Initial: `economic` · Owner: color · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `forced-color-adjust`
- Inherited: Yes · Initial: `auto` · Owner: color · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `bg` — §5.16.2 — backgrounds render under content in paint order
- 10 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `background`
- Inherited: No · Initial: `see longhands` · Owner: bg · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-color`
- Inherited: No · Initial: `transparent` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-image`
- Inherited: No · Initial: `none` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-repeat`
- Inherited: No · Initial: `repeat` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-position`
- Inherited: No · Initial: `0% 0%` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-size`
- Inherited: No · Initial: `auto` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-clip`
- Inherited: No · Initial: `border-box` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-origin`
- Inherited: No · Initial: `padding-box` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-attachment`
- Inherited: No · Initial: `scroll` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-blend-mode`
- Inherited: No · Initial: `normal` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `border` — §5.16.2 — borders render as part of the box edge path
- 37 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `border`
- Inherited: No · Initial: `see longhands (medium none currentcolor)` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-width`
- Inherited: No · Initial: `see longhands (medium)` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-style`
- Inherited: No · Initial: `see longhands (none)` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-color`
- Inherited: No · Initial: `see longhands (currentcolor)` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-top`
- Inherited: No · Initial: `see longhands` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-right`
- Inherited: No · Initial: `see longhands` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-bottom`
- Inherited: No · Initial: `see longhands` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-left`
- Inherited: No · Initial: `see longhands` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-top-width`
- Inherited: No · Initial: `medium` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-right-width`
- Inherited: No · Initial: `medium` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-bottom-width`
- Inherited: No · Initial: `medium` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-left-width`
- Inherited: No · Initial: `medium` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-top-style`
- Inherited: No · Initial: `none` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-right-style`
- Inherited: No · Initial: `none` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-bottom-style`
- Inherited: No · Initial: `none` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-left-style`
- Inherited: No · Initial: `none` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-top-color`
- Inherited: No · Initial: `currentcolor` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-right-color`
- Inherited: No · Initial: `currentcolor` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-bottom-color`
- Inherited: No · Initial: `currentcolor` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-left-color`
- Inherited: No · Initial: `currentcolor` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-radius`
- Inherited: No · Initial: `see longhands (0)` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-top-left-radius`
- Inherited: No · Initial: `0` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-top-right-radius`
- Inherited: No · Initial: `0` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-bottom-right-radius`
- Inherited: No · Initial: `0` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-bottom-left-radius`
- Inherited: No · Initial: `0` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-image`
- Inherited: No · Initial: `see longhands` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-image-source`
- Inherited: No · Initial: `none` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-image-slice`
- Inherited: No · Initial: `100%` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-image-width`
- Inherited: No · Initial: `1` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-image-outset`
- Inherited: No · Initial: `0` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-image-repeat`
- Inherited: No · Initial: `stretch` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `outline`
- Inherited: No · Initial: `see longhands` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `outline-width`
- Inherited: No · Initial: `medium` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `outline-style`
- Inherited: No · Initial: `none` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `outline-color`
- Inherited: No · Initial: `auto` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `outline-offset`
- Inherited: No · Initial: `0` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `box-shadow`
- Inherited: No · Initial: `none` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `fragmentation` — §5.9.7 — break control between fragmentainers
- 6 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `box-decoration-break`
- Inherited: No · Initial: `slice` · Owner: fragmentation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `break-before`
- Inherited: No · Initial: `auto` · Owner: fragmentation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `break-after`
- Inherited: No · Initial: `auto` · Owner: fragmentation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `break-inside`
- Inherited: No · Initial: `auto` · Owner: fragmentation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `orphans`
- Inherited: Yes · Initial: `2` · Owner: fragmentation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `widows`
- Inherited: Yes · Initial: `2` · Owner: fragmentation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `transform` — §5.16.1 — transforms create stacking contexts and affect hit testing
- 10 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `transform`
- Inherited: No · Initial: `none` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `transform-origin`
- Inherited: No · Initial: `50% 50%` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `transform-box`
- Inherited: No · Initial: `view-box` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `transform-style`
- Inherited: No · Initial: `flat` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `perspective`
- Inherited: No · Initial: `none` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `perspective-origin`
- Inherited: No · Initial: `50% 50%` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `backface-visibility`
- Inherited: No · Initial: `visible` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `translate`
- Inherited: No · Initial: `none` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `rotate`
- Inherited: No · Initial: `none` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scale`
- Inherited: No · Initial: `none` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `transition` — §5.15 — transition machinery lives in the runtime, paints via interpolators
- 6 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `transition`
- Inherited: No · Initial: `see longhands` · Owner: transition · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `transition-property`
- Inherited: No · Initial: `all` · Owner: transition · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `transition-duration`
- Inherited: No · Initial: `0s` · Owner: transition · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `transition-timing-function`
- Inherited: No · Initial: `ease` · Owner: transition · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `transition-delay`
- Inherited: No · Initial: `0s` · Owner: transition · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `transition-behavior`
- Inherited: No · Initial: `normal` · Owner: transition · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `animation` — §5.15 — keyframe machinery lives in the runtime, paints via interpolators
- 11 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `animation`
- Inherited: No · Initial: `see longhands` · Owner: animation · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-name`
- Inherited: No · Initial: `none` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-duration`
- Inherited: No · Initial: `0s` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-timing-function`
- Inherited: No · Initial: `ease` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-delay`
- Inherited: No · Initial: `0s` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-iteration-count`
- Inherited: No · Initial: `1` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-direction`
- Inherited: No · Initial: `normal` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-fill-mode`
- Inherited: No · Initial: `none` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-play-state`
- Inherited: No · Initial: `running` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-composition`
- Inherited: No · Initial: `replace` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-timeline`
- Inherited: No · Initial: `auto` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `list` — §5.9/§5.16 — list items generate ::marker boxes
- 4 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `list-style`
- Inherited: No · Initial: `see longhands` · Owner: list · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `list-style-type`
- Inherited: Yes · Initial: `disc` · Owner: list · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `list-style-position`
- Inherited: No · Initial: `outside` · Owner: list · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `list-style-image`
- Inherited: No · Initial: `none` · Owner: list · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `multicol` — §5.9.7 — multi-column fragmentation
- 10 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `columns`
- Inherited: No · Initial: `see longhands (auto)` · Owner: multicol · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-count`
- Inherited: No · Initial: `auto` · Owner: multicol · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-width`
- Inherited: No · Initial: `auto` · Owner: multicol · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-gap`
- Inherited: No · Initial: `normal` · Owner: multicol · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-rule`
- Inherited: No · Initial: `see longhands (medium none currentcolor)` · Owner: multicol · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-rule-width`
- Inherited: No · Initial: `medium` · Owner: multicol · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-rule-style`
- Inherited: No · Initial: `none` · Owner: multicol · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-rule-color`
- Inherited: No · Initial: `currentcolor` · Owner: multicol · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-span`
- Inherited: No · Initial: `none` · Owner: multicol · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-fill`
- Inherited: No · Initial: `balance` · Owner: multicol · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `page` — §5.9.7 — paged media (parse-only in MVP)
- 4 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `page`
- Inherited: No · Initial: `auto` · Owner: page · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `page-break-before`
- Inherited: No · Initial: `auto` · Owner: page · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `page-break-after`
- Inherited: No · Initial: `auto` · Owner: page · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `page-break-inside`
- Inherited: No · Initial: `auto` · Owner: page · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `mask` — §5.16.3 — masks and clipping feed the clip chain
- 10 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `clip-path`
- Inherited: No · Initial: `none` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask`
- Inherited: No · Initial: `see longhands` · Owner: mask · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask-image`
- Inherited: No · Initial: `none` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask-mode`
- Inherited: No · Initial: `match-source` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask-repeat`
- Inherited: No · Initial: `repeat` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask-position`
- Inherited: No · Initial: `center` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask-clip`
- Inherited: No · Initial: `border-box` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask-origin`
- Inherited: No · Initial: `border-box` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask-size`
- Inherited: No · Initial: `auto` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask-composite`
- Inherited: No · Initial: `add` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `filter` — §5.16.4 — filters run as raster post-processes
- 2 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `filter`
- Inherited: No · Initial: `none` · Owner: filter · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `backdrop-filter`
- Inherited: No · Initial: `none` · Owner: filter · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `logical` — §5.9 — logical properties map to physical at computed-value time
- 28 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `margin-block`
- Inherited: No · Initial: `see longhands` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-block-start`
- Inherited: No · Initial: `0` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-block-end`
- Inherited: No · Initial: `0` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-inline`
- Inherited: No · Initial: `see longhands` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-inline-start`
- Inherited: No · Initial: `0` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-inline-end`
- Inherited: No · Initial: `0` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-block`
- Inherited: No · Initial: `see longhands (0)` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-block-start`
- Inherited: No · Initial: `0` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-block-end`
- Inherited: No · Initial: `0` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-inline`
- Inherited: No · Initial: `see longhands (0)` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-inline-start`
- Inherited: No · Initial: `0` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-inline-end`
- Inherited: No · Initial: `0` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-block-start`
- Inherited: No · Initial: `see longhands` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-block-end`
- Inherited: No · Initial: `see longhands` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-inline-start`
- Inherited: No · Initial: `see longhands` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-inline-end`
- Inherited: No · Initial: `see longhands` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `inline-size`
- Inherited: No · Initial: `auto` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `block-size`
- Inherited: No · Initial: `auto` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `min-inline-size`
- Inherited: No · Initial: `auto` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `min-block-size`
- Inherited: No · Initial: `auto` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `max-inline-size`
- Inherited: No · Initial: `none` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `max-block-size`
- Inherited: No · Initial: `none` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `inset-block-start`
- Inherited: No · Initial: `auto` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `inset-block-end`
- Inherited: No · Initial: `auto` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `inset-inline-start`
- Inherited: No · Initial: `auto` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `inset-inline-end`
- Inherited: No · Initial: `auto` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overflow-block`
- Inherited: No · Initial: `see overflow-x` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overflow-inline`
- Inherited: No · Initial: `see overflow-y` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `text` — §5.11/§5.9.3 — text processing and inline layout
- 40 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `line-height`
- Inherited: Yes · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `letter-spacing`
- Inherited: Yes · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `word-spacing`
- Inherited: Yes · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-align`
- Inherited: Yes · Initial: `start` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-align-last`
- Inherited: Yes · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-indent`
- Inherited: Yes · Initial: `0` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-transform`
- Inherited: Yes · Initial: `none` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-decoration`
- Inherited: No · Initial: `see longhands (none)` · Owner: text · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-decoration-line`
- Inherited: No · Initial: `none` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-decoration-style`
- Inherited: No · Initial: `solid` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-decoration-color`
- Inherited: No · Initial: `currentcolor` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-decoration-thickness`
- Inherited: No · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-underline-offset`
- Inherited: No · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-decoration-skip-ink`
- Inherited: No · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-emphasis`
- Inherited: No · Initial: `see longhands` · Owner: text · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-emphasis-style`
- Inherited: No · Initial: `none` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-emphasis-color`
- Inherited: No · Initial: `currentcolor` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-shadow`
- Inherited: Yes · Initial: `none` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `white-space`
- Inherited: Yes · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `word-break`
- Inherited: No · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overflow-wrap`
- Inherited: No · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `word-wrap`
- Inherited: No · Initial: `alias of overflow-wrap` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `hyphens`
- Inherited: Yes · Initial: `manual` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `tab-size`
- Inherited: Yes · Initial: `8` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `line-break`
- Inherited: No · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `vertical-align`
- Inherited: No · Initial: `baseline` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `direction`
- Inherited: Yes · Initial: `ltr` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `unicode-bidi`
- Inherited: No · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `writing-mode`
- Inherited: No · Initial: `horizontal-tb` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-orientation`
- Inherited: No · Initial: `mixed` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-combine-upright`
- Inherited: No · Initial: `none` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-rendering`
- Inherited: No · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-justify`
- Inherited: Yes · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-underline-position`
- Inherited: Yes · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `initial-letter`
- Inherited: No · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `quotes`
- Inherited: Yes · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `content`
- Inherited: No · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `counter-reset`
- Inherited: No · Initial: `none` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `counter-increment`
- Inherited: No · Initial: `none` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `counter-set`
- Inherited: No · Initial: `none` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `font` — §5.11.1 — font selection, matching, metrics (inherited by default)
- 22 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `font`
- Inherited: Yes · Initial: `see longhands` · Owner: font · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-family`
- Inherited: Yes · Initial: `per UA stylesheet` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-size`
- Inherited: Yes · Initial: `medium` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-weight`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-style`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-stretch`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-variant`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-variant-ligatures`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-variant-caps`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-variant-numeric`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-variant-east-asian`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-variant-position`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-variant-alternates`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-kerning`
- Inherited: Yes · Initial: `auto` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-feature-settings`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-variation-settings`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-size-adjust`
- Inherited: Yes · Initial: `none` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-synthesis-weight`
- Inherited: Yes · Initial: `auto` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-synthesis-style`
- Inherited: Yes · Initial: `auto` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-synthesis-small-caps`
- Inherited: Yes · Initial: `auto` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-optical-sizing`
- Inherited: Yes · Initial: `auto` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-language-override`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

### §6.4 CSS at-rules checklist

#### `@charset`
- Behavior: Must be first; selects the stylesheet encoding; ignored elsewhere. Parser consumes and reports it.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@import`
- Behavior: Fetches and cascades a sheet; conditions (media/supports/layer) gate it; relative URLs resolve against the sheet's base URL.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@media`
- Behavior: Gate rules on media queries: type (all/screen/print), width/height/aspect-ratio/resolution/orientation, prefers-color-scheme/reduced-motion/contrast; nested media allowed per CSS Conditional.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@supports`
- Behavior: Feature-query gate: property:value checks and (not/and/or) combinations evaluated against the property registry.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@page`
- Behavior: Paged media page boxes: margins, size; parse in MVP, layout as stretch.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@font-face`
- Behavior: Registers a font family: src list with format() and unicode-range, font-display, weight/style/stretch descriptors; contributes to §5.11 font matching.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@keyframes`
- Behavior: Named keyframe list: 0%/100%/from/to stops with declaration blocks; interpolation per animation-timing-function; consumed by the animation machinery of §5.15.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@namespace`
- Behavior: Declares namespace prefixes for type/attribute selector matching in XML-ish documents; affects selector namespace resolution only.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@layer`
- Behavior: Cascade layers: named and anonymous layer declarations with nested blocks; layer order participates in the cascade (§5.8.2).
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@container`
- Behavior: Container queries: size/inline-size/style conditions evaluated against the nearest ancestor container (§box container-type); MVP: size only.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@property`
- Behavior: Registers custom properties with syntax/initial-value/inherits; typed registration changes substitution and animation behavior.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@counter-style`
- Behavior: Defines list/counter marker styles (system, symbols, prefix/suffix, range, pad); referenced by list-style-type and counters.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@font-feature-values`
- Behavior: Named font feature value sets for font-variant-alternates; parse and expose to the shaper.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@scope`
- Behavior: Scopes nested rules to a range between a root and an optional scoping limit; affects matching (§5.8.1); MVP: parse + root-only scoping.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@starting-style`
- Behavior: Transitions-in entry styles: declarations applied at first style computation, then dropped.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@document (legacy)`
- Behavior: Non-standard; parse as Unknown rule block, never apply (§5.7 forward-compat rule).
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

### §6.5 Selector engine checklist

Each selector: parsed into the `Selector` AST, compiled into the matching
plan (§5.8.1), invalidation-registered (§5.8.6), and covered by a matching
test with positive and negative cases.

- [ ] `*` — *basic*: Universal selector; matches any element; zero specificity alone.
- [ ] `E` — *basic*: Type selector with optional namespace prefix (svg | rect); case-insensitivity per document language.
- [ ] `.c` — *basic*: Class selector; compound list matching on DOMTokenList; whitespace-separated.
- [ ] `[a]` — *basic*: Attribute presence; matches namespaced attributes with the standard default namespace rules.
- [ ] `[a=v]` — *basic*: Exact match.
- [ ] `[a~=v]` — *basic*: Whitespace-list match.
- [ ] `[a` — *=v]*: basic | Prefix-with-dash match (lang codes).
- [ ] `[a^=v]` — *basic*: Prefix match (empty value never matches).
- [ ] `[a$=v]` — *basic*: Suffix match (empty value never matches).
- [ ] `[a*=v]` — *basic*: Substring match.
- [ ] `[a=v i]` — *basic*: Case-insensitive flag; [a=v s] case-sensitive flag.
- [ ] `:is(x, y)` — *logical*: Specificity = most specific argument; forgiving selector list (invalid arguments dropped).
- [ ] `:where(x, y)` — *logical*: Specificity = zero; forgiving list; otherwise identical to :is().
- [ ] `:not(x, y)` — *logical*: Negation; forgiving list; specificity = most specific argument.
- [ ] `:has(rel sel)` — *logical*: Relative selector match on descendants/siblings; requires the §5.8 invalidation machinery.
- [ ] `:scope` — *logical*: Matches the scoping root (querySelector context, @scope root).
- [ ] `:root` — *structural*: Matches the document root element.
- [ ] `:empty` — *structural*: Element with no children (text nodes count; whitespace counts).
- [ ] `:nth-child(an+b)` — *structural*: Full an+b grammar incl. of S syntax (:nth-child(2n of .c)); parse per css-syntax an+b tests.
- [ ] `:nth-last-child(an+b)` — *structural*: Counted from the end.
- [ ] `:nth-of-type(an+b)` — *structural*: Index among same-type siblings.
- [ ] `:nth-last-of-type(an+b)` — *structural*: Index among same-type siblings from the end.
- [ ] `:first-child` — *structural*: nth-child(1).
- [ ] `:last-child` — *structural*: nth-last-child(1).
- [ ] `:only-child` — *structural*: First and last child simultaneously.
- [ ] `:first-of-type` — *structural*: First among same-type siblings.
- [ ] `:last-of-type` — *structural*: Last among same-type siblings.
- [ ] `:only-of-type` — *structural*: Only among same-type siblings.
- [ ] `:link` — *resource*: Unvisited link state (elements with href that are links); see §5.18.7.
- [ ] `:any-link` — *resource*: Link regardless of visited state.
- [ ] `:visited` — *resource*: Visited state; restricted to paint-only properties (§5.18.7).
- [ ] `:target` — *resource*: Element targeted by the URL fragment.
- [ ] `:defined` — *resource*: Custom-element definedness (always true for built-ins in this engine).
- [ ] `:modal` — *resource*: True for elements in the top layer as modal dialogs.
- [ ] `:fullscreen` — *resource*: Always false in this engine (non-goal §1.4).
- [ ] `:hover` — *user*: Pointer-over state from the input system; ancestors of the hovered element also match.
- [ ] `:active` — *user*: Activation-in-progress state for buttons/links.
- [ ] `:focus` — *user*: The focused element (one per document).
- [ ] `:focus-within` — *user*: Focused element or its ancestors.
- [ ] `:focus-visible` — *user*: Heuristic per spec: keyboard-initiated focus gets the ring.
- [ ] `:enabled` — *form*: Form control that is not disabled.
- [ ] `:disabled` — *form*: Disabled form control (from the disabled attribute or fieldset propagation).
- [ ] `:checked` — *form*: Checked checkbox/radio or selected option.
- [ ] `:indeterminate` — *form*: Checkbox with indeterminate IDL state, radio group with none selected, progress without value.
- [ ] `:placeholder-shown` — *form*: Input currently showing placeholder.
- [ ] `:default` — *form*: Default checked/selected option of its group.
- [ ] `:required` — *form*: Required form control.
- [ ] `:optional` — *form*: Not required.
- [ ] `:valid` — *form*: Passes constraint validation (§5.6 forms subset).
- [ ] `:invalid` — *form*: Fails constraint validation.
- [ ] `:in-range` — *form*: Has value constraints and value is within them.
- [ ] `:out-of-range` — *form*: Has constraints and value is outside.
- [ ] `:read-only` — *form*: Not editable (readonly, disabled, or non-input element).
- [ ] `:read-write` — *form*: Editable.
- [ ] `:lang(x)` — *linguistic*: Language match with wildcard ranges; walks the lang attribute chain plus meta inheritance.
- [ ] `:dir(ltr/rtl)` — *linguistic*: Directionality from the bidi algorithm result (§5.11.4).
- [ ] `::before` — *pseudo-element*: Generated box before content per content property.
- [ ] `::after` — *pseudo-element*: Generated box after content per content property.
- [ ] `::placeholder` — *pseudo-element*: Styles the input placeholder text.
- [ ] `::selection` — *pseudo-element*: Styles the active selection; paint-only overlay.
- [ ] `::marker` — *pseudo-element*: List item marker box styling.
- [ ] `::first-line` — *pseudo-element*: First formatted line of a block (limited property set per spec).
- [ ] `::first-letter` — *pseudo-element*: First letter/leading punctuation box of a block.
- [ ] `::backdrop` — *pseudo-element*: Behind top-layer elements (dialog).
- [ ] `::file-selector-button` — *pseudo-element*: Button inside file inputs (sub-UI styling MVP).

### §6.6 DOM interface checklist

One block per script-visible interface (§5.15.2 binding rules apply to
every one): correct prototype chain, attribute getters/setters with the
right exceptions, method overloads and optional arguments, stringifier /
iterable / legacy platform object behaviors where marked.

#### `EventTarget`
- Surface:
method addEventListener(type, callback, options);
  method removeEventListener(type, callback, options); method dispatchEvent(event) -> bool.
- Tasks: Base of the dispatch machinery: capture, target, bubble phases (§5.6).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Node` — inherits `EventTarget`
- Surface:
attr nodeType; attr nodeName; attr baseURI; attr isConnected; attr ownerDocument;
  attr parentNode; attr parentElement; attr childNodes (live NodeList); attr firstChild;
  attr lastChild; attr previousSibling; attr nextSibling; attr textContent (get/set);
  method hasChildNodes(); method normalize(); method cloneNode(deep);
  method isEqualNode(other); method isSameNode(other);
  method compareDocumentPosition(other); method contains(other); method lookupPrefix(ns);
  method lookupNamespaceURI(prefix); method isDefaultNamespace(ns);
  method insertBefore(node, child); method appendChild(child);
  method replaceChild(node, child); method removeChild(child).
- Tasks: Arena-backed operations (§5.6); mutation observer hooks; all live collections recompute.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Document` — inherits `Node`
- Surface:
attr documentElement; attr doctype; attr body; attr head; attr title (get/set);
  attr cookie (get/set); attr domain; attr referrer; attr URL; attr documentURI;
  attr characterSet; attr contentType; attr compatMode; attr designMode; attr dir;
  attr forms (live); attr images (live); attr links (live); attr scripts (live);
  attr styleSheets; attr activeElement; attr currentScript; attr defaultView;
  method createElement(localName, options); method createElementNS(ns, qname);
  method createDocumentFragment(); method createTextNode(data); method createComment(data);
  method createProcessingInstruction(target, data); method createAttribute(name);
  method createAttributeNS(ns, name); method createEvent(type); method createRange();
  method createNodeIterator(root, filter); method createTreeWalker(root, filter);
  method createExpression/evaluate (XPath: non-goal, throw); method getElementById(id);
  method getElementsByName(name); method getElementsByTagName(qname);
  method getElementsByTagNameNS(ns, qname); method getElementsByClassName(names);
  method querySelector(sel); method querySelectorAll(sel); method adoptNode(node);
  method importNode(node, deep); method open(url, name); method close();
  method write(...text); method writeln(...text); method hasFocus();
  method execCommand (no-op with false, legacy).
- Tasks: Owner of the arena; base URL + origin resolution; cookie access via storage; write() re-enters the parser (§5.5.6).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `DocumentFragment` — inherits `Node`
- Surface:
attr children; method getElementById; method querySelector(All).
- Tasks: Template contents and createContextualFragment substrate.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `DocumentType` — inherits `Node`
- Surface:
attr name; attr publicId; attr systemId.
- Tasks: Serialization and quirks derivation.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `ShadowRoot` — inherits `DocumentFragment`
- Surface:
attr mode; attr host; attr delegatesFocus; attr slotAssignment; attr innerHTML (get/set).
- Tasks: Flat-tree construction; slot assignment algorithm.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Element` — inherits `Node`
- Surface:
attr namespaceURI; attr prefix; attr localName; attr tagName; attr id (get/set);
  attr className (get/set); attr classList (DOMTokenList); attr attributes (NamedNodeMap);
  attr children (live HTMLCollection); attr firstElementChild; attr lastElementChild;
  attr previousElementSibling; attr nextElementSibling; attr childElementCount;
  attr innerHTML (get/set); attr outerHTML (get/set); attr shadowRoot; attr assignedSlot;
  method hasAttribute(name); method getAttribute(name); method getAttributeNS(ns, name);
  method setAttribute(name, value); method setAttributeNS(ns, qname, value);
  method removeAttribute(name); method removeAttributeNS(ns, name);
  method toggleAttribute(name, force); method getAttributeNames(); method hasAttributes();
  method closest(sel); method matches(sel); method webkitMatchesSelector(sel);
  method insertAdjacentElement(pos, el); method insertAdjacentText(pos, text);
  method insertAdjacentHTML(pos, html); method before(...nodes); method after(...nodes);
  method replaceWith(...nodes); method remove(); method append(...nodes);
  method prepend(...nodes); method querySelector(sel); method querySelectorAll(sel);
  method getElementsByTagName(qname); method getElementsByTagNameNS(ns, qname);
  method getElementsByClassName(names); method attachShadow(init);
  method scroll/scrollTo/scrollBy (viewport + container); method scrollIntoView(arg);
  method getBoundingClientRect(); method getClientRects(); method checkVisibility(opts);
  method setHTMLUnsafe (parse-with-template).
- Tasks: Namespace-aware attribute map; dataset via DOMStringMap; style attribute; shadow attach.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLElement` — inherits `Element`
- Surface:
attr title; attr lang; attr dir; attr hidden; attr inert; attr accessKey; attr draggable;
  attr spellcheck; attr tabIndex (get/set); attr dataset (DOMStringMap);
  attr style (CSSStyleDeclaration); attr translate; method click(); method focus(opts);
  method blur(); method showPopover/hidePopover/togglePopover (MVP: no-op events).
- Tasks: Common attribute surface; inert subtree behavior.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Text` — inherits `CharacterData`
- Surface:
attr wholeText; attr assignedSlot; method splitText(offset) -> Text.
- Tasks: Splitting merges/normalizes per spec.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Comment` — inherits `CharacterData`
- Surface:
—.
- Tasks: Serialization keeps <!-- -->.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `ProcessingInstruction` — inherits `CharacterData`
- Surface:
attr target.
- Tasks: XML serialization path.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `CharacterData` — inherits `Node`
- Surface:
attr data (get/set); attr length; method substringData(offset, count);
  method appendData(text); method insertData(offset, text);
  method deleteData(offset, count); method replaceData(offset, count, text).
- Tasks: Base for Text/Comment/PI.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Attr` — inherits `Node`
- Surface:
attr name; attr value (get/set); attr namespaceURI; attr prefix; attr localName;
  attr specified (always true).
- Tasks: Not a child of elements in this engine's tree model; live value reflection.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `NodeList`
- Surface:
attr length; method item(i); method forEach(cb); iterable.
- Tasks: Static (querySelectorAll) or live variant flag.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLCollection`
- Surface:
attr length; attr namedItem support; method item(i); method namedItem(name); iterable.
- Tasks: Live; recomputed against the tree on access (§5.6 pitfalls).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `DOMTokenList`
- Surface:
attr length; attr value (get/set); method item(i); method contains(token);
  method add(...tokens); method remove(...tokens); method toggle(token, force);
  method replace(old, new); method supports(token); method keys/values/entries.
- Tasks: Whitespace-normalized; validation per interface (rel, class has none).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `DOMStringMap`
- Surface:
proxy-style named get/set/delete.
- Tasks: dataset backing store.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `CSSStyleDeclaration`
- Surface:
attr length; attr cssText (get/set); attr parentRule; method item(i);
  method getPropertyValue(prop); method setProperty(prop, value, priority);
  method removeProperty(prop); method getPropertyPriority(prop);
  method getPropertyShorthand; method isPropertyImplicit.
- Tasks: style attribute live object; getComputedStyle returns a read-only variant.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `CSSStyleSheet` — inherits `StyleSheet`
- Surface:
attr cssRules; attr ownerRule; attr ownerNode; attr href; attr title; attr media;
  attr disabled (get/set); method insertRule(rule, index); method deleteRule(index);
  method replaceSync(text).
- Tasks: Rule list with source locations for DevTools.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `StyleSheet`
- Surface:
attr type; attr href; attr ownerNode; attr parentStyleSheet; attr title;
  attr media (MediaList).
- Tasks: Base class.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `MediaQueryList`
- Surface:
attr matches; attr media; method addListener(cb); method removeListener(cb);
  method addEventListener; onchange.
- Tasks: Evaluated against current viewport; change events on resize.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Range`
- Surface:
attr startContainer; attr startOffset; attr endContainer; attr endOffset; attr collapsed;
  attr commonAncestorContainer; method setStart(node, off); method setEnd(node, off);
  method setStartBefore(node); method setStartAfter(node); method setEndBefore(node);
  method setEndAfter(node); method selectNode(node); method selectNodeContents(node);
  method collapse(toStart); method selectNodeContents;
  method compareBoundaryPoints(how, range); method deleteContents();
  method extractContents(); method cloneContents(); method insertNode(node);
  method surroundContents(node); method cloneRange(); method detach();
  method isPointInRange(node, off); method intersectsNode(node); method toString().
- Tasks: Boundary-point model with document-order comparison.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Selection`
- Surface:
attr anchorNode; attr anchorOffset; attr focusNode; attr focusOffset; attr isCollapsed;
  attr rangeCount; attr type; method getRangeAt(i); method addRange(range);
  method removeRange(range); method removeAllRanges(); method collapse(node, off);
  method setPosition; method collapseToStart/ToEnd; method extend(node, off);
  method setBaseAndExtent(anchor, aoff, focus, foff); method selectAllChildren(node);
  method containsNode(node, partial); method toString().
- Tasks: Backed by one document range; syncs with focus/inputs.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `NodeIterator`
- Surface:
attr root; attr referenceNode; attr pointerBeforeReferenceNode; attr whatToShow;
  attr filter; method nextNode(); method previousNode(); method detach().
- Tasks: Traversal with filter states.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `TreeWalker`
- Surface:
attr root; attr currentNode; attr whatToShow; attr filter; method parentNode();
  method firstChild(); method lastChild(); method previousSibling(); method nextSibling();
  method previousNode(); method nextNode().
- Tasks: Stateful cursor traversal.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `MutationObserver`
- Surface:
method observe(target, options); method disconnect(); method takeRecords().
- Tasks: Records queued until the microtask checkpoint (§5.15).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `MutationRecord`
- Surface:
attr type; attr target; attr addedNodes; attr removedNodes; attr previousSibling;
  attr nextSibling; attr attributeName; attr attributeNamespace; attr oldValue.
- Tasks: Constructed by the mutation hooks.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `ResizeObserver`
- Surface:
method observe(target, options); method unobserve(target); method disconnect().
- Tasks: Fires before paint in the rendering update (§5.15.1).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `IntersectionObserver`
- Surface:
attr root; attr rootMargin; attr thresholds; method observe(target);
  method unobserve(target); method disconnect(); method takeRecords().
- Tasks: Single-viewport MVP per §5.6.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Performance`
- Surface:
attr timeOrigin; attr memory (reported); method now(); method mark(name);
  method measure(name, start, end); method getEntries();
  method getEntriesByName(name, type); method getEntriesByType(type).
- Tasks: High-res clock capped to 100us granularity (timing-attack surface).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `History`
- Surface:
attr length; attr scrollRestoration; attr state; method back(); method forward();
  method go(delta); method pushState(data, title, url);
  method replaceState(data, title, url).
- Tasks: Same-document session history entry mutation.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Location`
- Surface:
attr href (get/set); attr protocol; attr host; attr hostname; attr port; attr pathname;
  attr search; attr hash; attr origin; method assign(url); method replace(url);
  method reload(); method toString().
- Tasks: Unforgeable on window; navigation triggers (§5.19.2).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Navigator`
- Surface:
attr userAgent; attr language; attr languages; attr platform; attr onLine;
  attr hardwareConcurrency; attr deviceMemory; attr maxTouchPoints; attr cookieEnabled;
  attr doNotTrack; attr globalPrivacyControl; method sendBeacon(url, data);
  method canShare/share (not supported, throw).
- Tasks: Static per browser; no fingerprinting extras.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Screen`
- Surface:
attr width; attr height; attr availWidth; attr availHeight; attr colorDepth;
  attr pixelDepth; attr devicePixelRatio (on window).
- Tasks: From the platform layer.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Window` — inherits `EventTarget`
- Surface:
attr window/self/parent/top/frames; attr document; attr location (unforgeable);
  attr history; attr navigator; attr screen; attr innerWidth/innerHeight;
  attr outerWidth/outerHeight; attr pageXOffset/pageYOffset; attr scrollX/scrollY;
  attr devicePixelRatio; attr name (get/set); attr status; attr closed;
  attr length (frames); attr frames list; attr event (legacy, sloppy only);
  attr localStorage; attr sessionStorage; attr console; attr customElements;
  attr performance; attr origin; method open(url, target, features); method close();
  method stop(); method focus(); method blur(); method print (no-op + console note);
  method alert(msg); method confirm(msg); method prompt(msg, def);
  method scroll/scrollTo/scrollBy(x, y);
  method moveTo/moveBy/resizeTo/resizeBy (shell-clamped);
  method getComputedStyle(el, pseudo); method matchMedia(q);
  method requestAnimationFrame(cb); method cancelAnimationFrame(id);
  method requestIdleCallback(cb); method cancelIdleCallback(id);
  method postMessage(message, targetOrigin, transfer); method getSelection();
  method btoa/atob; method structuredClone; method fetch(input, init);
  method setInterval/setTimeout/clearInterval/clearTimeout; method queueMicrotask(cb);
  method reportError(err); method createImageBitmap (not supported).
- Tasks: The global object; event handlers via IDL attributes (on*); named property access on frames/named elements (legacy).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Storage`
- Surface:
attr length; method key(i); method getItem(key); method setItem(key, value);
  method removeItem(key); method clear().
- Tasks: Per-origin (local) or per-tab (session); quota 5MB UTF-16 units (§5.17).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Event`
- Surface:
attr type; attr target; attr currentTarget; attr eventPhase; attr bubbles; attr cancelable;
  attr defaultPrevented; attr composed; attr isTrusted; attr timeStamp;
  method stopPropagation(); method stopImmediatePropagation(); method preventDefault();
  attr NONE/CAPTURING_PHASE/AT_TARGET/BUBBLING_PHASE.
- Tasks: Base dispatch machinery (§5.6).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `CustomUIEvent variants` — inherits `see below`
- Surface:
—.
- Tasks: Each typed event is its own binding block with its init dict.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `UIEvent` — inherits `Event`
- Surface:
attr view; attr detail.
- Tasks: Base for input-derived events.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `FocusEvent` — inherits `UIEvent`
- Surface:
attr relatedTarget.
- Tasks: focus/blur/focusin/focusout.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `MouseEvent` — inherits `UIEvent`
- Surface:
attr screenX; attr screenY; attr clientX; attr clientY; attr offsetX; attr offsetY;
  attr pageX; attr pageY; attr button; attr buttons; attr relatedTarget; attr movementX;
  attr movementY; modifier getters (ctrlKey, shiftKey, altKey, metaKey);
  method getModifierState(keyArg).
- Tasks: click/dblclick/contextmenu/mouse* family.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `WheelEvent` — inherits `MouseEvent`
- Surface:
attr deltaX; attr deltaY; attr deltaZ; attr deltaMode.
- Tasks: Scroll chaining decision lives in the shell/engine boundary.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `PointerEvent` — inherits `MouseEvent`
- Surface:
attr pointerId; attr width; attr height; attr pressure; attr tangentialPressure; attr tiltX;
  attr tiltY; attr twist; attr pointerType; attr isPrimary.
- Tasks: Mouse events are synthesized from pointers per UI Events.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `KeyboardEvent` — inherits `UIEvent`
- Surface:
attr key; attr code; attr location; attr ctrlKey; attr shiftKey; attr altKey; attr metaKey;
  attr repeat; attr isComposing; attr charCode (legacy); attr keyCode (legacy);
  method getModifierState(keyArg).
- Tasks: keydown/keyup (+ legacy keypress); Appendix F mapping.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `InputEvent` — inherits `UIEvent`
- Surface:
attr data; attr dataTransfer; attr isComposing; attr inputType.
- Tasks: beforeinput/input on editable elements.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `CompositionEvent` — inherits `UIEvent`
- Surface:
attr data; attr locale.
- Tasks: IME composition start/update/end.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `DragEvent` — inherits `MouseEvent`
- Surface:
attr dataTransfer (DataTransfer: MVP types/files subset).
- Tasks: drag* family with the simple drag-store model.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `ProgressEvent` — inherits `Event`
- Surface:
attr lengthComputable; attr loaded; attr total.
- Tasks: load/error/progress on fetches.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `MessageEvent` — inherits `Event`
- Surface:
attr data; attr origin; attr lastEventId; attr source; attr ports.
- Tasks: postMessage delivery.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `ErrorEvent` — inherits `Event`
- Surface:
attr message; attr filename; attr lineno; attr colno; attr error.
- Tasks: window error reporting.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `PromiseRejectionEvent` — inherits `Event`
- Surface:
attr promise; attr reason.
- Tasks: unhandledrejection/rejectionhandled.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HashChangeEvent` — inherits `Event`
- Surface:
attr oldURL; attr newURL.
- Tasks: hashchange.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `AnimationEvent` — inherits `Event`
- Surface:
attr animationName; attr elapsedTime; attr pseudoElement.
- Tasks: animationstart/iteration/end/cancel.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `TransitionEvent` — inherits `Event`
- Surface:
attr propertyName; attr elapsedTime; attr pseudoElement.
- Tasks: transitionstart/transitionrun/end/cancel.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `CloseEvent` — inherits `Event`
- Surface:
attr wasClean; attr code; attr reason.
- Tasks: WebSocket close.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `SecurityPolicyViolationEvent` — inherits `Event`
- Surface:
attr documentURI; attr referrer; attr blockedURL; attr statusCode; attr effectiveDirective;
  attr originalPolicy; attr sourceFile; attr lineNumber; attr columnNumber;
  attr disposition; attr sample.
- Tasks: CSP violation reporting (§5.18.4).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `FormData`
- Surface:
method append(name, value, filename); method delete(name); method get(name);
  method getAll(name); method has(name); method set(name, value, filename); entries/iterable.
- Tasks: Used by form submission and fetch bodies.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `URL`
- Surface:
constructor(url, base); attr href (get/set); attr origin; attr protocol; attr username;
  attr password; attr host; attr hostname; attr port; attr pathname; attr search; attr hash;
  attr searchParams; method toJSON; static createObjectURL/revokeObjectURL (blob: MVP).
- Tasks: Wraps §5.1 parser; searchParams is URLSearchParams.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `URLSearchParams`
- Surface:
constructor(init); attr size; method append(name, value); method delete(name);
  method get(name); method getAll(name); method has(name, value); method set(name, value);
  method sort(); entries/iterable; method forEach.
- Tasks: application/x-www-form-urlencoded semantics (§5.1).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Blob`
- Surface:
attr size; attr type; method slice(start, end, contentType);
  method stream (MVP: arrayBuffer); method arrayBuffer(); method text().
- Tasks: Backing store for File/fetch bodies.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `File` — inherits `Blob`
- Surface:
attr name; attr lastModified.
- Tasks: From input files and drag-drop.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `FileReader`
- Surface:
attr readyState; attr result; attr error; method readAsArrayBuffer(blob);
  method readAsText(blob, encoding); method readAsDataURL(blob); method abort().
- Tasks: Event-driven async reads on the document thread.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Request`
- Surface:
constructor(input, init); attr method; attr url; attr headers; attr destination;
  attr referrer; attr referrerPolicy; attr mode; attr credentials; attr cache;
  attr redirect; attr integrity; attr keepalive; attr signal; attr bodyUsed; method clone();
  method arrayBuffer/text/blob/json.
- Tasks: Fetch API client side (§5.15.5).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Response`
- Surface:
constructor(body, init); attr url; attr ok; attr status; attr statusText; attr headers;
  attr redirected; attr type; attr bodyUsed; static error(); static redirect(url, status);
  method clone(); method arrayBuffer(); method text(); method json(); method blob().
- Tasks: Fetch API server side.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Headers`
- Surface:
constructor(init); method append(name, value); method delete(name); method get(name);
  method getSetCookie(); method has(name); method set(name, value); entries/iterable;
  method forEach.
- Tasks: Case-insensitive; forbidden header names enforced.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `AbortController`
- Surface:
attr signal; method abort(reason).
- Tasks: Cancels fetches and timers hooked to the signal.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `AbortSignal` — inherits `EventTarget`
- Surface:
attr aborted; attr reason; attr onabort; static abort(reason); static timeout(ms);
  method throwIfAborted().
- Tasks: Wired to the loader/timer cancellation tokens.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `WebSocket`
- Surface:
constructor(url, protocols); attr url; attr readyState; attr bufferedAmount;
  attr extensions; attr protocol; attr binaryType; method send(data);
  method close(code, reason); events open/message/error/close.
- Tasks: RFC 6455 handshake + framing (WBS §6.9); no subprotocol negotiation beyond echo.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `MessageChannel`
- Surface:
attr port1; attr port2.
- Tasks: Entangled ports for structured-clone messaging.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `MessagePort` — inherits `EventTarget`
- Surface:
method postMessage(message, transfer); method start(); method close();
  events message/messageerror.
- Tasks: MVP: same-document entangled pair only.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `DOMParser`
- Surface:
method parseFromString(str, type) -> Document.
- Tasks: text/html path reuses the full parser (§5.5); XML path rejects on well-formedness errors.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `XMLSerializer`
- Surface:
method serializeToString(node).
- Tasks: Escaping table per §5.6.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLInputElement` — inherits `HTMLElement`
- Surface:
attr type (get/set); attr accept; attr alt; attr autocomplete; attr autofocus;
  attr checked (get/set); attr defaultChecked; attr form; attr formAction; attr formEnctype;
  attr formMethod; attr formNoValidate; attr formTarget; attr files; attr height/width;
  attr list; attr max/min; attr maxLength/minLength; attr multiple; attr name; attr pattern;
  attr placeholder; attr readOnly; attr required; attr size; attr src; attr step;
  attr value (get/set, value-modes per type); attr defaultValue; attr willValidate;
  attr validity (ValidityState fields); attr validationMessage; attr labels;
  method stepUp/stepDown(n); method select(); method setRangeText(rep, start, end, mode);
  method setSelectionRange(start, end, dir); attr selectionStart/End/Direction;
  method checkValidity(); method reportValidity(); method setCustomValidity(msg).
- Tasks: The workhorse control: text/checkbox/radio/button/submit/reset/file/hidden/password/range/number/email/url/search/tel/date (MVP subset); activation + constraint validation (§5.6).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLTextAreaElement` — inherits `HTMLElement`
- Surface:
attr cols; attr rows; attr dirName; attr disabled; attr form; attr maxLength/minLength;
  attr name; attr placeholder; attr readOnly; attr required; attr wrap;
  attr value (get/set); attr defaultValue; attr textLength; selection attrs + methods;
  validation methods.
- Tasks: Editable multi-line control; input events on mutation.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLSelectElement` — inherits `HTMLElement`
- Surface:
attr multiple; attr name; attr required; attr size; attr selectedIndex (get/set);
  attr value; attr length; attr options (live); attr selectedOptions; attr form;
  method add(el, before); method remove(index); method remove(); method item(i);
  method namedItem(name); validation methods.
- Tasks: Option list model; change events.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLOptionElement` — inherits `HTMLElement`
- Surface:
attr disabled; attr form; attr label; attr defaultSelected; attr selected (get/set);
  attr value; attr text; attr index.
- Tasks: Selectedness rules.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLButtonElement` — inherits `HTMLElement`
- Surface:
attr type; attr value; attr name; attr form;
  attr formAction/Enctype/Method/NoValidate/Target; attr disabled; validation methods.
- Tasks: Activation: submit/reset/button (§5.6).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLFormElement` — inherits `HTMLElement`
- Surface:
attr acceptCharset; attr action; attr autocomplete; attr enctype; attr encoding;
  attr method; attr name; attr noValidate; attr target; attr rel; attr elements (live);
  attr length; method submit(); method requestSubmit(submitter); method reset();
  method checkValidity(); method reportValidity(); events submit/reset.
- Tasks: Submission algorithm (GET url-encoding / POST body), constraint validation pass.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLLabelElement` — inherits `HTMLElement`
- Surface:
attr form; attr htmlFor (get/set); attr control.
- Tasks: Activation forwarding to the labeled control.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLFieldSetElement` — inherits `HTMLElement`
- Surface:
attr form; attr name; attr disabled; attr type; attr elements.
- Tasks: Disabled propagation to descendants.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLAnchorElement` — inherits `HTMLElement`
- Surface:
attr href (reflected); attr target; attr download; attr rel; attr relList; attr hreflang;
  attr type; attr referrerPolicy; attr text; protocol/host/pathname etc URL reflectors.
- Tasks: Activation navigation (§5.19.2).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLImageElement` — inherits `HTMLElement`
- Surface:
attr alt; attr src (get/set); attr srcset; attr sizes; attr crossOrigin; attr useMap;
  attr isMap; attr width/height (get/set); attr naturalWidth; attr naturalHeight;
  attr complete; attr currentSrc; attr decoding; attr loading; attr referrerPolicy;
  attr fetchPriority.
- Tasks: decode pipeline wiring (§5.12).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLCanvasElement` — inherits `HTMLElement`
- Surface:
attr width/height (get/set); method getContext(type, opts); method toDataURL(type);
  method toBlob(cb, type).
- Tasks: 2D context only (§5.6 ctx below).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `CanvasRenderingContext2D`
- Surface:
attr canvas; save/restore; scale/rotate/translate/transform/setTransform/resetTransform;
  globalAlpha; globalCompositeOperation; fillStyle/strokeStyle (colors, gradients MVP);
  lineWidth; lineCap; lineJoin; miterLimit; lineDash attrs + setLineDash/getLineDash;
  shadow attrs; clearRect/fillRect/strokeRect;
  beginPath/closePath/moveTo/lineTo/quadraticCurveTo/bezierCurveTo/arc/arcTo/rect/ellipse/roundRect;
  fill/stroke/clip (path + Path2D); isPointInPath; drawImage (src rect variants);
  createLinearGradient/createRadialGradient (addColorStop);
  getImageData/putImageData/createImageData; measureText -> TextMetrics;
  fillText/strokeText (fonts: §5.11 path); direction attr.
- Tasks: Backed by the same rasterizer (§5.16); state stack with fill/stroke paint objects.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `ImageData`
- Surface:
attr width; attr height; attr data (Uint8ClampedArray).
- Tasks: Pixel format RGBA8.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `TextMetrics`
- Surface:
attr width; attr actualBoundingBox* family; attr fontBoundingBox* family.
- Tasks: From the shaper metrics.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLIFrameElement` — inherits `HTMLElement`
- Surface:
attr src; attr srcdoc; attr name; attr sandbox (DOMTokenList); attr allow;
  attr allowFullscreen; attr loading; attr width/height;
  contentDocument/contentWindow (SOP-checked).
- Tasks: Nested browsing context placeholder (§1.3).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLMediaElement` — inherits `HTMLElement`
- Surface:
attr src; attr currentSrc; attr networkState; attr readyState; attr paused; attr duration;
  attr currentTime (get/set); attr volume; attr muted; attr playbackRate; method load();
  method play() -> promise; method pause();
  events loadstart/loadedmetadata/canplay/play/pause/ended/error.
- Tasks: MVP: state machine + events fire, no codec decode (§1.4).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLScriptElement` — inherits `HTMLElement`
- Surface:
attr src; attr type; attr noModule; attr async; attr defer; attr crossOrigin; attr text;
  attr integrity; attr referrerPolicy; attr fetchPriority.
- Tasks: Execution queueing per §5.5.6.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLLinkElement` — inherits `HTMLElement`
- Surface:
attr href; attr rel; attr relList; attr media; attr hreflang; attr type; attr as;
  attr crossOrigin; attr referrerPolicy; attr disabled; attr sheet.
- Tasks: Stylesheet loading (§5.7), favicon, preloads.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLStyleElement` — inherits `HTMLElement`
- Surface:
attr media; attr type; attr disabled; attr sheet.
- Tasks: Inline sheet ownership.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLMetaElement` — inherits `HTMLElement`
- Surface:
attr name; attr content; attr httpEquiv; attr charset.
- Tasks: Encoding + viewport MVP.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLTableElement` — inherits `HTMLElement`
- Surface:
attr caption; attr tHead; attr tFoot; attr rows (live); attr tBodies (live);
  method createCaption/deleteCaption/createTHead/deleteTHead/createTFoot/deleteTFoot;
  method insertRow(index); method deleteRow(index).
- Tasks: Table model helpers.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLTableRowElement` — inherits `HTMLElement`
- Surface:
attr rowIndex; attr sectionRowIndex; attr cells (live); method insertCell(index);
  method deleteCell(index).
- Tasks: Row model.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLTableCellElement` — inherits `HTMLElement`
- Surface:
attr colSpan; attr rowSpan; attr headers; attr cellIndex; attr scope (th); attr abbr (th).
- Tasks: Span validation for grid layout.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLDialogElement` — inherits `HTMLElement`
- Surface:
attr open (get/set); attr returnValue; method show(); method showModal();
  method close(returnValue); event cancel.
- Tasks: Top-layer + focus trapping + :modal.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLDetailsElement` — inherits `HTMLElement`
- Surface:
attr open (get/set); toggle event.
- Tasks: Name-group accordion behavior.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLProgressElement` — inherits `HTMLElement`
- Surface:
attr value/max (get/set); attr position.
- Tasks: Indeterminate state.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLMeterElement` — inherits `HTMLElement`
- Surface:
attr value/min/max/low/high/optimum (get/set).
- Tasks: Gauge regions.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLTemplateElement` — inherits `HTMLElement`
- Surface:
attr content (DocumentFragment).
- Tasks: Parser-invisible contents (§5.5).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLSlotElement` — inherits `HTMLElement`
- Surface:
attr name; method assignedNodes(options); method assignedElements(options).
- Tasks: Slot assignment.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLOutputElement` — inherits `HTMLElement`
- Surface:
attr htmlFor; attr form; attr name; attr value (get/set); attr defaultValue.
- Tasks: form-associated readout.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Crypto`
- Surface:
method getRandomValues(array); attr subtle (unsupported: throws cleanly).
- Tasks: CSPRNG from the platform layer only.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Console`
- Surface:
method log/debug/info/warn/error(table); method assert(cond, ...args); method clear();
  method count(label); method countReset; method group/groupEnd/groupCollapsed;
  method time(label); method timeEnd/timeLog; method trace(...args); method table(data).
- Tasks: Console Standard formatting to the DevTools buffer (§5.15.4).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

### §6.7 JavaScript builtin checklist

Every builtin is implemented with the interpreter's primitives (§5.14)
and verified against the adopted test262 subset (§8.6). `M9` marks the
baseline inventory; anything beyond lands in M13 polish unless a milestone
section pulls it earlier.

#### `globalThis functions`
- Host additions on the same global: setTimeout/setInterval/clear*/queueMicrotask/structuredClone/atob/btoa/fetch/console/performance
- Surface:
globalThis; undefined; NaN; Infinity; eval(x); isFinite(x); isNaN(x); parseFloat(x);
  parseInt(x, radix); encodeURIComponent(s); decodeURIComponent(s); encodeURI(s);
  decodeURI(s); String/Number/Boolean/BigInt/Symbol/Object/Array constructors;
  ArrayBuffer/SharedArrayBuffer; DataView; TypedArray family;
  Map/Set/WeakMap/WeakSet/WeakRef/FinalizationRegistry; Promise; Proxy; Reflect; Date;
  RegExp; Error family; JSON; Math; Atomics; Intl; Function; AggregateError;
  parse module keys (import, import.meta in modules).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Object`
- Property descriptors: value/writable/get/set/enumerable/configurable; accessor vs data slots per spec
- Surface:
static assign(target, ...src); static create(proto, props);
  static defineProperty(obj, key, desc); static defineProperties; static entries;
  static freeze; static isFrozen; static fromEntries; static getOwnPropertyDescriptor(s);
  static getOwnPropertyNames; static getOwnPropertySymbols;
  static getPrototypeOf/setPrototypeOf; static hasOwn; static is; static isExtensible;
  static isSealed; static keys; static preventExtensions; static seal; static values;
  proto constructor; proto hasOwnProperty; proto isPrototypeOf; proto propertyIsEnumerable;
  proto toLocaleString; proto toString ([object Tag]); proto valueOf;
  proto __proto__ accessor;
  proto __defineGetter__/__defineSetter__/__lookupGetter__/__lookupSetter__ (legacy).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Function`
- [[Call]]/[[Construct]] separation; new.target; default-arg/rest/destructuring parameter forms
- Surface:
proto length; proto name; proto prototype; proto apply(thisArg, args);
  proto bind(thisArg, ...args); proto call(thisArg, ...args);
  proto toString (source slice per §5.14.7); proto arguments/caller poisoned in strict;
  static (no).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Boolean`
- Wrapper objects with sloppy-mode coercion
- Surface:
constructor(value); proto toString; proto valueOf.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Symbol`
- Well-known symbol dispatch in the interpreter (§5.14.2)
- Surface:
static for(key); static keyFor(sym); static asyncIterator; static hasInstance;
  static isConcatSpreadable; static iterator; static match; static matchAll; static replace;
  static search; static species; static split; static toPrimitive; static toStringTag;
  static dispose/asyncDispose; proto description; proto toString; proto valueOf.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Error`
- options.cause; stack formatting with source spans (§5.13)
- Surface:
constructor(message, options); proto name; proto message;
  proto stack (own, captureStackTrace semantics); static isError (new);
  subclasses EvalError/RangeError/ReferenceError/SyntaxError/TypeError/URIError/AggregateError(errors, message).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Number`
- Canonical numeric string forms per spec §6.1.6.1
- Surface:
EPSILON; MAX_SAFE_INTEGER; MIN_SAFE_INTEGER; MAX_VALUE; MIN_VALUE; NEGATIVE_INFINITY;
  POSITIVE_INFINITY; NaN; static isFinite; static isInteger; static isNaN;
  static isSafeInteger; static parseFloat; static parseInt; constructor(value);
  proto toExponential(digits); proto toFixed(digits); proto toLocaleString;
  proto toPrecision(precision); proto toString(radix); proto valueOf.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `BigInt`
- Mixed BigInt/Number arithmetic throws per spec
- Surface:
constructor(value); static asIntN(bits, v); static asUintN(bits, v); proto toString(radix);
  proto valueOf; proto toLocaleString.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Math`
- All semantics exactly per spec (rounding modes matter: round-half-up, floor toward -inf)
- Surface:
E; LN10; LN2; LOG10E; LOG2E; PI; SQRT1_2; SQRT2; abs; acos; acosh; asin; asinh; atan; atan2;
  atanh; cbrt; ceil; clz32; cos; cosh; exp; expm1; floor; fround; hypot; imul; log; log10;
  log1p; log2; max; min; pow; random (seeded, test-injected); round; sign; sin; sinh; sqrt;
  tan; tanh; trunc.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Date`
- Local timezone from the platform; ISO parse subset per spec; invalid-date NaN semantics
- Surface:
constructor variants (no-arg, ms, string ISO, y/m/d/h/m/s/ms); static now();
  static parse(s); static UTC(...);
  proto getDate/getDay/getFullYear/getHours/getMilliseconds/getMinutes/getMonth/getSeconds/getTime/getTimezoneOffset/getDate UTC variants;
  proto setDate/setFullYear/setHours/setMilliseconds/setMinutes/setMonth/setSeconds/setTime/setMilliseconds + UTC variants;
  proto toISOString; proto toJSON;
  proto toString/toDateString/toTimeString/toUTCString/toISOString/toLocaleString;
  proto valueOf; proto [Symbol.toPrimitive].
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `String`
- UTF-16 string model (§5.14.1); well-formedness on USV boundaries in host crossings
- Surface:
static fromCharCode(...units); static fromCodePoint(...points); static raw(template);
  constructor(value); proto length (UTF-16 units); proto [index]; proto at(i);
  proto charAt(i); proto charCodeAt(i); proto codePointAt(i); proto concat;
  proto endsWith(s, end); proto includes(s, pos); proto indexOf(s, from);
  proto lastIndexOf(s, from); proto localeCompare; proto match(re); proto matchAll(re);
  proto normalize(form); proto padEnd(len, fill); proto padStart(len, fill);
  proto repeat(n); proto replace(search, repl); proto replaceAll(search, repl);
  proto search(re); proto slice(start, end); proto split(sep, limit);
  proto startsWith(s, pos); proto substring(a, b); proto substr (legacy annex);
  proto toLowerCase/toUpperCase; proto toLocaleLowerCase/UpperCase; proto trim;
  proto trimStart/trimEnd; proto [Symbol.iterator]; proto toString/valueOf.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `RegExp`
- Own regex engine: backtracking with the ES semantics (§6.7 WBS); no lookbehind initially — ADR when added
- Surface:
constructor(pattern, flags); static (species); proto source; proto flags; proto global;
  proto ignoreCase; proto multiline; proto dotAll; proto unicode; proto unicodeSets;
  proto sticky; proto hasIndices; proto lastIndex (get/set); proto exec(s); proto test(s);
  proto toString; proto [Symbol.match/matchAll/replace/replaceAll/search/split].
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Array`
- Hole semantics (empty slots) in iteration; species-driven subclass results
- Surface:
static isArray(v); static from(src, mapFn, thisArg); static of(...items); static fromAsync;
  constructor(len or items); proto length (get/set with truncation rules); proto at(i);
  proto concat; proto copyWithin(target, start, end); proto entries;
  proto every(cb, thisArg); proto fill(value, start, end); proto filter(cb);
  proto find(cb)/findLast; proto findIndex/findLastIndex; proto flat(depth);
  proto flatMap(cb); proto forEach(cb); proto includes(v, from); proto indexOf(v, from);
  proto join(sep); proto keys; proto lastIndexOf(v, from); proto map(cb); proto pop;
  proto push(...items); proto reduce(cb, init)/reduceRight; proto reverse; proto shift;
  proto slice(start, end); proto some(cb); proto sort(cmp) (stable per spec);
  proto splice(start, deleteCount, ...items); proto toLocaleString;
  proto toReversed/toSorted/toSpliced/with (immutable set); proto unshift(...items);
  proto values; proto [Symbol.iterator]; proto [Symbol.species].
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `%TypedArray%`
- Bounds checks; canonical-numeric-index strings; detach semantics (no detach in MVP — never transferred)
- Surface:
static from/of; proto buffer/byteLength/byteOffset/length; proto set(arr, offset);
  proto subarray(start, end); proto fill; proto copyWithin;
  proto indexOf/includes/lastIndexOf; proto join; proto reverse;
  proto sort(cmp) (numeric default); proto slice; proto entries/keys/values; proto at;
  proto find/findIndex family; proto every/some/forEach/map/filter/reduce family;
  static Int8Array/Uint8Array/Uint8ClampedArray/Int16Array/Uint16Array/Int32Array/Uint32Array/Float32Array/Float64Array/BigInt64Array/BigUint64Array.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `ArrayBuffer`
- SharedArrayBuffer: same shape, isShared flag
- Surface:
constructor(byteLength, opts); proto byteLength; proto slice(start, end); static isView(v).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `DataView`
- Alignment-free typed access; endianness argument default false (big)
- Surface:
constructor(buffer, offset, length); proto buffer/byteLength/byteOffset;
  proto getBigInt64/BigUint64/getFloat32/Float64/getInt8/16/32/getUint8/16/32(offset, littleEndian);
  proto set* family (offset, value, littleEndian).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Atomics`
- MVP: non-shared buffers allowed where spec permits; wait throws on non-shared
- Surface:
static add/and/compareExchange/exchange/load/or/store/sub/wait/notify/xor(typedArray, index, ...).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Map`
- Insertion order; SameValueZero keys; hash-consed key table
- Surface:
constructor(iterable); attr size; proto clear(); proto delete(key); proto entries;
  proto forEach(cb, thisArg); proto get(key); proto has(key); proto keys;
  proto set(key, value); proto values; proto [Symbol.iterator]; proto [Symbol.toStringTag].
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Set`
- SameValueZero membership
- Surface:
constructor(iterable); attr size; proto add(value); proto clear(); proto delete(value);
  proto entries; proto forEach; proto has(value); proto keys/values;
  proto union/intersection/difference/symmetricDifference/isSubsetOf/isSupersetOf/isDisjointFrom (ES2025 set methods);
  proto [Symbol.iterator].
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `WeakMap`
- Keys are objects/symbols registered as weak refs in the GC
- Surface:
constructor; proto delete(key); proto get(key); proto has(key); proto set(key, value).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `WeakSet`
- Weak membership
- Surface:
constructor; proto add(value); proto delete(value); proto has(value).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `WeakRef`
- Sweep-time liveness
- Surface:
constructor(target); proto deref().
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `FinalizationRegistry`
- Callbacks queued as tasks, never during GC (§5.14.6)
- Surface:
constructor(cleanup); proto register(target, held, token); proto unregister(token);
  proto cleanupSome().
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Promise`
- Reaction jobs drain at the microtask checkpoint (§5.15.3); unhandled-rejection tracking
- Surface:
static all(iterable); static allSettled; static any; static race; static resolve(v);
  static reject(r); static try(fn); static withResolvers; constructor(executor);
  proto then(onFul, onRej); proto catch(onRej); proto finally(onSettled).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Iterator helpers`
- Lazy iterator adapters; manual protocol implementation
- Surface:
proto map/filter/take/drop/flatMap/reduce/toArray/toAsync; proto [Symbol.iterator];
  %IteratorPrototype% chain.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Generator`
- Frame-suspend resume state machine (§5.14.4); async generators add the queue model
- Surface:
%GeneratorPrototype%: proto next(v); proto return(v); proto throw(e);
  proto [Symbol.iterator].
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Proxy`
- Invariant enforcement per spec; revocable via Proxy.revocable
- Surface:
constructor(target, handler);
  13 traps invoked in spec order: getPrototypeOf/setPrototypeOf/isExtensible/preventExtensions/getOwnPropertyDescriptor/defineProperty/has/get/set/deleteOwnProperty/ownKeys/apply/construct.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Reflect`
- Mirror of the internal methods; used by Proxy default behaviors
- Surface:
static apply(f, thisArg, args); static construct(f, args, newTarget); static defineProperty;
  static deleteProperty; static get(target, key, receiver); static getOwnPropertyDescriptor;
  static getPrototypeOf; static has; static isExtensible; static ownKeys;
  static preventExtensions; static set(target, key, value, receiver); static setPrototypeOf.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `JSON`
- Own parser/serializer; exact number formatting (shortest round-trip double printing); well-formed stringify (lone surrogates escaped)
- Surface:
static parse(text, reviver); static stringify(value, replacer, space); static rawJSON (new);
  static isRawJSON.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Intl`
- Graceful stub per §5.14.7; ADR to extend
- Surface:
static Intl object with Collator/DateTimeFormat/NumberFormat/PluralRules/Segmenter constructors present but constructing throws NotSupportedError with a clear message.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `console`
- §5.15.4 formatting into the DevTools ring buffer
- Surface:
log/debug/info/warn/error; assert; clear; count/countReset; group/groupEnd/groupCollapsed;
  time/timeEnd/timeLog; trace; table.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `URI functions`
- Escape/unescape (annex B) included for compatibility
- Surface:
encodeURI/encodeURIComponent/decodeURI/decodeURIComponent per RFC 3986 tables.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Host timers`
- Clamping rules per HTML §8.1.4.2; token invalidation on navigation
- Surface:
setTimeout(fn, delay, ...args); setInterval; clearTimeout; clearInterval.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Host misc`
- Wired per §5.15; structured clone supports the plain-data subset + ArrayBuffer/Map/Set/Date/RegExp/Error/Blob
- Surface:
queueMicrotask(fn); structuredClone(value, opts); atob(s); btoa(s); reportError(err);
  fetch(input, init).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Module records`
- Module map with cycle handling (§5.13)
- Surface:
import declarations; export forms (named/default/asterisk/string); import.meta;
  dynamic import(); top-level await.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

### §6.8 DOM event catalog

For each event: dispatch path (§5.6), the interface binding (§6.6), the
default action (if any) and its `preventDefault` behavior, and an
integration test with the scripted input driver (§8.4).

- [ ] `load` — bubbles: N; cancelable: N; `Event` — fired when: Resource/document finished loading (window, img, script, link, media).
- [ ] `DOMContentLoaded` — bubbles: Y; cancelable: N; `Event` — fired when: HTML fully parsed and deferred scripts ran.
- [ ] `readystatechange` — bubbles: Y; cancelable: N; `Event` — fired when: document.readyState changed (loading/interactive/complete).
- [ ] `beforeunload` — bubbles: N; cancelable: Y; `BeforeUnloadEvent (MVP: Event)` — fired when: Window about to unload; prompts are shell policy.
- [ ] `unload` — bubbles: N; cancelable: N; `Event` — fired when: Document unloading (legacy support).
- [ ] `pagehide/pageshow` — bubbles: N; cancelable: N; `PageTransitionEvent` — fired when: Session history traversal in/out.
- [ ] `error` — bubbles: N/N; cancelable: N; `ErrorEvent or Event` — fired when: Script error (window, bubbles N) or resource fetch error (element, bubbles Y).
- [ ] `abort` — bubbles: Y; cancelable: N; `Event` — fired when: Fetch aborted before completion (media elements).
- [ ] `hashchange` — bubbles: Y; cancelable: N; `HashChangeEvent` — fired when: URL fragment changed.
- [ ] `popstate` — bubbles: Y; cancelable: N; `PopStateEvent` — fired when: Session history entry traversed.
- [ ] `click` — bubbles: Y; cancelable: Y; `MouseEvent` — fired when: Primary activation on an element (synthesized from pointer events).
- [ ] `dblclick` — bubbles: Y; cancelable: Y; `MouseEvent` — fired when: Two clicks within platform threshold.
- [ ] `contextmenu` — bubbles: Y; cancelable: Y; `MouseEvent` — fired when: Secondary button / context menu key.
- [ ] `mousedown/mouseup` — bubbles: Y; cancelable: Y; `MouseEvent` — fired when: Button press/release.
- [ ] `mousemove` — bubbles: Y; cancelable: Y; `MouseEvent` — fired when: Pointer moved over the document.
- [ ] `mouseover/mouseout` — bubbles: Y; cancelable: Y; `MouseEvent` — fired when: Pointer entered/left an element's hit area (bubbling).
- [ ] `mouseenter/mouseleave` — bubbles: N; cancelable: N; `MouseEvent` — fired when: Non-bubbling enter/leave on the element itself.
- [ ] `wheel` — bubbles: Y; cancelable: Y; `WheelEvent` — fired when: Scroll wheel/delta input; preventDefault stops scrolling.
- [ ] `pointerdown/pointerup` — bubbles: Y; cancelable: Y; `PointerEvent` — fired when: Pointer press/release (pointerId model).
- [ ] `pointermove` — bubbles: Y; cancelable: Y; `PointerEvent` — fired when: Pointer moved.
- [ ] `pointerover/pointerout` — bubbles: Y; cancelable: Y; `PointerEvent` — fired when: Bubbling pointer enter/leave.
- [ ] `pointerenter/pointerleave` — bubbles: N; cancelable: N; `PointerEvent` — fired when: Non-bubbling enter/leave.
- [ ] `pointercancel` — bubbles: Y; cancelable: N; `PointerEvent` — fired when: Pointer interaction taken over (touch scroll).
- [ ] `gotpointercapture/lostpointercapture` — bubbles: Y; cancelable: N; `PointerEvent` — fired when: setPointerCapture transitions (MVP: implicit capture on touch).
- [ ] `keydown` — bubbles: Y; cancelable: Y; `KeyboardEvent` — fired when: Key pressed; preventDefault stops text input and default actions.
- [ ] `keyup` — bubbles: Y; cancelable: Y; `KeyboardEvent` — fired when: Key released.
- [ ] `keypress` — bubbles: Y; cancelable: Y; `KeyboardEvent` — fired when: Legacy character-producing key (sourced from keydown).
- [ ] `beforeinput` — bubbles: Y; cancelable: Y; `InputEvent` — fired when: Editable content about to change; preventDefault blocks it.
- [ ] `input` — bubbles: Y; cancelable: N; `InputEvent` — fired when: Editable content changed (text areas, inputs, select).
- [ ] `change` — bubbles: Y; cancelable: N; `Event` — fired when: Commit of a new value (checkbox, radio, select, file, blur-commit of text).
- [ ] `compositionstart/compositionupdate/compositionend` — bubbles: Y; cancelable: N; `CompositionEvent` — fired when: IME session lifecycle.
- [ ] `focus` — bubbles: N; cancelable: N; `FocusEvent` — fired when: Element received focus.
- [ ] `blur` — bubbles: N; cancelable: N; `FocusEvent` — fired when: Element lost focus.
- [ ] `focusin/focusout` — bubbles: Y; cancelable: N; `FocusEvent` — fired when: Bubbling focus transitions.
- [ ] `submit` — bubbles: Y; cancelable: Y; `SubmitEvent` — fired when: Form submission requested (preventDefault blocks navigation).
- [ ] `reset` — bubbles: Y; cancelable: Y; `Event` — fired when: Form reset requested.
- [ ] `select` — bubbles: Y; cancelable: N; `Event` — fired when: Text selection changed inside an editable.
- [ ] `invalid` — bubbles: Y; cancelable: N; `Event` — fired when: Constraint validation failed (reportValidity path).
- [ ] `search` — bubbles: Y; cancelable: N; `Event` — fired when: type=search Enter (legacy).
- [ ] `dragstart/dragend` — bubbles: Y; cancelable: Y/N; `DragEvent` — fired when: Drag session lifecycle (simple drag-store MVP).
- [ ] `dragenter/dragleave/dragover/drop` — bubbles: Y; cancelable: Y for over/drop; `DragEvent` — fired when: Drag over targets; drop needs preventDefault(over)+drop.
- [ ] `scroll` — bubbles: Y/N; cancelable: N; `Event` — fired when: Element scrolled (bubbles: document) — also fires rAF-aligned on programmatic scrolls.
- [ ] `resize` — bubbles: N; cancelable: N; `Event` — fired when: Viewport (window) resized.
- [ ] `canplaythrough/loadedmetadata/loadeddata` — bubbles: N; cancelable: N; `Event` — fired when: Media element state advances (stub per §1.4).
- [ ] `play/pause/ended` — bubbles: N; cancelable: N; `Event` — fired when: Media element state (stub).
- [ ] `transitionrun/transitionstart/transitionend/transitioncancel` — bubbles: Y; cancelable: N; `TransitionEvent` — fired when: CSS transition lifecycle.
- [ ] `animationstart/animationiteration/animationend/animationcancel` — bubbles: Y; cancelable: N; `AnimationEvent` — fired when: CSS animation lifecycle.
- [ ] `message` — bubbles: N; cancelable: N; `MessageEvent` — fired when: postMessage/MessagePort delivery.
- [ ] `messageerror` — bubbles: N; cancelable: N; `MessageEvent` — fired when: Undeserializable message (MVP: rare).
- [ ] `online/offline` — bubbles: N; cancelable: N; `Event` — fired when: Network connectivity changed.
- [ ] `storage` — bubbles: N; cancelable: N; `StorageEvent` — fired when: localStorage changed in another same-origin tab.
- [ ] `visibilitychange` — bubbles: Y; cancelable: N; `Event` — fired when: Tab visibility changed (shell hook).
- [ ] `fullscreenchange` — bubbles: Y; cancelable: N; `Event` — fired when: Never fires in this engine (non-goal §1.4); binding exists.
- [ ] `copy/cut/paste` — bubbles: Y; cancelable: Y; `ClipboardEvent` — fired when: Clipboard operations (text-only MVP).
- [ ] `securitypolicyviolation` — bubbles: Y; cancelable: N; `SecurityPolicyViolationEvent` — fired when: CSP violation detected (§5.18.4).
- [ ] `unhandledrejection` — bubbles: Y; cancelable: Y; `PromiseRejectionEvent` — fired when: Promise rejected with no handler at checkpoint.
- [ ] `rejectionhandled` — bubbles: Y; cancelable: N; `PromiseRejectionEvent` — fired when: Late handler attached to a reported rejection.
- [ ] `toggle` — bubbles: N; cancelable: N; `Event` — fired when: details open state changed.

### §6.9 Network and protocol checklist

Loader-level behaviors (§5.1–§5.3); each item carries a mock-server test.

- [ ] Scheme handling: `http`, `https`, `file`, `data`, `about:blank`, `about:srcdoc`; unknown scheme → error page.
- [ ] URL normalization before fetch; fragment stripped on the wire; base resolution for every subresource.
- [ ] GET pipeline end-to-end: DNS → connect → TLS → request → response, all phases cancelable (§5.2.5).
- [ ] Redirect chain semantics (301/302/303/307/308) including body dropping and method preservation.
- [ ] Keep-alive pooling with idle expiry; connection error → one clean retry on a fresh connection.
- [ ] Chunked body decoding; content-length framing; until-close fallback with a console note.
- [ ] gzip/deflate content decoding (owned inflate, or the approved crate until it lands).
- [ ] Conditional revalidation flow (ETag + Last-Modified) against the disk cache.
- [ ] Vary-keyed cache entries; no-store honored; stale-while-revalidate treated as stale (documented).
- [ ] Cookie jar read/write on send/response with SameSite + Secure rules (§5.17.1).
- [ ] CORS simple request, preflight round-trip, credentialed request, wildcard rules (§5.18.3).
- [ ] Referrer generation per policy, downgrade stripping.
- [ ] HSTS upgrade-before-connect and policy expiry (§5.3).
- [ ] Content sniffing table for images and top-level text/html/text/plain ambiguity; nosniff honored.
- [ ] Timeout matrix per phase; abort mid-body surfaces a clean network error to the pipeline.
- [ ] HTTP/1.1 protocol violations mapped to NetError::Protocol with the offending bytes logged (debug).
- [ ] Mock-server harness: per-test server with scripted responses, delays, and truncations (§8.2).
- [ ] Byte-exactness tests: emitted request lines/headers match the recorded golden bytes.
- [ ] WebSocket handshake + frame codecs (client side) with the event surface of §6.6.
- [ ] Non-GET methods (POST/PUT/DELETE/HEAD/OPTIONS) for fetch with body framing rules.
- [ ] DNS resolution with TTL caching and the hosts-file override for tests.
- [ ] Connection coalescing guard: two concurrent fetches to one origin use the pool, not two sockets (unless over the 6-connection cap).
- [ ] Request body streaming from fetch (chunked upload) and its backpressure story.
- [ ] Response body error injection tests: truncation, invalid chunk size, premature close.
- [ ] Proxy configuration (env vars) honored at the loader layer with tests via a local proxy harness.
- [ ] Integrity metadata (SRI) verification for scripts/styles with failure = network error.
- [ ] DevTools network event emission for every request lifecycle transition (§5.19.5).

### §6.10 Storage and persistence checklist

Persistence surface (§5.17); every item includes a crash-recovery or
corruption-recovery test.

- [ ] Cookie jar: RFC 6265bis §5.1–5.6 algorithms, host-only vs domain cookies, path-match, sort order.
- [ ] Cookie prefixes `__Secure-`/`__Host-` enforced; Secure-only delivery; SameSite default Lax.
- [ ] Cookie partitioning by top-level site; public-suffix list file loaded and versioned.
- [ ] document.cookie serialization (one string, semicolon-joined) honoring HttpOnly invisibility.
- [ ] localStorage per-origin file: write-temp-rename, version byte, CRC per record, load-recovery test.
- [ ] sessionStorage per-tab lifecycle: cleared on tab close, not shared across tabs, no storage events.
- [ ] Storage events delivered to other same-origin tabs with old/new values.
- [ ] Quota enforcement (5 MB UTF-16 units) with QuotaExceededError and a console message.
- [ ] HTTP disk cache: entry format (headers + body + metadata + version), sharded directories.
- [ ] Cache freshness math: Age, heuristic freshness, must-revalidate, no-cache revalidation.
- [ ] LRU eviction on byte budget with pinning for in-flight resources; eviction test with tiny budget.
- [ ] HSTS store persistence and expiry; security-state versioning (§4.8).
- [ ] Profile layout `~/.aurora/<profile>/` per §5.17; `--profile` flag; temp profiles in tests.
- [ ] Clear-browsing-data (cookies, storage, cache) with in-flight navigation safety.
- [ ] Cookie jar eviction: expired-cookie sweep on load and on a periodic timer.
- [ ] Storage keying includes the origin's port and scheme (tuple origin, §5.18.1).
- [ ] Disk-space accounting: stores report their footprint to DevTools (§5.19.5).
- [ ] Cache checksum-per-record verified on read; corrupt entry evicted, store survives.
- [ ] sessionStorage survives tab reloads but not tab close (test both).
- [ ] document.cookie set/delete round-trip through the jar with path-scoped deletion.
- [ ] Preferences file (shell) versioned, atomically written, hot-reloaded on the settings page.

### §6.11 Keyboard and input map

Input plumbing checks (§5.19.3): platform events → engine input messages →
DOM events with correct `key`/`code`/modifiers. The full key table is
Appendix F; here, the wiring items:

- [ ] Platform key events mapped through the Appendix F table to KeyboardEvent key/code values.
- [ ] Modifier liveness: ctrl/shift/alt/meta state correct across focus changes and getModifierState.
- [ ] Text input funnel: keydown (default-check) → composition (if IME) → beforeinput → input.
- [ ] Focus navigation via Tab/Shift+Tab over the sequential focus navigation order.
- [ ] Scroll keys (Space, arrows, PageUp/Down, Home/End) hit the focused scroller or the document.
- [ ] Shortcut dispatch order: shell shortcuts first, then page keydown handlers (§5.19.1).
- [ ] Mouse: hit-test → enter/leave pairing → down/up → click synthesis with button/bitmask rules.
- [ ] Wheel: delta normalization, scroll chaining from innermost scroller outward, listener default action.
- [ ] Pointer: pointerId assignment, implicit capture for touch, mouse-event synthesis from pointers.
- [ ] IME: composition event sequence with correct data/isComposing through the editing funnel.
- [ ] Drag-and-drop of files onto the window routed to the drop event or navigation (§5.19.3).
- [ ] Cursor and tooltip updates from the hit-test result rendered by the shell.
- [ ] Repeat-key rate: keydown repeat timing surfaces per platform conventions.
- [ ] Alt-key menu acceleration does not leak into page key handlers when the shell consumes it.
- [ ] Zoom (Ctrl+wheel) changes the page zoom factor and re-runs layout, not a bitmap scale.
- [ ] Text selection drag: mousemove selection updates with shift-extension and double/triple-click word/line selection.
- [ ] Focus follows click on editable areas with caret placement at the click point.

### §6.13 Milestone → WBS mapping

Which WBS sections each milestone claims. An item may be *introduced*
in one milestone and *completed* in another — the ledger (§6.12) records
completion; this table records planning intent. `→` marks completion of
work introduced earlier.

| Milestone | Claims (introduce → complete) |
|---|---|
| M0 Bootstrap | — (infrastructure only; no WBS items) |
| M1 Fetch | §6.9 network items 1–7; Appendices C/D tables land |
| M2 HTML→DOM | §6.2 all parse/tree tasks; §6.6 DOM-core interfaces; §6.5 selector parsing only |
| M3 Style | §6.3 properties flagged [M3]; §6.4 at-rules parse+evaluate; §6.5 matching for basic/structural selectors |
| M4 Block layout | §6.3 box/flex/grid group *parsing* complete; layout integration for box+table groups begins |
| M5 Paint | §6.3 color/bg/border integration; pixel corpus opens |
| M6 Window | §6.3 ui-group scroll subset; §6.11 wiring items 1–4 (keys) and 7 (mouse) |
| M7 Text | §6.3 font+text integration; §6.5 linguistic selectors; Appendix E unit resolution complete |
| M8 Images | §6.3 image-bearing properties (object-fit, background-image); decoder fuzz targets open |
| M9 JS engine | §6.7 all [M9] builtin blocks; test262 slices adopted |
| M10 Scriptable DOM | §6.6 remaining [M10] interfaces; §6.8 all events; §6.11 wiring items 5–6, 8–10 |
| M11 Shell | — (§5.19 surface; §6.11 shell shortcuts; DevTools v1) |
| M12 Storage | §6.10 all; §6.9 items 8–13 →; §6.5 resource-state selectors |
| M13 Release | every remaining unchecked item → complete or struck with waiver (§6.12) |

Reading rule for §12.3 task selection: the live claim set of the current
milestone defines which sections' unchecked items are eligible; the
dependency-first override (§12.3) may pull from a later claim set only
when the current section names it as a prerequisite.
### §6.12 The completion ledger rules

1. **Check means verified.** A box is ticked only with: code merged, the
   item's named tests green, and the session report (§11.4) referencing it.
2. **Waivers are explicit.** Anything consciously not done gets `~~struck~~`
   text plus a one-line waiver reason and, if architectural, an ADR. Silent
   gaps are the one dishonesty this project cannot survive.
3. **Regeneration discipline.** This section is generated
   (`tools/generate_wbs.py`); edits go to the data files, never the generated
   markdown. The generator is idempotent: regenerating with unchanged data
   must produce a byte-identical file (CI asserts this).
4. **Progress accounting.** `scripts/wbs-progress.sh` counts checked/total per
   §6.x and writes the percentage into `PROGRESS.md` (§12.1). Milestone exit
   reports quote it.
5. **Ordering is advisory.** Within a section, work top-to-bottom; across
   sections, the milestone's exit criteria (Part 7) choose which sections
   are live. Dependency-first overrides (§12.3) beat document order.

### §6.14 The progress script specification

`scripts/wbs-progress.sh` (implemented in the shell of your choice, keep it
dependency-free) produces the numbers quoted in `PROGRESS.md`:

1. Input: this document (Part 6 only). A WBS item is a line matching
   `^- \[ \] ` (open) or `^- \[x\] ` (done, case-insensitive on the x).
2. Output: a table of `section | open | done | total | percent` for §6.2
   through §6.13, plus a grand total row, sorted by section number.
3. Struck-through items (`~~...~~`) count in `total` as waived: they are
   excluded from both open and done, and reported in their own column
   so waivers stay visible (§6.12 rule 2).
4. Exit code 0 always; it is a reporting tool, not a gate — the gates
   are the test tiers (§8.8).
5. A `--strict` flag exits non-zero if any §6.x shows done + waived < total
   (used at milestone exits and at M13).

## APPENDIX G — Default (UA) Stylesheet

The user-agent origin stylesheet (§7.5), given here as the reference text. It
is a product artifact: encode it exactly in `aurora_css::ua_sheet` (or generated
from this source), keep `docs/UA-STYLESHEET.md` in sync, and treat any deviation
from this listing as a bug unless an ADR says otherwise. Quirks-mode deltas are
listed separately at the end. Cascade note: UA origin, importance normal — every
rule here loses to any author rule of equal-or-greater weight (§5.8.2).

```css
/* === Display and visibility === */
html, body          { display: block; }
head, title, base, link, meta, style, script, noscript, template, slot,
datalist, colgroup, col, source, track, area, basefont, param, rp {
                      display: none; }
head > meta, head > title, head > base, head > link, head > style,
head > script       { /* covered above; kept for fragments */ }
template            { display: none; }
[hidden]            { display: none; }
*                   { visibility: inherit; }
dialog:not([open])  { display: none; }
details > *:not(summary) { display: none; }  /* when closed; open removes */
details[open] > *:not(summary) { display: block; }

/* === Root and structure === */
html                { display: block; }
body                { display: block; margin: 8px; }
p, dl, multicol     { display: block; margin-block: 1em; }
dd                  { display: block; margin-inline-start: 40px; }
blockquote, figure  { display: block; margin-block: 1em; margin-inline: 40px; }
figcaption          { display: block; }
center              { display: block; text-align: center; }

/* === Headings === */
h1                  { display: block; margin-block: 0.67em; font-weight: bold;
                      font-size: 2em; }
h2                  { display: block; margin-block: 0.83em; font-weight: bold;
                      font-size: 1.5em; }
h3                  { display: block; margin-block: 1em;    font-weight: bold;
                      font-size: 1.17em; }
h4                  { display: block; margin-block: 1.33em; font-weight: bold;
                      font-size: 1em; }
h5                  { display: block; margin-block: 1.67em; font-weight: bold;
                      font-size: 0.83em; }
h6                  { display: block; margin-block: 2.33em; font-weight: bold;
                      font-size: 0.67em; }

/* === Lists === */
ol, ul, menu        { display: block; margin-block: 1em;
                      padding-inline-start: 40px; list-style-type: disc; }
ol                  { list-style-type: decimal; }
ol[type="1"]        { list-style-type: decimal; }
ol[type="a"]        { list-style-type: lower-alpha; }
ol[type="A"]        { list-style-type: upper-alpha; }
ol[type="i"]        { list-style-type: lower-roman; }
ol[type="I"]        { list-style-type: upper-roman; }
ul[type="circle"]   { list-style-type: circle; }
ul[type="disc"]     { list-style-type: disc; }
ul[type="square"]   { list-style-type: square; }
li                  { display: list-item; text-align: match-parent; }
dir, dd, dt         { /* dd above; dt: */ }
dt                  { display: block; }

/* === Inline text === */
:link               { color: linktext; text-decoration: underline; }
b, strong           { font-weight: bolder; }
i, em, cite, var, dfn { font-style: italic; }
u, ins              { text-decoration: underline; }
s, strike, del      { text-decoration: line-through; }
tt, code, kbd, samp { font-family: monospace; font-size: 1em; }
small               { font-size: smaller; }
big                 { font-size: larger; }
abbr[title], acronym[title] { text-decoration: dotted underline; }
sub                 { vertical-align: sub;   font-size: smaller; }
sup                 { vertical-align: super; font-size: smaller; }
q                   { display: inline; }           /* quotes generated per content/quotes */
q::before           { content: open-quote; }
q::after            { content: close-quote; }
nobr                { white-space: nowrap; }
bdo[dir="ltr"]      { direction: ltr; unicode-bidi: bidi-override; }
bdo[dir="rtl"]      { direction: rtl; unicode-bidi: bidi-override; }
bdi[dir]            { unicode-bidi: isolate; }
ruby, rt, rb, rbc   { /* MVP: ruby treated inline; rt: */ display: inline; }
rp                  { display: none; }

/* === Whitespace preservation === */
pre, xmp, plaintext, listing,
textarea            { font-family: monospace; font-size: 1em;
                      white-space: pre; }
pre, xmp, plaintext, listing { display: block; margin-block: 1em; }

/* === Tables === */
table               { display: table; border-collapse: separate;
                      border-spacing: 2px; border-color: gray;
                      box-sizing: border-box; text-indent: initial; }
caption             { display: table-caption; text-align: center; }
colgroup, col       { display: table-column-group; }   /* col: table-column */
col                 { display: table-column; }
thead               { display: table-header-group;  vertical-align: middle; }
tbody               { display: table-row-group;     vertical-align: middle; }
tfoot               { display: table-footer-group;  vertical-align: middle; }
tr                  { display: table-row;           vertical-align: inherit; }
td, th              { display: table-cell; padding: 1px; vertical-align: inherit; }
th                  { font-weight: bold; text-align: center; }
table[border] > tr, table[border] > thead > tr, table[border] > tbody > tr,
table[border] > tfoot > tr { /* border presentational hints map to cells */ }
table[rules]        { /* rules hints map to border-style on cells */ }

/* === Replaced and embedded === */
img, iframe, embed, object, video, canvas, input[type="image"] {
                      display: inline-block; vertical-align: baseline; }
iframe              { border: 2px inset; }  /* frame chrome per §5.19 */
embed, object       { width: 300px; height: 150px; }  /* default intrinsic */
video[controls], audio[controls] { /* control chrome drawn by the shell */ }
audio:not([controls]) { display: none; }
canvas[width], canvas[height] { /* attribute-driven intrinsic size */ }

/* === Forms === */
form                { display: block; margin-block: 1em; }
button, input[type="button"], input[type="submit"], input[type="reset"],
input[type="checkbox"], input[type="radio"], input[type="range"],
input[type="file"]  { /* control chrome owned by the shell painter */ }
input[type="text"], input[type="search"], input[type="tel"], input[type="url"],
input[type="email"], input[type="password"], input[type="date"],
input, textarea     { font-family: monospace; }   /* MVP control typography */
input[type="hidden"]{ display: none; }
input[disabled], select[disabled], textarea[disabled],
button[disabled], optgroup[disabled], option[disabled] { color: graytext; }
fieldset            { display: block; margin-inline: 2px; border: groove 2px;
                      border-block-start: groove 2px; border-block-end: groove 2px;
                      padding-block: 0.35em 0.625em; padding-inline: 0.75em;
                      min-inline-size: min-content; }
legend              { display: block; padding-inline: 2px; }
label               { display: inline; cursor: default; }
output              { display: inline; }
option              { /* rendered in the select list UI, not in flow */ }
optgroup            { /* group header in the list UI */ }
progress, meter     { display: inline-block; width: 10em; height: 1em;
                      vertical-align: baseline; }

/* === Interactive === */
summary             { display: block; }
summary::marker     { /* disclosure triangle drawn by the shell painter */ }
dialog              { display: block; margin: auto; border: solid;
                      padding: 1em; background: canvas; color: canvastext; }
dialog::backdrop    { background: rgba(0, 0, 0, 0.5); }
marquee             { display: block; overflow: hidden; }  /* static (§6.2) */
frame, frameset     { display: none; }                     /* parse-only */
noframes            { display: none; }

/* === Text behavior defaults === */
body                { text-rendering: auto; }
:dir(ltr)           { /* directionality comes from the bidi pass, not a rule */ }
br                  { /* forced break; no box of its own */ }
wbr                 { /* soft wrap opportunity */ }

/* === Legacy presentational-hint targets (attributes map into style) === */
font[size]          { /* size hint → font-size table */ }
font[color]         { /* color hint → color */ }
font[face]          { /* face hint → font-family */ }
bgcolor, background, align, valign, border, cellpadding, cellspacing, hspace,
vspace, width, height attributes on their respective elements
                    { /* handled by the presentational-hint layer (§6.2) */ }
```

### Quirks-mode deltas

In quirks mode (DOCTYPE-less documents, §5.5.3):

1. The line-height of block containers uses the quirks "normal" computation
   (small strut) instead of the standards strut for the body font.
2. `td`, `th`, and table captions inherit `text-align` from their rows
   (standards mode: `match-parent`-like centering for `th` only).
3. Percentage heights on blocks resolve against the *viewport* when the
   containing block's height is auto (the classic quirks height rule).
4. Font-size keywords (`small` … `xx-large`) use the quirks scaling table.
5. Empty inline elements with explicit heights/widths are not collapsed the
   standards way (the "haslayout" era behaviors) — implemented only as far as
   the quirks corpus (§8.6) requires, and no further.

### Maintenance rule

This appendix is generated-by-hand but versioned: any change to the UA sheet
source lands in the same commit as the matching change here, and the pixel
corpus (§8.5) re-runs in full. The UA sheet is also where `prefers-color-scheme`
dark defaults will live when theming lands (ADR required).

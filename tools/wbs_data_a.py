"""Dataset A for the AURORA prompt WBS generator: CSS surface + reference tables.

Every line is pipe-delimited; fields must not contain a pipe character.
These tables are work-aids for the implementing agent, not normative specs:
where a row and the CSSWG standard disagree, the standard wins (Part 0 §0.4).
"""

CSS_GROUPS = """box|§5.9/§5.10 — box generation, display, positioning; layout owns geometry
flex|§5.10.1 — flexbox layout module
grid|§5.10.2 — grid layout module
table|§5.9 — table formatting (§5.9.2 box building + anonymous table boxes)
text|§5.11/§5.9.3 — text processing and inline layout
font|§5.11.1 — font selection, matching, metrics (inherited by default)
color|§5.16 — color and opacity resolve to paint values
bg|§5.16.2 — backgrounds render under content in paint order
border|§5.16.2 — borders render as part of the box edge path
ui|§5.19 — interaction surface: cursors, input, scrolling UX
transform|§5.16.1 — transforms create stacking contexts and affect hit testing
transition|§5.15 — transition machinery lives in the runtime, paints via interpolators
animation|§5.15 — keyframe machinery lives in the runtime, paints via interpolators
list|§5.9/§5.16 — list items generate ::marker boxes
multicol|§5.9.7 — multi-column fragmentation
fragmentation|§5.9.7 — break control between fragmentainers
page|§5.9.7 — paged media (parse-only in MVP)
mask|§5.16.3 — masks and clipping feed the clip chain
filter|§5.16.4 — filters run as raster post-processes
logical|§5.9 — logical properties map to physical at computed-value time
misc|multiple — cross-cutting properties"""

# name|group|initial|inherited(Y/N)
CSS_PROPERTIES = """
display|box|inline|N
box-sizing|box|content-box|N
margin|box|see longhands (0)|N
margin-top|box|0|N
margin-right|box|0|N
margin-bottom|box|0|N
margin-left|box|0|N
padding|box|see longhands (0)|N
padding-top|box|0|N
padding-right|box|0|N
padding-bottom|box|0|N
padding-left|box|0|N
width|box|auto|N
height|box|auto|N
min-width|box|auto|N
min-height|box|auto|N
max-width|box|none|N
max-height|box|none|N
inset|box|auto|N
top|box|auto|N
right|box|auto|N
bottom|box|auto|N
left|box|auto|N
position|box|static|N
z-index|box|auto|N
float|box|none|N
clear|box|none|N
overflow|box|see longhands (visible)|N
overflow-x|box|visible|N
overflow-y|box|visible|N
visibility|box|visible|Y
object-fit|box|fill|N
object-position|box|50% 50%|N
aspect-ratio|box|auto|N
container-type|box|normal|N
container-name|box|none|N
all|misc|see individual (reset)|N
appearance|ui|none|N
flex|flex|see longhands|N
flex-grow|flex|0|N
flex-shrink|flex|1|N
flex-basis|flex|auto|N
flex-direction|flex|row|N
flex-wrap|flex|nowrap|N
flex-flow|flex|see longhands|N
order|flex|0|N
align-items|flex|normal|N
align-self|flex|auto|N
align-content|flex|normal|N
justify-items|flex|legacy|N
justify-self|flex|auto|N
justify-content|flex|normal|N
gap|flex|normal|N
row-gap|flex|normal|N
column-gap|flex|normal|N
place-items|flex|see longhands|N
place-content|flex|see longhands|N
place-self|flex|see longhands|N
grid|grid|see longhands|N
grid-template|grid|see longhands|N
grid-template-rows|grid|none|N
grid-template-columns|grid|none|N
grid-template-areas|grid|none|N
grid-auto-rows|grid|auto|N
grid-auto-columns|grid|auto|N
grid-auto-flow|grid|row|N
grid-row|grid|see longhands (auto)|N
grid-column|grid|see longhands (auto)|N
grid-area|grid|see longhands (auto)|N
grid-row-start|grid|auto|N
grid-row-end|grid|auto|N
grid-column-start|grid|auto|N
grid-column-end|grid|auto|N
table-layout|table|auto|N
border-collapse|table|separate|N
border-spacing|table|0|N
empty-cells|table|show|N
caption-side|table|top|N
color|color|canvastext|Y
opacity|color|1|N
color-scheme|color|normal|Y
accent-color|color|auto|N
caret-color|color|auto|N
print-color-adjust|color|economic|Y
forced-color-adjust|color|auto|Y
background|bg|see longhands|N
background-color|bg|transparent|N
background-image|bg|none|N
background-repeat|bg|repeat|N
background-position|bg|0% 0%|N
background-size|bg|auto|N
background-clip|bg|border-box|N
background-origin|bg|padding-box|N
background-attachment|bg|scroll|N
background-blend-mode|bg|normal|N
border|border|see longhands (medium none currentcolor)|N
border-width|border|see longhands (medium)|N
border-style|border|see longhands (none)|N
border-color|border|see longhands (currentcolor)|N
border-top|border|see longhands|N
border-right|border|see longhands|N
border-bottom|border|see longhands|N
border-left|border|see longhands|N
border-top-width|border|medium|N
border-right-width|border|medium|N
border-bottom-width|border|medium|N
border-left-width|border|medium|N
border-top-style|border|none|N
border-right-style|border|none|N
border-bottom-style|border|none|N
border-left-style|border|none|N
border-top-color|border|currentcolor|N
border-right-color|border|currentcolor|N
border-bottom-color|border|currentcolor|N
border-left-color|border|currentcolor|N
border-radius|border|see longhands (0)|N
border-top-left-radius|border|0|N
border-top-right-radius|border|0|N
border-bottom-right-radius|border|0|N
border-bottom-left-radius|border|0|N
border-image|border|see longhands|N
border-image-source|border|none|N
border-image-slice|border|100%|N
border-image-width|border|1|N
border-image-outset|border|0|N
border-image-repeat|border|stretch|N
outline|border|see longhands|N
outline-width|border|medium|N
outline-style|border|none|N
outline-color|border|auto|N
outline-offset|border|0|N
box-shadow|border|none|N
box-decoration-break|fragmentation|slice|N
cursor|ui|auto|Y
pointer-events|ui|auto|Y
user-select|ui|auto|N
resize|ui|none|N
touch-action|ui|auto|N
overscroll-behavior|ui|see longhands (auto)|N
overscroll-behavior-x|ui|auto|N
overscroll-behavior-y|ui|auto|N
scroll-behavior|ui|auto|N
scroll-snap-type|ui|none|N
scroll-snap-align|ui|none|N
scroll-snap-stop|ui|normal|N
scroll-margin|ui|see longhands (0)|N
scroll-margin-top|ui|0|N
scroll-margin-right|ui|0|N
scroll-margin-bottom|ui|0|N
scroll-margin-left|ui|0|N
scroll-padding|ui|see longhands (auto)|N
scroll-padding-top|ui|auto|N
scroll-padding-right|ui|auto|N
scroll-padding-bottom|ui|auto|N
scroll-padding-left|ui|auto|N
will-change|ui|auto|N
contain|ui|none|N
content-visibility|ui|visible|N
isolation|ui|auto|N
mix-blend-mode|ui|normal|N
transform|transform|none|N
transform-origin|transform|50% 50%|N
transform-box|transform|view-box|N
transform-style|transform|flat|N
perspective|transform|none|N
perspective-origin|transform|50% 50%|N
backface-visibility|transform|visible|N
translate|transform|none|N
rotate|transform|none|N
scale|transform|none|N
transition|transition|see longhands|N
transition-property|transition|all|N
transition-duration|transition|0s|N
transition-timing-function|transition|ease|N
transition-delay|transition|0s|N
transition-behavior|transition|normal|N
animation|animation|see longhands|N
animation-name|animation|none|N
animation-duration|animation|0s|N
animation-timing-function|animation|ease|N
animation-delay|animation|0s|N
animation-iteration-count|animation|1|N
animation-direction|animation|normal|N
animation-fill-mode|animation|none|N
animation-play-state|animation|running|N
animation-composition|animation|replace|N
animation-timeline|animation|auto|N
list-style|list|see longhands|N
list-style-type|list|disc|Y
list-style-position|list|outside|N
list-style-image|list|none|N
columns|multicol|see longhands (auto)|N
column-count|multicol|auto|N
column-width|multicol|auto|N
column-gap|multicol|normal|N
column-rule|multicol|see longhands (medium none currentcolor)|N
column-rule-width|multicol|medium|N
column-rule-style|multicol|none|N
column-rule-color|multicol|currentcolor|N
column-span|multicol|none|N
column-fill|multicol|balance|N
break-before|fragmentation|auto|N
break-after|fragmentation|auto|N
break-inside|fragmentation|auto|N
orphans|fragmentation|2|Y
widows|fragmentation|2|Y
page|page|auto|N
page-break-before|page|auto|N
page-break-after|page|auto|N
page-break-inside|page|auto|N
clip-path|mask|none|N
mask|mask|see longhands|N
mask-image|mask|none|N
mask-mode|mask|match-source|N
mask-repeat|mask|repeat|N
mask-position|mask|center|N
mask-clip|mask|border-box|N
mask-origin|mask|border-box|N
mask-size|mask|auto|N
mask-composite|mask|add|N
filter|filter|none|N
backdrop-filter|filter|none|N
margin-block|logical|see longhands|N
margin-block-start|logical|0|N
margin-block-end|logical|0|N
margin-inline|logical|see longhands|N
margin-inline-start|logical|0|N
margin-inline-end|logical|0|N
padding-block|logical|see longhands (0)|N
padding-block-start|logical|0|N
padding-block-end|logical|0|N
padding-inline|logical|see longhands (0)|N
padding-inline-start|logical|0|N
padding-inline-end|logical|0|N
border-block-start|logical|see longhands|N
border-block-end|logical|see longhands|N
border-inline-start|logical|see longhands|N
border-inline-end|logical|see longhands|N
inline-size|logical|auto|N
block-size|logical|auto|N
min-inline-size|logical|auto|N
min-block-size|logical|auto|N
max-inline-size|logical|none|N
max-block-size|logical|none|N
inset-block-start|logical|auto|N
inset-block-end|logical|auto|N
inset-inline-start|logical|auto|N
inset-inline-end|logical|auto|N
overflow-block|logical|see overflow-x|N
overflow-inline|logical|see overflow-y|N
line-height|text|normal|Y
letter-spacing|text|normal|Y
word-spacing|text|normal|Y
text-align|text|start|Y
text-align-last|text|auto|Y
text-indent|text|0|Y
text-transform|text|none|Y
text-decoration|text|see longhands (none)|N
text-decoration-line|text|none|N
text-decoration-style|text|solid|N
text-decoration-color|text|currentcolor|N
text-decoration-thickness|text|auto|N
text-underline-offset|text|auto|N
text-decoration-skip-ink|text|auto|N
text-emphasis|text|see longhands|N
text-emphasis-style|text|none|N
text-emphasis-color|text|currentcolor|N
text-shadow|text|none|Y
white-space|text|normal|Y
word-break|text|normal|N
overflow-wrap|text|normal|N
word-wrap|text|alias of overflow-wrap|N
hyphens|text|manual|Y
tab-size|text|8|Y
line-break|text|auto|N
vertical-align|text|baseline|N
direction|text|ltr|Y
unicode-bidi|text|normal|N
writing-mode|text|horizontal-tb|N
text-orientation|text|mixed|N
text-combine-upright|text|none|N
text-rendering|text|auto|N
text-justify|text|auto|Y
text-underline-position|text|auto|Y
initial-letter|text|normal|N
quotes|text|auto|Y
content|text|normal|N
counter-reset|text|none|N
counter-increment|text|none|N
counter-set|text|none|N
font|font|see longhands|Y
font-family|font|per UA stylesheet|Y
font-size|font|medium|Y
font-weight|font|normal|Y
font-style|font|normal|Y
font-stretch|font|normal|Y
font-variant|font|normal|Y
font-variant-ligatures|font|normal|Y
font-variant-caps|font|normal|Y
font-variant-numeric|font|normal|Y
font-variant-east-asian|font|normal|Y
font-variant-position|font|normal|Y
font-variant-alternates|font|normal|Y
font-kerning|font|auto|Y
font-feature-settings|font|normal|Y
font-variation-settings|font|normal|Y
font-size-adjust|font|none|Y
font-synthesis-weight|font|auto|Y
font-synthesis-style|font|auto|Y
font-synthesis-small-caps|font|auto|Y
font-optical-sizing|font|auto|Y
font-language-override|font|normal|Y
image-rendering|misc|auto|Y
image-orientation|misc|from-image|N
shape-outside|misc|none|N
shape-margin|misc|0|N
shape-image-threshold|misc|0.5? verify|N
"""

# rule|purpose-and-engine-behavior (single line each)
CSS_AT_RULES = """@charset|Must be first; selects the stylesheet encoding; ignored elsewhere. Parser consumes and reports it.
@import|Fetches and cascades a sheet; conditions (media/supports/layer) gate it; relative URLs resolve against the sheet's base URL.
@media|Gate rules on media queries: type (all/screen/print), width/height/aspect-ratio/resolution/orientation, prefers-color-scheme/reduced-motion/contrast; nested media allowed per CSS Conditional.
@supports|Feature-query gate: property:value checks and (not/and/or) combinations evaluated against the property registry.
@page|Paged media page boxes: margins, size; parse in MVP, layout as stretch.
@font-face|Registers a font family: src list with format() and unicode-range, font-display, weight/style/stretch descriptors; contributes to §5.11 font matching.
@keyframes|Named keyframe list: 0%/100%/from/to stops with declaration blocks; interpolation per animation-timing-function; consumed by the animation machinery of §5.15.
@namespace|Declares namespace prefixes for type/attribute selector matching in XML-ish documents; affects selector namespace resolution only.
@layer|Cascade layers: named and anonymous layer declarations with nested blocks; layer order participates in the cascade (§5.8.2).
@container|Container queries: size/inline-size/style conditions evaluated against the nearest ancestor container (§box container-type); MVP: size only.
@property|Registers custom properties with syntax/initial-value/inherits; typed registration changes substitution and animation behavior.
@counter-style|Defines list/counter marker styles (system, symbols, prefix/suffix, range, pad); referenced by list-style-type and counters.
@font-feature-values|Named font feature value sets for font-variant-alternates; parse and expose to the shaper.
@scope|Scopes nested rules to a range between a root and an optional scoping limit; affects matching (§5.8.1); MVP: parse + root-only scoping.
@starting-style|Transitions-in entry styles: declarations applied at first style computation, then dropped.
@document (legacy)|Non-standard; parse as Unknown rule block, never apply (§5.7 forward-compat rule)."""

# selector|kind|notes (single line each)
CSS_SELECTORS = """*|basic|Universal selector; matches any element; zero specificity alone.
E|basic|Type selector with optional namespace prefix (svg|rect); case-insensitivity per document language.
.c|basic|Class selector; compound list matching on DOMTokenList; whitespace-separated.
#id|basic|ID selector; document-unique assumption allowed for fast paths but must stay correct if violated.
[a]|basic|Attribute presence; matches namespaced attributes with the standard default namespace rules.
[a=v]|basic|Exact match.
[a~=v]|basic|Whitespace-list match.
[a|=v]|basic|Prefix-with-dash match (lang codes).
[a^=v]|basic|Prefix match (empty value never matches).
[a$=v]|basic|Suffix match (empty value never matches).
[a*=v]|basic|Substring match.
[a=v i]|basic|Case-insensitive flag; [a=v s] case-sensitive flag.
:is(x, y)|logical|Specificity = most specific argument; forgiving selector list (invalid arguments dropped).
:where(x, y)|logical|Specificity = zero; forgiving list; otherwise identical to :is().
:not(x, y)|logical|Negation; forgiving list; specificity = most specific argument.
:has(rel sel)|logical|Relative selector match on descendants/siblings; requires the §5.8 invalidation machinery.
:scope|logical|Matches the scoping root (querySelector context, @scope root).
:root|structural|Matches the document root element.
:empty|structural|Element with no children (text nodes count; whitespace counts).
:nth-child(an+b)|structural|Full an+b grammar incl. of S syntax (:nth-child(2n of .c)); parse per css-syntax an+b tests.
:nth-last-child(an+b)|structural|Counted from the end.
:nth-of-type(an+b)|structural|Index among same-type siblings.
:nth-last-of-type(an+b)|structural|Index among same-type siblings from the end.
:first-child|structural|nth-child(1).
:last-child|structural|nth-last-child(1).
:only-child|structural|First and last child simultaneously.
:first-of-type|structural|First among same-type siblings.
:last-of-type|structural|Last among same-type siblings.
:only-of-type|structural|Only among same-type siblings.
:link|resource|Unvisited link state (elements with href that are links); see §5.18.7.
:any-link|resource|Link regardless of visited state.
:visited|resource|Visited state; restricted to paint-only properties (§5.18.7).
:target|resource|Element targeted by the URL fragment.
:defined|resource|Custom-element definedness (always true for built-ins in this engine).
:modal|resource|True for elements in the top layer as modal dialogs.
:fullscreen|resource|Always false in this engine (non-goal §1.4).
:hover|user|Pointer-over state from the input system; ancestors of the hovered element also match.
:active|user|Activation-in-progress state for buttons/links.
:focus|user|The focused element (one per document).
:focus-within|user|Focused element or its ancestors.
:focus-visible|user|Heuristic per spec: keyboard-initiated focus gets the ring.
:enabled|form|Form control that is not disabled.
:disabled|form|Disabled form control (from the disabled attribute or fieldset propagation).
:checked|form|Checked checkbox/radio or selected option.
:indeterminate|form|Checkbox with indeterminate IDL state, radio group with none selected, progress without value.
:placeholder-shown|form|Input currently showing placeholder.
:default|form|Default checked/selected option of its group.
:required|form|Required form control.
:optional|form|Not required.
:valid|form|Passes constraint validation (§5.6 forms subset).
:invalid|form|Fails constraint validation.
:in-range|form|Has value constraints and value is within them.
:out-of-range|form|Has constraints and value is outside.
:read-only|form|Not editable (readonly, disabled, or non-input element).
:read-write|form|Editable.
:lang(x)|linguistic|Language match with wildcard ranges; walks the lang attribute chain plus meta inheritance.
:dir(ltr/rtl)|linguistic|Directionality from the bidi algorithm result (§5.11.4).
::before|pseudo-element|Generated box before content per content property.
::after|pseudo-element|Generated box after content per content property.
::placeholder|pseudo-element|Styles the input placeholder text.
::selection|pseudo-element|Styles the active selection; paint-only overlay.
::marker|pseudo-element|List item marker box styling.
::first-line|pseudo-element|First formatted line of a block (limited property set per spec).
::first-letter|pseudo-element|First letter/leading punctuation box of a block.
::backdrop|pseudo-element|Behind top-layer elements (dialog).
::file-selector-button|pseudo-element|Button inside file inputs (sub-UI styling MVP)."""

# color|hex (148 CSS named colors)
CSS_COLORS = """
aliceblue|f0f8ff
antiquewhite|faebd7
aqua|00ffff
aquamarine|7fffd4
azure|f0ffff
beige|f5f5dc
bisque|ffe4c4
black|000000
blanchedalmond|ffebcd
blue|0000ff
blueviolet|8a2be2
brown|a52a2a
burlywood|deb887
cadetblue|5f9ea0
chartreuse|7fff00
chocolate|d2691e
coral|ff7f50
cornflowerblue|6495ed
cornsilk|fff8dc
crimson|dc143c
cyan|00ffff
darkblue|00008b
darkcyan|008b8b
darkgoldenrod|b8860b
darkgray|a9a9a9
darkgreen|006400
darkgrey|a9a9a9
darkkhaki|bdb76b
darkmagenta|8b008b
darkolivegreen|556b2f
darkorange|ff8c00
darkorchid|9932cc
darkred|8b0000
darksalmon|e9967a
darkseagreen|8fbc8f
darkslateblue|483d8b
darkslategray|2f4f4f
darkslategrey|2f4f4f
darkturquoise|00ced1
darkviolet|9400d3
deeppink|ff1493
deepskyblue|00bfff
dimgray|696969
dimgrey|696969
dodgerblue|1e90ff
firebrick|b22222
floralwhite|fffaf0
forestgreen|228b22
fuchsia|ff00ff
gainsboro|dcdcdc
ghostwhite|f8f8ff
gold|ffd700
goldenrod|daa520
gray|808080
green|008000
greenyellow|adff2f
grey|808080
honeydew|f0fff0
hotpink|ff69b4
indianred|cd5c5c
indigo|4b0082
ivory|fffff0
khaki|f0e68c
lavender|e6e6fa
lavenderblush|fff0f5
lawngreen|7cfc00
lemonchiffon|fffacd
lightblue|add8e6
lightcoral|f08080
lightcyan|e0ffff
lightgoldenrodyellow|fafad2
lightgray|d3d3d3
lightgreen|90ee90
lightgrey|d3d3d3
lightpink|ffb6c1
lightsalmon|ffa07a
lightseagreen|20b2aa
lightskyblue|87cefa
lightslategray|778899
lightslategrey|778899
lightsteelblue|b0c4de
lightyellow|ffffe0
lime|00ff00
limegreen|32cd32
linen|faf0e6
magenta|ff00ff
maroon|800000
mediumaquamarine|66cdaa
mediumblue|0000cd
mediumorchid|ba55d3
mediumpurple|9370db
mediumseagreen|3cb371
mediumslateblue|7b68ee
mediumspringgreen|00fa9a
mediumturquoise|48d1cc
mediumvioletred|c71585
midnightblue|191970
mintcream|f5fffa
mistyrose|ffe4e1
moccasin|ffe4b5
navajowhite|ffdead
navy|000080
oldlace|fdf5e6
olive|808000
olivedrab|6b8e23
orange|ffa500
orangered|ff4500
orchid|da70d6
palegoldenrod|eee8aa
palegreen|98fb98
paleturquoise|afeeee
palevioletred|db7093
papayawhip|ffefd5
peachpuff|ffdab9
peru|cd853f
pink|ffc0cb
plum|dda0dd
powderblue|b0e0e6
purple|800080
rebeccapurple|663399
red|ff0000
rosybrown|bc8f8f
royalblue|4169e1
saddlebrown|8b4513
salmon|fa8072
sandybrown|f4a460
seagreen|2e8b57
seashell|fff5ee
sienna|a0522d
silver|c0c0c0
skyblue|87ceeb
slateblue|6a5acd
slategray|708090
slategrey|708090
snow|fffafa
springgreen|00ff7f
steelblue|4682b4
tan|d2b48c
teal|008080
thistle|d8bfd8
tomato|ff6347
turquoise|40e0d0
violet|ee82ee
wheat|f5deb3
white|ffffff
whitesmoke|f5f5f5
yellow|ffff00
yellowgreen|9acd32
"""

# entity|codepoint|glyph-note
HTML_ENTITIES = """
amp|38|ampersand &
lt|60|less-than <
gt|62|greater-than >
quot|34|double quote "
apos|39|apostrophe '
nbsp|160|no-break space (non-ASCII in serialization)
copy|169|copyright sign
reg|174|registered sign
trade|8482|trademark sign
hellip|8230|horizontal ellipsis
mdash|8212|em dash
ndash|8211|en dash
lsquo|8216|left single quote
rsquo|8217|right single quote
ldquo|8220|left double quote
rdquo|8221|right double quote
sbquo|8218|single low quote
bdquo|8222|double low quote
laquo|171|left guillemet
raquo|187|right guillemet
lsaquo|8249|single left angle quote
rsaquo|8250|single right angle quote
times|215|multiplication sign
divide|247|division sign
plusmn|177|plus-minus sign
deg|176|degree sign
middot|183|middle dot
bull|8226|bullet
dagger|8224|dagger
Dagger|8225|double dagger
permil|8240|per mille sign
euro|8364|euro sign
cent|162|cent sign
pound|163|pound sign
yen|165|yen sign
sect|167|section sign
para|182|pilcrow sign
larr|8592|leftwards arrow
uarr|8593|upwards arrow
rarr|8594|rightwards arrow
darr|8595|downwards arrow
harr|8596|left-right arrow
minus|8722|minus sign
lowast|8727|asterisk operator
radic|8730|square root
infin|8734|infinity
cap|8745|intersection
cup|8746|union
int|8747|integral
asymp|8776|almost equal
ne|8800|not equal
le|8804|less-or-equal
ge|8805|greater-or-equal
alpha|945|greek small alpha
beta|946|greek small beta
gamma|947|greek small gamma
pi|960|greek small pi
Omega|937|greek capital omega
sum|8721|n-ary summation
prod|8719|n-ary product
part|8706|partial differential
nabla|8711|nabla
isin|8712|element of
notin|8713|not an element of
empty|8709|empty set
forall|8704|for all
exist|8707|there exists
oplus|8853|circled plus
otimes|8855|circled times
perp|8869|up tack
sdot|8901|dot operator
lceil|8968|left ceiling
rceil|8969|right ceiling
lfloor|8970|left floor
rfloor|8971|right floor
lang|10216|left angle bracket
rang|10217|right angle bracket
loz|9674|lozenge
spades|9824|black spade suit
clubs|9827|black club suit
hearts|9829|black heart suit
diams|9830|black diamond suit
shy|173|soft hyphen (invisible)
zwnj|8204|zero-width non-joiner
zwj|8205|zero-width joiner
lrm|8206|left-to-right mark
rlm|8207|right-to-left mark
"""

# unit|kind|notes
CSS_UNITS = """
px|length|CSS reference pixel; the anchor all others resolve to at style time.
em|length|Relative to the element's own font-size (for font-size: the parent's).
rem|length|Root element font-size; resolves at style time to absolute px.
ex|length|x-height of the first available font.
ch|length|Advance of the 0 glyph (U+0030) of the first available font.
cap|length|Cap height of the first available font.
ic|length|Advance of the water ideograph (U+6C34).
lh|length|Line height of the element.
rlh|length|Root line height.
vw|length|1% of viewport width.
vh|length|1% of viewport height.
vi|length|1% of viewport inline axis.
vb|length|1% of viewport block axis.
vmin|length|1% of the smaller viewport dimension.
vmax|length|1% of the larger viewport dimension.
sv* family|length|Small-viewport units (svw/svh); MVP maps to vw/vh (ADR if used).
lv* family|length|Large-viewport units (lvw/lvh); MVP maps to vw/vh.
cm|length|1cm = 96px/2.54.
mm|length|1mm = 96px/25.4.
q|length|quarter-millimeter.
in|length|1in = 96px.
pt|length|1pt = 96px/72.
pc|length|1pc = 12pt.
%|percentage|Resolution context is per-property (containing block, font, or own box); see the property registry.
deg|angle|360 per turn.
grad|angle|400 per turn.
rad|angle|2pi per turn.
turn|angle|1 turn.
s|time|Seconds; animation/transition longhands resolve to ms.
ms|time|Milliseconds.
Hz|frequency|Parse-only (voice/a11y stretch).
kHz|frequency|Parse-only.
dpi|resolution|Dots per inch; resolution media queries and image-set().
dpcm|resolution|Dots per centimeter.
dppx|x|Dots per pixel unit (= 96dpi); resolution queries.
fr|flex|Grid free-space unit; only inside track lists (§5.10.2).
"""

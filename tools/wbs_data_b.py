"""Dataset B for the AURORA prompt WBS generator: HTML elements, DOM interfaces,
JavaScript builtins, events, HTTP headers, MIME types, keyboard map.

Pipe-delimited rows; the generator splits multi-value fields on "; ".
Work-aid, not normative: the WHATWG/ECMA standards win on disagreement.
"""

# name|category|content-model|parser-behavior|ua-defaults|layout-note
HTML_ELEMENTS = """
html|root|One head, one body|In body mode the start tag is merged into the existing element|display:block|Establishes the initial containing block; quirks mode holder
head|metadata|Metadata content only|In head insertion mode; most stray content pops it|display:none|No box
title|metadata|Text|RCDATA; text becomes the document title|display:none|No box
base|metadata|Empty|Sets the document base URL (first wins)|display:none|No box
link|metadata|Empty|Triggers stylesheet/favicon/preload handling per rel|display:none|No box
meta|metadata|Empty|charset form restarts encoding detection (§5.5.5)|display:none|No box
style|metadata|Raw text|Contents parsed as CSS, appended to the document sheets|display:none|No box
script|metadata|Script data|Execution queued per async/defer/type (§5.5.6)|display:none|No box
noscript|metadata|Depends on scripting flag|When scripting: RAWTEXT; contents not rendered|display:none|No box
template|metadata|Its own content fragment|Contents go to a DocumentFragment, invisible to the open-element stack|display:none|Contents lay out only when adopted into the tree
slot|metadata|Transparent|Shadow-DOM slotting target|display:contents|Distributes assigned nodes in the flat tree
body|section|Flow content|Implied when tokens hit it|display:block|Scrolling box owner in the default document structure
header|section|Flow content|Generic|display:block|Block container
footer|section|Flow content|Generic|display:block|Block container
main|section|Flow content|Generic|display:block|Block container
section|section|Flow content|Generic|display:block|Block container
nav|section|Flow content|Generic|display:block|Block container
article|section|Flow content|Generic|display:block|Block container
aside|section|Flow content|Generic|display:block|Block container
address|section|Flow content|Generic|display:block; font-style:italic|Block container
h1|section|Phrasing|Generic|display:block; bold; size scale by level|Block container
h2|section|Phrasing|Generic|display:block; bold; scale|Block container
h3|section|Phrasing|Generic|display:block; bold; scale|Block container
h4|section|Phrasing|Generic|display:block; bold; scale|Block container
h5|section|Phrasing|Generic|display:block; bold; scale|Block container
h6|section|Phrasing|Generic|display:block; bold; scale|Block container
hgroup|section|Heading content|Generic|display:block|Block container
p|grouping|Phrasing|Implied end tags; closes an open p|display:block; margins 1em|Block container; margin-collapsing showcase
hr|grouping|Empty|Generic|display:block; border:1px inset; margin|Atomic block; replaced-ish paint
pre|grouping|Text|Newline after open tag dropped; preserves whitespace|display:block; monospace; pre|Inline layout with pre white-space
blockquote|grouping|Flow|Generic|display:block; margins 1em 40px|Block container
ol|grouping|Zero+ li|Generic|display:block; padding-inline-start:40px; list-style-type:decimal|List-item container generating markers
ul|grouping|Zero+ li|Generic|display:block; padding-inline-start:40px; list-style-type:disc|List-item container
menu|grouping|Zero+ li|Semantic alias of ul|display:block; list-style-type:disc|List-item container
li|grouping|Flow|Implied end tags between siblings|display:list-item|Marker box + block/inline content
dl|grouping|dt/dd groups|Generic|display:block; margin:1em 0|Block container
dt|grouping|Flow|Implied end tags|display:block|Block container
dd|grouping|Flow|Implied end tags|display:block; margin-inline-start:40px|Block container
figure|grouping|Flow + figcaption|Generic|display:block; margin:1em 40px|Block container
figcaption|grouping|Flow|Generic|display:block|Block container
div|grouping|Flow|Generic|display:block|Block container; the default case
a|text|Transparent|Implied end tags on block nesting|display:inline; color:-webkit-link; text-decoration:underline|Inline; activation behavior navigates (§5.6)
em|text|Phrasing|Generic|display:inline; font-style:italic|Inline
strong|text|Phrasing|Generic|display:inline; font-weight:bolder|Inline
small|text|Phrasing|Generic|display:inline; font-size:smaller|Inline
s|text|Phrasing|Generic|display:inline; text-decoration:line-through|Inline
cite|text|Phrasing|Generic|display:inline; font-style:italic|Inline
q|text|Phrasing|Generic|display:inline; quotes auto|Inline; generated quote marks per content/quotes
dfn|text|Phrasing|Generic|display:inline; font-style:italic|Inline
abbr|text|Phrasing|Generic|display:inline|Inline; title tooltip source
ruby|text|Phrasing|Special in-body handling|display:ruby|Ruby annotation layout (MVP: inline fallback)
rt|text|Phrasing|Only inside ruby|display:ruby-text|MVP: inline
rp|text|Phrasing|Only inside ruby|display:none|No box
code|text|Phrasing|Generic|display:inline; monospace|Inline
kbd|text|Phrasing|Generic|display:inline; monospace|Inline
samp|text|Phrasing|Generic|display:inline; monospace|Inline
var|text|Phrasing|Generic|display:inline; font-style:italic|Inline
time|text|Phrasing|Generic|display:inline|Inline; datetime attribute parsed
data|text|Phrasing|Generic|display:inline|Inline; value attribute
bdi|text|Phrasing|Generic|display:inline; unicode-bidi:isolate|Inline; isolate in bidi pass
bdo|text|Phrasing|Generic|display:inline; unicode-bidi:bidi-override|Inline; override direction from dir
span|text|Phrasing|Generic|display:inline|Inline; the inline default case
br|text|Empty|Generic|display:inline? (break)|Forced line break in inline layout
wbr|text|Empty|Generic|display:inline|Soft wrap opportunity
ins|edits|Transparent|Generic|display:inline; text-decoration:underline|Inline
del|edits|Transparent|Generic|display:inline; text-decoration:line-through|Inline
img|embedded|Empty|Fires load/error async (§5.12)|display:inline (replaced)|Atomic inline; intrinsic size + object-fit
picture|embedded|source+img|Generic; sources pick the URL|display:inline|No box itself
source|embedded|Empty|Only inside picture/audio/video|display:none|No box
iframe|embedded|Fallback content|Creates a nested browsing context placeholder|display:inline (replaced); border:2px inset|Atomic inline box, nested document area
embed|embedded|Empty|Plugin content placeholder|display:inline (replaced)|Atomic inline placeholder box
object|embedded|Fallback content|Placeholder; data URL fetch attempt|display:inline (replaced)|Atomic inline placeholder
video|embedded|Source elements + fallback|Placeholder box, media element stub|display:inline (replaced)|Atomic inline; intrinsic from attributes if any
audio|embedded|Source elements|No visual box; media stub with controls|display:none unless controls|Controls UI when controls attribute present
track|embedded|Empty|Only inside media elements|display:none|No box
map|embedded|Transparent|Generic|display:inline|No box
area|embedded|Empty|Only inside map|display:none|No box; hit regions parsed
table|tabular|caption/colgroup/sections|Foster parenting of stray content (§5.5)|display:table; border-spacing 2px; border-collapse:separate|Table layout: anonymous boxes for stray rows/cells
caption|tabular|Flow|Only as first table child|display:table-caption|Caption box above/below per side
colgroup|tabular|col elements|Generic|display:table-column-group|Column grouping for spans
col|tabular|Empty|Generic|display:table-column|Column sizing contributor
tbody|tabular|tr elements|Implied when rows appear directly|display:table-row-group|Row group
thead|tabular|tr elements|Generic|display:table-header-group|Row group
tfoot|tabular|tr elements|Generic|display:table-footer-group|Row group
tr|tabular|td/th|Implied in row groups|display:table-row|Row box
td|tabular|Flow|Implied end tags|display:table-cell; padding:1px|Cell box; anonymous row wrapping
th|tabular|Flow|Implied end tags|display:table-cell; bold; center|Cell box
form|forms|Flow, no nested form|Form pointer management (§5.5)|display:block|Block container
label|forms|Phrasing, no nested label|Activation forwards to labeled control (§5.6)|display:inline|Inline
input|forms|Empty|Type-driven control construction|varies by type (text: inline-block ~size)|Replaced-ish inline-block controls
button|forms|Phrasing|Default button type; activation behavior|display:inline-block; UA chrome styling|Inline-block control with border/background defaults
select|forms|option/optgroup|Option list construction|display:inline-block|Control; popup list is shell UI
datalist|forms|options|Provides suggestions; renders nothing|display:none|No box
optgroup|forms|options|Only inside select|display:block (in list UI)|List UI group
option|forms|Text|Selectedness rules|display:block (in list UI)|List UI entry
textarea|forms|Text|RCDATA default value|display:inline-block; monospace; resize both|Multi-line editable control
output|forms|Phrasing|Generic|display:inline|Inline
progress|forms|Phrasing|Determinateness from value/max|display:inline-block|Replaced-ish; bar rendering
meter|forms|Phrasing|Gauge from value/min/max/low/high/optimum|display:inline-block|Replaced-ish; gauge rendering
fieldset|forms|Flow + legend|Generic|display:block; border groove; margins|Block container; legend special-placed
legend|forms|Phrasing|First-child special case|display:block; padding; float rules|Renders in the fieldset border gap
details|interactive|summary + flow|Toggle via name group (§5.6)|display:block|Content hidden unless open
summary|interactive|Phrasing + heading|First summary is the widget|display:block; list-item marker|Marker + click target
dialog|interactive|Flow|open attribute; top layer when modal|display:none; block when open|Top-layer rendering with ::backdrop
canvas|embedded|Fallback|Bitmap backing store (§5.6 ctx2d)|display:inline (replaced)|Atomic inline; default 300x150
center|legacy|Flow|Treated as div with presentational hint|display:block; text-align:center|Block container
font|legacy|Phrasing|Presentational hints: face/size/color|display:inline|Inline; hints map to style
marquee|legacy|Phrasing|Parse; render as static block (non-goal animation)|display:block; overflow:hidden|Block container
frameset|legacy|frame/frameset|Parse-only; content not rendered (non-goal)|display:none|No box
frame|legacy|Empty|Parse-only|display:none|No box
noframes|legacy|Anything|RAWTEXT in frameset docs|display:none|No box
xmp|legacy|Text|RAWTEXT; literal text|display:block; monospace; pre|Block container
plaintext|legacy|Text|Rest of document becomes text tokens|display:block; monospace; pre|Block container
"""

# name|inherits|members|tasks (members separated by "; ")
DOM_INTERFACES = """
EventTarget||method addEventListener(type, callback, options); method removeEventListener(type, callback, options); method dispatchEvent(event) -> bool|Base of the dispatch machinery: capture, target, bubble phases (§5.6)
Node|EventTarget|attr nodeType; attr nodeName; attr baseURI; attr isConnected; attr ownerDocument; attr parentNode; attr parentElement; attr childNodes (live NodeList); attr firstChild; attr lastChild; attr previousSibling; attr nextSibling; attr textContent (get/set); method hasChildNodes(); method normalize(); method cloneNode(deep); method isEqualNode(other); method isSameNode(other); method compareDocumentPosition(other); method contains(other); method lookupPrefix(ns); method lookupNamespaceURI(prefix); method isDefaultNamespace(ns); method insertBefore(node, child); method appendChild(child); method replaceChild(node, child); method removeChild(child)|Arena-backed operations (§5.6); mutation observer hooks; all live collections recompute
Document|Node|attr documentElement; attr doctype; attr body; attr head; attr title (get/set); attr cookie (get/set); attr domain; attr referrer; attr URL; attr documentURI; attr characterSet; attr contentType; attr compatMode; attr designMode; attr dir; attr forms (live); attr images (live); attr links (live); attr scripts (live); attr styleSheets; attr activeElement; attr currentScript; attr defaultView; method createElement(localName, options); method createElementNS(ns, qname); method createDocumentFragment(); method createTextNode(data); method createComment(data); method createProcessingInstruction(target, data); method createAttribute(name); method createAttributeNS(ns, name); method createEvent(type); method createRange(); method createNodeIterator(root, filter); method createTreeWalker(root, filter); method createExpression/evaluate (XPath: non-goal, throw); method getElementById(id); method getElementsByName(name); method getElementsByTagName(qname); method getElementsByTagNameNS(ns, qname); method getElementsByClassName(names); method querySelector(sel); method querySelectorAll(sel); method adoptNode(node); method importNode(node, deep); method open(url, name); method close(); method write(...text); method writeln(...text); method hasFocus(); method execCommand (no-op with false, legacy)|Owner of the arena; base URL + origin resolution; cookie access via storage; write() re-enters the parser (§5.5.6)
DocumentFragment|Node|attr children; method getElementById; method querySelector(All)|Template contents and createContextualFragment substrate
DocumentType|Node|attr name; attr publicId; attr systemId|Serialization and quirks derivation
ShadowRoot|DocumentFragment|attr mode; attr host; attr delegatesFocus; attr slotAssignment; attr innerHTML (get/set)|Flat-tree construction; slot assignment algorithm
Element|Node|attr namespaceURI; attr prefix; attr localName; attr tagName; attr id (get/set); attr className (get/set); attr classList (DOMTokenList); attr attributes (NamedNodeMap); attr children (live HTMLCollection); attr firstElementChild; attr lastElementChild; attr previousElementSibling; attr nextElementSibling; attr childElementCount; attr innerHTML (get/set); attr outerHTML (get/set); attr shadowRoot; attr assignedSlot; method hasAttribute(name); method getAttribute(name); method getAttributeNS(ns, name); method setAttribute(name, value); method setAttributeNS(ns, qname, value); method removeAttribute(name); method removeAttributeNS(ns, name); method toggleAttribute(name, force); method getAttributeNames(); method hasAttributes(); method closest(sel); method matches(sel); method webkitMatchesSelector(sel); method insertAdjacentElement(pos, el); method insertAdjacentText(pos, text); method insertAdjacentHTML(pos, html); method before(...nodes); method after(...nodes); method replaceWith(...nodes); method remove(); method append(...nodes); method prepend(...nodes); method querySelector(sel); method querySelectorAll(sel); method getElementsByTagName(qname); method getElementsByTagNameNS(ns, qname); method getElementsByClassName(names); method attachShadow(init); method scroll/scrollTo/scrollBy (viewport + container); method scrollIntoView(arg); method getBoundingClientRect(); method getClientRects(); method checkVisibility(opts); method setHTMLUnsafe (parse-with-template)|Namespace-aware attribute map; dataset via DOMStringMap; style attribute; shadow attach
HTMLElement|Element|attr title; attr lang; attr dir; attr hidden; attr inert; attr accessKey; attr draggable; attr spellcheck; attr tabIndex (get/set); attr dataset (DOMStringMap); attr style (CSSStyleDeclaration); attr translate; method click(); method focus(opts); method blur(); method showPopover/hidePopover/togglePopover (MVP: no-op events)|Common attribute surface; inert subtree behavior
Text|CharacterData|attr wholeText; attr assignedSlot; method splitText(offset) -> Text|Splitting merges/normalizes per spec
Comment|CharacterData|—|Serialization keeps <!-- -->
ProcessingInstruction|CharacterData|attr target|XML serialization path
CharacterData|Node|attr data (get/set); attr length; method substringData(offset, count); method appendData(text); method insertData(offset, text); method deleteData(offset, count); method replaceData(offset, count, text)|Base for Text/Comment/PI
Attr|Node|attr name; attr value (get/set); attr namespaceURI; attr prefix; attr localName; attr specified (always true)|Not a child of elements in this engine's tree model; live value reflection
NodeList||attr length; method item(i); method forEach(cb); iterable|Static (querySelectorAll) or live variant flag
HTMLCollection||attr length; attr namedItem support; method item(i); method namedItem(name); iterable|Live; recomputed against the tree on access (§5.6 pitfalls)
DOMTokenList||attr length; attr value (get/set); method item(i); method contains(token); method add(...tokens); method remove(...tokens); method toggle(token, force); method replace(old, new); method supports(token); method keys/values/entries|Whitespace-normalized; validation per interface (rel, class has none)
DOMStringMap||proxy-style named get/set/delete|dataset backing store
CSSStyleDeclaration||attr length; attr cssText (get/set); attr parentRule; method item(i); method getPropertyValue(prop); method setProperty(prop, value, priority); method removeProperty(prop); method getPropertyPriority(prop); method getPropertyShorthand; method isPropertyImplicit|style attribute live object; getComputedStyle returns a read-only variant
CSSStyleSheet|StyleSheet|attr cssRules; attr ownerRule; attr ownerNode; attr href; attr title; attr media; attr disabled (get/set); method insertRule(rule, index); method deleteRule(index); method replaceSync(text)|Rule list with source locations for DevTools
StyleSheet||attr type; attr href; attr ownerNode; attr parentStyleSheet; attr title; attr media (MediaList)|Base class
MediaQueryList||attr matches; attr media; method addListener(cb); method removeListener(cb); method addEventListener; onchange|Evaluated against current viewport; change events on resize
Range||attr startContainer; attr startOffset; attr endContainer; attr endOffset; attr collapsed; attr commonAncestorContainer; method setStart(node, off); method setEnd(node, off); method setStartBefore(node); method setStartAfter(node); method setEndBefore(node); method setEndAfter(node); method selectNode(node); method selectNodeContents(node); method collapse(toStart); method selectNodeContents; method compareBoundaryPoints(how, range); method deleteContents(); method extractContents(); method cloneContents(); method insertNode(node); method surroundContents(node); method cloneRange(); method detach(); method isPointInRange(node, off); method intersectsNode(node); method toString()|Boundary-point model with document-order comparison
Selection||attr anchorNode; attr anchorOffset; attr focusNode; attr focusOffset; attr isCollapsed; attr rangeCount; attr type; method getRangeAt(i); method addRange(range); method removeRange(range); method removeAllRanges(); method collapse(node, off); method setPosition; method collapseToStart/ToEnd; method extend(node, off); method setBaseAndExtent(anchor, aoff, focus, foff); method selectAllChildren(node); method containsNode(node, partial); method toString()|Backed by one document range; syncs with focus/inputs
NodeIterator||attr root; attr referenceNode; attr pointerBeforeReferenceNode; attr whatToShow; attr filter; method nextNode(); method previousNode(); method detach()|Traversal with filter states
TreeWalker||attr root; attr currentNode; attr whatToShow; attr filter; method parentNode(); method firstChild(); method lastChild(); method previousSibling(); method nextSibling(); method previousNode(); method nextNode()|Stateful cursor traversal
MutationObserver||method observe(target, options); method disconnect(); method takeRecords()|Records queued until the microtask checkpoint (§5.15)
MutationRecord||attr type; attr target; attr addedNodes; attr removedNodes; attr previousSibling; attr nextSibling; attr attributeName; attr attributeNamespace; attr oldValue|Constructed by the mutation hooks
ResizeObserver||method observe(target, options); method unobserve(target); method disconnect()|Fires before paint in the rendering update (§5.15.1)
IntersectionObserver||attr root; attr rootMargin; attr thresholds; method observe(target); method unobserve(target); method disconnect(); method takeRecords()|Single-viewport MVP per §5.6
Performance||attr timeOrigin; attr memory (reported); method now(); method mark(name); method measure(name, start, end); method getEntries(); method getEntriesByName(name, type); method getEntriesByType(type)|High-res clock capped to 100us granularity (timing-attack surface)
History||attr length; attr scrollRestoration; attr state; method back(); method forward(); method go(delta); method pushState(data, title, url); method replaceState(data, title, url)|Same-document session history entry mutation
Location||attr href (get/set); attr protocol; attr host; attr hostname; attr port; attr pathname; attr search; attr hash; attr origin; method assign(url); method replace(url); method reload(); method toString()|Unforgeable on window; navigation triggers (§5.19.2)
Navigator||attr userAgent; attr language; attr languages; attr platform; attr onLine; attr hardwareConcurrency; attr deviceMemory; attr maxTouchPoints; attr cookieEnabled; attr doNotTrack; attr globalPrivacyControl; method sendBeacon(url, data); method canShare/share (not supported, throw)|Static per browser; no fingerprinting extras
Screen||attr width; attr height; attr availWidth; attr availHeight; attr colorDepth; attr pixelDepth; attr devicePixelRatio (on window)|From the platform layer
Window|EventTarget|attr window/self/parent/top/frames; attr document; attr location (unforgeable); attr history; attr navigator; attr screen; attr innerWidth/innerHeight; attr outerWidth/outerHeight; attr pageXOffset/pageYOffset; attr scrollX/scrollY; attr devicePixelRatio; attr name (get/set); attr status; attr closed; attr length (frames); attr frames list; attr event (legacy, sloppy only); attr localStorage; attr sessionStorage; attr console; attr customElements; attr performance; attr origin; method open(url, target, features); method close(); method stop(); method focus(); method blur(); method print (no-op + console note); method alert(msg); method confirm(msg); method prompt(msg, def); method scroll/scrollTo/scrollBy(x, y); method moveTo/moveBy/resizeTo/resizeBy (shell-clamped); method getComputedStyle(el, pseudo); method matchMedia(q); method requestAnimationFrame(cb); method cancelAnimationFrame(id); method requestIdleCallback(cb); method cancelIdleCallback(id); method postMessage(message, targetOrigin, transfer); method getSelection(); method btoa/atob; method structuredClone; method fetch(input, init); method setInterval/setTimeout/clearInterval/clearTimeout; method queueMicrotask(cb); method reportError(err); method createImageBitmap (not supported)|The global object; event handlers via IDL attributes (on*); named property access on frames/named elements (legacy)
Storage||attr length; method key(i); method getItem(key); method setItem(key, value); method removeItem(key); method clear()|Per-origin (local) or per-tab (session); quota 5MB UTF-16 units (§5.17)
Event||attr type; attr target; attr currentTarget; attr eventPhase; attr bubbles; attr cancelable; attr defaultPrevented; attr composed; attr isTrusted; attr timeStamp; method stopPropagation(); method stopImmediatePropagation(); method preventDefault(); attr NONE/CAPTURING_PHASE/AT_TARGET/BUBBLING_PHASE|Base dispatch machinery (§5.6)
CustomUIEvent variants|see below|—|Each typed event is its own binding block with its init dict
UIEvent|Event|attr view; attr detail|Base for input-derived events
FocusEvent|UIEvent|attr relatedTarget|focus/blur/focusin/focusout
MouseEvent|UIEvent|attr screenX; attr screenY; attr clientX; attr clientY; attr offsetX; attr offsetY; attr pageX; attr pageY; attr button; attr buttons; attr relatedTarget; attr movementX; attr movementY; modifier getters (ctrlKey, shiftKey, altKey, metaKey); method getModifierState(keyArg)|click/dblclick/contextmenu/mouse* family
WheelEvent|MouseEvent|attr deltaX; attr deltaY; attr deltaZ; attr deltaMode|Scroll chaining decision lives in the shell/engine boundary
PointerEvent|MouseEvent|attr pointerId; attr width; attr height; attr pressure; attr tangentialPressure; attr tiltX; attr tiltY; attr twist; attr pointerType; attr isPrimary|Mouse events are synthesized from pointers per UI Events
KeyboardEvent|UIEvent|attr key; attr code; attr location; attr ctrlKey; attr shiftKey; attr altKey; attr metaKey; attr repeat; attr isComposing; attr charCode (legacy); attr keyCode (legacy); method getModifierState(keyArg)|keydown/keyup (+ legacy keypress); Appendix F mapping
InputEvent|UIEvent|attr data; attr dataTransfer; attr isComposing; attr inputType|beforeinput/input on editable elements
CompositionEvent|UIEvent|attr data; attr locale|IME composition start/update/end
DragEvent|MouseEvent|attr dataTransfer (DataTransfer: MVP types/files subset)|drag* family with the simple drag-store model
ProgressEvent|Event|attr lengthComputable; attr loaded; attr total|load/error/progress on fetches
MessageEvent|Event|attr data; attr origin; attr lastEventId; attr source; attr ports|postMessage delivery
ErrorEvent|Event|attr message; attr filename; attr lineno; attr colno; attr error|window error reporting
PromiseRejectionEvent|Event|attr promise; attr reason|unhandledrejection/rejectionhandled
HashChangeEvent|Event|attr oldURL; attr newURL|hashchange
AnimationEvent|Event|attr animationName; attr elapsedTime; attr pseudoElement|animationstart/iteration/end/cancel
TransitionEvent|Event|attr propertyName; attr elapsedTime; attr pseudoElement|transitionstart/transitionrun/end/cancel
CloseEvent|Event|attr wasClean; attr code; attr reason|WebSocket close
SecurityPolicyViolationEvent|Event|attr documentURI; attr referrer; attr blockedURL; attr statusCode; attr effectiveDirective; attr originalPolicy; attr sourceFile; attr lineNumber; attr columnNumber; attr disposition; attr sample|CSP violation reporting (§5.18.4)
FormData||method append(name, value, filename); method delete(name); method get(name); method getAll(name); method has(name); method set(name, value, filename); entries/iterable|Used by form submission and fetch bodies
URL||constructor(url, base); attr href (get/set); attr origin; attr protocol; attr username; attr password; attr host; attr hostname; attr port; attr pathname; attr search; attr hash; attr searchParams; method toJSON; static createObjectURL/revokeObjectURL (blob: MVP)|Wraps §5.1 parser; searchParams is URLSearchParams
URLSearchParams||constructor(init); attr size; method append(name, value); method delete(name); method get(name); method getAll(name); method has(name, value); method set(name, value); method sort(); entries/iterable; method forEach|application/x-www-form-urlencoded semantics (§5.1)
Blob||attr size; attr type; method slice(start, end, contentType); method stream (MVP: arrayBuffer); method arrayBuffer(); method text()|Backing store for File/fetch bodies
File|Blob|attr name; attr lastModified|From input files and drag-drop
FileReader||attr readyState; attr result; attr error; method readAsArrayBuffer(blob); method readAsText(blob, encoding); method readAsDataURL(blob); method abort()|Event-driven async reads on the document thread
Request||constructor(input, init); attr method; attr url; attr headers; attr destination; attr referrer; attr referrerPolicy; attr mode; attr credentials; attr cache; attr redirect; attr integrity; attr keepalive; attr signal; attr bodyUsed; method clone(); method arrayBuffer/text/blob/json|Fetch API client side (§5.15.5)
Response||constructor(body, init); attr url; attr ok; attr status; attr statusText; attr headers; attr redirected; attr type; attr bodyUsed; static error(); static redirect(url, status); method clone(); method arrayBuffer(); method text(); method json(); method blob()|Fetch API server side
Headers||constructor(init); method append(name, value); method delete(name); method get(name); method getSetCookie(); method has(name); method set(name, value); entries/iterable; method forEach|Case-insensitive; forbidden header names enforced
AbortController||attr signal; method abort(reason)|Cancels fetches and timers hooked to the signal
AbortSignal|EventTarget|attr aborted; attr reason; attr onabort; static abort(reason); static timeout(ms); method throwIfAborted()|Wired to the loader/timer cancellation tokens
WebSocket||constructor(url, protocols); attr url; attr readyState; attr bufferedAmount; attr extensions; attr protocol; attr binaryType; method send(data); method close(code, reason); events open/message/error/close|RFC 6455 handshake + framing (WBS §6.9); no subprotocol negotiation beyond echo
MessageChannel||attr port1; attr port2|Entangled ports for structured-clone messaging
MessagePort|EventTarget|method postMessage(message, transfer); method start(); method close(); events message/messageerror|MVP: same-document entangled pair only
DOMParser||method parseFromString(str, type) -> Document|text/html path reuses the full parser (§5.5); XML path rejects on well-formedness errors
XMLSerializer||method serializeToString(node)|Escaping table per §5.6
HTMLInputElement|HTMLElement|attr type (get/set); attr accept; attr alt; attr autocomplete; attr autofocus; attr checked (get/set); attr defaultChecked; attr form; attr formAction; attr formEnctype; attr formMethod; attr formNoValidate; attr formTarget; attr files; attr height/width; attr list; attr max/min; attr maxLength/minLength; attr multiple; attr name; attr pattern; attr placeholder; attr readOnly; attr required; attr size; attr src; attr step; attr value (get/set, value-modes per type); attr defaultValue; attr willValidate; attr validity (ValidityState fields); attr validationMessage; attr labels; method stepUp/stepDown(n); method select(); method setRangeText(rep, start, end, mode); method setSelectionRange(start, end, dir); attr selectionStart/End/Direction; method checkValidity(); method reportValidity(); method setCustomValidity(msg)|The workhorse control: text/checkbox/radio/button/submit/reset/file/hidden/password/range/number/email/url/search/tel/date (MVP subset); activation + constraint validation (§5.6)
HTMLTextAreaElement|HTMLElement|attr cols; attr rows; attr dirName; attr disabled; attr form; attr maxLength/minLength; attr name; attr placeholder; attr readOnly; attr required; attr wrap; attr value (get/set); attr defaultValue; attr textLength; selection attrs + methods; validation methods|Editable multi-line control; input events on mutation
HTMLSelectElement|HTMLElement|attr multiple; attr name; attr required; attr size; attr selectedIndex (get/set); attr value; attr length; attr options (live); attr selectedOptions; attr form; method add(el, before); method remove(index); method remove(); method item(i); method namedItem(name); validation methods|Option list model; change events
HTMLOptionElement|HTMLElement|attr disabled; attr form; attr label; attr defaultSelected; attr selected (get/set); attr value; attr text; attr index|Selectedness rules
HTMLButtonElement|HTMLElement|attr type; attr value; attr name; attr form; attr formAction/Enctype/Method/NoValidate/Target; attr disabled; validation methods|Activation: submit/reset/button (§5.6)
HTMLFormElement|HTMLElement|attr acceptCharset; attr action; attr autocomplete; attr enctype; attr encoding; attr method; attr name; attr noValidate; attr target; attr rel; attr elements (live); attr length; method submit(); method requestSubmit(submitter); method reset(); method checkValidity(); method reportValidity(); events submit/reset|Submission algorithm (GET url-encoding / POST body), constraint validation pass
HTMLLabelElement|HTMLElement|attr form; attr htmlFor (get/set); attr control|Activation forwarding to the labeled control
HTMLFieldSetElement|HTMLElement|attr form; attr name; attr disabled; attr type; attr elements|Disabled propagation to descendants
HTMLAnchorElement|HTMLElement|attr href (reflected); attr target; attr download; attr rel; attr relList; attr hreflang; attr type; attr referrerPolicy; attr text; protocol/host/pathname etc URL reflectors|Activation navigation (§5.19.2)
HTMLImageElement|HTMLElement|attr alt; attr src (get/set); attr srcset; attr sizes; attr crossOrigin; attr useMap; attr isMap; attr width/height (get/set); attr naturalWidth; attr naturalHeight; attr complete; attr currentSrc; attr decoding; attr loading; attr referrerPolicy; attr fetchPriority|decode pipeline wiring (§5.12)
HTMLCanvasElement|HTMLElement|attr width/height (get/set); method getContext(type, opts); method toDataURL(type); method toBlob(cb, type)|2D context only (§5.6 ctx below)
CanvasRenderingContext2D||attr canvas; save/restore; scale/rotate/translate/transform/setTransform/resetTransform; globalAlpha; globalCompositeOperation; fillStyle/strokeStyle (colors, gradients MVP); lineWidth; lineCap; lineJoin; miterLimit; lineDash attrs + setLineDash/getLineDash; shadow attrs; clearRect/fillRect/strokeRect; beginPath/closePath/moveTo/lineTo/quadraticCurveTo/bezierCurveTo/arc/arcTo/rect/ellipse/roundRect; fill/stroke/clip (path + Path2D); isPointInPath; drawImage (src rect variants); createLinearGradient/createRadialGradient (addColorStop); getImageData/putImageData/createImageData; measureText -> TextMetrics; fillText/strokeText (fonts: §5.11 path); direction attr|Backed by the same rasterizer (§5.16); state stack with fill/stroke paint objects
ImageData||attr width; attr height; attr data (Uint8ClampedArray)|Pixel format RGBA8
TextMetrics||attr width; attr actualBoundingBox* family; attr fontBoundingBox* family|From the shaper metrics
HTMLIFrameElement|HTMLElement|attr src; attr srcdoc; attr name; attr sandbox (DOMTokenList); attr allow; attr allowFullscreen; attr loading; attr width/height; contentDocument/contentWindow (SOP-checked)|Nested browsing context placeholder (§1.3)
HTMLMediaElement|HTMLElement|attr src; attr currentSrc; attr networkState; attr readyState; attr paused; attr duration; attr currentTime (get/set); attr volume; attr muted; attr playbackRate; method load(); method play() -> promise; method pause(); events loadstart/loadedmetadata/canplay/play/pause/ended/error|MVP: state machine + events fire, no codec decode (§1.4)
HTMLScriptElement|HTMLElement|attr src; attr type; attr noModule; attr async; attr defer; attr crossOrigin; attr text; attr integrity; attr referrerPolicy; attr fetchPriority|Execution queueing per §5.5.6
HTMLLinkElement|HTMLElement|attr href; attr rel; attr relList; attr media; attr hreflang; attr type; attr as; attr crossOrigin; attr referrerPolicy; attr disabled; attr sheet|Stylesheet loading (§5.7), favicon, preloads
HTMLStyleElement|HTMLElement|attr media; attr type; attr disabled; attr sheet|Inline sheet ownership
HTMLMetaElement|HTMLElement|attr name; attr content; attr httpEquiv; attr charset|Encoding + viewport MVP
HTMLTableElement|HTMLElement|attr caption; attr tHead; attr tFoot; attr rows (live); attr tBodies (live); method createCaption/deleteCaption/createTHead/deleteTHead/createTFoot/deleteTFoot; method insertRow(index); method deleteRow(index)|Table model helpers
HTMLTableRowElement|HTMLElement|attr rowIndex; attr sectionRowIndex; attr cells (live); method insertCell(index); method deleteCell(index)|Row model
HTMLTableCellElement|HTMLElement|attr colSpan; attr rowSpan; attr headers; attr cellIndex; attr scope (th); attr abbr (th)|Span validation for grid layout
HTMLDialogElement|HTMLElement|attr open (get/set); attr returnValue; method show(); method showModal(); method close(returnValue); event cancel|Top-layer + focus trapping + :modal
HTMLDetailsElement|HTMLElement|attr open (get/set); toggle event|Name-group accordion behavior
HTMLProgressElement|HTMLElement|attr value/max (get/set); attr position|Indeterminate state
HTMLMeterElement|HTMLElement|attr value/min/max/low/high/optimum (get/set)|Gauge regions
HTMLTemplateElement|HTMLElement|attr content (DocumentFragment)|Parser-invisible contents (§5.5)
HTMLSlotElement|HTMLElement|attr name; method assignedNodes(options); method assignedElements(options)|Slot assignment
HTMLOutputElement|HTMLElement|attr htmlFor; attr form; attr name; attr value (get/set); attr defaultValue|form-associated readout
Crypto||method getRandomValues(array); attr subtle (unsupported: throws cleanly)|CSPRNG from the platform layer only
Console||method log/debug/info/warn/error(table); method assert(cond, ...args); method clear(); method count(label); method countReset; method group/groupEnd/groupCollapsed; method time(label); method timeEnd/timeLog; method trace(...args); method table(data)|Console Standard formatting to the DevTools buffer (§5.15.4)
"""

# object|members|notes (members separated by "; ")
JS_BUILTINS = """
globalThis functions|globalThis; undefined; NaN; Infinity; eval(x); isFinite(x); isNaN(x); parseFloat(x); parseInt(x, radix); encodeURIComponent(s); decodeURIComponent(s); encodeURI(s); decodeURI(s); String/Number/Boolean/BigInt/Symbol/Object/Array constructors; ArrayBuffer/SharedArrayBuffer; DataView; TypedArray family; Map/Set/WeakMap/WeakSet/WeakRef/FinalizationRegistry; Promise; Proxy; Reflect; Date; RegExp; Error family; JSON; Math; Atomics; Intl; Function; AggregateError; parse module keys (import, import.meta in modules)|Host additions on the same global: setTimeout/setInterval/clear*/queueMicrotask/structuredClone/atob/btoa/fetch/console/performance
Object|static assign(target, ...src); static create(proto, props); static defineProperty(obj, key, desc); static defineProperties; static entries; static freeze; static isFrozen; static fromEntries; static getOwnPropertyDescriptor(s); static getOwnPropertyNames; static getOwnPropertySymbols; static getPrototypeOf/setPrototypeOf; static hasOwn; static is; static isExtensible; static isSealed; static keys; static preventExtensions; static seal; static values; proto constructor; proto hasOwnProperty; proto isPrototypeOf; proto propertyIsEnumerable; proto toLocaleString; proto toString ([object Tag]); proto valueOf; proto __proto__ accessor; proto __defineGetter__/__defineSetter__/__lookupGetter__/__lookupSetter__ (legacy)|Property descriptors: value/writable/get/set/enumerable/configurable; accessor vs data slots per spec
Function|proto length; proto name; proto prototype; proto apply(thisArg, args); proto bind(thisArg, ...args); proto call(thisArg, ...args); proto toString (source slice per §5.14.7); proto arguments/caller poisoned in strict; static (no)|[[Call]]/[[Construct]] separation; new.target; default-arg/rest/destructuring parameter forms
Boolean|constructor(value); proto toString; proto valueOf|Wrapper objects with sloppy-mode coercion
Symbol|static for(key); static keyFor(sym); static asyncIterator; static hasInstance; static isConcatSpreadable; static iterator; static match; static matchAll; static replace; static search; static species; static split; static toPrimitive; static toStringTag; static dispose/asyncDispose; proto description; proto toString; proto valueOf|Well-known symbol dispatch in the interpreter (§5.14.2)
Error|constructor(message, options); proto name; proto message; proto stack (own, captureStackTrace semantics); static isError (new); subclasses EvalError/RangeError/ReferenceError/SyntaxError/TypeError/URIError/AggregateError(errors, message)|options.cause; stack formatting with source spans (§5.13)
Number|EPSILON; MAX_SAFE_INTEGER; MIN_SAFE_INTEGER; MAX_VALUE; MIN_VALUE; NEGATIVE_INFINITY; POSITIVE_INFINITY; NaN; static isFinite; static isInteger; static isNaN; static isSafeInteger; static parseFloat; static parseInt; constructor(value); proto toExponential(digits); proto toFixed(digits); proto toLocaleString; proto toPrecision(precision); proto toString(radix); proto valueOf|Canonical numeric string forms per spec §6.1.6.1
BigInt|constructor(value); static asIntN(bits, v); static asUintN(bits, v); proto toString(radix); proto valueOf; proto toLocaleString|Mixed BigInt/Number arithmetic throws per spec
Math|E; LN10; LN2; LOG10E; LOG2E; PI; SQRT1_2; SQRT2; abs; acos; acosh; asin; asinh; atan; atan2; atanh; cbrt; ceil; clz32; cos; cosh; exp; expm1; floor; fround; hypot; imul; log; log10; log1p; log2; max; min; pow; random (seeded, test-injected); round; sign; sin; sinh; sqrt; tan; tanh; trunc|All semantics exactly per spec (rounding modes matter: round-half-up, floor toward -inf)
Date|constructor variants (no-arg, ms, string ISO, y/m/d/h/m/s/ms); static now(); static parse(s); static UTC(...); proto getDate/getDay/getFullYear/getHours/getMilliseconds/getMinutes/getMonth/getSeconds/getTime/getTimezoneOffset/getDate UTC variants; proto setDate/setFullYear/setHours/setMilliseconds/setMinutes/setMonth/setSeconds/setTime/setMilliseconds + UTC variants; proto toISOString; proto toJSON; proto toString/toDateString/toTimeString/toUTCString/toISOString/toLocaleString; proto valueOf; proto [Symbol.toPrimitive]|Local timezone from the platform; ISO parse subset per spec; invalid-date NaN semantics
String|static fromCharCode(...units); static fromCodePoint(...points); static raw(template); constructor(value); proto length (UTF-16 units); proto [index]; proto at(i); proto charAt(i); proto charCodeAt(i); proto codePointAt(i); proto concat; proto endsWith(s, end); proto includes(s, pos); proto indexOf(s, from); proto lastIndexOf(s, from); proto localeCompare; proto match(re); proto matchAll(re); proto normalize(form); proto padEnd(len, fill); proto padStart(len, fill); proto repeat(n); proto replace(search, repl); proto replaceAll(search, repl); proto search(re); proto slice(start, end); proto split(sep, limit); proto startsWith(s, pos); proto substring(a, b); proto substr (legacy annex); proto toLowerCase/toUpperCase; proto toLocaleLowerCase/UpperCase; proto trim; proto trimStart/trimEnd; proto [Symbol.iterator]; proto toString/valueOf|UTF-16 string model (§5.14.1); well-formedness on USV boundaries in host crossings
RegExp|constructor(pattern, flags); static (species); proto source; proto flags; proto global; proto ignoreCase; proto multiline; proto dotAll; proto unicode; proto unicodeSets; proto sticky; proto hasIndices; proto lastIndex (get/set); proto exec(s); proto test(s); proto toString; proto [Symbol.match/matchAll/replace/replaceAll/search/split]|Own regex engine: backtracking with the ES semantics (§6.7 WBS); no lookbehind initially — ADR when added
Array|static isArray(v); static from(src, mapFn, thisArg); static of(...items); static fromAsync; constructor(len or items); proto length (get/set with truncation rules); proto at(i); proto concat; proto copyWithin(target, start, end); proto entries; proto every(cb, thisArg); proto fill(value, start, end); proto filter(cb); proto find(cb)/findLast; proto findIndex/findLastIndex; proto flat(depth); proto flatMap(cb); proto forEach(cb); proto includes(v, from); proto indexOf(v, from); proto join(sep); proto keys; proto lastIndexOf(v, from); proto map(cb); proto pop; proto push(...items); proto reduce(cb, init)/reduceRight; proto reverse; proto shift; proto slice(start, end); proto some(cb); proto sort(cmp) (stable per spec); proto splice(start, deleteCount, ...items); proto toLocaleString; proto toReversed/toSorted/toSpliced/with (immutable set); proto unshift(...items); proto values; proto [Symbol.iterator]; proto [Symbol.species]|Hole semantics (empty slots) in iteration; species-driven subclass results
%TypedArray%|static from/of; proto buffer/byteLength/byteOffset/length; proto set(arr, offset); proto subarray(start, end); proto fill; proto copyWithin; proto indexOf/includes/lastIndexOf; proto join; proto reverse; proto sort(cmp) (numeric default); proto slice; proto entries/keys/values; proto at; proto find/findIndex family; proto every/some/forEach/map/filter/reduce family; static Int8Array/Uint8Array/Uint8ClampedArray/Int16Array/Uint16Array/Int32Array/Uint32Array/Float32Array/Float64Array/BigInt64Array/BigUint64Array|Bounds checks; canonical-numeric-index strings; detach semantics (no detach in MVP — never transferred)
ArrayBuffer|constructor(byteLength, opts); proto byteLength; proto slice(start, end); static isView(v)|SharedArrayBuffer: same shape, isShared flag
DataView|constructor(buffer, offset, length); proto buffer/byteLength/byteOffset; proto getBigInt64/BigUint64/getFloat32/Float64/getInt8/16/32/getUint8/16/32(offset, littleEndian); proto set* family (offset, value, littleEndian)|Alignment-free typed access; endianness argument default false (big)
Atomics|static add/and/compareExchange/exchange/load/or/store/sub/wait/notify/xor(typedArray, index, ...)|MVP: non-shared buffers allowed where spec permits; wait throws on non-shared
Map|constructor(iterable); attr size; proto clear(); proto delete(key); proto entries; proto forEach(cb, thisArg); proto get(key); proto has(key); proto keys; proto set(key, value); proto values; proto [Symbol.iterator]; proto [Symbol.toStringTag]|Insertion order; SameValueZero keys; hash-consed key table
Set|constructor(iterable); attr size; proto add(value); proto clear(); proto delete(value); proto entries; proto forEach; proto has(value); proto keys/values; proto union/intersection/difference/symmetricDifference/isSubsetOf/isSupersetOf/isDisjointFrom (ES2025 set methods); proto [Symbol.iterator]|SameValueZero membership
WeakMap|constructor; proto delete(key); proto get(key); proto has(key); proto set(key, value)|Keys are objects/symbols registered as weak refs in the GC
WeakSet|constructor; proto add(value); proto delete(value); proto has(value)|Weak membership
WeakRef|constructor(target); proto deref()|Sweep-time liveness
FinalizationRegistry|constructor(cleanup); proto register(target, held, token); proto unregister(token); proto cleanupSome()|Callbacks queued as tasks, never during GC (§5.14.6)
Promise|static all(iterable); static allSettled; static any; static race; static resolve(v); static reject(r); static try(fn); static withResolvers; constructor(executor); proto then(onFul, onRej); proto catch(onRej); proto finally(onSettled)|Reaction jobs drain at the microtask checkpoint (§5.15.3); unhandled-rejection tracking
Iterator helpers|proto map/filter/take/drop/flatMap/reduce/toArray/toAsync; proto [Symbol.iterator]; %IteratorPrototype% chain|Lazy iterator adapters; manual protocol implementation
Generator|%GeneratorPrototype%: proto next(v); proto return(v); proto throw(e); proto [Symbol.iterator]|Frame-suspend resume state machine (§5.14.4); async generators add the queue model
Proxy|constructor(target, handler); 13 traps invoked in spec order: getPrototypeOf/setPrototypeOf/isExtensible/preventExtensions/getOwnPropertyDescriptor/defineProperty/has/get/set/deleteOwnProperty/ownKeys/apply/construct|Invariant enforcement per spec; revocable via Proxy.revocable
Reflect|static apply(f, thisArg, args); static construct(f, args, newTarget); static defineProperty; static deleteProperty; static get(target, key, receiver); static getOwnPropertyDescriptor; static getPrototypeOf; static has; static isExtensible; static ownKeys; static preventExtensions; static set(target, key, value, receiver); static setPrototypeOf|Mirror of the internal methods; used by Proxy default behaviors
JSON|static parse(text, reviver); static stringify(value, replacer, space); static rawJSON (new); static isRawJSON|Own parser/serializer; exact number formatting (shortest round-trip double printing); well-formed stringify (lone surrogates escaped)
Intl|static Intl object with Collator/DateTimeFormat/NumberFormat/PluralRules/Segmenter constructors present but constructing throws NotSupportedError with a clear message|Graceful stub per §5.14.7; ADR to extend
console|log/debug/info/warn/error; assert; clear; count/countReset; group/groupEnd/groupCollapsed; time/timeEnd/timeLog; trace; table|§5.15.4 formatting into the DevTools ring buffer
URI functions|encodeURI/encodeURIComponent/decodeURI/decodeURIComponent per RFC 3986 tables|Escape/unescape (annex B) included for compatibility
Host timers|setTimeout(fn, delay, ...args); setInterval; clearTimeout; clearInterval|Clamping rules per HTML §8.1.4.2; token invalidation on navigation
Host misc|queueMicrotask(fn); structuredClone(value, opts); atob(s); btoa(s); reportError(err); fetch(input, init)|Wired per §5.15; structured clone supports the plain-data subset + ArrayBuffer/Map/Set/Date/RegExp/Error/Blob
Module records|import declarations; export forms (named/default/asterisk/string); import.meta; dynamic import(); top-level await|Module map with cycle handling (§5.13)
"""

# event|bubbles|cancelable|interface|fired-when (single line each)
DOM_EVENTS = """
load|N|N|Event|Resource/document finished loading (window, img, script, link, media)
DOMContentLoaded|Y|N|Event|HTML fully parsed and deferred scripts ran
readystatechange|Y|N|Event|document.readyState changed (loading/interactive/complete)
beforeunload|N|Y|BeforeUnloadEvent (MVP: Event)|Window about to unload; prompts are shell policy
unload|N|N|Event|Document unloading (legacy support)
pagehide/pageshow|N|N|PageTransitionEvent|Session history traversal in/out
error|N/N|N|ErrorEvent or Event|Script error (window, bubbles N) or resource fetch error (element, bubbles Y)
abort|Y|N|Event|Fetch aborted before completion (media elements)
hashchange|Y|N|HashChangeEvent|URL fragment changed
popstate|Y|N|PopStateEvent|Session history entry traversed
click|Y|Y|MouseEvent|Primary activation on an element (synthesized from pointer events)
dblclick|Y|Y|MouseEvent|Two clicks within platform threshold
contextmenu|Y|Y|MouseEvent|Secondary button / context menu key
mousedown/mouseup|Y|Y|MouseEvent|Button press/release
mousemove|Y|Y|MouseEvent|Pointer moved over the document
mouseover/mouseout|Y|Y|MouseEvent|Pointer entered/left an element's hit area (bubbling)
mouseenter/mouseleave|N|N|MouseEvent|Non-bubbling enter/leave on the element itself
wheel|Y|Y|WheelEvent|Scroll wheel/delta input; preventDefault stops scrolling
pointerdown/pointerup|Y|Y|PointerEvent|Pointer press/release (pointerId model)
pointermove|Y|Y|PointerEvent|Pointer moved
pointerover/pointerout|Y|Y|PointerEvent|Bubbling pointer enter/leave
pointerenter/pointerleave|N|N|PointerEvent|Non-bubbling enter/leave
pointercancel|Y|N|PointerEvent|Pointer interaction taken over (touch scroll)
gotpointercapture/lostpointercapture|Y|N|PointerEvent|setPointerCapture transitions (MVP: implicit capture on touch)
keydown|Y|Y|KeyboardEvent|Key pressed; preventDefault stops text input and default actions
keyup|Y|Y|KeyboardEvent|Key released
keypress|Y|Y|KeyboardEvent|Legacy character-producing key (sourced from keydown)
beforeinput|Y|Y|InputEvent|Editable content about to change; preventDefault blocks it
input|Y|N|InputEvent|Editable content changed (text areas, inputs, select)
change|Y|N|Event|Commit of a new value (checkbox, radio, select, file, blur-commit of text)
compositionstart/compositionupdate/compositionend|Y|N|CompositionEvent|IME session lifecycle
focus|N|N|FocusEvent|Element received focus
blur|N|N|FocusEvent|Element lost focus
focusin/focusout|Y|N|FocusEvent|Bubbling focus transitions
submit|Y|Y|SubmitEvent|Form submission requested (preventDefault blocks navigation)
reset|Y|Y|Event|Form reset requested
select|Y|N|Event|Text selection changed inside an editable
invalid|Y|N|Event|Constraint validation failed (reportValidity path)
search|Y|N|Event|type=search Enter (legacy)
dragstart/dragend|Y|Y/N|DragEvent|Drag session lifecycle (simple drag-store MVP)
dragenter/dragleave/dragover/drop|Y|Y for over/drop|DragEvent|Drag over targets; drop needs preventDefault(over)+drop
scroll|Y/N|N|Event|Element scrolled (bubbles: document) — also fires rAF-aligned on programmatic scrolls
resize|N|N|Event|Viewport (window) resized
canplaythrough/loadedmetadata/loadeddata|N|N|Event|Media element state advances (stub per §1.4)
play/pause/ended|N|N|Event|Media element state (stub)
transitionrun/transitionstart/transitionend/transitioncancel|Y|N|TransitionEvent|CSS transition lifecycle
animationstart/animationiteration/animationend/animationcancel|Y|N|AnimationEvent|CSS animation lifecycle
message|N|N|MessageEvent|postMessage/MessagePort delivery
messageerror|N|N|MessageEvent|Undeserializable message (MVP: rare)
online/offline|N|N|Event|Network connectivity changed
storage|N|N|StorageEvent|localStorage changed in another same-origin tab
visibilitychange|Y|N|Event|Tab visibility changed (shell hook)
fullscreenchange|Y|N|Event|Never fires in this engine (non-goal §1.4); binding exists
copy/cut/paste|Y|Y|ClipboardEvent|Clipboard operations (text-only MVP)
securitypolicyviolation|Y|N|SecurityPolicyViolationEvent|CSP violation detected (§5.18.4)
unhandledrejection|Y|Y|PromiseRejectionEvent|Promise rejected with no handler at checkpoint
rejectionhandled|Y|N|PromiseRejectionEvent|Late handler attached to a reported rejection
toggle|N|N|Event|details open state changed
"""

# header|direction|engine-behavior (single line each)
HTTP_HEADERS = """
Host|request|Mandatory for http/1.1 special schemes; validated non-empty.
User-Agent|request|`Aurora/0.1 (+project)`; single well-known string, no spoofing options.
Accept|request|`text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8` for documents; narrower for subresources.
Accept-Language|request|From shell prefs (`en-US,en;q=0.9` default); feeds :lang fallback matching.
Accept-Encoding|request|`gzip, deflate` (identity until compressor lands; §5.2.2).
Connection|both|`keep-alive` default; `close` honored both directions.
Content-Length|both|Framing for bodies; conflicts with chunked are protocol errors (§5.2.3).
Transfer-Encoding|response|`chunked` decoding implemented; other values are protocol errors.
Content-Type|both|Drives document-type sniffing (§5.2.7) and stylesheet/script/image load decisions; `nosniff` interplay.
Cache-Control|response|Freshness directives: max-age, no-store, no-cache, must-revalidate, immutable, s-maxage ignored (no shared cache) (§5.17.4).
Expires|response|Legacy freshness; superseded by Cache-Control when both present.
Pragma|response|`no-cache` legacy honored; otherwise ignored.
Age|response|Seconds since origin generation; feeds freshness math.
Date|response|Stored for heuristic freshness and Age computation.
ETag|response|Strong/weak validators stored; used for If-None-Match revalidation.
If-None-Match|request|Conditional GET on revalidation; 304 applies stored headers.
If-Modified-Since|request|Date-based revalidation fallback.
Last-Modified|response|Stored for If-Modified-Since.
Vary|response|Per-header-value cache keying; `*` disables reuse (§5.2.6).
Location|response|Redirect target resolution per §5.2.4 (relative allowed).
Refresh|response|Non-standard meta refresh equivalent: parse and schedule navigation with a console note.
Content-Encoding|response|`gzip`/`deflate`/`br`(unsupported → error) decoded before the pipeline.
Content-Disposition|response|`attachment` triggers the download manager with filename parsing (§5.19.2).
Set-Cookie|response|Full RFC 6265bis parse in §5.17.1; multiple headers supported.
Cookie|request|Jar lookup with path/domain/secure/samesite rules.
Strict-Transport-Security|response|HSTS policy parse and persist (§5.3).
Access-Control-Allow-Origin|response|CORS check; wildcard-with-credentials rules (§5.18.3).
Access-Control-Allow-Methods|response|Preflight method check.
Access-Control-Allow-Headers|response|Preflight header check (case-insensitive).
Access-Control-Expose-Headers|response|Restricts JS-visible response headers on CORS responses.
Access-Control-Allow-Credentials|response|true enables credentialed CORS.
Access-Control-Max-Age|response|Preflight result cache TTL.
Access-Control-Request-Method|request|Preflight request marker.
Access-Control-Request-Headers|request|Preflight header list.
Origin|request|Sent on CORS and non-GET same-site requests; feeds the CORS check.
Referer|request|Per referrer policy (§5.18.5); strip on downgrade.
Referrer-Policy|response|Updates the document's referrer policy (strict-origin-when-cross-origin default).
Content-Security-Policy|response|Parse and enforce per §5.18.4 (report-only variant reports only).
X-Content-Type-Options|response|`nosniff` disables sniffing (§5.2.7); blocks wrong-type scripts/styles.
X-Frame-Options|response|DENY/SAMEORIGIN honored for iframe embedding decisions.
Retry-After|response|Parse only (no auto-retry policy in MVP).
Range/Accept-Ranges/Content-Range|both|Range requests for media (MVP: not sent; parse and ignore).
Allow|response|405 reporting only.
WWW-Authenticate/Authorization|both|Parsed; engine sends Basic only when the shell credential prompt (MVP: none) supplies it; otherwise 401 error page.
Upgrade|response|101 for WebSocket handshake (§6.9); otherwise ignored.
Sec-WebSocket-*|both|RFC 6455 handshake headers generated/parsed by the WebSocket client.
Server|response|Logged in the network panel only.
Alt-Svc|response|Ignored (no HTTP/2 in MVP) with a DevTools note.
Link|response|Preload hints parsed as best-effort; stylesheet variant honored.
Timing-Allow-Origin|response|Marks cross-origin resources for high-res timing exposure.
Cross-Origin-Opener-Policy / -Embedder-Policy / -Resource-Policy|response|COOP: report-only in MVP; COEP: ignored with console note; CORP: enforced for no-cors subresource fetches.
"""

# mime|engine-behavior (single line each)
MIME_TYPES = """
text/html|HTML document pipeline (§5.4-5.5); charset parameter honored.
application/xhtml+xml|XML-ish: parse well-formed-only; failure renders the error document (MVP lenient note).
text/plain|Plain text document; wrap in <pre>-like display document.
text/css|Stylesheet only when the embedding allows it (nosniff honored).
text/javascript + application/javascript|Classic/module script depending on the element's type attribute.
application/json|Fetchable; standalone navigation renders a JSON viewer (MVP: plain text).
image/png, image/jpeg, image/gif, image/bmp|Image decoders (§5.12); standalone navigation renders the image in a generated document.
image/x-icon + image/vnd.microsoft.icon|ICO decoder; favicon selection source.
image/webp, image/avif, image/svg+xml|Not decodable (§1.3): treated as unsupported image (placeholder + error event for svg-in-img; webp/avif error).
application/pdf|Download (non-goal to render).
audio/*, video/*|Media element stub accepts; no decode (§1.4).
application/octet-stream|Download.
multipart/form-data|Request-side encoding for form POST (§5.6 forms); response-side multipart/x-mixed-replace ignored.
application/x-www-form-urlencoded|Form GET/POST body encoding (§5.1).
text/xml, application/xml|Well-formedness-checking parse; used by DOMParser text/xml.
font/woff2, font/woff, font/ttf, font/otf|Font loading from @font-face src (§5.11); format hints validated.
application/wasm|Not supported (stretch goal §1.5): fetch error page.
unknown/missing types|Sniffing per §5.2.7 for images and top-level documents; otherwise download.
"""

# key-or-code|category|default-action-note
KEYBOARD_MAP = """
Backquote/Digit1..Digit0/Minus/Equal|typing keys|Produce characters via the platform layout map
KeyQ..KeyM (letters)|typing keys|Key values per current layout; printable keys feed text input
Space|typing keys|Character U+0020; page-scrolls when focus is not editable
Tab|navigation|Moves focus in DOM order (Shift reverses); preventDefault gives raw key
Enter|activation|Activates focused control; submits single-input forms; text areas insert newline
NumpadEnter|activation|Same as Enter
Backspace|editing|Deletes backwards in editables; history-back only when shell policy allows
Delete|editing|Deletes forwards in editables
Escape|navigation|Closes dialogs/menus; exits fullscreen (no-op here); cancels IME
ArrowLeft/ArrowRight|navigation|Text caret movement in editables; horizontal scroll fallback
ArrowUp/ArrowDown|navigation|Line navigation in editables; option cycling in select; page scroll fallback
Home/End|navigation|Line start/end in editables; scroll to edges in pages
PageUp/PageDown|navigation|Scroll by page
Insert|editing|Toggle overwrite mode (MVP: no-op)
F1..F12|function keys|Shell shortcuts where mapped (F12 DevTools, F5 reload, F11 fullscreen toggle, F6 focus omnibox, F10 menu)
Shift/Control/Alt/Meta (Left/Right)|modifiers|Modifier state machinery for getModifierState and shortcuts
CapsLock|modifier|Toggle state surfaced to getModifierState
NumLock/ScrollLock|modifier|State surfaced; no default behavior
ContextMenu|menu|Opens the context menu at selection/caret
PrintScreen|misc|No engine behavior
Ctrl+T / Ctrl+W / Ctrl+Shift+T|shell|New tab / close tab / reopen closed tab
Ctrl+L / Alt+D / F6|shell|Focus the omnibox
Ctrl+R / F5 / Ctrl+Shift+R|shell|Reload / reload (bypass cache)
Alt+Left / Alt+Right|shell|Back / forward
Ctrl+D|shell|Bookmark current page
Ctrl+F|shell|Find in page
Ctrl+Plus/Minus/Zero|shell|Zoom in/out/reset (10% steps)
Ctrl+U|shell|View source (generated document from --dump-dom path)
Ctrl+S|shell|Save page (MHTML-lite MVP: single-file HTML)
Ctrl+J|shell|Open downloads
Ctrl+H|shell|Open history
Ctrl+Shift+I / F12|shell|Toggle DevTools
Ctrl+Shift+C|shell|Inspect element mode (pick on next click)
Ctrl+Tab / Ctrl+Shift+Tab|shell|Next/previous tab
Ctrl+1..8 / Ctrl+9|shell|Select tab by index / last tab
Ctrl+P|shell|Print (no-op with console note; non-goal §1.4)
"""

# message-id|level|trigger (single line each)
CONSOLE_MESSAGES = """
ParseError.html|info|HTML parse error with line:col and the standard's error code (§5.4).
ParseError.css|info|CSS parse error with line:col and the recovered-at token (§5.7).
Encoding.restarted|info|A late <meta charset> forced re-tokenization (§5.5.5).
Encoding.fallback|warn|No encoding declaration found; windows-1252 assumed per spec.
Net.protocolError|error|HTTP/1.1 violation with the offending bytes summarized (§5.2.3).
Net.badStatus|error|Status line outside 100–599; treated as network error.
Tls.verificationFailed|error|Certificate failure with the concrete rustls alert (§5.3).
HSTS.applied|info|Navigation upgraded http→https by HSTS policy.
CORS.blocked|error|Fetch failed the CORS check; reason not exposed to script (§5.18.3).
MixedContent.blocked|error|Blockable mixed content refused on an https page (§5.18.5).
MixedContent.allowed|warn|Optionally-blockable mixed content loaded (images) and reported.
CSP.violation|error|Directive violated; the resource was blocked; event fired (§5.18.4).
CSP.reportOnly|warn|Directive violated in report-only mode; resource allowed.
Storage.quotaExceeded|error|setItem over quota; QuotaExceededError thrown to script (§6.10).
Cookie.rejected|info|Set-Cookie rejected with the RFC rule that rejected it (§5.17.1).
Cache.revalidated|info|Conditional request returned 304; served from cache.
Dom.liveCollectionMutation|warn|Live collection mutated during iteration (perf hint).
Dom.passiveDefault|warn|preventDefault called on a passive wheel/touch listener (§5.6).
Js.unhandledRejection|error|Promise rejected with no handler at the microtask checkpoint (§5.14.5).
Js.longTask|warn|Task exceeded 50 ms with its source (§5.15.6).
Js.trapInvariant|error|Proxy trap returned a spec-invariant-violating value (§6.7).
Image.decodeError|error|Image bytes failed decoding; error event fired (§5.12).
Image.unsupportedFormat|warn|Format outside the supported set (webp/avif/svg-in-img, §1.3).
Font.loadFailed|error|@font-face source failed; fallback face used (§5.11.1).
Font.fallbackUsed|info|Glyphs missing in the primary font; fallback chain applied.
Shaper.complexDelegated|info|Complex script shaped via the HarfBuzz-backed shaper (§5.11.2).
Layout.percentageIndefinite|info|Percentage height resolved as auto due to indefinite containing block (§5.9.2).
Paint.clipUnbalanced|error|Internal: display list pushed/popped clips unevenly (should be unreachable; a bug report).
Shell.downloadStarted|info|Attachment Content-Disposition routed to the download manager (§5.19.2).
Shell.fileOrigin|warn|file:// fetch permitted within the directory policy (§5.18.6).
NotSupported.feature|info|Script requested a non-goal feature (§1.4); NotSupportedError path taken.
"""

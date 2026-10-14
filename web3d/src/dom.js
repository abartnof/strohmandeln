// A small element builder shared by the table's DOM modules (the slate, the
// settings menu). It takes the document from the root it builds under, so a
// test can give it a tiny fake one.

// h(doc, "button", {class: "x", onclick: fn, "aria-label": "..."}, ...children)
// `class` sets the class name, `on<event>` adds a listener, a null or false
// attribute is left out, and a string child is text.
export function h(doc, tag, attrs = {}, ...kids) {
  const el = doc.createElement(tag);
  for (const [k, v] of Object.entries(attrs)) {
    if (v === null || v === undefined || v === false) continue;
    if (k === "class") el.className = v;
    else if (k.startsWith("on")) el.addEventListener(k.slice(2), v);
    else el.setAttribute(k, v === true ? "" : v);
  }
  el.append(...kids.flat().filter((c) => c !== null && c !== undefined && c !== false));
  return el;
}

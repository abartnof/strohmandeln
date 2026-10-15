// A tiny DOM for the table's modules: enough of Element for dom.js, the
// settings menu and the slate to build a tree and for a test to click it.
// Not a browser: no layout, no selectors beyond `find`.

class ClassList {
  constructor(el) { this.el = el; }
  _set() { return new Set(this.el.className.split(/\s+/).filter(Boolean)); }
  add(...c) { const s = this._set(); c.forEach((x) => s.add(x)); this.el.className = [...s].join(" "); }
  remove(...c) { const s = this._set(); c.forEach((x) => s.delete(x)); this.el.className = [...s].join(" "); }
  contains(c) { return this._set().has(c); }
  toggle(c, force) { const on = force ?? !this.contains(c); if (on) this.add(c); else this.remove(c); return on; }
}

export class FakeElement {
  constructor(tag, doc) {
    this.tagName = tag.toUpperCase();
    this.ownerDocument = doc;
    this.children = [];
    this.parent = null;
    this.attributes = {};
    this.className = "";
    this.hidden = false;
    this.listeners = {};
    this.classList = new ClassList(this);
    this._text = "";
    this.focused = false;
  }
  get textContent() { return this._text + this.children.map((c) => c.textContent).join(""); }
  set textContent(v) { this.children = []; this._text = String(v); }
  setAttribute(k, v) { this.attributes[k] = String(v); }
  getAttribute(k) { return k in this.attributes ? this.attributes[k] : null; }
  removeAttribute(k) { delete this.attributes[k]; }
  append(...kids) { for (const k of kids) { const n = typeof k === "string" ? new FakeText(k) : k; n.parent = this; this.children.push(n); } }
  appendChild(k) { this.append(k); return k; }
  replaceChildren(...kids) { this.children = []; this._text = ""; this.append(...kids); }
  addEventListener(type, fn) { (this.listeners[type] ??= []).push(fn); }
  dispatch(type, extra = {}) {
    const ev = { type, target: this, defaultPrevented: false, preventDefault() { this.defaultPrevented = true; }, ...extra };
    for (const fn of this.listeners[type] ?? []) fn(ev);
    return ev;
  }
  click() { return this.dispatch("click"); }
  focus() { this.focused = true; }
  // All descendants (and this) satisfying `pred`.
  findAll(pred, out = []) {
    if (pred(this)) out.push(this);
    for (const c of this.children) if (c instanceof FakeElement) c.findAll(pred, out);
    return out;
  }
  find(pred) { return this.findAll(pred)[0] ?? null; }
  byAttr(k, v) { return this.findAll((e) => e.getAttribute(k) === String(v)); }
  byClass(c) { return this.findAll((e) => e.classList.contains(c)); }
}

class FakeText {
  constructor(s) { this.data = String(s); this.parent = null; }
  get textContent() { return this.data; }
}

export function fakeDocument() {
  const doc = { createElement: (tag) => new FakeElement(tag, doc) };
  return doc;
}

export function fakeRoot() {
  return fakeDocument().createElement("div");
}

// A storage like localStorage; `broken` makes every call throw.
export function fakeStore(initial = {}, { broken = false } = {}) {
  const m = new Map(Object.entries(initial));
  const guard = () => { if (broken) throw new Error("blocked"); };
  return {
    getItem: (k) => { guard(); return m.has(k) ? m.get(k) : null; },
    setItem: (k, v) => { guard(); m.set(k, String(v)); },
    removeItem: (k) => { guard(); m.delete(k); },
    map: m,
  };
}

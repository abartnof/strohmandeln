// The tutor, a period Kibitzer (docs/TABLE3D.md 6): the tutorial pages for the
// sitting's settings. It holds no rules of
// play: it reads the events and the module's state (docs/PROTOCOL.md) and the
// words it says are keys of the strings and of the phrase bank. The view
// (the tab, the dialogs) is the page's; this is what is decided.
//
// Cassino's tutorial.js (pageDue, the seen list) is the pattern.

import { parseTutorial } from "../../kit/js/tutorial.js";

// The pages of a tutorial file for the sitting's rule settings. A block of
// lines between `<!-- if name=value -->` and `<!-- end -->` is kept when the
// setting `name` is `value`, and dropped, markers and all, when it is not
// (the bidding page says what happens when both pass, by `both_pass`).
export function tutorialPages(markdown, keys, settings = {}) {
  let keep = true;
  const lines = [];
  for (const line of markdown.split("\n")) {
    const open = /^<!-- if (\w+)=(\S+) -->$/.exec(line.trim());
    if (open) keep = String(settings[open[1]]) === open[2];
    else if (line.trim() === "<!-- end -->") keep = true;
    else if (keep) lines.push(line);
  }
  return parseTutorial(lines.join("\n"), keys);
}

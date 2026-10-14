// When each thing is seen at the table, in ms on the director's clock: every
// event at its beat, and the two moments the talk is told of the count
// (docs/TABLE3D.md 5.2.3): `count_start` as the first card turns, and
// `count_close` just before the count's last popup is done -- before the
// result is passed on, which the talk drops while the count is still quiet.
//
// momentsOf({ events, since, beats, count }) -> [{ ms, event, index, synthetic }] in order.
export function momentsOf({ events, since, beats, count }) {
  const out = events.map((event, k) => ({ ms: beats[since + k] ?? 0, event, index: since + k, synthetic: false }));
  if (count) {
    // As the count begins (its stage's start), and in any case before the first card.
    const began = Math.min(beats[count.at] ?? Infinity, count.lines[0] - 1);
    out.push({ ms: Math.max(0, began), event: { type: "count_start" }, index: -1, synthetic: true });
    out.push({ ms: Math.max(0, count.end - 1), event: { type: "count_close" }, index: -1, synthetic: true });
  }
  return out.map((m, order) => ({ ...m, order })).sort((a, b) => a.ms - b.ms || a.order - b.order);
}

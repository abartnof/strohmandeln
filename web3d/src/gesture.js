// What a finger or a mouse does on a card (docs/TABLE3D.md 3.1.2, 3.1.3):
// a drag toward the trick plays the card, a sideways drag scrolls a wide row,
// and a refused card shakes. Gestures only: which cards may be played is
// `state.legal`, and nothing here knows.

export const DRAG = Object.freeze({ tap: 6, up: 48, steep: 1.5 });

// What a press-move-release came to, from its distance in screen pixels:
// "tap" (hardly moved), "play" (drawn up toward the trick, more up than aside),
// "scroll" (drawn sideways) or "none" (moved, but neither).
export function dragKind(dx, dy, { tap = DRAG.tap, up = DRAG.up, steep = DRAG.steep } = {}) {
  if (Math.hypot(dx, dy) < tap) return "tap";
  if (-dy >= up && -dy > steep * Math.abs(dx)) return "play";
  if (Math.abs(dx) >= tap && Math.abs(dx) > Math.abs(dy)) return "scroll";
  return "none";
}

// The sideways offset of a shaken card, in table units, `ms` into the shake:
// three swings that die away, back at rest when it ends.
export const SHAKE = Object.freeze({ ms: 360, swings: 3, amplitude: 0.45 });
export function shakeOffset(ms, { ms: total = SHAKE.ms, swings = SHAKE.swings, amplitude = SHAKE.amplitude } = {}) {
  if (ms <= 0 || ms >= total) return 0;
  const u = ms / total;
  return amplitude * (1 - u) * Math.sin(2 * Math.PI * swings * u);
}

// Shake a mesh about where it lies: `frame(fn)` asks for the next frame (the
// caller's requestAnimationFrame), `now()` is the clock, `render()` draws.
// Reduced motion makes no movement. Returns whether it shook.
const shaking = new WeakMap(); // mesh -> { x0, run }: the resting place of a card being shaken
export function shake(mesh, { now, frame, render, reduced = false }) {
  if (reduced || !mesh) return false;
  const x0 = shaking.get(mesh)?.x0 ?? mesh.position.x; // pressed again mid-shake: the rest stays where it was
  const start = now();
  const run = () => {
    if (shaking.get(mesh)?.run !== run) return; // a later shake took over
    const ms = now() - start;
    mesh.position.x = x0 + shakeOffset(ms);
    render();
    if (ms < SHAKE.ms) frame(run);
    else {
      mesh.position.x = x0;
      shaking.delete(mesh);
    }
  };
  shaking.set(mesh, { x0, run });
  frame(run);
  return true;
}

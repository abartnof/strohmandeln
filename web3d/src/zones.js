// Where everything rests (docs/TABLE3D.md 1), in centimetres on the table: the
// human at +z, the opponent at -z, x to the human's right, y up. Three
// arrangements share one shape, so the layout reads any of them:
//
//   ZONES            a computer, or a tablet held sideways: your hand a fan
//   ZONES_TOUCH      a tablet held upright, or a large touch screen: nearer eye,
//                    cards about a tenth larger; your hand a row
//   ZONES_PORTRAIT   a phone held upright: stacked, your hand a row that scrolls
//
// The eyes are the kit's camera numbers with their own `reach` (the extent of
// the table the eye must keep in view, as tangents of its axis, measured by
// test/layout.test.js against these zones).
import { CAMERA, CAMERA_PORTRAIT, CAMERA_TOUCH, CARD, PITCH, facingEye } from "../../kit/js/units.js";

const DEG = Math.PI / 180;
const over = (target, distance) => Object.freeze([target[0], target[1] + distance * Math.sin(PITCH * DEG), target[2] + distance * Math.cos(PITCH * DEG)]);

const TARGET = Object.freeze([0, 0, -12]);
export const EYE = Object.freeze({
  ...CAMERA,
  position: over(TARGET, 118),
  target: TARGET,
  fov: 34,
  widthTan: 0.4,
  reach: Object.freeze({ up: 0.31, foot: 0.315 }),
});
const TARGET_TOUCH = Object.freeze([0, 0, -12]);
export const EYE_TOUCH = Object.freeze({
  ...CAMERA_TOUCH,
  position: over(TARGET_TOUCH, 108),
  target: TARGET_TOUCH,
  fov: 30,
  widthTan: 0.4,
  reach: Object.freeze({ up: 0.335, foot: 0.3 }),
});
const TARGET_PORTRAIT = Object.freeze([0, 0, -12]);
export const EYE_PORTRAIT = Object.freeze({
  ...CAMERA_PORTRAIT,
  position: over(TARGET_PORTRAIT, 96),
  target: TARGET_PORTRAIT,
  reach: Object.freeze({ up: 0.32, down: -0.34, across: 0.24 }),
});

// The shape of every arrangement.
//   yourHand / theirHand  centre [x,y,z]; a fan { radius, spacing } or a row { pitch }
//   packets               z of each side's row, x of packets 1 to 3
//   trick                 centre of the open trick; the lead and the follow `gap` apart
//   won                   each side's pile
//   pack                  where the pack lies, by who deals
//   count                 the count's rows, by side
const hand = (centre, eye, extra) => Object.freeze({ centre: Object.freeze(centre), lean: facingEye(eye.position, centre), ...extra });

function arrangement({ eye, yours, theirs, packets, trick, won, pack, count, held = 3.2 }) {
  return Object.freeze({
    eye: eye.position,
    camera: eye,
    yourHand: yours,
    theirHand: theirs,
    packets: Object.freeze(packets),
    trick: Object.freeze(trick),
    won: Object.freeze(won),
    pack: Object.freeze(pack),
    count: Object.freeze(count),
    held,
    stack: 0.05, // a card's thickness plus the hair between
  });
}

export const ZONES = arrangement({
  eye: EYE,
  yours: hand([0, 2.5, 10], EYE, { fan: true, radius: 40, spacing: 2.9, width: 46 }),
  theirs: hand([0, 7, -44], EYE, { fan: true, radius: 40, spacing: 1.4, width: 30 }),
  packets: { you: -3, them: -27, xs: [-17, 0, 17] },
  trick: { x: 0, z: -15, gap: 0.8, offset: 2.4 },
  won: { x: 40, you: 2, them: -17 },
  pack: { you: { x: 40, z: 2 }, them: { x: -40, z: -17 } },
  count: { perRow: 6, step: 2.3, gap: 1.4, row: 5.5, you: { z: 12 }, them: { z: -38 }, x: 0 },
});

export const ZONES_TOUCH = arrangement({
  eye: EYE_TOUCH,
  yours: hand([0, 1.2, 14], EYE_TOUCH, { fan: false, pitch: 3.2, width: 52 }),
  theirs: hand([0, 7, -44], EYE_TOUCH, { fan: true, radius: 40, spacing: 1.4, width: 30 }),
  packets: { you: 1, them: -27, xs: [-17, 0, 17] },
  trick: { x: 0, z: -12, gap: 0.8, offset: 2.4 },
  won: { x: 36, you: 4, them: -17 },
  pack: { you: { x: 36, z: 4 }, them: { x: -36, z: -17 } },
  count: { perRow: 5, step: 2.3, gap: 1.4, row: 5.5, you: { z: 10 }, them: { z: -38 }, x: 0 },
});

export const ZONES_PORTRAIT = arrangement({
  eye: EYE_PORTRAIT,
  yours: hand([0, 2, 13], EYE_PORTRAIT, { fan: false, pitch: 3.2, width: 34 }),
  theirs: hand([0, 6, -38], EYE_PORTRAIT, { fan: true, radius: 30, spacing: 1.1, width: 20 }),
  packets: { you: -3, them: -27, xs: [-11, 0, 11] },
  trick: { x: 0, z: -15, gap: 0.8, offset: 2.2 },
  won: { x: 17, you: 8, them: -8 },
  pack: { you: { x: 17, z: 8 }, them: { x: -17, z: -8 } },
  count: { perRow: 3, step: 2.3, gap: 1.2, row: 5.5, you: { z: 12 }, them: { z: -38 }, x: 0 },
});

export { CARD };

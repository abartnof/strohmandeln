// The layout: where all 54 cards rest for a model, and which of them show
// their faces (docs/TABLE3D.md 1). A pure function of the model and of how
// the person has asked to see it (the card chosen, the row scrolled).
//
// layout(model, view) -> 54 slots { key, zone, index, code, faceUp, pose, public }
//
// `code` is a card only where the person may know it AND the card is drawn
// face up: their hand, the covers, the held card, the trick, the cards turned
// up in the count. The opponent's hand (known cards among it too), the hidden
// cards of every packet, the pack and the won piles are null: backs. Nothing
// in the scene can show a card the human could not see at a real table (0.2).
//
// The structure is cassino's layout.js (jitter, hands, piles), for this
// table's zones.

import { Quaternion, Vector3 } from "three";
import { fan, lying, row } from "../../kit/js/kinematics.js";
import { CARD, facingEye } from "../../kit/js/units.js";
import { ZONES } from "./zones.js";

const DEG = Math.PI / 180;
const REST = 0.02;
export const CHOSEN_LIFT = 1.6;
const STEP = CARD.thickness + 0.02;
// A cover is propped toward the eye (1.5.2): flat on the stack it is
// foreshortened and farther than a hand card, so it would read smaller.
export const COVER_TILT = 14;

// A small fixed turn for each card, so piles look placed by a hand, the same
// every time that card lands there.
export function jitter(key, degrees = 3) {
  let h = 2166136261;
  for (const ch of String(key)) h = Math.imul(h ^ ch.charCodeAt(0), 16777619);
  return ((h >>> 0) / 4294967295 - 0.5) * 2 * degrees * DEG;
}

const toward = (centre, side) => centre.clone().add(new Vector3(0, 0, 300 * side));

// How far a hand of `n` cards spreads: `spacing` cm between neighbours, closer
// when there are too many for `width`.
export const spacingFor = (n, spacing, width) => (n < 2 ? spacing : Math.min(spacing, (width - CARD.width) / (n - 1)));

// Your hand: fanned (a computer) or in a row (a touch screen). A row wider
// than the screen scrolls: `scroll` cm. On a phone held upright (`rows`) a
// hand too long for one row is laid in two, the first half behind and above
// the second, each card's index (`minPitch` wide) clear of its neighbour, so
// that it never scrolls. `fill` (framing.js) draws such a row out where the
// window has width to spare (a phone on its side).
export function handShape(n, zone, fill = 1) {
  if (!(zone.rows > 1)) return { rows: 1, perRow: n, pitch: zone.pitch, width: zone.width };
  const width = zone.width * fill;
  const rows = (n - 1) * zone.minPitch + CARD.width > width ? zone.rows : 1;
  const perRow = Math.ceil(n / rows);
  const open = Math.min(zone.pitch * fill, CARD.width + 0.6);
  const pitch = perRow > 1 ? Math.min(open, (width - CARD.width) / (perRow - 1)) : zone.pitch;
  return { rows, perRow, pitch, width };
}

export function handRange(n, zone, fill = 1) {
  if (zone.fan) return 0;
  const { perRow, pitch, width } = handShape(n, zone, fill);
  const total = (perRow - 1) * pitch + CARD.width;
  return Math.max(0, (total - width) / 2);
}

// A card a hair nearer the eye than the one before it, so overlapping cards
// of a row do not share a plane (they would z-fight, and the faces smear).
const LAYER = 0.06;

function yourHand(cards, view, Z) {
  const zone = Z.yourHand;
  const n = cards.length;
  if (!n) return [];
  const centre = new Vector3(...zone.centre);
  let poses;
  if (zone.fan) {
    const s = spacingFor(n, zone.spacing, zone.width);
    const step = s / (zone.radius + 3);
    const angles = Array.from({ length: n }, (_, i) => (i - (n - 1) / 2) * step);
    poses = fan({ count: n, angles, centre, facing: toward(centre, 1), radius: zone.radius, tilt: zone.lean * DEG });
  } else {
    const fill = view.fill ?? 1;
    const { rows, perRow, pitch } = handShape(n, zone, fill);
    const range = handRange(n, zone, fill);
    const at = centre.clone().add(new Vector3(-Math.max(-range, Math.min(range, view.scroll ?? 0)), 0, 0));
    poses = [];
    for (let r = 0; r < rows; r++) {
      const count = Math.min(perRow, n - r * perRow);
      const line = row({ count, centre: at, facing: toward(at, 1), gap: pitch - CARD.width, tilt: zone.lean * DEG });
      const behind = rows - 1 - r; // the first row lies behind and above the last
      line.forEach((pose, i) => {
        const up = new Vector3(0, 1, 0).applyQuaternion(pose.quaternion);
        const out = new Vector3(0, 0, 1).applyQuaternion(pose.quaternion);
        pose.position.addScaledVector(up, behind * (zone.rowLift ?? 0)).addScaledVector(out, i * LAYER - behind * 0.5);
        poses.push(pose);
      });
    }
  }
  return cards.map((c, i) => {
    const pose = poses[i];
    if (c.code === view.chosen) pose.position.addScaledVector(new Vector3(0, 1, 0).applyQuaternion(pose.quaternion), CHOSEN_LIFT);
    return { key: c.code, zone: "your-hand", index: i, code: c.code, faceUp: true, public: c.public, pose };
  });
}

function theirHand(cards, Z) {
  const zone = Z.theirHand;
  const n = cards.length;
  if (!n) return [];
  const centre = new Vector3(...zone.centre);
  const s = spacingFor(n, zone.spacing, zone.width);
  const step = s / (zone.radius + 3);
  const angles = Array.from({ length: n }, (_, i) => (i - (n - 1) / 2) * step);
  return fan({ count: n, angles, centre, facing: toward(centre, -1), radius: zone.radius, tilt: -zone.lean * DEG }).map((pose, i) => ({
    key: `their-hand:${i}`,
    zone: "their-hand",
    index: i,
    code: null,
    faceUp: false,
    public: cards[i].public,
    known: cards[i].code,
    pose,
  }));
}

export const sideOf = (model, seat) => (seat === model.human ? "you" : "them");

// Where packet i of `seat` is, and the stack's cards: the hidden ones face
// down (a hair off square, so the thickness reads), the cover face up and
// square on top.
// A lying pose tipped toward the eye about its centre (its near edge kept
// off the stack).
function propped(pose, degrees) {
  const a = degrees * DEG;
  const q = new Quaternion().setFromAxisAngle(new Vector3(1, 0, 0), a).multiply(pose.quaternion);
  const position = pose.position.clone();
  position.y += (CARD.height / 2) * Math.sin(a);
  return { position, quaternion: q };
}

export function packetPlace(model, seat, i, Z) {
  const side = sideOf(model, seat);
  return { x: Z.packets.xs[i], z: Z.packets[side], side };
}

function packets(model, Z) {
  const slots = [];
  for (const seat of [0, 1]) {
    for (let i = 0; i < 3; i++) {
      const p = model.packets[seat][i];
      const { x, z, side } = packetPlace(model, seat, i, Z);
      const zone = side === "you" ? "your-packets" : "their-packets";
      const turned = model.held && model.held.seat === seat && model.held.packet - 1 === i ? 1 : 0; // the card held up is not in the stack
      for (let k = 0; k < p.hidden - turned; k++) {
        const key = `pk:${seat}:${i}:${k}`;
        slots.push({
          key,
          zone,
          index: k,
          code: null,
          faceUp: false,
          pose: lying({ x: x + jitter(key, 1) * 12, z: z + jitter(key + "z", 1) * 12, height: REST + k * STEP, faceUp: false, yaw: jitter(key, 2.5) }),
          packet: { seat, i },
        });
      }
      if (p.cover) {
        const key = `pk:${seat}:${i}:${p.hidden}`;
        slots.push({ key, zone, index: p.hidden, code: p.cover, faceUp: true, pose: propped(lying({ x, z, height: REST + p.hidden * STEP }), COVER_TILT), packet: { seat, i }, cover: true });
      }
    }
  }
  return slots;
}

// The card turned and held up over its packet, facing the human.
export function heldPose(model, held, Z) {
  const { x, z } = packetPlace(model, held.seat, held.packet - 1, Z);
  const centre = new Vector3(x, Z.held + 3.4, z + 1.5);
  return fan({ count: 1, centre, facing: centre.clone().add(new Vector3(0, 0, 300)), tilt: facingEye(Z.eye, centre.toArray()) * DEG })[0];
}

function held(model, Z) {
  if (!model.held) return [];
  const h = model.held;
  const side = sideOf(model, h.seat);
  return [{ key: `held`, zone: side === "you" ? "your-packets" : "their-packets", index: 99, code: h.code, faceUp: true, pose: heldPose(model, h, Z), packet: { seat: h.seat, i: h.packet - 1 }, held: true }];
}

// The open trick: the lead and the follow side by side, each a little toward
// its player's side of the table.
export function trickPose(model, k, Z) {
  const t = Z.trick;
  const c = model.trick[k];
  const side = sideOf(model, c.seat) === "you" ? 1 : -1;
  const d = (CARD.width + t.gap) / 2;
  return lying({ x: t.x + (k === 0 ? -d : d) + jitter(c.code, 1) * 4, z: t.z + side * t.offset + jitter(c.code + "z", 1) * 6, height: REST + k * 0.3, yaw: jitter(c.code, 2) });
}

function trick(model, Z) {
  return model.trick.map((c, k) => ({ key: c.code, zone: "trick", index: k, code: c.code, faceUp: true, pose: trickPose(model, k, Z), seat: c.seat }));
}

export function wonPlace(model, seat, Z, mirror = false) {
  const side = sideOf(model, seat);
  return { x: (mirror ? -1 : 1) * Z.won.x, z: Z.won[side], side };
}

function won(model, Z, mirror) {
  const slots = [];
  for (const seat of [0, 1]) {
    const { x, z, side } = wonPlace(model, seat, Z, mirror);
    model.won[seat].forEach((code, i) => {
      const key = `won:${seat}:${i}`;
      slots.push({ key, zone: side === "you" ? "your-won" : "their-won", index: i, code: null, faceUp: false, known: code, pose: lying({ x, z, height: REST + i * STEP, faceUp: false, yaw: jitter(key, 2.5) }) });
    });
  }
  return slots;
}

// The count: each take's cards turned up in groups of three, in a row in
// front of its owner, a group's cards overlapping so their indices read; the
// rows grow toward the middle (nearer the human on top).
export function countPose(side, g, w, Z, total) {
  const c = Z.count;
  const within = w * c.step;
  const groupW = 2 * c.step + CARD.width;
  const rowIndex = Math.floor(g / c.perRow);
  const col = g % c.perRow;
  const inRow = Math.min(c.perRow, Math.ceil(total / 3) - rowIndex * c.perRow);
  const pitch = groupW + c.gap;
  const x = c.x - (inRow * pitch - c.gap) / 2 + CARD.width / 2 + col * pitch + within;
  const dir = side === "you" ? -1 : 1;
  const z = c[side].z + dir * rowIndex * c.row;
  const layer = side === "you" ? 5 - rowIndex : rowIndex;
  return { x, z, height: REST + layer * 0.2 + (g % c.perRow) * 0.004 + w * STEP };
}

function counted(model, Z) {
  const slots = [];
  for (const seat of [0, 1]) {
    const side = sideOf(model, seat);
    const cards = model.counted[seat];
    cards.forEach((code, i) => {
      const g = Math.floor(i / 3);
      const p = countPose(side, g, i % 3, Z, cards.length);
      slots.push({ key: code, zone: "count", index: i, code, faceUp: true, pose: lying({ x: p.x, z: p.z, height: p.height, yaw: jitter(code, 0.8) }), seat });
    });
  }
  return slots;
}

export function packPose(model, i, Z, dealer = model.dealer) {
  const side = dealer === model.human ? "you" : "them";
  const at = Z.pack[side];
  return lying({ x: at.x + jitter(`pk${i}`, 1) * 8, z: at.z + jitter(`pz${i}`, 1) * 8, height: REST + i * STEP, faceUp: false, yaw: jitter(`pack${i}`, 2) });
}

function pack(model, Z) {
  return Array.from({ length: model.pack }, (_, i) => ({ key: `pack:${i}`, zone: "pack", index: i, code: null, faceUp: false, pose: packPose(model, i, Z) }));
}

// `view`: { chosen, scroll, left } -- the hand card chosen, the row scrolled,
// left-handed mode (the piles on the other side).
export function layout(model, { zones = ZONES, chosen = null, scroll = 0, left = false, fill = 1 } = {}) {
  const Z = zones;
  const view = { chosen, scroll, fill };
  const mirrored = left ? mirrorZones(Z) : Z;
  return [
    ...pack(model, mirrored),
    ...yourHand(model.hand, view, Z),
    ...theirHand(model.opp, Z),
    ...packets(model, Z),
    ...held(model, Z),
    ...trick(model, Z),
    ...won(model, mirrored, false),
    ...counted(model, Z),
  ];
}

// Left-handed (3.8): the piles and the pack on the other side; the packets keep their order.
export function mirrorZones(Z) {
  return { ...Z, won: { ...Z.won, x: -Z.won.x }, pack: { you: { ...Z.pack.you, x: -Z.pack.you.x }, them: { ...Z.pack.them, x: -Z.pack.them.x } } };
}

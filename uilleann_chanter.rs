//! Cut files for an uilleann pipe chanter whose bore is a tapering square of
//! flat strips of ply, Craig Fischer's square chanter (1997) cut on a laser:
//! a back, two sides and a lid, with a stacked tenon, collar and foot.
//!
//!     target/release/uilleann_chanter --no-write             # the checks
//!     target/release/uilleann_chanter --out=DIR              # the sheet
//!     target/release/uilleann_chanter --plate=2.9 ...        # for ply as measured
//!     target/release/uilleann_chanter --kerf=0.18 ...        # for that kerf
//!     target/release/uilleann_chanter --tenon=18 ...         # the tenon for your stock
//!     target/release/uilleann_chanter --page-to=turn.html    # the 3D view
//!     target/release/uilleann_chanter --bores                # what bore-tmm reads
//!
//! THE REFERENCE is Craig Fischer's "Make a Square Uilleann Pipe Chanter"
//! (1997, hosted by David Daye at daye1.com/bagpage/makepipe/squint1.html): a
//! square cone 3.7mm at the top to 11.5mm at the sole over 365mm, the
//! cross-section of a Quinn chanter's bore, made as a box of four flat pieces
//! of wood, eight holes, and a lid held on with silicone while the chanter is
//! tuned, then glued. This generator keeps his bore and his holes and their
//! starting drill sizes exactly; nothing here is modelled. His tuning is still
//! done by hand: holes opened toward his final sizes, and the bore enlarged
//! further where a note asks for it.
//!
//! HIS FOUR STARTING ENLARGEMENTS, the oval dishes he gouges into the lid's
//! inside face before it goes on (DISHES), are a greyscale engrave on the lid's
//! bore layer, darker deeper: each an elliptic paraboloid, deepest at its
//! middle, never wider than the bore's face less DISH_EDGE either side, so no
//! dish runs under a side strip's glue line. The first starts at 30mm, above the
//! strips, so the plates there have it CUT instead: a lobe on the front of
//! their throat, as deep as the dish is at that plate. Engrave depth in ply is
//! not linear and the glue lines engrave unlike the wood, so set black to
//! DEPTH_BLACK on scrap of the same sheet first, and burn shallow rather than
//! deep: gouging more out is easy, putting it back is not.
//!
//! Fischer's lid is no thinner than 7mm, since a hole's depth sets its note.
//! Ply is 3mm, so the lid is LID_LAYERS strips glued face to face, 9mm: a
//! deeper hole sounds flatter, and a chanter tuned by opening holes up wants
//! to start flat. He thins a thick lid over its holes from the outside, and so
//! can you. The back, under the thumb, can be thinner: BACK_LAYERS, 6mm.
//!
//! From the top down:
//!
//!   * the TENON: N_TENON round plates with the reed's seat stepped through
//!     them, 7.0mm at the top narrowing at a taper reamer's rate to Fischer's
//!     5.16mm (13/64in) throat, which runs on to the strips;
//!   * the COLLAR: N_COLLAR wider round plates, the stop Fischer suggests so
//!     the tenon goes only so far into the stock; the tenon and collar press
//!     together on two pins;
//!   * the SOCKET: N_SOCK round plates with a rectangular hole the strips' top
//!     stands in; dry while you tune, glued when the lid is;
//!   * the STRIPS, from the socket to the sole: the back, BACK_LAYERS thick and
//!     the bore's width plus both sides wide, with the thumb's hole; two sides,
//!     one ply thick and exactly the bore's width, glued on the back's face;
//!     and the lid, LID_LAYERS thick, with the seven front holes;
//!   * the FOOT: N_FOOT round plates round the strips' bottom, the tube Fischer
//!     suggests to hold the glue lines at the sole.
use chanters::chanter_model::{self as m, plate, Setting};
use chanters::practice_chanters::{self as pc, cc, circle, page_part, plate_m, CPart, Checks, Dish, Parts, Sheet, C, P2};
use lasermade_tools::die;
use lasermade_tools::py::{self, g};
use lasermade_tools::turn3d::{self, Mat, F, I, J};
use square_flute::{self as sf, kerf, Cut};
use std::f64::consts::PI;

const INCH: f64 = 25.4;
// --- the reference chanter, Fischer's figures (mm, down from the top) --------
const REF_TOP: f64 = 3.7; // the square's side, top and sole
const REF_SOLE_SIDE: f64 = 11.5;
const REF_SOLE: f64 = 365.0;
const REF_THROAT: f64 = 13.0 / 64.0 * INCH; // drilled through the top: 5.16mm
const REF_SEAT_TOP: f64 = 7.0; // the reed's seat at the top, reamed
const REAMER: f64 = (16.0 - 3.0) / 85.0; // his reamer's taper, across per mm
const REF_TENON: f64 = 35.0; // the tenon's length
const REF_LID: f64 = 7.0; // the lid no thinner than this
/// each hole: down from the top, drill size and final size (in), and its strip
const REF_HOLES: [(&str, f64, f64, f64, &str); 8] = [
    ("D'", 92.5, 7.0 / 32.0, 9.0 / 32.0, "back"),
    ("C#", 110.0, 11.0 / 64.0, 7.0 / 32.0, "lid"),
    ("B", 139.0, 9.0 / 64.0, 7.0 / 32.0, "lid"),
    ("A", 172.0, 3.0 / 16.0, 9.0 / 32.0, "lid"),
    ("G", 212.0, 11.0 / 64.0, 11.0 / 64.0, "lid"),
    ("F#", 242.0, 7.0 / 32.0, 9.0 / 32.0, "lid"),
    ("E", 275.0, 11.0 / 64.0, 11.0 / 64.0, "lid"),
    ("Eb", 306.0, 3.0 / 16.0, 9.0 / 32.0, "lid"),
];
/// his starting bore enlargements in the lid's inside face: from, to (mm
/// down) and deepest. His text gives the first, 30 to 90 at 1.5mm, and the
/// depths; his sketch of the lid, to scale from C# down, gives the other
/// three's ends. The sketch draws the first at 58 to 96: the text is kept.
const DISHES: [(f64, f64, f64); 4] = [(30.0, 90.0, 1.5), (112.0, 162.0, 1.2), (247.0, 275.0, 1.2), (311.0, 355.0, 1.2)];
// --- this chanter -------------------------------------------------------------
const LID_LAYERS: usize = 3; // 9mm: at least Fischer's 7
const BACK_LAYERS: usize = 2; // 6mm under the thumb
const N_TENON: usize = 12; // 36mm, Fischer's 35
static TENON_D: Setting = Setting::new(0x4031000000000000); // 17.0: hemped into the stock; --tenon= for yours
const N_COLLAR: usize = 2;
const COLLAR_D: f64 = 30.0;
const N_SOCK: usize = 2;
const SOCK_CLEAR: f64 = 0.1; // each side of the strips: a sliding fit while you tune
const N_FOOT: usize = 2;
const FOOT_D: f64 = 38.0;
const DISH_EDGE: f64 = 0.3; // each dish this much inside the bore's face, each side
const DEPTH_BLACK: f64 = 1.5; // the engrave's black, mm: the deepest dish
const PX: usize = 10; // the engrave's pixels per mm
const BLUE: &str = "#0000ff"; // the house engrave ink
const PIN_AT: (f64, f64) = (80.0, 250.0); // the tenon's pins, degrees round from the front
const BY: &str = "uilleann_chanter.rs";

/// The bore's square, z down from the top.
fn side(z: f64) -> f64 {
    REF_TOP + (REF_SOLE_SIDE - REF_TOP) * z / REF_SOLE
}

/// The reed's seat, then the throat, z down.
fn seat_d(z: f64) -> f64 {
    py::max(REF_THROAT, REF_SEAT_TOP - REAMER * z)
}

/// The strips' outside at z, in the plate frame: x across, y from the lid
/// (-y) to the back, the bore centred.
fn bx(z: f64, t: f64, clear: f64) -> Vec<P2> {
    let s = side(z);
    let hw = s / 2.0 + t + clear;
    let (lid, back) = (-s / 2.0 - LID_LAYERS as f64 * t - clear, s / 2.0 + BACK_LAYERS as f64 * t + clear);
    vec![(-hw, lid), (hw, lid), (hw, back), (-hw, back)]
}

/// A dish's width at its middle: the face there, less DISH_EDGE each side.
fn dish_w(dish: (f64, f64, f64)) -> f64 {
    side((dish.0 + dish.1) / 2.0) - 2.0 * DISH_EDGE
}

/// How deep the dishes are at z down, y across the lid's face from its
/// centre line.
fn dish_depth(z: f64, y: f64) -> f64 {
    if y.abs() > side(z) / 2.0 - DISH_EDGE {
        return 0.0;
    }
    let mut out = 0.0;
    for d in DISHES {
        let (z0, z1, dd) = d;
        let u = (z - (z0 + z1) / 2.0) / ((z1 - z0) / 2.0);
        let v = y / (dish_w(d) / 2.0);
        out = py::max(out, dd * (1.0 - u * u - v * v));
    }
    out
}

/// A round hole r with the dishes at z added on its front (-y).
fn lobed(r: f64, z: f64, n: usize) -> Vec<P2> {
    (0..n)
        .map(|i| {
            let a = 2.0 * PI * i as f64 / n as f64;
            let (x, mut y) = (r * py::mcos(a), r * py::msin(a));
            if y < 0.0 {
                y -= dish_depth(z, x);
            }
            (x, y)
        })
        .collect()
}

#[link(name = "z")]
extern "C" {
    fn compress2(dest: *mut u8, dest_len: *mut std::os::raw::c_ulong, source: *const u8, source_len: std::os::raw::c_ulong, level: i32) -> i32;
    fn compressBound(source_len: std::os::raw::c_ulong) -> std::os::raw::c_ulong;
    fn crc32(crc: std::os::raw::c_ulong, buf: *const u8, len: u32) -> std::os::raw::c_ulong;
}

/// zlib.compress(data, 9), by the system's zlib, as Python's is.
fn zcompress(data: &[u8]) -> Vec<u8> {
    unsafe {
        let mut n = compressBound(data.len() as _);
        let mut out = vec![0u8; n as usize];
        if compress2(out.as_mut_ptr(), &mut n, data.as_ptr(), data.len() as _, 9) != 0 {
            die("zlib: compress2 failed");
        }
        out.truncate(n as usize);
        out
    }
}

/// A greyscale PNG from rows of 0-255.
fn png(rows: &[Vec<u8>]) -> Vec<u8> {
    let (h, w) = (rows.len() as u32, rows[0].len() as u32);
    let mut raw = vec![];
    for r in rows {
        raw.push(0u8);
        raw.extend_from_slice(r);
    }
    let chunk = |tag: &[u8], data: &[u8]| -> Vec<u8> {
        let mut c = (data.len() as u32).to_be_bytes().to_vec();
        c.extend_from_slice(tag);
        c.extend_from_slice(data);
        let crc = unsafe { crc32(0, c[4..].as_ptr(), (c.len() - 4) as u32) } as u32;
        c.extend_from_slice(&crc.to_be_bytes());
        c
    };
    let mut ihdr = w.to_be_bytes().to_vec();
    ihdr.extend_from_slice(&h.to_be_bytes());
    ihdr.extend_from_slice(&[8, 0, 0, 0, 0]);
    let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
    out.extend(chunk(b"IHDR", &ihdr));
    out.extend(chunk(b"IDAT", &zcompress(&raw)));
    out.extend(chunk(b"IEND", b""));
    out
}

fn base64(b: &[u8]) -> String {
    const A: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut s = String::with_capacity(b.len().div_ceil(3) * 4);
    for c in b.chunks(3) {
        let n = (c[0] as u32) << 16 | (*c.get(1).unwrap_or(&0) as u32) << 8 | *c.get(2).unwrap_or(&0) as u32;
        for k in 0..4 {
            if k <= c.len() {
                s.push(A[(n >> (18 - 6 * k) & 63) as usize] as char);
            } else {
                s.push('=');
            }
        }
    }
    s
}

/// A dish's engrave on the lid's bore layer, in the strip's frame (x along
/// from its top, y across): white nothing, black DEPTH_BLACK.
fn dish_image(dish: (f64, f64, f64), x0: f64) -> Dish {
    let (z0, z1, _) = dish;
    let za = py::max(z0, x0);
    let w = dish_w(dish);
    let px = PX as f64;
    let (nx, ny) = (((z1 - za) * px).ceil() as usize, (w * px).ceil() as usize);
    let rows = (0..ny)
        .map(|j| {
            let y = -w / 2.0 + (j as f64 + 0.5) / px;
            (0..nx).map(|i| py::round(255.0 * (1.0 - dish_depth(za + (i as f64 + 0.5) / px, y) / DEPTH_BLACK)).clamp(0, 255) as u8).collect()
        })
        .collect();
    Dish { x: za - x0, y: -w / 2.0, w: nx as f64 / px, h: ny as f64 / px, rows }
}

/// practice_chanters' turned(), which also turns a part's engraves.
fn turned(p: &CPart) -> CPart {
    let mut q = pc::turned(p);
    q.flip = !p.flip;
    q
}

struct Built {
    t: f64,
    x0: f64,
    ls: f64,
    pins: Vec<P2>,
    pin_d: f64,
}

fn build() -> (Vec<CPart>, Built) {
    let t = plate();
    let mut parts = Parts::default();
    let x0 = (N_TENON + N_COLLAR) as f64 * t;
    let ls = REF_SOLE - x0;
    let strip = |wide: f64| -> Vec<P2> {
        let (w0, w1) = (side(x0) + wide, side(REF_SOLE) + wide);
        vec![(0.0, -w0 / 2.0), (ls, -w1 / 2.0), (ls, w1 / 2.0), (0.0, w0 / 2.0)]
    };
    let holes = |st: &str| -> Vec<C> { REF_HOLES.iter().filter(|r| r.4 == st).map(|&(_, z, d, _, _)| cc(z - x0, 0.0, d * INCH / 2.0)).collect() };
    for k in 0..BACK_LAYERS {
        let note = format!("back layer {} of {BACK_LAYERS}", k + 1) + if k == 0 { ", the bore face" } else { "" };
        let p = parts.add("back", &strip(2.0 * t), holes("back"), x0, "strip", &note, None);
        p.strip = "back";
        p.layer = k;
    }
    for k in 0..2 {
        let p = parts.add("side", &strip(0.0), vec![], x0, "strip", &format!("side {}, the bore's width", k + 1), None);
        p.strip = ["left", "right"][k];
        p.layer = 0;
    }
    for k in 0..LID_LAYERS {
        let note = format!("lid layer {} of {LID_LAYERS}", k + 1) + if k == 0 { ", the bore face, the dishes engraved" } else { "" };
        let p = parts.add("lid", &strip(2.0 * t), holes("lid"), x0, "strip", &note, None);
        p.strip = "lid";
        p.layer = k;
        p.dishes = if k == 0 { DISHES.iter().filter(|d| d.1 > x0).map(|&d| dish_image(d, x0)).collect() } else { vec![] };
    }
    let tenon_d = TENON_D.get();
    let pin_d = pc::PIN - pc::PIN_INTERF;
    let r_pin = (REF_SEAT_TOP / 2.0 + tenon_d / 2.0) / 2.0;
    let pins = vec![pc::polar(PIN_AT.0, r_pin), pc::polar(PIN_AT.1, r_pin)];
    let pin_cut = || pins.iter().map(|&(x, y)| cc(x, y, pin_d / 2.0)).collect::<Vec<_>>();
    let bore = |d: f64, z: f64| -> C {
        if py::max(dish_depth(z, 0.0), 0.0) > 0.0 {
            C::Poly(lobed(d / 2.0, z, 96))
        } else {
            cc(0.0, 0.0, d / 2.0)
        }
    };
    for k in 0..N_TENON {
        let z = (k as f64 + 0.5) * t;
        let d = seat_d(z);
        let mut cuts = vec![bore(d, z)];
        cuts.extend(pin_cut());
        parts.add("tenon", &circle(0.0, 0.0, tenon_d / 2.0, 72), cuts, k as f64 * t, "tenon", &format!("tenon plate {}, {d:.2}mm", k + 1), Some(tenon_d / 2.0));
    }
    for k in 0..N_COLLAR {
        let z = ((N_TENON + k) as f64 + 0.5) * t;
        let mut cuts = vec![bore(REF_THROAT, z)];
        cuts.extend(pin_cut());
        parts.add("collar", &circle(0.0, 0.0, COLLAR_D / 2.0, 72), cuts, (N_TENON + k) as f64 * t, "collar", &format!("collar plate {}", k + 1), Some(COLLAR_D / 2.0));
    }
    for k in 0..N_SOCK {
        let x = x0 + k as f64 * t;
        parts.add(
            "socket",
            &circle(0.0, 0.0, COLLAR_D / 2.0, 72),
            vec![C::Poly(bx(x + t, t, SOCK_CLEAR))],
            x,
            "sock",
            &format!("socket plate {}", k + 1),
            Some(COLLAR_D / 2.0),
        );
    }
    for k in 0..N_FOOT {
        let x = REF_SOLE - (N_FOOT - k) as f64 * t;
        parts.add("foot", &circle(0.0, 0.0, FOOT_D / 2.0, 72), vec![C::Poly(bx(x + t, t, SOCK_CLEAR))], x, "foot", &format!("foot plate {}", k + 1), Some(FOOT_D / 2.0));
    }
    (parts.v, Built { t, x0, ls, pins, pin_d })
}

fn checks(parts: &[CPart], info: &Built, sheets: &[Sheet]) -> Checks {
    let mut out: Checks = vec![];
    let mut note = |ok: bool, what: &str, detail: String| out.push((ok, what.to_string(), detail));
    let (t, x0, ls) = (info.t, info.x0, info.ls);
    let (lid, back) = (LID_LAYERS as f64 * t, BACK_LAYERS as f64 * t);
    note(
        (x0 + ls - REF_SOLE).abs() < 1e-9 && side(0.0) == REF_TOP && side(REF_SOLE) == REF_SOLE_SIDE,
        "the bore is Fischer's square cone",
        format!(
            "{:.2}mm square at the strips' top to {:.2}mm at the sole, {} to {}mm over {}mm from the top; strips {ls:.1}mm",
            side(x0),
            side(REF_SOLE),
            g(REF_TOP),
            g(REF_SOLE_SIDE),
            g(REF_SOLE)
        ),
    );
    note(
        (N_TENON as f64 * t - REF_TENON).abs() <= t,
        "the tenon is Fischer's length",
        format!("{}mm, his {}; {}mm round for hemp", g(N_TENON as f64 * t), g(REF_TENON), g(TENON_D.get())),
    );
    let seats: Vec<f64> = (0..N_TENON).map(|k| seat_d((k as f64 + 0.5) * t)).collect();
    note(
        seats[0] <= REF_SEAT_TOP && sf::fmin(&seats) >= REF_THROAT - 1e-9,
        "the seat steps down to Fischer's throat",
        format!(
            "{:.2}mm at the top to {:.2}mm, his {} reamed to {REF_THROAT:.2}mm drilled; ream it smooth",
            seats[0],
            sf::fmin(&seats),
            g(REF_SEAT_TOP)
        ),
    );
    note(
        lid >= REF_LID,
        "the lid is as deep as Fischer asks",
        format!("{LID_LAYERS} layers, {}mm, his at least {}; the back {}mm", g(lid), g(REF_LID), g(back)),
    );
    let (mut ok, mut detail) = (true, vec![]);
    for &(h, z, d, _, _) in &REF_HOLES {
        let (dm, s) = (d * INCH, side(z));
        ok &= dm < s && z - dm / 2.0 > x0 + N_SOCK as f64 * t;
        detail.push(format!("{h} {dm:.2}/{s:.2}"));
    }
    note(ok, "every hole starts inside its face, below the socket", "hole/face mm: ".to_string() + &detail.join(", "));
    let mut ms = vec![];
    for p in parts.iter().filter(|p| p.kind == "strip") {
        let rows: Vec<_> = REF_HOLES.iter().filter(|r| r.4 == p.strip).collect();
        for (c, r) in p.p.cuts.iter().zip(rows) {
            if let Cut::Circle(cx, _, _) = c {
                ms.push((cx + x0 - r.1).abs());
            }
        }
    }
    let moved = sf::fmax(&ms);
    let lid_cuts: usize = parts.iter().filter(|p| p.strip == "lid").map(|p| p.p.cuts.len()).sum();
    note(
        moved < 1e-9 && lid_cuts == 7 * LID_LAYERS,
        "every hole is where Fischer's is",
        format!("{} holes, within {}mm, through every layer of its strip", REF_HOLES.len(), py::e(moved, 1)),
    );
    let pr = info.pin_d / 2.0;
    let (mut to_bore, mut to_edge) = (f64::INFINITY, f64::INFINITY);
    for p in parts.iter().filter(|p| p.kind == "tenon" || p.kind == "collar") {
        let b = &p.p.cuts[0];
        for &(x, y) in &info.pins {
            let d = match b {
                Cut::Poly(v) => pc::poly_dist((x, y), &pc::pf(v)),
                Cut::Circle(_, _, r) => py::hypot(x, y) - r,
            };
            to_bore = py::min(to_bore, d - pr);
            to_edge = py::min(to_edge, p.round.unwrap() - py::hypot(x, y) - pr);
        }
    }
    note(
        py::min(to_bore, to_edge) >= pc::MIN_WEB,
        "every pin keeps ply round it",
        format!("{to_bore:.2}mm to the bore, lobes and all, {to_edge:.2}mm to the edge, against {}", g(pc::MIN_WEB)),
    );
    let a = [pc::polar(PIN_AT.0, 1.0), pc::polar(PIN_AT.1, 1.0)];
    let maps: [fn(P2) -> P2; 2] = [|(x, y)| (-x, -y), |(x, y)| (-x, y)];
    let one_way = maps.iter().all(|mf| {
        let cost = |q: P2, r: P2| {
            let (m0, m1) = (mf(a[0]), mf(a[1]));
            py::hypot(m0.0 - q.0, m0.1 - q.1) + py::hypot(m1.0 - r.0, m1.1 - r.1)
        };
        py::min(cost(a[0], a[1]), cost(a[1], a[0])) > 0.1
    });
    note(
        one_way,
        "a tenon or collar plate goes on only one way",
        format!(
            "pins at {} and {} degrees from the front: no half turn or flip brings them back, so every lobe faces the lid",
            g(PIN_AT.0),
            g(PIN_AT.1)
        ),
    );
    let lid0 = parts.iter().find(|p| p.strip == "lid" && p.layer == 0).unwrap();
    let mut edge = f64::INFINITY;
    let px = PX as f64;
    for d in &lid0.dishes {
        for (j, row) in d.rows.iter().enumerate() {
            for (i, &gv) in row.iter().enumerate() {
                if gv < 255 {
                    let z = x0 + d.x + (i as f64 + 0.5) / px;
                    let yy = d.y + (j as f64 + 0.5) / px;
                    edge = py::min(edge, side(z) / 2.0 - yy.abs() - DISH_EDGE);
                }
            }
        }
    }
    let deep = sf::fmax(&DISHES.iter().map(|d| d.2).collect::<Vec<_>>());
    let lobes: Vec<&CPart> = parts.iter().filter(|p| (p.kind == "tenon" || p.kind == "collar") && matches!(p.p.cuts[0], Cut::Poly(_))).collect();
    let mut worst_lobe = (0.0, 0.0);
    for (i, p) in lobes.iter().enumerate() {
        let Cut::Poly(v) = &p.p.cuts[0] else { unreachable!() };
        let fr = sf::fmax(&v.iter().map(|q| -q[1].f()).collect::<Vec<_>>());
        let c = (fr - if p.kind == "collar" { REF_THROAT / 2.0 } else { seat_d(p.x + t / 2.0) / 2.0 }, p.x);
        if i == 0 || c.partial_cmp(&worst_lobe) == Some(std::cmp::Ordering::Greater) {
            worst_lobe = c;
        }
    }
    note(
        edge >= -1e-9 && deep <= DEPTH_BLACK && lid0.dishes.len() == DISHES.len(),
        "Fischer's four dishes are in the lid's bore face",
        format!(
            "{} greyscale engraves, black {}mm, deepest {}mm; every engraved pixel {}mm or more inside the face ({:.2} at the least)",
            lid0.dishes.len(),
            g(DEPTH_BLACK),
            g(deep),
            g(DISH_EDGE),
            edge + DISH_EDGE
        ),
    );
    note(
        lobes.iter().all(|p| p.x + t > DISHES[0].0) && lobes.len() as i64 == py::round((x0 - DISHES[0].0) / t),
        "the first dish, above the strips, is cut in its plates",
        format!("{} plates from {}mm, a lobe on the front of each, {:.2}mm deep at the deepest", lobes.len(), g(DISHES[0].0), worst_lobe.0),
    );
    let mut worst = f64::INFINITY;
    for p in parts.iter().filter(|p| p.kind == "sock" || p.kind == "foot") {
        let Cut::Poly(v) = &p.p.cuts[0] else { unreachable!() };
        let far = sf::fmax(&v.iter().map(|q| py::hypot(q[0].f(), q[1].f())).collect::<Vec<_>>());
        worst = py::min(worst, p.round.unwrap() - far);
    }
    note(worst >= pc::MIN_WEB, "the socket and foot keep ply round the strips", format!("{worst:.2}mm at their corners, against {}", g(pc::MIN_WEB)));
    let narrow = side(x0);
    note(narrow >= 4.0, "the sides are wide enough to cut", format!("{narrow:.2}mm at their top, {:.2}mm at the sole", side(REF_SOLE)));
    pc::sheet_checks(&mut note, sheets, parts.len(), "");
    out
}

/// practice_chanters' sheet, with the dishes' engraves before the cuts.
fn svg_sheet(k: usize, nsh: usize, sh: &Sheet) -> String {
    let kf = kerf();
    let mut imgs = vec![];
    for (p, sx, sy) in &sh.parts {
        let (dx, dy) = (sx - p.p.x0 + kf / 2.0, sy - p.p.y0 + kf / 2.0);
        for d in &p.dishes {
            let (x, y, rows) = if p.flip {
                (-d.x - d.w, -d.y - d.h, d.rows.iter().rev().map(|r| r.iter().rev().copied().collect()).collect::<Vec<Vec<u8>>>())
            } else {
                (d.x, d.y, d.rows.clone())
            };
            let data = base64(&png(&rows));
            imgs.push(format!(
                "<image x=\"{:.3}\" y=\"{:.3}\" width=\"{:.3}\" height=\"{:.3}\" preserveAspectRatio=\"none\" href=\"data:image/png;base64,{data}\"/>",
                x + dx,
                y + dy,
                d.w,
                d.h
            ));
        }
    }
    let mut txt = pc::svg_sheet(k, nsh, sh, 4)
        .replace("Practice chanter, 4-sided bore", "Uilleann chanter, Craig Fischer's square bore in strips")
        .replacen(
            "Orange",
            &format!(
                "Engraves first: blue {BLUE} marks the greyscale image engraves, Fischer's four bore dishes in the lid's bore layer, \
                 Grayscale mode in xTool Creative Space, black {}mm deep, set on scrap of this sheet; burn shallow rather than deep. Then orange",
                g(DEPTH_BLACK)
            ),
            1,
        )
        .replace("cuts the bores, seats and pin holes first; then black", "cuts the seats, finger and pin holes first; then black");
    if !imgs.is_empty() {
        let block = "</desc>\n  <g id=\"images\">\n".to_string() + &imgs.iter().map(|i| format!("    {i}\n")).collect::<String>() + "  </g>\n";
        txt = txt.replacen("</desc>\n", &block, 1);
    }
    txt
}

/// Where a part sits: a plate's x across, y, z through, to z, y, x; a strip's
/// x along it, y across its centre line, w through from the bore out, its
/// face leaning with the cone.
fn placed(p: &CPart, info: &Built) -> Mat {
    if p.kind != "strip" {
        return plate_m(p.x);
    }
    let (t, x) = (info.t, info.x0);
    let (h0, kh) = (side(x) / 2.0, (side(x + 1.0) - side(x)) / 2.0);
    let off = h0 + p.layer as f64 * t;
    let top = vec![I(1), I(0), I(0), F(x)];
    let last = vec![I(0), I(0), I(0), I(1)];
    match p.strip {
        "lid" => vec![top, vec![F(-kh), I(0), I(-1), F(-off)], vec![I(0), I(1), I(0), I(0)], last],
        "back" => vec![top, vec![F(kh), I(0), I(1), F(off)], vec![I(0), I(-1), I(0), I(0)], last],
        "left" => vec![top, vec![I(0), I(1), I(0), I(0)], vec![F(kh), I(0), I(1), F(h0)], last],
        _ => vec![top, vec![I(0), I(-1), I(0), I(0)], vec![F(-kh), I(0), I(-1), F(-h0)], last],
    }
}

/// The 3D view: the strips darker, every part the cut file's outline.
fn page(path_out: &str) {
    let (parts, info) = build();
    let mid = REF_SOLE / 2.0;
    let out: Vec<J> = parts
        .iter()
        .map(|p| {
            let ex = if p.kind == "strip" {
                let k = (p.layer + 1) as f64;
                match p.strip {
                    "lid" => [I(0), F(-0.15 * k), I(0)],
                    "back" => [I(0), F(0.15 * k), I(0)],
                    "left" => [I(0), I(0), F(0.15)],
                    _ => [I(0), I(0), F(-0.15)],
                }
            } else {
                [F((p.x - mid) / 60.0), I(0), I(0)]
            };
            page_part(p, &placed(p, &info), ex, if p.kind == "strip" { "dark" } else { "ply" })
        })
        .collect();
    let mut designs = vec![("square bore".to_string(), pc::design_j(out))];
    turn3d::write(
        path_out,
        "Uilleann chanter in 3D",
        &mut designs,
        "Drag to turn; right-drag, shift-drag or move two fingers to slide the view; \
         scroll or pinch to zoom in on whatever is under the pointer; double-click a part \
         to turn round it, and Reset view to go back. Draw Explode to pull the strips outward and \
         the plates apart along the chanter. Every part is the cut file\u{2019}s own outline. \
         The strips, darker, taper with the cone: wider toward the sole. The lid, three \
         layers, carries the seven finger holes; the back, two layers, the thumb\u{2019}s. \
         At the top, the tenon and the wider collar, then the socket the strips stand in; \
         at the sole, the foot.",
        BY,
        false,
    );
}

/// Python's str(Fraction(n, 64)).
fn fraction64(n: i64) -> String {
    let (mut a, mut b) = (n.abs(), 64i64);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    let (p, q) = (n / a, 64 / a);
    if q == 1 {
        format!("{p}")
    } else {
        format!("{p}/{q}")
    }
}

/// What bore-tmm's tools/read_cut_files.py needs to read this chanter's
/// bore, on stdout as JSON: each hole as cut (mm down from the top, round
/// equivalent, the depth of its strip), sorted down the bore; Fischer's holes
/// in that order; the strips' top and the sole; the square's side at both;
/// the throat and the inch.
fn bores() {
    use chanters::bores::{cut_d, n as num, st};
    let (parts, info) = build();
    let x0 = info.x0;
    let t = info.t;
    let mut found: Vec<(f64, f64, f64)> = vec![];
    for name in ["back", "lid"] {
        // the last layer of each strip, as a dict by name keeps it
        let p = parts.iter().filter(|p| p.kind == "strip" && p.p.name == name).last().unwrap();
        let chimney = if name == "back" { BACK_LAYERS as f64 * t } else { LID_LAYERS as f64 * t };
        for c in &p.p.cuts {
            let (z, d) = cut_d(c);
            found.push((x0 + z, d, chimney));
        }
    }
    found.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut rf: Vec<_> = REF_HOLES.to_vec();
    rf.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let sole = x0 + info.ls;
    let j = J::obj(vec![
        ("x0", num(x0)),
        ("sole", num(sole)),
        ("side", J::A(vec![num(side(x0)), num(side(sole))])),
        ("throat", num(REF_THROAT)),
        ("inch", num(INCH)),
        ("found", J::A(found.iter().map(|&(z, d, c)| J::A(vec![num(z), num(d), num(c)])).collect())),
        ("ref", J::A(rf.iter().map(|&(h, z, d, f, s)| J::A(vec![st(h), num(z), num(d), num(f), st(s)])).collect())),
    ]);
    let mut s = String::new();
    j.dump(&mut s);
    println!("{s}");
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--bores") {
        return bores();
    }
    let (mut out, mut write, mut page_to) = (None::<String>, true, None::<String>);
    for x in std::env::args().skip(1) {
        let val = || x.split_once('=').unwrap().1.to_string();
        if x == "--no-write" {
            write = false;
        } else if x.starts_with("--out=") {
            out = Some(val());
        } else if x.starts_with("--plate=") {
            m::PLATE.set(pc::num_arg(&x));
        } else if x.starts_with("--kerf=") {
            sf::set_kerf(&x);
        } else if x.starts_with("--tenon=") {
            TENON_D.set(pc::num_arg(&x));
        } else if x.starts_with("--page-to=") {
            page_to = Some(val());
        } else {
            die(format!("error: {x} is not a flag this generator reads. It takes --out=, --plate=, --kerf=, --tenon=, --page-to= and --no-write."));
        }
    }
    if let Some(pt) = page_to {
        page(&pt);
        println!("  wrote {}", pc::basename(&pt));
        return;
    }
    if write && out.is_none() {
        die("error: --out=DIR, or --no-write to run the checks only.");
    }
    let (parts, info) = build();
    let sheets = pc::pack_with(&parts, &turned);
    let res = checks(&parts, &info, &sheets);
    let t = info.t;
    println!(
        "uilleann chanter, Craig Fischer's square bore in {}mm strips: {:.2} to {:.2}mm square, strips {:.1}mm; {}mm from the top to the sole",
        g(t),
        side(info.x0),
        side(REF_SOLE),
        info.ls,
        g(REF_SOLE)
    );
    for &(h, z, d, f, st) in &REF_HOLES {
        println!(
            "  {h:<3} {st:<4} {z:6.1}mm down, cut {:.2}mm ({}in), open to about {:.2}mm; the face {:.2}mm",
            d * INCH,
            fraction64(py::round(d * 64.0)),
            f * INCH,
            side(z)
        );
    }
    println!(
        "  top down: {N_TENON} tenon, {N_COLLAR} collar, {N_SOCK} socket over the strips; the back {BACK_LAYERS} layers, 2 sides, the lid {LID_LAYERS} layers; {N_FOOT} foot plates at the sole"
    );
    if pc::print_checks(&res) {
        std::process::exit(1);
    }
    if write {
        pc::write_sheets(&out.unwrap(), &sheets, &|k| format!("uilleann-chanter-sheet{k}-cut-files.svg"), &|k, n, sh| svg_sheet(k, n, sh));
    }
}

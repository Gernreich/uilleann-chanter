//! The chanter as a parametric SolidCraft design (2026-11-08):
//! `uilleann_chanter --solidcraft=DIR` writes it, and SolidCraft builds the
//! STEP file in step/ from it.
//!
//! The frame is the 3D view's: X down the chanter from the top, the lid to
//! -Y. Each strip leans with the square cone, so it is sketched in the plane
//! through the axis along its face, extruded through, and intersected with
//! the slab it fills: from the bore's face (or the layer under it) out ply,
//! leaning as the cone does. Fischer's bore, the ply, the holes and the
//! plates' sizes are parameters; the layers and plates keep their counts. The
//! dishes engraved in the lid have no depth here, as in the 3D view; where the
//! first one reaches up into the tenon and collar plates, their bore has its
//! lobe, an expression of the dish's.
use super::*;
use lasermade_tools::solidcraft::{n as nn, Script};

/// sum of coef * (expr), the zero terms dropped.
fn lin(terms: &[(f64, &str)]) -> String {
    let t: Vec<String> = terms.iter().filter(|(c, _)| c.abs() > 1e-12).map(|(c, e)| format!("{}*({e})", nn(*c))).collect();
    if t.is_empty() { "0 mm".into() } else { t.join(" + ") }
}

/// The bore's square at z down.
fn sq(z: &str) -> String {
    format!("(ref_top + (sole_side - ref_top)*({z})/sole)")
}

fn circ(s: &mut Script, d: &str) {
    s.cmd("sketch.circle.center", format!("{{\"center\":[\"0 mm\",\"0 mm\"],\"diameter\":\"{d}\"}}"));
}

fn across(s: &mut Script, x: &str, name: &str) -> String {
    s.sketch_on([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], Some(x), name)
}

/// A plate's bore, `d` across at z: round, or with the first dish's lobe on
/// its front, as lobed() draws it, where the dish reaches it.
fn bore(s: &mut Script, d: &str, z: f64, zs: &str) {
    if dish_depth(z, 0.0) <= 0.0 {
        circ(s, d);
        return;
    }
    let r = format!("({d})/2");
    let pts: Vec<[String; 2]> = (0..96)
        .map(|i| {
            let a = 2.0 * PI * i as f64 / 96.0;
            let (c, sn) = (a.cos(), a.sin());
            let x = lin(&[(c, &r)]);
            let mut y = lin(&[(sn, &r)]);
            if sn < -1e-12 {
                let inside = format!("(1 - min(1, ceil(max(0, abs({x}) - ({}/2 - dish_edge))/(0.000001 mm))))", sq(zs));
                let u = format!("(({zs}) - (dish_z0 + dish_z1)/2)/((dish_z1 - dish_z0)/2)");
                let v = format!("({x})/(({} - 2*dish_edge)/2)", sq("(dish_z0 + dish_z1)/2"));
                y = format!("{y} - {inside}*max(0 mm, dish_deep*(1 - ({u})^2 - ({v})^2))");
            }
            // plate (x, y) to the sketch's (Y, Z)
            [y, format!("-({x})")]
        })
        .collect();
    s.poly(&pts);
}

pub fn script() -> String {
    let t = plate();
    let mut s = Script::new();
    s.len("ply", format!("{} mm", nn(t)), "plate thickness: every strip layer and plate, as the generator's --plate=");
    s.len("ref_top", format!("{} mm", nn(REF_TOP)), "Fischer's square bore at the top");
    s.len("sole_side", format!("{} mm", nn(REF_SOLE_SIDE)), "and at the sole");
    s.len("sole", format!("{} mm", nn(REF_SOLE)), "the top to the sole");
    s.len("throat", format!("{} mm", nn(REF_THROAT)), "the throat, drilled through the top: 13/64 in");
    s.len("seat_top", format!("{} mm", nn(REF_SEAT_TOP)), "the reed's seat at the top, reamed");
    s.num("reamer", nn(REAMER), "the reamer's taper, across per mm down, to the throat");
    s.len("tenon_d", format!("{} mm", nn(TENON_D.get())), "the tenon, hemped into your stock: the generator's --tenon=");
    s.len("collar_d", format!("{} mm", nn(COLLAR_D)), "the collar, the socket and its stop");
    s.len("foot_d", format!("{} mm", nn(FOOT_D)), "the foot's plates");
    s.len("sock_clear", format!("{} mm", nn(SOCK_CLEAR)), "the socket's and foot's clearance each side of the strips");
    s.len("x0", format!("{}*ply", N_TENON + N_COLLAR), &format!("read only: the strips' top, under {N_TENON} tenon and {N_COLLAR} collar plates"));
    s.len("pin", format!("{} mm", nn(pc::PIN)), "the tenon's and collar's brass pins");
    s.len("pin_interf", format!("{} mm", nn(pc::PIN_INTERF)), "their holes this much under the pin, a press-fit");
    s.len("r_pin", "(seat_top/2 + tenon_d/2)/2", "read only: the pins' distance from the axis");
    let d1 = DISHES[0];
    s.len("dish_z0", format!("{} mm", nn(d1.0)), "Fischer's first dish in the lid, from");
    s.len("dish_z1", format!("{} mm", nn(d1.1)), "to");
    s.len("dish_deep", format!("{} mm", nn(d1.2)), "and its deepest; it reaches up into the tenon and collar plates");
    s.len("dish_edge", format!("{} mm", nn(DISH_EDGE)), "each dish this much inside the bore's face, each side");
    for (k, &(h, z, d, f, st)) in REF_HOLES.iter().enumerate() {
        let k = k + 1;
        s.len(&format!("z_{k}"), format!("{} mm", nn(z)), &format!("the {h} hole in the {st}, down from the top: Fischer's"));
        s.len(&format!("d_{k}"), format!("{} mm", nn(d * INCH)), &format!("its starting drill, Fischer's; open it toward {:.2} mm as you tune", f * INCH));
    }
    // the strips: (name, layer, its u along, its w out, and its width past the bore's)
    let strips: Vec<(String, usize, [f64; 3], [f64; 3], &str, &str)> = (0..BACK_LAYERS)
        .map(|k| (format!("back layer {}", k + 1), k, [0.0, 0.0, -1.0], [0.0, 1.0, 0.0], "2*ply", "back"))
        .chain((0..2).map(|k| (format!("{} side", ["left", "right"][k]), 0, [0.0, [1.0, -1.0][k], 0.0], [0.0, 0.0, [1.0, -1.0][k]], "0 mm", "side")))
        .chain((0..LID_LAYERS).map(|k| (format!("lid layer {}", k + 1), k, [0.0, 0.0, 1.0], [0.0, -1.0, 0.0], "2*ply", "lid")))
        .collect();
    for (body, layer, u, w, wide, which) in &strips {
        let sk = s.sketch_on([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], *u, None, &format!("{body} sketch"));
        let hw = |z: &str| format!("({}/2 + ({wide})/2)", sq(z));
        s.poly(&[["x0".to_string(), format!("-{}", hw("x0"))], ["sole".into(), format!("-{}", hw("sole"))], ["sole".into(), hw("sole")], ["x0".into(), hw("x0")]]);
        s.finish();
        s.cmd("solid.extrude", format!("{{\"sketch\":\"{sk}\",\"distance\":\"4*foot_d\",\"direction\":\"symmetric\",\"operation\":\"new\",\"name\":\"{body}\",\"body_name\":\"{body}\"}}"));
        let kw = s.sketch_on([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], *w, None, &format!("{body}, wall sketch"));
        let off = |z: &str| format!("{}/2 + {layer}*ply", sq(z));
        let (xa, xb) = ("x0 - 1 mm", "sole + 1 mm");
        s.poly(&[[xa.to_string(), off(xa)], [xb.to_string(), off(xb)], [xb.to_string(), format!("{} + ply", off(xb))], [xa.to_string(), format!("{} + ply", off(xa))]]);
        s.finish();
        s.cmd(
            "solid.extrude",
            format!("{{\"sketch\":\"{kw}\",\"distance\":\"4*foot_d\",\"direction\":\"symmetric\",\"operation\":\"intersect\",\"targets\":[\"{body}\"],\"name\":\"{body}, wall\"}}"),
        );
        let _ = which;
    }
    // the holes, through every layer of their strip
    for (k, &(h, _, _, _, st)) in REF_HOLES.iter().enumerate() {
        let k = k + 1;
        let tg: Vec<&str> = strips.iter().filter(|e| e.5 == st).map(|e| e.0.as_str()).collect();
        let w = strips.iter().find(|e| e.5 == st).unwrap().3;
        s.cylinder(&format!("{h} hole"), [format!("z_{k}"), "0 mm".into(), "0 mm".into()], Some(w), &format!("d_{k}"), "2*foot_d", "cut", &tg);
    }
    // the tenon, the collar, each plate's bore its height's
    let mut pinned = vec![];
    for k in 0..N_TENON {
        let name = format!("tenon plate {}", k + 1);
        let z = (k as f64 + 0.5) * t;
        let zs = format!("{}*ply", nn(k as f64 + 0.5));
        let kk = across(&mut s, &format!("{k}*ply"), &format!("{name} sketch"));
        circ(&mut s, "tenon_d");
        bore(&mut s, &format!("max(throat, seat_top - reamer*{zs})"), z, &zs);
        s.finish();
        s.extrude(&kk, "ply", false, "new", &[], &name);
        pinned.push(name);
    }
    for k in 0..N_COLLAR {
        let name = format!("collar plate {}", k + 1);
        let z = ((N_TENON + k) as f64 + 0.5) * t;
        let zs = format!("{}*ply", nn((N_TENON + k) as f64 + 0.5));
        let kk = across(&mut s, &format!("{}*ply", N_TENON + k), &format!("{name} sketch"));
        circ(&mut s, "collar_d");
        bore(&mut s, "throat", z, &zs);
        s.finish();
        s.extrude(&kk, "ply", false, "new", &[], &name);
        pinned.push(name);
    }
    // the socket and the foot: the strips' outside at each plate's far face, and the clearance
    let plates: Vec<(String, String, &str)> = (0..N_SOCK)
        .map(|k| (format!("socket plate {}", k + 1), format!("x0 + {k}*ply"), "collar_d"))
        .chain((0..N_FOOT).map(|k| (format!("foot plate {}", k + 1), format!("sole - {}*ply", N_FOOT - k), "foot_d")))
        .collect();
    for (name, x, d) in &plates {
        let kk = across(&mut s, x, &format!("{name} sketch"));
        circ(&mut s, d);
        let sz = sq(&format!("{x} + ply"));
        let hw = format!("({sz}/2 + ply + sock_clear)");
        let lid = format!("(-{sz}/2 - {LID_LAYERS}*ply - sock_clear)");
        let back = format!("({sz}/2 + {BACK_LAYERS}*ply + sock_clear)");
        // plate (x, y) = (across, lid to back) is the sketch's (y, -x)
        s.poly(&[[lid.clone(), hw.clone()], [back.clone(), hw.clone()], [back.clone(), format!("-{hw}")], [lid.clone(), format!("-{hw}")]]);
        s.finish();
        s.extrude(&kk, "ply", false, "new", &[], name);
    }
    let tg: Vec<&str> = pinned.iter().map(|x| x.as_str()).collect();
    for (k, ang) in [(1, PIN_AT.0), (2, PIN_AT.1)] {
        // polar(ang, r) lands at Y = -r cos, Z = -r sin
        let a = ang.to_radians();
        s.cylinder(&format!("pin {k}"), ["-1 mm".to_string(), lin(&[(-a.cos(), "r_pin")]), lin(&[(-a.sin(), "r_pin")])], Some([1.0, 0.0, 0.0]), "pin - pin_interf", "x0 + 1 mm", "cut", &tg);
    }
    s.measure();
    s.text()
}

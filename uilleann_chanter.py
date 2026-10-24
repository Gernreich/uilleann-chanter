#!/usr/bin/env python3
"""Cut files for an uilleann pipe chanter whose bore is a tapering square of
flat strips of ply, Craig Fischer's square chanter (1997) cut on a laser:
a back, two sides and a lid, with a stacked tenon, collar and foot.

    python3 uilleann_chanter.py --no-write             # the checks
    python3 uilleann_chanter.py --out=DIR              # the sheet
    python3 uilleann_chanter.py --plate=2.9 ...        # for ply as measured
    python3 uilleann_chanter.py --kerf=0.18 ...        # for that kerf
    python3 uilleann_chanter.py --tenon=18 ...         # the tenon for your stock

THE REFERENCE is Craig Fischer's "Make a Square Uilleann Pipe Chanter"
(1997, hosted by David Daye at daye1.com/bagpage/makepipe/squint1.html): a
square cone 3.7mm at the top to 11.5mm at the sole over 365mm, the
cross-section of a Quinn chanter's bore, made as a box of four flat pieces
of wood, eight holes, and a lid held on with silicone while the chanter is
tuned, then glued. This generator keeps his bore and his holes and their
starting drill sizes exactly; nothing here is modelled. His tuning is still
done by hand: holes opened toward his final sizes, and the bore enlarged
further where a note asks for it.

HIS FOUR STARTING ENLARGEMENTS, the oval dishes he gouges into the lid's
inside face before it goes on (DISHES), are a greyscale engrave on the lid's
bore layer, darker deeper: each an elliptic paraboloid, deepest at its
middle, never wider than the bore's face less DISH_EDGE either side, so no
dish runs under a side strip's glue line. The first starts at 30mm, above the
strips, so the plates there have it CUT instead: a lobe on the front of
their throat, as deep as the dish is at that plate. Engrave depth in ply is
not linear and the glue lines engrave unlike the wood, so set black to
DEPTH_BLACK on scrap of the same sheet first, and burn shallow rather than
deep: gouging more out is easy, putting it back is not.

Fischer's lid is no thinner than 7mm, since a hole's depth sets its note.
Ply is 3mm, so the lid is LID_LAYERS strips glued face to face, 9mm: a
deeper hole sounds flatter, and a chanter tuned by opening holes up wants
to start flat. He thins a thick lid over its holes from the outside, and so
can you. The back, under the thumb, can be thinner: BACK_LAYERS, 6mm.

From the top down:

  * the TENON: N_TENON round plates with the reed's seat stepped through
    them, 7.0mm at the top narrowing at a taper reamer's rate to Fischer's
    5.16mm (13/64in) throat, which runs on to the strips;
  * the COLLAR: N_COLLAR wider round plates, the stop Fischer suggests so
    the tenon goes only so far into the stock; the tenon and collar press
    together on two pins;
  * the SOCKET: N_SOCK round plates with a rectangular hole the strips' top
    stands in; dry while you tune, glued when the lid is;
  * the STRIPS, from the socket to the sole: the back, BACK_LAYERS thick and
    the bore's width plus both sides wide, with the thumb's hole; two sides,
    one ply thick and exactly the bore's width, glued on the back's face;
    and the lid, LID_LAYERS thick, with the seven front holes;
  * the FOOT: N_FOOT round plates round the strips' bottom, the tube Fischer
    suggests to hold the glue lines at the sole.
"""
import math
from fractions import Fraction
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, '..', 'practice-chanters'))
import practice_chanters as P                                  # noqa: E402
from practice_chanters import M, SF, Part, circle              # noqa: E402

KERF = P.KERF
INCH = 25.4
# --- the reference chanter, Fischer's figures (mm, down from the top) --------
REF_TOP, REF_SOLE_SIDE, REF_SOLE = 3.7, 11.5, 365.0   # the square's side, top and sole
REF_THROAT = 13 / 64 * INCH        # drilled through the top: 5.16mm
REF_SEAT_TOP = 7.0                 # the reed's seat at the top, reamed
REAMER = (16.0 - 3.0) / 85.0       # his reamer's taper, across per mm
REF_TENON = 35.0                   # the tenon's length
REF_LID = 7.0                      # the lid no thinner than this
# each hole: down from the top, drill size and final size (in), and its strip
REF_HOLES = [("D'", 92.5, 7 / 32, 9 / 32, 'back'), ('C#', 110.0, 11 / 64, 7 / 32, 'lid'),
             ('B', 139.0, 9 / 64, 7 / 32, 'lid'), ('A', 172.0, 3 / 16, 9 / 32, 'lid'),
             ('G', 212.0, 11 / 64, 11 / 64, 'lid'), ('F#', 242.0, 7 / 32, 9 / 32, 'lid'),
             ('E', 275.0, 11 / 64, 11 / 64, 'lid'), ('Eb', 306.0, 3 / 16, 9 / 32, 'lid')]
# his starting bore enlargements in the lid's inside face: from, to (mm down)
# and deepest. His text gives the first, 30 to 90 at 1.5mm, and the depths;
# his sketch of the lid, to scale from C# down (its holes within 1.5mm of
# his table), gives the other three's ends, which his text names only by the
# holes beside them. The sketch draws the first at 58 to 96: the text is kept.
DISHES = [(30.0, 90.0, 1.5), (112.0, 162.0, 1.2), (247.0, 275.0, 1.2), (311.0, 355.0, 1.2)]
# --- this chanter -------------------------------------------------------------
LID_LAYERS = 3                     # 9mm: at least Fischer's 7
BACK_LAYERS = 2                    # 6mm under the thumb
N_TENON = 12                       # 36mm, Fischer's 35
TENON_D = 17.0                     # hemped into the stock; --tenon= for yours
N_COLLAR = 2
COLLAR_D = 30.0
N_SOCK = 2
SOCK_CLEAR = 0.1                   # each side of the strips: a sliding fit while you tune
N_FOOT = 2
FOOT_D = 38.0
DISH_EDGE = 0.3                    # each dish this much inside the bore's face, each side
DEPTH_BLACK = 1.5                  # the engrave's black, mm: the deepest dish
PX = 10                            # the engrave's pixels per mm
BLUE = '#0000ff'                   # the house engrave ink
PIN_AT = (80.0, 250.0)             # the tenon's pins, degrees round from the front


def side(z):
    """The bore's square, z down from the top."""
    return REF_TOP + (REF_SOLE_SIDE - REF_TOP) * z / REF_SOLE


def seat_d(z):
    """The reed's seat, then the throat, z down."""
    return max(REF_THROAT, REF_SEAT_TOP - REAMER * z)


def box(z, t, clear=0.0):
    """The strips' outside at z, in the plate frame: x across, y from the
    lid (-y) to the back, the bore centred."""
    s = side(z)
    hw = s / 2 + t + clear
    return [(-hw, -s / 2 - LID_LAYERS * t - clear), (hw, -s / 2 - LID_LAYERS * t - clear),
            (hw, s / 2 + BACK_LAYERS * t + clear), (-hw, s / 2 + BACK_LAYERS * t + clear)]


def dish_w(dish):
    """A dish's width at its middle: the face there, less DISH_EDGE each side."""
    z0, z1, D = dish
    return side((z0 + z1) / 2) - 2 * DISH_EDGE


def dish_depth(z, y):
    """How deep the dishes are at z down, y across the lid's face from its
    centre line: each an elliptic paraboloid, nothing past the face less
    DISH_EDGE."""
    if abs(y) > side(z) / 2 - DISH_EDGE:
        return 0.0
    out = 0.0
    for z0, z1, D in DISHES:
        u = (z - (z0 + z1) / 2) / ((z1 - z0) / 2)
        v = y / (dish_w((z0, z1, D)) / 2)
        out = max(out, D * (1 - u * u - v * v))
    return out


def lobed(r, z, n=96):
    """A round hole r with the dishes at z added on its front (-y): each
    point of the front half pushed out by the dish's depth across from it."""
    pts = []
    for i in range(n):
        a = 2 * math.pi * i / n
        x, y = r * math.cos(a), r * math.sin(a)
        if y < 0:
            y -= dish_depth(z, x)
        pts.append((x, y))
    return pts


def png(rows):
    """A greyscale PNG from rows of 0-255, without PIL."""
    import struct
    import zlib
    h, w = len(rows), len(rows[0])
    raw = b''.join(b'\0' + bytes(r) for r in rows)

    def chunk(tag, data):
        c = struct.pack('>I', len(data)) + tag + data
        return c + struct.pack('>I', zlib.crc32(tag + data) & 0xffffffff)
    return (b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', w, h, 8, 0, 0, 0, 0))
            + chunk(b'IDAT', zlib.compress(raw, 9)) + chunk(b'IEND', b''))


def dish_image(dish, x0):
    """A dish's engrave on the lid's bore layer, in the strip's frame (x
    along from its top, y across): (x, y, w, h, rows), white nothing,
    black DEPTH_BLACK."""
    z0, z1, D = dish
    za = max(z0, x0)
    w = dish_w(dish)
    nx, ny = int(math.ceil((z1 - za) * PX)), int(math.ceil(w * PX))
    rows = []
    for j in range(ny):
        y = -w / 2 + (j + 0.5) / PX
        rows.append([max(0, min(255, round(255 * (1 - dish_depth(za + (i + 0.5) / PX, y)
                                                   / DEPTH_BLACK)))) for i in range(nx)])
    return (za - x0, -w / 2, nx / PX, ny / PX, rows)


_turned = P.turned


def turned(p):
    """practice_chanters' turned(), which also turns a part's engraves."""
    q = _turned(p)
    q.flip = not getattr(p, 'flip', False)
    return q


P.turned = turned


def build():
    t = M.PLATE
    num = iter(range(1, 9999))
    parts = []

    def part(name, outline, cuts, x, kind, note='', rnd=None):
        p = Part(name, next(num), outline, note)
        p.cuts = list(cuts)
        p.x, p.kind, p.round = x, kind, rnd
        parts.append(p)
        return p
    x0 = (N_TENON + N_COLLAR) * t              # the strips' top, down from the top
    Ls = REF_SOLE - x0
    # the strips, x along from their top, y across from their centre line
    def strip(wide):
        w0, w1 = side(x0) + wide, side(REF_SOLE) + wide
        return [(0.0, -w0 / 2), (Ls, -w1 / 2), (Ls, w1 / 2), (0.0, w0 / 2)]
    holes = {st: [('circle', z - x0, 0.0, d * INCH / 2) for h, z, d, f, s in REF_HOLES if s == st]
             for st in ('back', 'lid')}
    for k in range(BACK_LAYERS):
        p = part('back', strip(2 * t), holes['back'], x0, 'strip',
                 f'back layer {k + 1} of {BACK_LAYERS}' + (', the bore face' if k == 0 else ''))
        p.strip, p.layer = 'back', k
    for k in range(2):
        p = part('side', strip(0.0), [], x0, 'strip', f'side {k + 1}, the bore\'s width')
        p.strip, p.layer = ('left', 'right')[k], 0
    for k in range(LID_LAYERS):
        p = part('lid', strip(2 * t), holes['lid'], x0, 'strip',
                 f'lid layer {k + 1} of {LID_LAYERS}' + (', the bore face, the dishes engraved'
                                                         if k == 0 else ''))
        p.strip, p.layer = 'lid', k
        p.dishes = [dish_image(d, x0) for d in DISHES if d[1] > x0] if k == 0 else []
    # the tenon and collar, pinned midway between the seat and the tenon's edge
    pin_d = P.PIN - P.PIN_INTERF
    r_pin = (REF_SEAT_TOP / 2 + TENON_D / 2) / 2
    # off the front, where the first dish's lobe is, and not opposite each
    # other, so a plate goes on only one way and its lobe faces the lid
    pins = [P.polar(a, r_pin) for a in PIN_AT]
    pin_cut = [('circle', x, y, pin_d / 2) for x, y in pins]

    def bore(d, z):
        """A plate's bore: round, or lobed where a dish reaches it."""
        if max(dish_depth(z, 0.0), 0.0) > 0:
            return ('poly', lobed(d / 2, z))
        return ('circle', 0.0, 0.0, d / 2)
    for k in range(N_TENON):
        z = (k + 0.5) * t
        d = seat_d(z)
        part('tenon', circle(0, 0, TENON_D / 2, 72), [bore(d, z)] + pin_cut,
             k * t, 'tenon', f'tenon plate {k + 1}, {d:.2f}mm', rnd=TENON_D / 2)
    for k in range(N_COLLAR):
        z = (N_TENON + k + 0.5) * t
        part('collar', circle(0, 0, COLLAR_D / 2, 72), [bore(REF_THROAT, z)] + pin_cut,
             (N_TENON + k) * t, 'collar', f'collar plate {k + 1}', rnd=COLLAR_D / 2)
    for k in range(N_SOCK):
        x = x0 + k * t
        part('socket', circle(0, 0, COLLAR_D / 2, 72), [('poly', box(x + t, t, SOCK_CLEAR))], x,
             'sock', f'socket plate {k + 1}', rnd=COLLAR_D / 2)
    for k in range(N_FOOT):
        x = REF_SOLE - (N_FOOT - k) * t
        part('foot', circle(0, 0, FOOT_D / 2, 72), [('poly', box(x + t, t, SOCK_CLEAR))], x,
             'foot', f'foot plate {k + 1}', rnd=FOOT_D / 2)
    info = dict(t=t, x0=x0, Ls=Ls, pins=pins, r_pin=r_pin, pin_d=pin_d)
    return parts, info


def checks(parts, info, sheets):
    out = []

    def note(ok, what, detail):
        out.append((ok, what, detail))
    t, x0, Ls = info['t'], info['x0'], info['Ls']
    lid, back = LID_LAYERS * t, BACK_LAYERS * t
    note(abs(x0 + Ls - REF_SOLE) < 1e-9 and side(0) == REF_TOP and side(REF_SOLE) == REF_SOLE_SIDE,
         'the bore is Fischer\'s square cone',
         f'{side(x0):.2f}mm square at the strips\' top to {side(REF_SOLE):.2f}mm at the sole, '
         f'{REF_TOP:g} to {REF_SOLE_SIDE:g}mm over {REF_SOLE:g}mm from the top; strips {Ls:.1f}mm')
    note(abs(N_TENON * t - REF_TENON) <= t, 'the tenon is Fischer\'s length',
         f'{N_TENON * t:g}mm, his {REF_TENON:g}; {TENON_D:g}mm round for hemp')
    seats = [seat_d((k + 0.5) * t) for k in range(N_TENON)]
    note(seats[0] <= REF_SEAT_TOP and min(seats) >= REF_THROAT - 1e-9,
         'the seat steps down to Fischer\'s throat',
         f'{seats[0]:.2f}mm at the top to {min(seats):.2f}mm, his {REF_SEAT_TOP:g} reamed to '
         f'{REF_THROAT:.2f}mm drilled; ream it smooth')
    note(lid >= REF_LID, 'the lid is as deep as Fischer asks',
         f'{LID_LAYERS} layers, {lid:g}mm, his at least {REF_LID:g}; the back {back:g}mm')
    # every hole: on its strip's centre line, inside the bore's face at its
    # starting size, below the socket
    ok, detail = True, []
    for h, z, d, f, st in REF_HOLES:
        dm, s = d * INCH, side(z)
        ok &= dm < s and z - dm / 2 > x0 + N_SOCK * t
        detail.append(f'{h} {dm:.2f}/{s:.2f}')
    note(ok, 'every hole starts inside its face, below the socket',
         'hole/face mm: ' + ', '.join(detail))
    moved = max(abs(c[1] + x0 - z) for p in parts if p.kind == 'strip'
                for c, (h, z, d, f, st) in zip(p.cuts, [r for r in REF_HOLES if r[4] == p.strip]))
    note(moved < 1e-9 and sum(len(p.cuts) for p in parts if p.layer_of('lid')) == 7 * LID_LAYERS,
         'every hole is where Fischer\'s is', f'{len(REF_HOLES)} holes, within {moved:.1e}mm, '
         f'through every layer of its strip')
    # webs: the pins in the tenon and collar
    pr = info['pin_d'] / 2
    to_bore, to_edge = math.inf, math.inf
    for p in parts:
        if p.kind not in ('tenon', 'collar'):
            continue
        b = p.cuts[0]
        for x, y in info['pins']:
            d = (P.poly_dist((x, y), b[1]) if b[0] == 'poly' else math.hypot(x, y) - b[3])
            to_bore = min(to_bore, d - pr)
            to_edge = min(to_edge, p.round - math.hypot(x, y) - pr)
    note(min(to_bore, to_edge) >= P.MIN_WEB, 'every pin keeps ply round it',
         f'{to_bore:.2f}mm to the bore, lobes and all, {to_edge:.2f}mm to the edge, '
         f'against {P.MIN_WEB:g}')
    a = [P.polar(x, 1.0) for x in PIN_AT]
    one_way = all(min(math.hypot(m(*a[0])[0] - q[0], m(*a[0])[1] - q[1]) +
                      math.hypot(m(*a[1])[0] - r[0], m(*a[1])[1] - r[1])
                      for q, r in (a, a[::-1])) > 0.1
                  for m in (lambda x, y: (-x, -y), lambda x, y: (-x, y)))
    note(one_way, 'a tenon or collar plate goes on only one way',
         f'pins at {PIN_AT[0]:g} and {PIN_AT[1]:g} degrees from the front: no half turn or flip '
         f'brings them back, so every lobe faces the lid')
    # the dishes: inside the face, in the lid's bore layer, and their part
    # above the strips cut as lobes
    lid0 = next(p for p in parts if getattr(p, 'strip', '') == 'lid' and p.layer == 0)
    # every engraved pixel, read back: how far inside the face it stays
    edge = math.inf
    for x, y, w, h, rows in lid0.dishes:
        for j, row in enumerate(rows):
            for i, g in enumerate(row):
                if g < 255:
                    z = x0 + x + (i + 0.5) / PX
                    yy = y + (j + 0.5) / PX
                    edge = min(edge, side(z) / 2 - abs(yy) - DISH_EDGE)
    deep = max(d[2] for d in DISHES)
    lobes = [p for p in parts if p.kind in ('tenon', 'collar') and p.cuts[0][0] == 'poly']
    worst_lobe = max((max(-y for x, y in p.cuts[0][1]) - (REF_THROAT / 2 if p.kind == 'collar'
                      else seat_d(p.x + t / 2) / 2), p.x) for p in lobes) if lobes else (0, 0)
    note(edge >= -1e-9 and deep <= DEPTH_BLACK and len(lid0.dishes) == len(DISHES),
         'Fischer\'s four dishes are in the lid\'s bore face',
         f'{len(lid0.dishes)} greyscale engraves, black {DEPTH_BLACK:g}mm, deepest {deep:g}mm; '
         f'every engraved pixel {DISH_EDGE:g}mm or more inside the face ({edge + DISH_EDGE:.2f} at the least)')
    note(all(p.x + t > DISHES[0][0] for p in lobes) and len(lobes) == round((x0 - DISHES[0][0]) / t),
         'the first dish, above the strips, is cut in its plates',
         f'{len(lobes)} plates from {DISHES[0][0]:g}mm, a lobe on the front of each, '
         f'{worst_lobe[0]:.2f}mm deep at the deepest')
    worst = math.inf
    for p in parts:
        if p.kind in ('sock', 'foot'):
            far = max(math.hypot(*v) for v in p.cuts[0][1])
            worst = min(worst, p.round - far)
    note(worst >= P.MIN_WEB, 'the socket and foot keep ply round the strips',
         f'{worst:.2f}mm at their corners, against {P.MIN_WEB:g}')
    narrow = side(x0)
    note(narrow >= 4.0, 'the sides are wide enough to cut',
         f'{narrow:.2f}mm at their top, {side(REF_SOLE):.2f}mm at the sole')
    note(all(sh['w'] <= P.BED[0] and sh['h'] <= P.BED[1] for sh in sheets), 'every sheet fits the bed',
         f'{len(sheets)} sheet(s), largest {max(sh["w"] for sh in sheets):.0f} x '
         f'{max(sh["h"] for sh in sheets):.0f}mm')
    least = math.inf
    for sh in sheets:
        its = sh['parts']
        for i, (a, ax, ay) in enumerate(its):
            for c, cx, cy in its[i + 1:]:
                if (cx > ax + a.w + SF.GAP + KERF or ax > cx + c.w + SF.GAP + KERF
                        or cy > ay + a.h + SF.GAP + KERF or ay > cy + c.h + SF.GAP + KERF):
                    continue
                least = min(least, P.apart(a, ax, ay, c, cx, cy))
    note(least >= SF.GAP + KERF - 1e-3, 'no two parts on a sheet come closer than the gap',
         f'{least - KERF:.2f}mm of ply between cuts at the least, against {SF.GAP:g}')
    placed_n = sum(len(sh['parts']) for sh in sheets)
    note(placed_n == len(parts), 'every part is on a sheet', f'{placed_n} of {len(parts)}')
    return out


Part.layer_of = lambda self, st: getattr(self, 'strip', None) == st


def svg_sheet(k, nsh, sh):
    """practice_chanters' sheet, with the dishes' engraves before the cuts."""
    import base64
    imgs = []
    for p, sx, sy in sh['parts']:
        dx, dy = sx - p.x0 + KERF / 2, sy - p.y0 + KERF / 2
        for x, y, w, h, rows in getattr(p, 'dishes', []):
            if getattr(p, 'flip', False):
                x, y, rows = -x - w, -y - h, [r[::-1] for r in rows[::-1]]
            data = base64.b64encode(png(rows)).decode()
            imgs.append(f'<image x="{x + dx:.3f}" y="{y + dy:.3f}" width="{w:.3f}" '
                        f'height="{h:.3f}" preserveAspectRatio="none" '
                        f'href="data:image/png;base64,{data}"/>')
    txt = P.svg_sheet(k, nsh, sh, 4)
    txt = (txt.replace('Practice chanter, 4-sided bore',
                       'Uilleann chanter, Craig Fischer\'s square bore in strips')
           .replace('Orange', f'Engraves first: blue {BLUE} marks the greyscale image engraves, '
                    f'Fischer\'s four bore dishes in the lid\'s bore layer, Grayscale mode in xTool '
                    f'Creative Space, black {DEPTH_BLACK:g}mm deep, set on scrap of this sheet; '
                    f'burn shallow rather than deep. Then orange', 1)
           .replace('cuts the bores, seats and pin holes first; then black',
                    'cuts the seats, finger and pin holes first; then black'))
    if imgs:
        txt = txt.replace('</desc>\n', '</desc>\n  <g id="images">\n'
                          + ''.join(f'    {i}\n' for i in imgs) + '  </g>\n', 1)
    return txt


def main():
    global KERF, TENON_D
    out, write = None, True
    for x in sys.argv[1:]:
        if x == '--no-write':
            write = False
        elif x.startswith('--out='):
            out = x.split('=', 1)[1]
        elif x.startswith('--plate='):
            M.PLATE = float(x.split('=', 1)[1])
        elif x.startswith('--kerf='):
            KERF = P.KERF = SF.set_kerf(x)
        elif x.startswith('--tenon='):
            TENON_D = float(x.split('=', 1)[1])
        else:
            raise SystemExit(f'error: {x} is not a flag this generator reads. It takes --out=, '
                             f'--plate=, --kerf=, --tenon= and --no-write.')
    if write and not out:
        raise SystemExit('error: --out=DIR, or --no-write to run the checks only.')
    parts, info = build()
    sheets = P.pack(parts)
    res = checks(parts, info, sheets)
    t = info['t']
    print(f'uilleann chanter, Craig Fischer\'s square bore in {t:g}mm strips: '
          f'{side(info["x0"]):.2f} to {side(REF_SOLE):.2f}mm square, strips {info["Ls"]:.1f}mm; '
          f'{REF_SOLE:g}mm from the top to the sole')
    for h, z, d, f, st in REF_HOLES:
        print(f'  {h:3} {st:4} {z:6.1f}mm down, cut {d * INCH:.2f}mm ({Fraction(round(d * 64), 64)}in), '
              f'open to about {f * INCH:.2f}mm; the face {side(z):.2f}mm')
    print(f'  top down: {N_TENON} tenon, {N_COLLAR} collar, {N_SOCK} socket over the strips; '
          f'the back {BACK_LAYERS} layers, 2 sides, the lid {LID_LAYERS} layers; '
          f'{N_FOOT} foot plates at the sole')
    bad = 0
    for ok, what, detail in res:
        print(f'  {"pass" if ok else "FAIL"}  {what:52} {detail}')
        bad += not ok
    if bad:
        print(f'\n  {bad} check(s) failed. Nothing written.')
        return 1
    if write:
        os.makedirs(out, exist_ok=True)
        for k, sh in enumerate(sheets, 1):
            f = os.path.join(out, f'uilleann-chanter-sheet{k}-cut-files.svg')
            open(f, 'w').write(svg_sheet(k, len(sheets), sh))
            print(f'  wrote {os.path.basename(f)}  {sh["w"]:.0f} x {sh["h"]:.0f}mm, '
                  f'{len(sh["parts"])} parts')
    return 0


if __name__ == '__main__':
    sys.exit(main())

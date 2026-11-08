# Uilleann chanter

**[← All projects](https://gernreich.github.io/)**

<!-- readme-only -->
**[Read the writeup](https://gernreich.github.io/uilleann-chanter/)** ·
**[Turn it in 3D](https://gernreich.github.io/uilleann-chanter/)**

<!-- page-only **[Read the README](https://github.com/Gernreich/uilleann-chanter)** · **[Turn it in 3D on a page of its own](turn.html)** -->

<!-- page-only <iframe src="turn.html?embed" title="The uilleann chanter as solid parts you can drag to turn and pull apart: a long, darker, tapering square tube of strips with seven holes along the front, a narrow round tenon and a wider collar at the narrow end, and a round foot at the wide end" style="width:100%;height:560px;border:0"></iframe> -->

An uilleann pipe chanter in 3 mm plywood: Craig Fischer's square chanter (1997), cut on
a laser. Its conical bore is a tapering square made from four flat strips: a back, two
sides and a lid. Fischer's article is
[Make a Square Uilleann Pipe Chanter](https://www.daye1.com/bagpage/makepipe/squint1.html),
hosted by David Daye. This generator keeps his bore, his hole positions and his
starting drill sizes, and engraves his starting bore enlargements. Tuning is still done by hand, his way.

**It has not been cut or played yet.**

<table>
<tr>
<td align="center"><a href="cut-files/uilleann-chanter-sheet1-cut-files.svg"><img src="previews/uilleann-chanter-sheet1-cut-files.svg" alt="A sheet of parts: seven long tapering strips, two with one hole near their narrow end, three with seven holes in a row and grey oval shading along the first of them, and two plain narrow ones; two large round plates and two smaller ones with a rectangular hole each; two round plates with a small teardrop hole and two pin holes; and twelve small round plates with a hole and two pin holes, the last two of them teardrop-shaped" width="100%"></a></td>
</tr>
<tr>
<td align="center"><sub>the whole chanter on one 587 × 166 mm sheet</sub></td>
</tr>
</table>

Built for **[LaserMadeMusic](https://www.youtube.com/@LaserMadeMusic)**.

**[The rest of the build files](https://gernreich.github.io/lasermademusic.html)** — every instrument,
generator and tool, indexed.

**[Download everything as a ZIP](https://github.com/Gernreich/uilleann-chanter/archive/refs/heads/main.zip)**

## The chanter

| | |
|---|---|
| Bore | Fischer's square cone, 3.7 mm square at the top to 11.5 mm at the sole over 365 mm, matching a Quinn chanter's cross-section. The strips start 42 mm down at 4.60 mm and end at 11.50 mm |
| Back | 2 layers glued face to face, 6 mm, with the thumb's hole (back D) |
| Sides | 1 layer each, exactly as wide as the bore, glued on the back's face |
| Lid | 3 layers, 9 mm, with the seven front holes. Fischer asks for at least 7 mm, because a hole's depth sets its note. A deeper hole plays flatter, and you tune by opening holes, so the lid starts deep. Thin it over the holes from outside if a note won't come up |
| Dishes | Fischer's four starting bore enlargements, engraved in greyscale into the lid's bore layer (darker is deeper, black 1.5 mm): 30–90 mm 1.5 mm deep, 112–162 mm, 247–275 mm and 311–355 mm 1.2 mm deep. Each is an oval dish, never closer than 0.3 mm to the edge of the bore's face, so none runs under a side's glue line. The 30–42 mm part sits above the strips and is cut instead, as a lobe on the front of 4 plates' bore holes |
| Tenon | 12 round plates, 17 mm across (`--tenon=` for your stock). The reed's seat steps down from 6.8 mm to Fischer's 5.16 mm (13/64in) throat. Ream it smooth |
| Collar | 2 plates 30 mm across, the stop Fischer suggests. The tenon and collar press-fit on two 2 mm pins |
| Socket | 2 plates the strips' top stands in. Leave them dry while you tune, and glue them when the lid goes on |
| Foot | 2 plates 38 mm across round the sole, the tube Fischer suggests to hold the glue lines |
| Sheet | one 587 × 166 mm sheet of 3 mm ply, 25 parts |

| Hole | Down from the top | Strip | Cut | Open up to about |
|---|---|---|---|---|
| back D | 92.5 mm | back | 5.56 mm (7/32in) | 7.14 mm (9/32in) |
| C# | 110 mm | lid | 4.37 mm (11/64in) | 5.56 mm (7/32in) |
| B | 139 mm | lid | 3.57 mm (9/64in) | 5.56 mm (7/32in) |
| A | 172 mm | lid | 4.76 mm (3/16in) | 7.14 mm (9/32in) |
| G | 212 mm | lid | 4.37 mm (11/64in) | 4.37 mm (11/64in) |
| F# | 242 mm | lid | 5.56 mm (7/32in) | 7.14 mm (9/32in) |
| E | 275 mm | lid | 4.37 mm (11/64in) | 4.37 mm (11/64in) |
| E flat | 306 mm | lid | 4.76 mm (3/16in) | 6.35–7.14 mm (1/4–9/32in) |

## Building it

1. Glue each strip's layers face to face, with drill bits through the holes to line them up.
2. Glue the two sides onto the back's bore face, flush with its edges.
3. The dishes are engraved when you cut, before the cuts: the images, in Grayscale mode in xTool
   Creative Space. Depth in ply doesn't follow the grey evenly, and glue lines burn
   differently from wood, so on scrap from the same sheet, set black to 1.5 mm.
   Burn shallow rather than deep: gouging more out is easy, putting wood back isn't.
   Fischer's sketch draws the first dish at 58–96 mm, not the 30–90 mm his text gives.
   The text is kept here (`DISHES` in the generator).
4. Hold the lid on with silicone. The tenon and collar plates' pins aren't opposite
   each other, so each plate goes on one way, with its lobe toward the lid.
5. Slide the socket on dry, press the tenon and collar together on their pins, and tune
   by Fischer's method: open the holes from the high notes down, and lift the lid to
   change the bore.
6. When it plays, glue the lid, the socket and the foot, and seal the seams and the
   bore with shellac.

The reed is Fischer's: a wide-headed reed on a tubing staple. His article covers that too.

## Regenerating

```
cargo build --release                                # once, and after any change
target/release/uilleann_chanter --no-write          # the checks
target/release/uilleann_chanter --out=cut-files     # the sheet
target/release/uilleann_chanter --plate=2.9 --kerf=0.18 --tenon=18 --out=cut-files
target/release/uilleann_chanter --page-to=turn.html # the 3D view
target/release/uilleann_chanter --bores             # what bore-tmm reads of its bore
```

The generator, `uilleann_chanter.rs`, is Rust since 2026-11-07, and Python before that:
the move changed no byte of the cut file or the page. It builds on `../practice-chanters`
for the sheet packer and the SVG writer, and on `../square-flute` and
`../lasermade-tools` beside it; the engraves' PNGs are compressed by the system's zlib,
as Python's were. `Cargo.toml` and `Cargo.lock` are the crate and its pinned build. This page,
`index.html`, is built from this README, and the sheet's preview from the cut file,
by the tools in [lasermade-tools](https://github.com/Gernreich/lasermade-tools).
`.repro` records how each file is drawn, for the gate.

## STEP files

`step/` holds the 3D view as STEP files, one per design, to open or import in a
CAD program such as SolidCraft, Fusion or FreeCAD: one solid body per part, in
millimetres, assembled as the view shows it. Every face is flat, so a curve comes
in as short straight runs; engraving has no depth and is left out. They are written from `turn.html` by
lasermade-tools' `step-export`, and `.repro` checks them:

```
../lasermade-tools/target/release/step-export turn.html --name=uilleann-chanter
```

## Credit

The design is Craig Fischer's: the square bore, the hole positions and drill sizes, the
bore enlargements, the tuning method and the reed are all from his article
[Make a Square Uilleann Pipe Chanter](https://www.daye1.com/bagpage/makepipe/squint1.html)
(© 1997 Craig Fischer), hosted by David Daye, who also sells reeds for it. Read the
article for the tuning and the reed; this repository only turns his measurements into
cut files.

This repository is written by Michael Gernreich with [Claude](https://www.anthropic.com/claude) (Anthropic), working in Claude Code, which co-wrote the generator and the documentation. Commits Claude made carry a `Co-Authored-By: Claude` line.

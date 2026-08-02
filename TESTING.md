# Manual Terminal Box-Drawing Visual Checks

Perform these checks in a development build. They cover rendering details that are not verified by automated tests.

## Terminal Panel

Use Iosevka or PragmataPro if installed because the original seams were particularly visible with these fonts. Start with line height 1.0:

```json
{
  "terminal": {
    "font_family": "Iosevka",
    "line_height": {
      "custom": 1.0
    }
  }
}
```

Render box borders and decorated cells:

```sh
printf '%b\r\n' \
  '╭────────╮' \
  '│ A─┄═B │' \
  '├────────┤' \
  '╰────────╯'

printf '%b\n' \
  'underline:     \033[31;4mA─┄═│╭─╮B\033[0m' \
  'undercurl:     \033[32;4:3mA─┄═│╭─╮B\033[0m' \
  'strikethrough: \033[34;9mA─┄═│╭─╮B\033[0m' \
  'combined:      \033[35;4;9mA├──┤B\033[0m' \
  'dim underline: \033[2;4mA│B\033[0m'
```

Visually verify:

- Vertical strokes connect between rows without horizontal seams.
- Horizontal strokes and mixed-weight junctions connect without gaps or blurry transitions.
- Rounded arcs meet their straight stubs without a kink, gap, or visible change in weight.
- Underline, undercurl, and strikethrough remain visible beneath or across procedural box cells.
- Underlines beneath procedural glyphs have the same vertical position as underlines beneath the adjacent regular text (`A`, `B`), without stepping at cell boundaries.
- Decorations have the same thickness and color across regular text (`A`, `B`), procedural glyphs (`─`, `│`, `╭`, `╮`), and font-fallback glyphs (`┄`, `═`).
- Wavy undercurls do not have a conspicuous phase jump at every procedural cell boundary.
- Intersections between dim decorations and box strokes are not noticeably darker.

Repeat these checks with `"line_height": "standard"` (1.3) and `"line_height": "comfortable"` (1.618). If available, repeat them on a display using 125% or 150% scaling.

## Block cursor

With line height set to `"comfortable"`, run:

```sh
python3 - <<'PY'
import sys
import time

glyphs = "│╭┼┄═"
sys.stdout.write("\033[1 q" + glyphs + "\r")
sys.stdout.flush()
for column in range(len(glyphs)):
    sys.stdout.write("\r" + (f"\033[{column}C" if column else ""))
    sys.stdout.flush()
    time.sleep(5)
PY
```

Visually verify:

- While the blinking block cursor covers `│`, `╭`, and `┼`, each glyph retains the same shape, stroke weight, and connections in the cursor-visible and cursor-hidden phases; only its colors change.
- In the cursor-visible phase, each procedural glyph remains visible in the terminal background color inside the solid cursor fill, confirming that it is repainted after the cursor.
- The fallback-font glyphs `┄` and `═` continue to render normally beneath the block cursor.
- While the command is running, focus another pane while keeping the terminal visible. The resulting hollow cursor does not recolor or replace the glyph beneath it.

## OSC 8 hyperlink styling

Run:

```sh
printf '%b\n' '\033]8;;https://example.com\033\\A├──┤B\033]8;;\033\\'
```

Visually verify:

- The underline runs continuously beneath `A├──┤B`.
- Holding `Ctrl` on Linux or Windows, or `Cmd` on macOS, and hovering applies the hover color to the entire link.
- Procedural box glyphs do not retain the old color or lose their underline while hovered.

## REPL output

In a Python file, run the following code through Zed's REPL:

```python
print("\033[31;4mA─┄═│╭─╮B\033[0m")
print("\033[32;4:3mA─┄═│╭─╮B\033[0m")
print("\033[34;9mA─┄═│╭─╮B\033[0m")
```

Visually verify:

- No missing decoration beneath procedural glyphs.
- The same vertical decoration placement across procedural and font-rendered characters.
- Correct red, green, and blue decoration colors.

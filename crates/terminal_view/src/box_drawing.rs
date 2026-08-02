use gpui::{
    Bounds, Hsla, PathBuilder, Pixels, Point, Size, StrikethroughStyle, TextRun, TextStyle,
    UnderlineStyle, Window, fill, point, px,
};
use terminal::TerminalBounds;
use util::ResultExt;

use crate::terminal_element::LayoutPoint;

const LIGHT_STROKE_DIVISOR: f32 = 8.;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stroke {
    Light,
    Heavy,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct BoxDrawingGlyph {
    left: Option<Stroke>,
    right: Option<Stroke>,
    up: Option<Stroke>,
    down: Option<Stroke>,
    rounded: bool,
}

fn glyph_for(ch: char) -> Option<BoxDrawingGlyph> {
    if !matches!(
        ch,
        '\u{2500}'..='\u{2503}'
            | '\u{250c}'..='\u{254b}'
            | '\u{256d}'..='\u{2570}'
            | '\u{2574}'..='\u{257f}'
    ) {
        return None;
    }

    use Stroke::{Heavy, Light};

    // Unicode box-drawing characters mix weights independently on each arm.
    let left = match ch {
        '\u{2500}' | '\u{2510}' | '\u{2512}' | '\u{2518}' | '\u{251a}' | '\u{2524}'
        | '\u{2526}' | '\u{2527}' | '\u{2528}' | '\u{252c}' | '\u{252e}' | '\u{2530}'
        | '\u{2532}' | '\u{2534}' | '\u{2536}' | '\u{2538}' | '\u{253a}' | '\u{253c}'
        | '\u{253e}' | '\u{2540}' | '\u{2541}' | '\u{2542}' | '\u{2544}' | '\u{2546}'
        | '\u{254a}' | '\u{256e}' | '\u{256f}' | '\u{2574}' | '\u{257c}' => Some(Light),
        '\u{2501}' | '\u{2511}' | '\u{2513}' | '\u{2519}' | '\u{251b}' | '\u{2525}'
        | '\u{2529}' | '\u{252a}' | '\u{252b}' | '\u{252d}' | '\u{252f}' | '\u{2531}'
        | '\u{2533}' | '\u{2535}' | '\u{2537}' | '\u{2539}' | '\u{253b}' | '\u{253d}'
        | '\u{253f}' | '\u{2543}' | '\u{2545}' | '\u{2547}' | '\u{2548}' | '\u{2549}'
        | '\u{254b}' | '\u{2578}' | '\u{257e}' => Some(Heavy),
        _ => None,
    };
    let right = match ch {
        '\u{2500}' | '\u{250c}' | '\u{250e}' | '\u{2514}' | '\u{2516}' | '\u{251c}'
        | '\u{251e}' | '\u{251f}' | '\u{2520}' | '\u{252c}' | '\u{252d}' | '\u{2530}'
        | '\u{2531}' | '\u{2534}' | '\u{2535}' | '\u{2538}' | '\u{2539}' | '\u{253c}'
        | '\u{253d}' | '\u{2540}' | '\u{2541}' | '\u{2542}' | '\u{2543}' | '\u{2545}'
        | '\u{2549}' | '\u{256d}' | '\u{2570}' | '\u{2576}' | '\u{257e}' => Some(Light),
        '\u{2501}' | '\u{250d}' | '\u{250f}' | '\u{2515}' | '\u{2517}' | '\u{251d}'
        | '\u{2521}' | '\u{2522}' | '\u{2523}' | '\u{252e}' | '\u{252f}' | '\u{2532}'
        | '\u{2533}' | '\u{2536}' | '\u{2537}' | '\u{253a}' | '\u{253b}' | '\u{253e}'
        | '\u{253f}' | '\u{2544}' | '\u{2546}' | '\u{2547}' | '\u{2548}' | '\u{254a}'
        | '\u{254b}' | '\u{257a}' | '\u{257c}' => Some(Heavy),
        _ => None,
    };
    let up = match ch {
        '\u{2502}' | '\u{2514}' | '\u{2515}' | '\u{2518}' | '\u{2519}' | '\u{251c}'
        | '\u{251d}' | '\u{251f}' | '\u{2522}' | '\u{2524}' | '\u{2525}' | '\u{2527}'
        | '\u{252a}' | '\u{2534}' | '\u{2535}' | '\u{2536}' | '\u{2537}' | '\u{253c}'
        | '\u{253d}' | '\u{253e}' | '\u{253f}' | '\u{2541}' | '\u{2545}' | '\u{2546}'
        | '\u{2548}' | '\u{256f}' | '\u{2570}' | '\u{2575}' | '\u{257d}' => Some(Light),
        '\u{2503}' | '\u{2516}' | '\u{2517}' | '\u{251a}' | '\u{251b}' | '\u{251e}'
        | '\u{2520}' | '\u{2521}' | '\u{2523}' | '\u{2526}' | '\u{2528}' | '\u{2529}'
        | '\u{252b}' | '\u{2538}' | '\u{2539}' | '\u{253a}' | '\u{253b}' | '\u{2540}'
        | '\u{2542}' | '\u{2543}' | '\u{2544}' | '\u{2547}' | '\u{2549}' | '\u{254a}'
        | '\u{254b}' | '\u{2579}' | '\u{257f}' => Some(Heavy),
        _ => None,
    };
    let down = match ch {
        '\u{2502}' | '\u{250c}' | '\u{250d}' | '\u{2510}' | '\u{2511}' | '\u{251c}'
        | '\u{251d}' | '\u{251e}' | '\u{2521}' | '\u{2524}' | '\u{2525}' | '\u{2526}'
        | '\u{2529}' | '\u{252c}' | '\u{252d}' | '\u{252e}' | '\u{252f}' | '\u{253c}'
        | '\u{253d}' | '\u{253e}' | '\u{253f}' | '\u{2540}' | '\u{2543}' | '\u{2544}'
        | '\u{2547}' | '\u{256d}' | '\u{256e}' | '\u{2577}' | '\u{257f}' => Some(Light),
        '\u{2503}' | '\u{250e}' | '\u{250f}' | '\u{2512}' | '\u{2513}' | '\u{251f}'
        | '\u{2520}' | '\u{2522}' | '\u{2523}' | '\u{2527}' | '\u{2528}' | '\u{252a}'
        | '\u{252b}' | '\u{2530}' | '\u{2531}' | '\u{2532}' | '\u{2533}' | '\u{2541}'
        | '\u{2542}' | '\u{2545}' | '\u{2546}' | '\u{2548}' | '\u{2549}' | '\u{254a}'
        | '\u{254b}' | '\u{257b}' | '\u{257d}' => Some(Heavy),
        _ => None,
    };

    Some(BoxDrawingGlyph {
        left,
        right,
        up,
        down,
        rounded: matches!(ch, '\u{256d}'..='\u{2570}'),
    })
}

fn centered_stroke_bounds(dimension: i32, stroke: Option<Stroke>, light_stroke: i32) -> (i32, i32) {
    let thickness = match stroke {
        Some(Stroke::Light) => light_stroke,
        Some(Stroke::Heavy) => light_stroke * 2,
        None => 0,
    }
    .min(dimension);
    let start = (dimension - thickness) / 2;
    (start, start + thickness)
}

fn for_each_straight_rect(
    glyph: BoxDrawingGlyph,
    cell: Size<i32>,
    light_stroke: i32,
    mut emit: impl FnMut(Bounds<i32>),
) {
    let left_bounds = centered_stroke_bounds(cell.height, glyph.left, light_stroke);
    let right_bounds = centered_stroke_bounds(cell.height, glyph.right, light_stroke);
    let up_bounds = centered_stroke_bounds(cell.width, glyph.up, light_stroke);
    let down_bounds = centered_stroke_bounds(cell.width, glyph.down, light_stroke);
    let has_horizontal = glyph.left.is_some() || glyph.right.is_some();
    let has_vertical = glyph.up.is_some() || glyph.down.is_some();

    let junction_left = up_bounds.0.min(down_bounds.0);
    let junction_right = up_bounds.1.max(down_bounds.1);
    let junction_top = left_bounds.0.min(right_bounds.0);
    let junction_bottom = left_bounds.1.max(right_bounds.1);
    let mut emit_rect = |left, top, right, bottom| {
        if left < right && top < bottom {
            emit(Bounds::from_corners(point(left, top), point(right, bottom)));
        }
    };

    if glyph.left.is_some() {
        emit_rect(
            0,
            left_bounds.0,
            if has_vertical {
                junction_left
            } else {
                cell.width / 2
            },
            left_bounds.1,
        );
    }
    if glyph.right.is_some() {
        emit_rect(
            if has_vertical {
                junction_right
            } else {
                cell.width / 2
            },
            right_bounds.0,
            cell.width,
            right_bounds.1,
        );
    }
    if glyph.up.is_some() {
        emit_rect(
            up_bounds.0,
            0,
            up_bounds.1,
            if has_horizontal {
                junction_top
            } else {
                cell.height / 2
            },
        );
    }
    if glyph.down.is_some() {
        emit_rect(
            down_bounds.0,
            if has_horizontal {
                junction_bottom
            } else {
                cell.height / 2
            },
            down_bounds.1,
            cell.height,
        );
    }
    if has_horizontal && has_vertical {
        emit_rect(junction_left, junction_top, junction_right, junction_bottom);
    }
}

#[derive(Debug)]
struct RoundedGeometry {
    arc_start: Point<f32>,
    arc_end: Point<f32>,
    radius: f32,
    sweep: bool,
}

fn rounded_geometry(
    glyph: BoxDrawingGlyph,
    cell: Size<i32>,
    light_stroke: i32,
    mut emit: impl FnMut(Bounds<i32>),
) -> Option<RoundedGeometry> {
    if !glyph.rounded {
        return None;
    }

    let left = glyph.left.is_some();
    let right = glyph.right.is_some();
    let up = glyph.up.is_some();
    let down = glyph.down.is_some();
    if left == right || up == down {
        return None;
    }

    let horizontal_bounds = centered_stroke_bounds(cell.height, Some(Stroke::Light), light_stroke);
    let vertical_bounds = centered_stroke_bounds(cell.width, Some(Stroke::Light), light_stroke);
    let center = point(
        (vertical_bounds.0 + vertical_bounds.1) as f32 / 2.,
        (horizontal_bounds.0 + horizontal_bounds.1) as f32 / 2.,
    );
    let radius = center
        .x
        .min(cell.width as f32 - center.x)
        .min(center.y)
        .min(cell.height as f32 - center.y);

    let arc_start = point(
        if left {
            center.x - radius
        } else {
            center.x + radius
        },
        center.y,
    );
    let arc_end = point(
        center.x,
        if up {
            center.y - radius
        } else {
            center.y + radius
        },
    );
    let arc_start_column = arc_start.x.round() as i32;
    let arc_end_line = arc_end.y.round() as i32;
    let mut emit_rect = |left, top, right, bottom| {
        if left < right && top < bottom {
            emit(Bounds::from_corners(point(left, top), point(right, bottom)));
        }
    };
    if left {
        emit_rect(
            0,
            horizontal_bounds.0,
            arc_start_column,
            horizontal_bounds.1,
        );
    } else {
        emit_rect(
            arc_start_column,
            horizontal_bounds.0,
            cell.width,
            horizontal_bounds.1,
        );
    }
    if up {
        emit_rect(vertical_bounds.0, 0, vertical_bounds.1, arc_end_line);
    } else {
        emit_rect(
            vertical_bounds.0,
            arc_end_line,
            vertical_bounds.1,
            cell.height,
        );
    }

    Some(RoundedGeometry {
        arc_start,
        arc_end,
        radius,
        sweep: left == down,
    })
}

fn paint_rect(
    rect: Bounds<i32>,
    cell_left: i32,
    cell_top: i32,
    scale_factor: f32,
    color: Hsla,
    window: &mut Window,
) {
    let bounds = Bounds::from_corners(
        point(
            px((cell_left + rect.left()) as f32 / scale_factor),
            px((cell_top + rect.top()) as f32 / scale_factor),
        ),
        point(
            px((cell_left + rect.right()) as f32 / scale_factor),
            px((cell_top + rect.bottom()) as f32 / scale_factor),
        ),
    );
    window.paint_quad(fill(bounds, color));
}

#[derive(Clone, Debug)]
pub struct BoxDrawingLayoutGlyph {
    point: LayoutPoint,
    glyph: BoxDrawingGlyph,
    color: Hsla,
    underline: Option<UnderlineStyle>,
    strikethrough: Option<StrikethroughStyle>,
}

impl BoxDrawingLayoutGlyph {
    pub(crate) fn new(point: LayoutPoint, ch: char, style: &TextRun) -> Option<Self> {
        let glyph = glyph_for(ch)?;
        let underline = style.underline.map(|mut underline| {
            underline.color = Some(underline.color.unwrap_or(style.color));
            underline
        });
        let strikethrough = style.strikethrough.map(|mut strikethrough| {
            strikethrough.color = Some(strikethrough.color.unwrap_or(style.color));
            strikethrough
        });
        Some(Self {
            point,
            glyph,
            color: style.color,
            underline,
            strikethrough,
        })
    }

    pub fn line(&self) -> i32 {
        self.point.line()
    }

    fn has_decorations(&self) -> bool {
        self.underline.is_some() || self.strikethrough.is_some()
    }

    fn paint(
        &self,
        origin: Point<Pixels>,
        dimensions: &TerminalBounds,
        decoration_metrics: Option<DecorationMetrics>,
        window: &mut Window,
    ) {
        let scale_factor = window.scale_factor();
        let to_device_pixel =
            |value| (window.pixel_snap(value).as_f32() * scale_factor).round() as i32;
        let cell_left =
            to_device_pixel(origin.x + self.point.column() as f32 * dimensions.cell_width);
        let cell_right =
            to_device_pixel(origin.x + (self.point.column() + 1) as f32 * dimensions.cell_width);
        let cell_top =
            to_device_pixel(origin.y + self.point.line() as f32 * dimensions.line_height);
        let cell_bottom =
            to_device_pixel(origin.y + (self.point.line() + 1) as f32 * dimensions.line_height);
        let cell = Size {
            width: cell_right - cell_left,
            height: cell_bottom - cell_top,
        };
        if cell.width <= 0 || cell.height <= 0 {
            return;
        }

        let nominal_cell_width = (dimensions.cell_width.as_f32() * scale_factor).round() as i32;
        let light_stroke =
            ((nominal_cell_width.max(1) as f32 / LIGHT_STROKE_DIVISOR).round() as i32).max(1);
        if let Some(geometry) = rounded_geometry(self.glyph, cell, light_stroke, |stub| {
            paint_rect(stub, cell_left, cell_top, scale_factor, self.color, window);
        }) {
            let to_pixels = |position: Point<f32>| {
                point(
                    px((cell_left as f32 + position.x) / scale_factor),
                    px((cell_top as f32 + position.y) / scale_factor),
                )
            };
            let mut builder = PathBuilder::stroke(px(light_stroke as f32 / scale_factor));
            builder.move_to(to_pixels(geometry.arc_start));
            builder.arc_to(
                point(
                    px(geometry.radius / scale_factor),
                    px(geometry.radius / scale_factor),
                ),
                px(0.),
                false,
                geometry.sweep,
                to_pixels(geometry.arc_end),
            );
            if let Some(path) = builder.build().log_err() {
                window.paint_path(path, self.color);
            }
        } else {
            for_each_straight_rect(self.glyph, cell, light_stroke, |rect| {
                paint_rect(rect, cell_left, cell_top, scale_factor, self.color, window);
            });
        }

        let Some(decoration_metrics) = decoration_metrics else {
            return;
        };
        let font_ascent = decoration_metrics.font_ascent;
        let font_descent = decoration_metrics.font_descent;
        let cell_origin_y = origin.y + self.point.line() as f32 * dimensions.line_height;
        // Match GPUI's shaped-line placement so decorations align with adjacent text cells.
        let padding_top = (dimensions.line_height - font_ascent - font_descent) / 2.;
        let baseline_offset = padding_top + font_ascent;
        let decoration_origin_x = px(cell_left as f32 / scale_factor);
        let decoration_width = px((cell_right - cell_left) as f32 / scale_factor);
        if let Some(underline) = &self.underline {
            window.paint_underline(
                point(
                    decoration_origin_x,
                    cell_origin_y + baseline_offset + font_descent * 0.618,
                ),
                decoration_width,
                underline,
            );
        }
        if let Some(strikethrough) = &self.strikethrough {
            window.paint_strikethrough(
                point(
                    decoration_origin_x,
                    cell_origin_y + (font_ascent * 0.5 + baseline_offset) * 0.5,
                ),
                decoration_width,
                strikethrough,
            );
        }
    }
}

#[derive(Clone, Copy)]
struct DecorationMetrics {
    font_ascent: Pixels,
    font_descent: Pixels,
}

pub struct BoxDrawingPainter<'a> {
    origin: Point<Pixels>,
    dimensions: TerminalBounds,
    text_style: &'a TextStyle,
    decoration_metrics: Option<DecorationMetrics>,
}

impl<'a> BoxDrawingPainter<'a> {
    pub fn new(
        origin: Point<Pixels>,
        dimensions: TerminalBounds,
        text_style: &'a TextStyle,
    ) -> Self {
        Self {
            origin,
            dimensions,
            text_style,
            decoration_metrics: None,
        }
    }

    pub fn paint(&mut self, glyph: &BoxDrawingLayoutGlyph, window: &mut Window) {
        let decoration_metrics = if glyph.has_decorations() {
            if let Some(metrics) = self.decoration_metrics {
                Some(metrics)
            } else {
                let text_system = window.text_system();
                let font_size = self.text_style.font_size.to_pixels(window.rem_size());
                let font_id = text_system.resolve_font(&self.text_style.font());
                let metrics = DecorationMetrics {
                    font_ascent: text_system.ascent(font_id, font_size),
                    font_descent: text_system.descent(font_id, font_size),
                };
                self.decoration_metrics = Some(metrics);
                Some(metrics)
            }
        } else {
            None
        };

        glyph.paint(self.origin, &self.dimensions, decoration_metrics, window);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn straight_rects(
        glyph: BoxDrawingGlyph,
        cell: Size<i32>,
        light_stroke: i32,
    ) -> Vec<Bounds<i32>> {
        let mut rects = Vec::new();
        for_each_straight_rect(glyph, cell, light_stroke, |rect| rects.push(rect));
        rects
    }

    fn rasterize(rects: &[Bounds<i32>], cell: Size<i32>) -> Vec<Vec<bool>> {
        let mut pixels = vec![vec![false; cell.width as usize]; cell.height as usize];
        for rect in rects {
            assert!(rect.left() >= 0);
            assert!(rect.top() >= 0);
            assert!(rect.right() <= cell.width);
            assert!(rect.bottom() <= cell.height);
            for line in rect.top()..rect.bottom() {
                for column in rect.left()..rect.right() {
                    assert!(!pixels[line as usize][column as usize]);
                    pixels[line as usize][column as usize] = true;
                }
            }
        }
        pixels
    }

    #[test]
    fn maps_straight_box_drawing_arms() {
        use Stroke::{Heavy, Light};

        assert_eq!(
            glyph_for('─'),
            Some(BoxDrawingGlyph {
                left: Some(Light),
                right: Some(Light),
                ..Default::default()
            })
        );
        assert_eq!(
            glyph_for('┃'),
            Some(BoxDrawingGlyph {
                up: Some(Heavy),
                down: Some(Heavy),
                ..Default::default()
            })
        );
        assert_eq!(
            glyph_for('┞'),
            Some(BoxDrawingGlyph {
                right: Some(Light),
                up: Some(Heavy),
                down: Some(Light),
                ..Default::default()
            })
        );
        assert_eq!(
            glyph_for('╿'),
            Some(BoxDrawingGlyph {
                up: Some(Heavy),
                down: Some(Light),
                ..Default::default()
            })
        );
        assert_eq!(
            glyph_for('╭'),
            Some(BoxDrawingGlyph {
                right: Some(Light),
                down: Some(Light),
                rounded: true,
                ..Default::default()
            })
        );
        for unsupported in ['┄', '═', '╱'] {
            assert_eq!(glyph_for(unsupported), None);
        }
    }

    #[test]
    fn layout_glyph_preserves_text_decorations() {
        let color = gpui::red();
        let underline = UnderlineStyle {
            color: None,
            thickness: px(2.),
            wavy: true,
        };
        let strikethrough = StrikethroughStyle {
            color: Some(gpui::blue()),
            thickness: px(3.),
        };
        let style = TextRun {
            len: '─'.len_utf8(),
            color,
            underline: Some(underline),
            strikethrough: Some(strikethrough),
            ..Default::default()
        };

        let Some(layout_glyph) = BoxDrawingLayoutGlyph::new(LayoutPoint::default(), '─', &style)
        else {
            panic!("light horizontal glyph should be custom-painted");
        };

        assert_eq!(layout_glyph.color, color);
        assert_eq!(
            layout_glyph.underline,
            Some(UnderlineStyle {
                color: Some(color),
                ..underline
            })
        );
        assert_eq!(layout_glyph.strikethrough, Some(strikethrough));
    }

    #[test]
    fn straight_glyph_rects_are_connected_disjoint_and_contained() {
        for cell in [
            Size {
                width: 9,
                height: 23,
            },
            Size {
                width: 10,
                height: 24,
            },
            Size {
                width: 11,
                height: 25,
            },
            Size {
                width: 2,
                height: 3,
            },
        ] {
            for light_stroke in 1..=3 {
                for codepoint in (0x2500..=0x2503)
                    .chain(0x250c..=0x254b)
                    .chain(0x2574..=0x257f)
                {
                    let ch = char::from_u32(codepoint).expect("valid box drawing codepoint");
                    let glyph = glyph_for(ch).unwrap_or_else(|| {
                        panic!("U+{codepoint:04X} {ch} should be custom-painted")
                    });
                    let pixels = rasterize(&straight_rects(glyph, cell, light_stroke), cell);
                    let case = format!(
                        "{ch} in a {}x{} cell with light stroke {light_stroke}",
                        cell.width, cell.height
                    );

                    if glyph.left.is_some() {
                        assert!(pixels.iter().any(|line| line[0]), "left arm for {case}");
                    }
                    if glyph.right.is_some() {
                        assert!(
                            pixels.iter().any(|line| line[cell.width as usize - 1]),
                            "right arm for {case}"
                        );
                    }
                    if glyph.up.is_some() {
                        assert!(pixels[0].iter().any(|pixel| *pixel), "up arm for {case}");
                    }
                    if glyph.down.is_some() {
                        assert!(
                            pixels[cell.height as usize - 1].iter().any(|pixel| *pixel),
                            "down arm for {case}"
                        );
                    }

                    let start = pixels.iter().enumerate().find_map(|(line, columns)| {
                        columns
                            .iter()
                            .position(|pixel| *pixel)
                            .map(|column| (line, column))
                    });
                    let Some(start) = start else {
                        panic!("{case} should paint at least one pixel");
                    };
                    let mut visited = vec![vec![false; cell.width as usize]; cell.height as usize];
                    let mut pending = vec![start];
                    while let Some((line, column)) = pending.pop() {
                        if visited[line][column] || !pixels[line][column] {
                            continue;
                        }
                        visited[line][column] = true;
                        for (line_offset, column_offset) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            let next_line = line as i32 + line_offset;
                            let next_column = column as i32 + column_offset;
                            if (0..cell.height).contains(&next_line)
                                && (0..cell.width).contains(&next_column)
                            {
                                pending.push((next_line as usize, next_column as usize));
                            }
                        }
                    }
                    assert!(
                        pixels.iter().enumerate().all(|(line, columns)| columns
                            .iter()
                            .enumerate()
                            .all(|(column, pixel)| !pixel || visited[line][column])),
                        "{case} should be one connected shape"
                    );
                }
            }
        }
    }

    #[test]
    fn straight_lines_use_one_stroke_thickness_for_both_axes() {
        let cell = Size {
            width: 12,
            height: 24,
        };
        let horizontal = rasterize(
            &straight_rects(glyph_for('─').expect("light horizontal glyph"), cell, 2),
            cell,
        );
        let vertical = rasterize(
            &straight_rects(glyph_for('│').expect("light vertical glyph"), cell, 2),
            cell,
        );

        assert_eq!(
            horizontal
                .iter()
                .filter(|line| line.iter().any(|pixel| *pixel))
                .count(),
            2
        );
        assert_eq!(
            (0..cell.width as usize)
                .filter(|column| vertical.iter().any(|line| line[*column]))
                .count(),
            2
        );
    }

    #[test]
    fn rounded_corners_reach_cell_edges_and_straight_stubs() {
        let cell = Size {
            width: 10,
            height: 24,
        };
        let cases = [
            ('╭', point(9., 11.5), point(4.5, 16.), false),
            ('╮', point(0., 11.5), point(4.5, 16.), true),
            ('╯', point(0., 11.5), point(4.5, 7.), false),
            ('╰', point(9., 11.5), point(4.5, 7.), true),
        ];

        for (ch, arc_start, arc_end, sweep) in cases {
            let glyph = glyph_for(ch).expect("rounded box-drawing glyph");
            let mut stubs = Vec::new();
            let geometry = rounded_geometry(glyph, cell, 1, |stub| stubs.push(stub))
                .expect("rounded geometry");
            assert_eq!(geometry.arc_start, arc_start, "arc start for {ch}");
            assert_eq!(geometry.arc_end, arc_end, "arc end for {ch}");
            assert_eq!(geometry.radius, 4.5, "radius for {ch}");
            assert_eq!(geometry.sweep, sweep, "arc sweep for {ch}");

            for stub in &stubs {
                assert!(stub.left() >= 0, "stub left for {ch}");
                assert!(stub.top() >= 0, "stub top for {ch}");
                assert!(stub.right() <= cell.width, "stub right for {ch}");
                assert!(stub.bottom() <= cell.height, "stub bottom for {ch}");
            }
            if glyph.left.is_some() {
                assert!(
                    geometry.arc_start.x == 0. || stubs.iter().any(|stub| stub.left() == 0),
                    "left edge for {ch}"
                );
            }
            if glyph.right.is_some() {
                assert!(
                    geometry.arc_start.x == cell.width as f32
                        || stubs.iter().any(|stub| stub.right() == cell.width),
                    "right edge for {ch}"
                );
            }
            if glyph.up.is_some() {
                assert!(
                    geometry.arc_end.y == 0. || stubs.iter().any(|stub| stub.top() == 0),
                    "top edge for {ch}"
                );
            }
            if glyph.down.is_some() {
                assert!(
                    geometry.arc_end.y == cell.height as f32
                        || stubs.iter().any(|stub| stub.bottom() == cell.height),
                    "bottom edge for {ch}"
                );
            }
        }
    }
}

use gpui::{Bounds, Hsla, PathBuilder, Pixels, Point, Size, TextStyle, Window, fill, point, px};
use terminal::TerminalBounds;
use util::ResultExt;

use crate::terminal_element::LayoutPoint;

const LIGHT_STROKE_DIVISOR: f32 = 8.;

fn box_drawing_stroke_width(underline_thickness: Pixels, cell_width: Pixels) -> Pixels {
    if underline_thickness.as_f32().is_finite() && underline_thickness > Pixels::ZERO {
        underline_thickness
    } else {
        cell_width / LIGHT_STROKE_DIVISOR
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stroke {
    Light,
    Heavy,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct LineGlyph {
    left: Option<Stroke>,
    right: Option<Stroke>,
    up: Option<Stroke>,
    down: Option<Stroke>,
    rounded: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DashedGlyph {
    axis: Axis,
    gaps: i32,
    stroke: Stroke,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DiagonalGlyph {
    rising: bool,
    falling: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BoxDrawingGlyph {
    Line(LineGlyph),
    Dashed(DashedGlyph),
    Double(char),
    Diagonal(DiagonalGlyph),
}

fn glyph_for(ch: char) -> Option<BoxDrawingGlyph> {
    use Axis::{Horizontal, Vertical};
    use Stroke::{Heavy, Light};

    let dashed = match ch {
        '\u{2504}' => Some((Horizontal, 2, Light)),
        '\u{2505}' => Some((Horizontal, 2, Heavy)),
        '\u{2506}' => Some((Vertical, 2, Light)),
        '\u{2507}' => Some((Vertical, 2, Heavy)),
        '\u{2508}' => Some((Horizontal, 3, Light)),
        '\u{2509}' => Some((Horizontal, 3, Heavy)),
        '\u{250a}' => Some((Vertical, 3, Light)),
        '\u{250b}' => Some((Vertical, 3, Heavy)),
        '\u{254c}' => Some((Horizontal, 1, Light)),
        '\u{254d}' => Some((Horizontal, 1, Heavy)),
        '\u{254e}' => Some((Vertical, 1, Light)),
        '\u{254f}' => Some((Vertical, 1, Heavy)),
        _ => None,
    };
    if let Some((axis, gaps, stroke)) = dashed {
        return Some(BoxDrawingGlyph::Dashed(DashedGlyph { axis, gaps, stroke }));
    }

    let glyph = match ch {
        '\u{2550}'..='\u{256c}' => BoxDrawingGlyph::Double(ch),
        '\u{2571}' => BoxDrawingGlyph::Diagonal(DiagonalGlyph {
            rising: true,
            falling: false,
        }),
        '\u{2572}' => BoxDrawingGlyph::Diagonal(DiagonalGlyph {
            rising: false,
            falling: true,
        }),
        '\u{2573}' => BoxDrawingGlyph::Diagonal(DiagonalGlyph {
            rising: true,
            falling: true,
        }),
        _ => BoxDrawingGlyph::Line(line_glyph_for(ch)?),
    };
    Some(glyph)
}

fn line_glyph_for(ch: char) -> Option<LineGlyph> {
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

    Some(LineGlyph {
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
    glyph: LineGlyph,
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

fn for_each_dashed_rect(
    glyph: DashedGlyph,
    cell: Size<i32>,
    light_stroke: i32,
    mut emit: impl FnMut(Bounds<i32>),
) {
    let (dimension, cross_dimension) = match glyph.axis {
        Axis::Horizontal => (cell.width, cell.height),
        Axis::Vertical => (cell.height, cell.width),
    };
    let gap_length = (dimension / 8).max(1);
    let dash_length = (dimension.saturating_sub(gap_length * glyph.gaps) / (glyph.gaps + 1)).max(1);
    let cross_bounds = centered_stroke_bounds(cross_dimension, Some(glyph.stroke), light_stroke);

    for segment in 0..=glyph.gaps {
        let start = (segment * (dash_length + gap_length)).min(dimension);
        let end = (start + dash_length).min(dimension);
        if start >= end || cross_bounds.0 >= cross_bounds.1 {
            continue;
        }
        let bounds = match glyph.axis {
            Axis::Horizontal => {
                Bounds::from_corners(point(start, cross_bounds.0), point(end, cross_bounds.1))
            }
            Axis::Vertical => {
                Bounds::from_corners(point(cross_bounds.0, start), point(cross_bounds.1, end))
            }
        };
        emit(bounds);
    }
}

fn stroke_bounds_at(dimension: i32, center: f32, thickness: i32) -> (i32, i32) {
    let thickness = thickness.min(dimension);
    let start = ((center - thickness as f32 / 2.) as i32).max(0);
    let end = ((center + thickness as f32 / 2.) as i32).min(dimension);
    (start, end)
}

fn horizontal_rect(
    cell: Size<i32>,
    start: f32,
    end: f32,
    center: f32,
    thickness: i32,
) -> Option<Bounds<i32>> {
    let (top, bottom) = stroke_bounds_at(cell.height, center, thickness);
    let left = (start as i32).clamp(0, cell.width);
    let right = (end as i32).clamp(0, cell.width);
    (left < right && top < bottom)
        .then(|| Bounds::from_corners(point(left, top), point(right, bottom)))
}

fn vertical_rect(
    cell: Size<i32>,
    start: f32,
    end: f32,
    center: f32,
    thickness: i32,
) -> Option<Bounds<i32>> {
    let (left, right) = stroke_bounds_at(cell.width, center, thickness);
    let top = (start as i32).clamp(0, cell.height);
    let bottom = (end as i32).clamp(0, cell.height);
    (left < right && top < bottom)
        .then(|| Bounds::from_corners(point(left, top), point(right, bottom)))
}

// Avoid blending translucent terminal colors more than once where double-line rails overlap.
fn emit_uncovered_rect(
    rect: Bounds<i32>,
    covered: &[Option<Bounds<i32>>],
    emit: &mut impl FnMut(Bounds<i32>),
) {
    let Some((covered_rect, remaining)) = covered.split_first() else {
        emit(rect);
        return;
    };
    let Some(covered_rect) = covered_rect else {
        emit_uncovered_rect(rect, remaining, emit);
        return;
    };
    let intersection = rect.intersect(covered_rect);
    if intersection.is_empty() {
        emit_uncovered_rect(rect, remaining, emit);
        return;
    }

    let pieces = [
        (rect.left(), rect.top(), rect.right(), intersection.top()),
        (
            rect.left(),
            intersection.bottom(),
            rect.right(),
            rect.bottom(),
        ),
        (
            rect.left(),
            intersection.top(),
            intersection.left(),
            intersection.bottom(),
        ),
        (
            intersection.right(),
            intersection.top(),
            rect.right(),
            intersection.bottom(),
        ),
    ];
    for (left, top, right, bottom) in pieces {
        if left < right && top < bottom {
            emit_uncovered_rect(
                Bounds::from_corners(point(left, top), point(right, bottom)),
                remaining,
                emit,
            );
        }
    }
}

fn for_each_double_rect(
    character: char,
    cell: Size<i32>,
    light_stroke: i32,
    mut emit: impl FnMut(Bounds<i32>),
) {
    let center_x = cell.width as f32 / 2.;
    let center_y = cell.height as f32 / 2.;
    let vertical_lines = if matches!(
        character,
        '\u{2552}'
            | '\u{2555}'
            | '\u{2558}'
            | '\u{255b}'
            | '\u{255e}'
            | '\u{2561}'
            | '\u{2564}'
            | '\u{2567}'
            | '\u{256a}'
    ) {
        (center_x, center_x)
    } else {
        let bounds = centered_stroke_bounds(cell.width, Some(Stroke::Light), light_stroke);
        (
            (bounds.0 - 1).max(0) as f32,
            (bounds.1 + 1).min(cell.width) as f32,
        )
    };
    let horizontal_lines = if matches!(
        character,
        '\u{2553}'
            | '\u{2556}'
            | '\u{2559}'
            | '\u{255c}'
            | '\u{255f}'
            | '\u{2562}'
            | '\u{2565}'
            | '\u{2568}'
            | '\u{256b}'
    ) {
        (center_y, center_y)
    } else {
        let bounds = centered_stroke_bounds(cell.height, Some(Stroke::Light), light_stroke);
        (
            (bounds.0 - 1).max(0) as f32,
            (bounds.1 + 1).min(cell.height) as f32,
        )
    };

    let vertical_left_bounds = stroke_bounds_at(cell.width, vertical_lines.0, light_stroke);
    let vertical_right_bounds = stroke_bounds_at(cell.width, vertical_lines.1, light_stroke);
    let horizontal_top_bounds = stroke_bounds_at(cell.height, horizontal_lines.0, light_stroke);
    let horizontal_bottom_bounds = stroke_bounds_at(cell.height, horizontal_lines.1, light_stroke);

    let (top_left_end, bottom_left_end) = match character {
        '\u{2550}' | '\u{256b}' => (center_x, center_x),
        '\u{2555}'..='\u{2557}' => (
            vertical_right_bounds.1 as f32,
            vertical_left_bounds.1 as f32,
        ),
        '\u{255b}'..='\u{255d}' => (
            vertical_left_bounds.1 as f32,
            vertical_right_bounds.1 as f32,
        ),
        '\u{2561}'..='\u{2563}' | '\u{256a}' | '\u{256c}' => {
            (vertical_left_bounds.1 as f32, vertical_left_bounds.1 as f32)
        }
        '\u{2564}'..='\u{2568}' => (center_x, vertical_left_bounds.1 as f32),
        '\u{2569}' => (vertical_left_bounds.1 as f32, center_x),
        _ => (0., 0.),
    };
    let (top_right_start, bottom_right_start) = match character {
        '\u{2550}' | '\u{2565}' | '\u{256b}' => (center_x, center_x),
        '\u{2552}'..='\u{2554}' | '\u{2568}' => (
            vertical_left_bounds.0 as f32,
            vertical_right_bounds.0 as f32,
        ),
        '\u{2558}'..='\u{255a}' => (
            vertical_right_bounds.0 as f32,
            vertical_left_bounds.0 as f32,
        ),
        '\u{255e}'..='\u{2560}' | '\u{256a}' | '\u{256c}' => (
            vertical_right_bounds.0 as f32,
            vertical_right_bounds.0 as f32,
        ),
        '\u{2564}' | '\u{2566}' => (center_x, vertical_right_bounds.0 as f32),
        '\u{2567}' | '\u{2569}' => (vertical_right_bounds.0 as f32, center_x),
        _ => (cell.width as f32, cell.width as f32),
    };
    let (left_top_end, right_top_end) = match character {
        '\u{2551}' | '\u{256a}' => (center_y, center_y),
        '\u{2558}'..='\u{255c}' | '\u{2568}' => (
            horizontal_bottom_bounds.1 as f32,
            horizontal_top_bounds.1 as f32,
        ),
        '\u{255d}' => (
            horizontal_top_bounds.1 as f32,
            horizontal_bottom_bounds.1 as f32,
        ),
        '\u{255e}'..='\u{2560}' => (center_y, horizontal_top_bounds.1 as f32),
        '\u{2561}'..='\u{2563}' => (horizontal_top_bounds.1 as f32, center_y),
        '\u{2567}' | '\u{2569}' | '\u{256b}' | '\u{256c}' => (
            horizontal_top_bounds.1 as f32,
            horizontal_top_bounds.1 as f32,
        ),
        _ => (0., 0.),
    };
    let (left_bottom_start, right_bottom_start) = match character {
        '\u{2551}' | '\u{256a}' => (center_y, center_y),
        '\u{2552}'..='\u{2554}' => (
            horizontal_top_bounds.0 as f32,
            horizontal_bottom_bounds.0 as f32,
        ),
        '\u{2555}'..='\u{2557}' => (
            horizontal_bottom_bounds.0 as f32,
            horizontal_top_bounds.0 as f32,
        ),
        '\u{255e}'..='\u{2560}' => (center_y, horizontal_bottom_bounds.0 as f32),
        '\u{2561}'..='\u{2563}' => (horizontal_bottom_bounds.0 as f32, center_y),
        '\u{2564}'..='\u{2566}' | '\u{256b}' | '\u{256c}' => (
            horizontal_bottom_bounds.0 as f32,
            horizontal_bottom_bounds.0 as f32,
        ),
        _ => (cell.height as f32, cell.height as f32),
    };

    let rects = [
        horizontal_rect(cell, 0., top_left_end, horizontal_lines.0, light_stroke),
        horizontal_rect(cell, 0., bottom_left_end, horizontal_lines.1, light_stroke),
        horizontal_rect(
            cell,
            top_right_start,
            cell.width as f32,
            horizontal_lines.0,
            light_stroke,
        ),
        horizontal_rect(
            cell,
            bottom_right_start,
            cell.width as f32,
            horizontal_lines.1,
            light_stroke,
        ),
        vertical_rect(cell, 0., left_top_end, vertical_lines.0, light_stroke),
        vertical_rect(cell, 0., right_top_end, vertical_lines.1, light_stroke),
        vertical_rect(
            cell,
            left_bottom_start,
            cell.height as f32,
            vertical_lines.0,
            light_stroke,
        ),
        vertical_rect(
            cell,
            right_bottom_start,
            cell.height as f32,
            vertical_lines.1,
            light_stroke,
        ),
    ];
    for (index, rect) in rects.iter().enumerate() {
        if let Some(rect) = rect {
            emit_uncovered_rect(*rect, &rects[..index], &mut emit);
        }
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
    glyph: LineGlyph,
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

fn for_each_diagonal_segment(
    glyph: DiagonalGlyph,
    cell: Size<i32>,
    mut emit: impl FnMut(Point<i32>, Point<i32>),
) {
    if glyph.rising {
        emit(point(0, cell.height), point(cell.width, 0));
    }
    if glyph.falling {
        emit(point(0, 0), point(cell.width, cell.height));
    }
}

fn paint_diagonal(
    glyph: DiagonalGlyph,
    cell: Size<i32>,
    cell_left: i32,
    cell_top: i32,
    scale_factor: f32,
    light_stroke: i32,
    color: Hsla,
    window: &mut Window,
) {
    let to_pixels = |position: Point<i32>| {
        point(
            px((cell_left + position.x) as f32 / scale_factor),
            px((cell_top + position.y) as f32 / scale_factor),
        )
    };
    let mut builder = PathBuilder::stroke(px(light_stroke as f32 / scale_factor));
    for_each_diagonal_segment(glyph, cell, |start, end| {
        builder.move_to(to_pixels(start));
        builder.line_to(to_pixels(end));
    });
    if let Some(path) = builder.build().log_err() {
        window.paint_path(path, color);
    }
}

#[derive(Clone, Debug)]
pub struct BoxDrawingLayoutGlyph {
    point: LayoutPoint,
    glyph: BoxDrawingGlyph,
    color: Hsla,
}

impl BoxDrawingLayoutGlyph {
    pub(crate) fn new(point: LayoutPoint, ch: char, color: Hsla) -> Option<Self> {
        let glyph = glyph_for(ch)?;
        Some(Self {
            point,
            glyph,
            color,
        })
    }

    pub fn line(&self) -> i32 {
        self.point.line()
    }

    fn paint(
        &self,
        origin: Point<Pixels>,
        dimensions: &TerminalBounds,
        light_stroke: i32,
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

        match self.glyph {
            BoxDrawingGlyph::Line(glyph) => {
                if let Some(geometry) = rounded_geometry(glyph, cell, light_stroke, |stub| {
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
                    for_each_straight_rect(glyph, cell, light_stroke, |rect| {
                        paint_rect(rect, cell_left, cell_top, scale_factor, self.color, window);
                    });
                }
            }
            BoxDrawingGlyph::Dashed(glyph) => {
                for_each_dashed_rect(glyph, cell, light_stroke, |rect| {
                    paint_rect(rect, cell_left, cell_top, scale_factor, self.color, window);
                });
            }
            BoxDrawingGlyph::Double(character) => {
                for_each_double_rect(character, cell, light_stroke, |rect| {
                    paint_rect(rect, cell_left, cell_top, scale_factor, self.color, window);
                });
            }
            BoxDrawingGlyph::Diagonal(glyph) => paint_diagonal(
                glyph,
                cell,
                cell_left,
                cell_top,
                scale_factor,
                light_stroke,
                self.color,
                window,
            ),
        }
    }
}

pub struct BoxDrawingPainter<'a> {
    origin: Point<Pixels>,
    dimensions: TerminalBounds,
    text_style: &'a TextStyle,
    light_stroke: Option<i32>,
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
            light_stroke: None,
        }
    }

    pub fn paint(&mut self, glyph: &BoxDrawingLayoutGlyph, window: &mut Window) {
        let light_stroke = if let Some(light_stroke) = self.light_stroke {
            light_stroke
        } else {
            let text_system = window.text_system();
            let font_size = self.text_style.font_size.to_pixels(window.rem_size());
            let font_id = text_system.resolve_font(&self.text_style.font());
            let stroke_width = box_drawing_stroke_width(
                text_system.underline_thickness(font_id, font_size),
                self.dimensions.cell_width,
            );
            let light_stroke = (window.pixel_snap(stroke_width).as_f32() * window.scale_factor())
                .round()
                .max(1.) as i32;
            self.light_stroke = Some(light_stroke);
            light_stroke
        };
        glyph.paint(self.origin, &self.dimensions, light_stroke, window);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn straight_rects(glyph: LineGlyph, cell: Size<i32>, light_stroke: i32) -> Vec<Bounds<i32>> {
        let mut rects = Vec::new();
        for_each_straight_rect(glyph, cell, light_stroke, |rect| rects.push(rect));
        rects
    }

    fn line_glyph(character: char) -> LineGlyph {
        let Some(BoxDrawingGlyph::Line(glyph)) = glyph_for(character) else {
            panic!("{character} should be a line glyph");
        };
        glyph
    }

    fn dashed_rects(character: char, cell: Size<i32>, light_stroke: i32) -> Vec<Bounds<i32>> {
        let Some(BoxDrawingGlyph::Dashed(glyph)) = glyph_for(character) else {
            panic!("{character} should be a dashed glyph");
        };
        let mut rects = Vec::new();
        for_each_dashed_rect(glyph, cell, light_stroke, |rect| rects.push(rect));
        rects
    }

    fn double_rects(character: char, cell: Size<i32>, light_stroke: i32) -> Vec<Bounds<i32>> {
        let Some(BoxDrawingGlyph::Double(character)) = glyph_for(character) else {
            panic!("{character} should be a double-line glyph");
        };
        let mut rects = Vec::new();
        for_each_double_rect(character, cell, light_stroke, |rect| rects.push(rect));
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

    fn run_count(pixels: impl IntoIterator<Item = bool>) -> usize {
        let mut previous = false;
        let mut count = 0;
        for pixel in pixels {
            if pixel && !previous {
                count += 1;
            }
            previous = pixel;
        }
        count
    }

    #[test]
    fn box_drawing_stroke_uses_font_metric_with_cell_width_fallback() {
        assert_eq!(box_drawing_stroke_width(px(0.8), px(16.)), px(0.8));
        for invalid_metric in [px(0.), px(-1.), px(f32::NAN)] {
            assert_eq!(box_drawing_stroke_width(invalid_metric, px(16.)), px(2.));
        }
    }

    #[test]
    fn maps_straight_box_drawing_arms() {
        use Stroke::{Heavy, Light};

        assert_eq!(
            glyph_for('─'),
            Some(BoxDrawingGlyph::Line(LineGlyph {
                left: Some(Light),
                right: Some(Light),
                ..Default::default()
            }))
        );
        assert_eq!(
            glyph_for('┃'),
            Some(BoxDrawingGlyph::Line(LineGlyph {
                up: Some(Heavy),
                down: Some(Heavy),
                ..Default::default()
            }))
        );
        assert_eq!(
            glyph_for('┞'),
            Some(BoxDrawingGlyph::Line(LineGlyph {
                right: Some(Light),
                up: Some(Heavy),
                down: Some(Light),
                ..Default::default()
            }))
        );
        assert_eq!(
            glyph_for('╿'),
            Some(BoxDrawingGlyph::Line(LineGlyph {
                up: Some(Heavy),
                down: Some(Light),
                ..Default::default()
            }))
        );
        assert_eq!(
            glyph_for('╭'),
            Some(BoxDrawingGlyph::Line(LineGlyph {
                right: Some(Light),
                down: Some(Light),
                rounded: true,
                ..Default::default()
            }))
        );
    }

    #[test]
    fn covers_every_box_drawing_character() {
        let mut family_counts = [0; 4];
        for codepoint in 0x2500..=0x257f {
            let character = char::from_u32(codepoint).expect("valid box-drawing codepoint");
            let glyph = glyph_for(character).unwrap_or_else(|| {
                panic!("U+{codepoint:04X} {character} should be custom-painted")
            });
            family_counts[match glyph {
                BoxDrawingGlyph::Line(_) => 0,
                BoxDrawingGlyph::Dashed(_) => 1,
                BoxDrawingGlyph::Double(_) => 2,
                BoxDrawingGlyph::Diagonal(_) => 3,
            }] += 1;
        }
        assert_eq!(family_counts, [84, 12, 29, 3]);

        for character in ['\u{24ff}', '\u{2580}'] {
            assert_eq!(glyph_for(character), None);
        }
    }

    #[test]
    fn dashed_lines_match_alacritty_segmentation() {
        let cell = Size {
            width: 16,
            height: 24,
        };
        assert_eq!(
            dashed_rects('┄', cell, 1),
            [
                Bounds::from_corners(point(0, 11), point(4, 12)),
                Bounds::from_corners(point(6, 11), point(10, 12)),
                Bounds::from_corners(point(12, 11), point(16, 12)),
            ]
        );
        assert_eq!(
            dashed_rects('╏', cell, 1),
            [
                Bounds::from_corners(point(7, 0), point(9, 10)),
                Bounds::from_corners(point(7, 13), point(9, 23)),
            ]
        );

        for cell in [
            Size {
                width: 9,
                height: 23,
            },
            Size {
                width: 10,
                height: 24,
            },
        ] {
            for light_stroke in 1..=3 {
                for codepoint in (0x2504..=0x250b).chain(0x254c..=0x254f) {
                    let character = char::from_u32(codepoint).expect("valid dashed codepoint");
                    rasterize(&dashed_rects(character, cell, light_stroke), cell);
                }
            }
        }
    }

    #[test]
    fn diagonals_connect_opposite_cell_corners() {
        let cell = Size {
            width: 10,
            height: 24,
        };
        for (character, expected) in [
            ('╱', vec![(point(0, 24), point(10, 0))]),
            ('╲', vec![(point(0, 0), point(10, 24))]),
            (
                '╳',
                vec![(point(0, 24), point(10, 0)), (point(0, 0), point(10, 24))],
            ),
        ] {
            let Some(BoxDrawingGlyph::Diagonal(glyph)) = glyph_for(character) else {
                panic!("{character} should be a diagonal glyph");
            };
            let mut segments = Vec::new();
            for_each_diagonal_segment(glyph, cell, |start, end| segments.push((start, end)));
            assert_eq!(segments, expected, "segments for {character}");
        }
    }

    #[test]
    fn double_lines_have_expected_edge_rails() {
        let cell = Size {
            width: 12,
            height: 24,
        };
        for (character, expected_runs) in [
            ('═', [2, 2, 0, 0]),
            ('║', [0, 0, 2, 2]),
            ('╒', [0, 2, 0, 1]),
            ('╬', [2, 2, 2, 2]),
        ] {
            let pixels = rasterize(&double_rects(character, cell, 1), cell);
            let actual_runs = [
                run_count(pixels.iter().map(|line| line[0])),
                run_count(pixels.iter().map(|line| line[cell.width as usize - 1])),
                run_count(pixels[0].iter().copied()),
                run_count(pixels[cell.height as usize - 1].iter().copied()),
            ];
            assert_eq!(actual_runs, expected_runs, "edge rails for {character}");
        }

        for cell in [
            Size {
                width: 9,
                height: 23,
            },
            Size {
                width: 10,
                height: 24,
            },
        ] {
            for light_stroke in 1..=3 {
                for codepoint in 0x2550..=0x256c {
                    let character = char::from_u32(codepoint).expect("valid double-line codepoint");
                    let rects = double_rects(character, cell, light_stroke);
                    assert!(!rects.is_empty(), "geometry for {character}");
                    rasterize(&rects, cell);
                }
            }
        }
    }

    #[test]
    fn layout_glyph_preserves_color() {
        let color = gpui::red();
        let Some(layout_glyph) = BoxDrawingLayoutGlyph::new(LayoutPoint::default(), '─', color)
        else {
            panic!("light horizontal glyph should be custom-painted");
        };

        assert_eq!(layout_glyph.color, color);
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
                    let glyph = line_glyph(ch);
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
        let horizontal = rasterize(&straight_rects(line_glyph('─'), cell, 2), cell);
        let vertical = rasterize(&straight_rects(line_glyph('│'), cell, 2), cell);

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
            let glyph = line_glyph(ch);
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

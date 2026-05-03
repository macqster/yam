#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinePoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineStep {
    pub point: LinePoint,
    pub step: i32,
    pub steps: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineFamily {
    Axis,
    VeryShallow,
    Shallow,
    Medium,
    Steep,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LengthBucket {
    Short,
    Medium,
    Long,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionFamily {
    LeftRight,
    RightLeft,
    UpDown,
    DownUp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhaseRole {
    Entry,
    Transition,
    Core,
    Exit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellBand {
    Top,
    Middle,
    Bottom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineGrammarKey {
    pub family: LineFamily,
    pub length: LengthBucket,
    pub direction: DirectionFamily,
    pub phase: PhaseRole,
    pub band: CellBand,
}

impl LineGrammarKey {
    pub fn new(
        family: LineFamily,
        length: LengthBucket,
        direction: DirectionFamily,
        phase: PhaseRole,
        band: CellBand,
    ) -> Self {
        Self {
            family,
            length,
            direction,
            phase,
            band,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineGlyphRule {
    pub family: LineFamily,
    pub length: LengthBucket,
    pub direction: DirectionFamily,
    pub phase: PhaseRole,
    pub band: CellBand,
    pub glyph: char,
}

pub fn classify_line(start: LinePoint, end: LinePoint, steps: i32, step: i32) -> LineGrammarKey {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let abs_dx = dx.abs();
    let abs_dy = dy.abs();
    let family = if abs_dx == 0 || abs_dy == 0 {
        LineFamily::Axis
    } else if abs_dx >= abs_dy * 3 {
        LineFamily::VeryShallow
    } else if abs_dx >= abs_dy * 2 {
        LineFamily::Shallow
    } else if abs_dy >= abs_dx * 2 {
        LineFamily::Steep
    } else {
        LineFamily::Medium
    };
    let length = if steps <= 4 {
        LengthBucket::Short
    } else if steps <= 10 {
        LengthBucket::Medium
    } else {
        LengthBucket::Long
    };
    let direction = if abs_dy >= abs_dx {
        if dy >= 0 {
            DirectionFamily::UpDown
        } else {
            DirectionFamily::DownUp
        }
    } else if dx >= 0 {
        DirectionFamily::LeftRight
    } else {
        DirectionFamily::RightLeft
    };
    let phase = if step == 0 {
        PhaseRole::Entry
    } else if step == steps {
        PhaseRole::Exit
    } else if step <= steps / 3 || step >= (steps * 2) / 3 {
        PhaseRole::Transition
    } else {
        PhaseRole::Core
    };
    let band = if abs_dx == 0 {
        CellBand::Middle
    } else if abs_dy == 0 {
        if phase == PhaseRole::Entry {
            CellBand::Top
        } else if phase == PhaseRole::Exit {
            CellBand::Bottom
        } else {
            CellBand::Middle
        }
    } else {
        let progress = if steps <= 0 {
            0.0
        } else {
            step as f32 / steps as f32
        };
        let offset = if dx >= 0 {
            progress - 0.5
        } else {
            0.5 - progress
        };
        if offset < -0.18 {
            CellBand::Top
        } else if offset > 0.18 {
            CellBand::Bottom
        } else {
            CellBand::Middle
        }
    };

    LineGrammarKey::new(family, length, direction, phase, band)
}

pub fn rasterize_line(start: LinePoint, end: LinePoint) -> Vec<LineStep> {
    let mut x0 = start.x;
    let mut y0 = start.y;
    let x1 = end.x;
    let y1 = end.y;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let steps = dx.max(-dy).max(1);
    let mut step = 0;
    let mut points = Vec::with_capacity((steps + 1) as usize);

    loop {
        points.push(LineStep {
            point: LinePoint { x: x0, y: y0 },
            step,
            steps,
        });
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = err * 2;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
        step += 1;
    }

    points
}

pub fn soft_line_glyph(start: LinePoint, end: LinePoint, step: i32, steps: i32) -> char {
    let key = classify_line(start, end, steps, step);
    if matches!(key.family, LineFamily::VeryShallow) && matches!(key.length, LengthBucket::Long) {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        return long_shallow_cadence_glyph(key.direction, dx.signum(), dy.signum(), step, steps);
    }
    glyph_for_key(key)
}

const LINE_GLYPH_ATLAS: &[LineGlyphRule] = &[
    LineGlyphRule {
        family: LineFamily::Axis,
        length: LengthBucket::Short,
        direction: DirectionFamily::LeftRight,
        phase: PhaseRole::Entry,
        band: CellBand::Middle,
        glyph: '-',
    },
    LineGlyphRule {
        family: LineFamily::Axis,
        length: LengthBucket::Short,
        direction: DirectionFamily::RightLeft,
        phase: PhaseRole::Entry,
        band: CellBand::Middle,
        glyph: '-',
    },
    LineGlyphRule {
        family: LineFamily::Axis,
        length: LengthBucket::Short,
        direction: DirectionFamily::UpDown,
        phase: PhaseRole::Entry,
        band: CellBand::Middle,
        glyph: '|',
    },
    LineGlyphRule {
        family: LineFamily::Axis,
        length: LengthBucket::Short,
        direction: DirectionFamily::DownUp,
        phase: PhaseRole::Entry,
        band: CellBand::Middle,
        glyph: '|',
    },
    LineGlyphRule {
        family: LineFamily::VeryShallow,
        length: LengthBucket::Long,
        direction: DirectionFamily::LeftRight,
        phase: PhaseRole::Entry,
        band: CellBand::Middle,
        glyph: '-',
    },
    LineGlyphRule {
        family: LineFamily::VeryShallow,
        length: LengthBucket::Long,
        direction: DirectionFamily::LeftRight,
        phase: PhaseRole::Transition,
        band: CellBand::Middle,
        glyph: '-',
    },
    LineGlyphRule {
        family: LineFamily::VeryShallow,
        length: LengthBucket::Long,
        direction: DirectionFamily::LeftRight,
        phase: PhaseRole::Core,
        band: CellBand::Middle,
        glyph: '\'',
    },
    LineGlyphRule {
        family: LineFamily::VeryShallow,
        length: LengthBucket::Long,
        direction: DirectionFamily::LeftRight,
        phase: PhaseRole::Exit,
        band: CellBand::Middle,
        glyph: '.',
    },
    LineGlyphRule {
        family: LineFamily::VeryShallow,
        length: LengthBucket::Long,
        direction: DirectionFamily::RightLeft,
        phase: PhaseRole::Entry,
        band: CellBand::Middle,
        glyph: '.',
    },
    LineGlyphRule {
        family: LineFamily::VeryShallow,
        length: LengthBucket::Long,
        direction: DirectionFamily::RightLeft,
        phase: PhaseRole::Transition,
        band: CellBand::Middle,
        glyph: '\'',
    },
    LineGlyphRule {
        family: LineFamily::VeryShallow,
        length: LengthBucket::Long,
        direction: DirectionFamily::RightLeft,
        phase: PhaseRole::Core,
        band: CellBand::Middle,
        glyph: '-',
    },
    LineGlyphRule {
        family: LineFamily::VeryShallow,
        length: LengthBucket::Long,
        direction: DirectionFamily::RightLeft,
        phase: PhaseRole::Exit,
        band: CellBand::Middle,
        glyph: '-',
    },
    LineGlyphRule {
        family: LineFamily::VeryShallow,
        length: LengthBucket::Short,
        direction: DirectionFamily::LeftRight,
        phase: PhaseRole::Entry,
        band: CellBand::Middle,
        glyph: '.',
    },
    LineGlyphRule {
        family: LineFamily::VeryShallow,
        length: LengthBucket::Short,
        direction: DirectionFamily::LeftRight,
        phase: PhaseRole::Transition,
        band: CellBand::Middle,
        glyph: '_',
    },
    LineGlyphRule {
        family: LineFamily::VeryShallow,
        length: LengthBucket::Short,
        direction: DirectionFamily::LeftRight,
        phase: PhaseRole::Core,
        band: CellBand::Middle,
        glyph: '-',
    },
    LineGlyphRule {
        family: LineFamily::VeryShallow,
        length: LengthBucket::Short,
        direction: DirectionFamily::LeftRight,
        phase: PhaseRole::Exit,
        band: CellBand::Middle,
        glyph: '\'',
    },
    LineGlyphRule {
        family: LineFamily::Shallow,
        length: LengthBucket::Short,
        direction: DirectionFamily::LeftRight,
        phase: PhaseRole::Entry,
        band: CellBand::Middle,
        glyph: '.',
    },
    LineGlyphRule {
        family: LineFamily::Shallow,
        length: LengthBucket::Short,
        direction: DirectionFamily::LeftRight,
        phase: PhaseRole::Transition,
        band: CellBand::Middle,
        glyph: '`',
    },
    LineGlyphRule {
        family: LineFamily::Shallow,
        length: LengthBucket::Short,
        direction: DirectionFamily::LeftRight,
        phase: PhaseRole::Core,
        band: CellBand::Middle,
        glyph: '-',
    },
    LineGlyphRule {
        family: LineFamily::Shallow,
        length: LengthBucket::Short,
        direction: DirectionFamily::LeftRight,
        phase: PhaseRole::Exit,
        band: CellBand::Middle,
        glyph: '\'',
    },
    LineGlyphRule {
        family: LineFamily::Medium,
        length: LengthBucket::Medium,
        direction: DirectionFamily::LeftRight,
        phase: PhaseRole::Entry,
        band: CellBand::Middle,
        glyph: '.',
    },
    LineGlyphRule {
        family: LineFamily::Medium,
        length: LengthBucket::Medium,
        direction: DirectionFamily::LeftRight,
        phase: PhaseRole::Transition,
        band: CellBand::Middle,
        glyph: '/',
    },
    LineGlyphRule {
        family: LineFamily::Medium,
        length: LengthBucket::Medium,
        direction: DirectionFamily::LeftRight,
        phase: PhaseRole::Core,
        band: CellBand::Middle,
        glyph: '\\',
    },
    LineGlyphRule {
        family: LineFamily::Medium,
        length: LengthBucket::Medium,
        direction: DirectionFamily::LeftRight,
        phase: PhaseRole::Exit,
        band: CellBand::Middle,
        glyph: '.',
    },
    LineGlyphRule {
        family: LineFamily::Medium,
        length: LengthBucket::Medium,
        direction: DirectionFamily::UpDown,
        phase: PhaseRole::Entry,
        band: CellBand::Middle,
        glyph: '|',
    },
    LineGlyphRule {
        family: LineFamily::Steep,
        length: LengthBucket::Medium,
        direction: DirectionFamily::UpDown,
        phase: PhaseRole::Entry,
        band: CellBand::Middle,
        glyph: '/',
    },
    LineGlyphRule {
        family: LineFamily::Steep,
        length: LengthBucket::Medium,
        direction: DirectionFamily::UpDown,
        phase: PhaseRole::Transition,
        band: CellBand::Middle,
        glyph: '/',
    },
    LineGlyphRule {
        family: LineFamily::Steep,
        length: LengthBucket::Medium,
        direction: DirectionFamily::UpDown,
        phase: PhaseRole::Core,
        band: CellBand::Middle,
        glyph: '\\',
    },
    LineGlyphRule {
        family: LineFamily::Steep,
        length: LengthBucket::Medium,
        direction: DirectionFamily::UpDown,
        phase: PhaseRole::Exit,
        band: CellBand::Middle,
        glyph: '.',
    },
    LineGlyphRule {
        family: LineFamily::Steep,
        length: LengthBucket::Medium,
        direction: DirectionFamily::DownUp,
        phase: PhaseRole::Entry,
        band: CellBand::Middle,
        glyph: '\\',
    },
    LineGlyphRule {
        family: LineFamily::Steep,
        length: LengthBucket::Medium,
        direction: DirectionFamily::DownUp,
        phase: PhaseRole::Transition,
        band: CellBand::Middle,
        glyph: '\\',
    },
    LineGlyphRule {
        family: LineFamily::Steep,
        length: LengthBucket::Medium,
        direction: DirectionFamily::DownUp,
        phase: PhaseRole::Core,
        band: CellBand::Middle,
        glyph: '/',
    },
    LineGlyphRule {
        family: LineFamily::Steep,
        length: LengthBucket::Medium,
        direction: DirectionFamily::DownUp,
        phase: PhaseRole::Exit,
        band: CellBand::Middle,
        glyph: '.',
    },
];

fn glyph_for_key(key: LineGrammarKey) -> char {
    LINE_GLYPH_ATLAS
        .iter()
        .find(|rule| {
            rule.family == key.family
                && rule.length == key.length
                && rule.direction == key.direction
                && rule.phase == key.phase
                && rule.band == key.band
        })
        .map(|rule| rule.glyph)
        .unwrap_or_else(|| fallback_glyph(key))
}

fn fallback_glyph(key: LineGrammarKey) -> char {
    match key.family {
        LineFamily::Axis => match key.direction {
            DirectionFamily::UpDown | DirectionFamily::DownUp => '|',
            _ => '-',
        },
        LineFamily::VeryShallow => match key.phase {
            PhaseRole::Entry => match key.band {
                CellBand::Top => '`',
                CellBand::Middle => '.',
                CellBand::Bottom => ',',
            },
            PhaseRole::Transition => match key.band {
                CellBand::Top => '`',
                CellBand::Middle => '_',
                CellBand::Bottom => ',',
            },
            PhaseRole::Core => match key.band {
                CellBand::Top => '-',
                CellBand::Middle => '-',
                CellBand::Bottom => '_',
            },
            PhaseRole::Exit => match key.band {
                CellBand::Top => '\'',
                CellBand::Middle => '.',
                CellBand::Bottom => ',',
            },
        },
        LineFamily::Shallow => match key.phase {
            PhaseRole::Entry => match key.band {
                CellBand::Top => '`',
                CellBand::Middle => '.',
                CellBand::Bottom => ',',
            },
            PhaseRole::Transition => match key.band {
                CellBand::Top => '`',
                CellBand::Middle => '-',
                CellBand::Bottom => '_',
            },
            PhaseRole::Core => match key.band {
                CellBand::Top => '-',
                CellBand::Middle => '-',
                CellBand::Bottom => '_',
            },
            PhaseRole::Exit => match key.band {
                CellBand::Top => '\'',
                CellBand::Middle => '.',
                CellBand::Bottom => ',',
            },
        },
        LineFamily::Medium => match key.direction {
            DirectionFamily::UpDown | DirectionFamily::DownUp => '|',
            _ => match key.phase {
                PhaseRole::Entry => match key.band {
                    CellBand::Top => '`',
                    CellBand::Middle => '.',
                    CellBand::Bottom => ',',
                },
                PhaseRole::Transition => '/',
                PhaseRole::Core => '\\',
                PhaseRole::Exit => match key.band {
                    CellBand::Top => '\'',
                    CellBand::Middle => '.',
                    CellBand::Bottom => ',',
                },
            },
        },
        LineFamily::Steep => match key.direction {
            DirectionFamily::UpDown => match key.phase {
                PhaseRole::Entry | PhaseRole::Transition => '/',
                PhaseRole::Core => '\\',
                PhaseRole::Exit => match key.band {
                    CellBand::Top => '\'',
                    CellBand::Middle => '.',
                    CellBand::Bottom => ',',
                },
            },
            DirectionFamily::DownUp => match key.phase {
                PhaseRole::Entry | PhaseRole::Transition => '\\',
                PhaseRole::Core => '/',
                PhaseRole::Exit => match key.band {
                    CellBand::Top => '`',
                    CellBand::Middle => '.',
                    CellBand::Bottom => ',',
                },
            },
            _ => match key.phase {
                PhaseRole::Entry => match key.band {
                    CellBand::Top => '`',
                    CellBand::Middle => '.',
                    CellBand::Bottom => ',',
                },
                PhaseRole::Transition => '/',
                PhaseRole::Core => '\\',
                PhaseRole::Exit => match key.band {
                    CellBand::Top => '\'',
                    CellBand::Middle => '.',
                    CellBand::Bottom => ',',
                },
            },
        },
    }
}

pub fn glyph_for_line_step(
    start: LinePoint,
    end: LinePoint,
    current: LinePoint,
    previous: Option<LinePoint>,
    next: Option<LinePoint>,
    step: i32,
    steps: i32,
) -> char {
    if step == 0 || step == steps {
        return soft_line_glyph(start, end, step, steps);
    }

    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let abs_dx = dx.abs();
    let abs_dy = dy.abs();
    let local_dx = next.map(|p| p.x - current.x).unwrap_or(0);
    let local_dy = next.map(|p| p.y - current.y).unwrap_or(0);
    let prev_dx = previous.map(|p| current.x - p.x).unwrap_or(local_dx);
    let prev_dy = previous.map(|p| current.y - p.y).unwrap_or(local_dy);
    let local_sign_x = if local_dx == 0 {
        prev_dx.signum()
    } else {
        local_dx.signum()
    };
    let local_sign_y = if local_dy == 0 {
        prev_dy.signum()
    } else {
        local_dy.signum()
    };
    let band = band_for_point(start, end, current);
    let key = LineGrammarKey::new(
        classify_line(start, end, steps, step).family,
        classify_line(start, end, steps, step).length,
        classify_line(start, end, steps, step).direction,
        classify_line(start, end, steps, step).phase,
        band,
    );

    if matches!(key.family, LineFamily::VeryShallow) && matches!(key.length, LengthBucket::Long) {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        return long_shallow_cadence_glyph(key.direction, dx.signum(), dy.signum(), step, steps);
    }
    if let Some(glyph) = glyph_for_key_with_band(key) {
        return glyph;
    }
    if abs_dx == 0 {
        return '|';
    }
    if abs_dy == 0 {
        return if step == 1 || step == steps - 1 {
            '_'
        } else {
            '-'
        };
    }

    if abs_dx >= abs_dy * 3 {
        return if local_dx == 0 { '_' } else { '-' };
    }

    if abs_dy >= abs_dx * 3 {
        return '|';
    }

    if local_sign_x == local_sign_y {
        if dx.signum() == dy.signum() {
            '\\'
        } else {
            '/'
        }
    } else if local_dx == 0 || local_dy == 0 {
        if abs_dx > abs_dy {
            '_'
        } else {
            '|'
        }
    } else if dx.signum() == dy.signum() {
        '\\'
    } else {
        '/'
    }
}

fn band_for_point(start: LinePoint, end: LinePoint, current: LinePoint) -> CellBand {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    if dx == 0 || dy == 0 {
        return CellBand::Middle;
    }
    let cross = (current.x - start.x) * dy - (current.y - start.y) * dx;
    let abs_dx = dx.abs().max(1);
    let normalized = cross as f32 / abs_dx as f32;
    if normalized < -0.35 {
        CellBand::Top
    } else if normalized > 0.35 {
        CellBand::Bottom
    } else {
        CellBand::Middle
    }
}

fn glyph_for_key_with_band(key: LineGrammarKey) -> Option<char> {
    LINE_GLYPH_ATLAS
        .iter()
        .find(|rule| {
            rule.family == key.family
                && rule.length == key.length
                && rule.direction == key.direction
                && rule.phase == key.phase
                && rule.band == key.band
        })
        .map(|rule| rule.glyph)
        .or_else(|| Some(fallback_glyph(key)))
}

fn long_shallow_cadence_glyph(
    direction: DirectionFamily,
    dx_sign: i32,
    dy_sign: i32,
    step: i32,
    steps: i32,
) -> char {
    let progress = if steps <= 0 {
        0.0
    } else {
        step as f32 / steps as f32
    };
    let phase = if progress < 0.12 {
        0
    } else if progress < 0.28 {
        1
    } else if progress < 0.52 {
        2
    } else if progress < 0.78 {
        3
    } else {
        4
    };

    let rising_right = dx_sign >= 0 && dy_sign <= 0;
    let falling_right = dx_sign >= 0 && dy_sign >= 0;
    let rising_left = dx_sign <= 0 && dy_sign <= 0;
    let falling_left = dx_sign <= 0 && dy_sign >= 0;

    match direction {
        DirectionFamily::LeftRight => match phase {
            0 => {
                if rising_right {
                    ','
                } else if falling_right {
                    '`'
                } else if rising_left {
                    '.'
                } else {
                    '\''
                }
            }
            1 => {
                if rising_right {
                    ','
                } else if falling_right {
                    '\''
                } else if rising_left {
                    '`'
                } else {
                    '.'
                }
            }
            2 => '-',
            3 => '_',
            _ => {
                if rising_right {
                    ','
                } else if falling_right {
                    '\''
                } else if rising_left {
                    '.'
                } else {
                    '`'
                }
            }
        },
        DirectionFamily::RightLeft => match phase {
            0 => {
                if rising_left {
                    ','
                } else if falling_left {
                    '`'
                } else if rising_right {
                    '.'
                } else {
                    '\''
                }
            }
            1 => {
                if rising_left {
                    ','
                } else if falling_left {
                    '\''
                } else if rising_right {
                    '`'
                } else {
                    '.'
                }
            }
            2 => '-',
            3 => '_',
            _ => {
                if rising_left {
                    ','
                } else if falling_left {
                    '\''
                } else if rising_right {
                    '.'
                } else {
                    '`'
                }
            }
        },
        _ => match phase {
            0 => {
                if dy_sign <= 0 {
                    ','
                } else {
                    '`'
                }
            }
            1 => {
                if dy_sign <= 0 {
                    '\''
                } else {
                    ','
                }
            }
            2 => '-',
            3 => '_',
            _ => {
                if dy_sign <= 0 {
                    ','
                } else {
                    '\''
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rasterize_line_is_endpoint_inclusive() {
        let points = rasterize_line(LinePoint { x: 2, y: 3 }, LinePoint { x: 6, y: 3 });
        assert_eq!(points.first().unwrap().point, LinePoint { x: 2, y: 3 });
        assert_eq!(points.last().unwrap().point, LinePoint { x: 6, y: 3 });
    }

    #[test]
    fn rasterize_line_is_symmetric_under_reversal() {
        let forward = rasterize_line(LinePoint { x: -3, y: 2 }, LinePoint { x: 4, y: 6 });
        let reverse = rasterize_line(LinePoint { x: 4, y: 6 }, LinePoint { x: -3, y: 2 });

        let forward_points: Vec<_> = forward.into_iter().map(|step| step.point).collect();
        let reverse_points: Vec<_> = reverse.into_iter().map(|step| step.point).collect();
        let mut reversed = reverse_points;
        reversed.reverse();

        assert_eq!(forward_points, reversed);
    }

    #[test]
    fn soft_line_glyph_prefers_axis_strokes_for_shallow_lines() {
        let start = LinePoint { x: 0, y: 0 };
        let end = LinePoint { x: 12, y: 1 };

        let glyphs: Vec<char> = [0, 4, 8, 12]
            .into_iter()
            .map(|step| soft_line_glyph(start, end, step, 12))
            .collect();

        assert!(glyphs.contains(&'-'));
        assert!(glyphs
            .iter()
            .any(|glyph| matches!(glyph, '.' | ',' | '`' | '\'' | '_')));
    }

    #[test]
    fn soft_line_glyph_prefers_directional_slashes_for_steep_lines() {
        let forward = soft_line_glyph(LinePoint { x: 0, y: 0 }, LinePoint { x: 1, y: 12 }, 11, 12);
        let backward =
            soft_line_glyph(LinePoint { x: 0, y: 0 }, LinePoint { x: 1, y: -12 }, 11, 12);

        assert_eq!(forward, '/');
        assert_eq!(backward, '\\');
    }

    #[test]
    fn classify_line_distinguishes_family_length_direction_and_phase() {
        let key = classify_line(LinePoint { x: 0, y: 0 }, LinePoint { x: 12, y: 4 }, 12, 0);
        assert!(matches!(
            key.family,
            LineFamily::VeryShallow | LineFamily::Shallow | LineFamily::Medium
        ));
        assert!(matches!(key.length, LengthBucket::Long));
        assert!(matches!(key.phase, PhaseRole::Entry));
    }

    #[test]
    fn glyph_for_line_step_prefers_structural_glyphs_midstroke() {
        let start = LinePoint { x: 0, y: 0 };
        let end = LinePoint { x: 12, y: 4 };
        let current = LinePoint { x: 6, y: 2 };
        let next = Some(LinePoint { x: 7, y: 2 });

        let glyph = glyph_for_line_step(
            start,
            end,
            current,
            Some(LinePoint { x: 5, y: 2 }),
            next,
            6,
            12,
        );
        assert!(matches!(glyph, '-' | '_' | '/' | '\\' | '.' | '\''));
    }

    #[test]
    fn soft_line_glyph_uses_a_long_shallow_cadence() {
        let start = LinePoint { x: -28, y: 22 };
        let end = LinePoint { x: 36, y: 12 };

        let glyphs: Vec<char> = [0, 8, 16, 32, 48, 64]
            .into_iter()
            .map(|step| soft_line_glyph(start, end, step, 64))
            .collect();

        assert!(glyphs.contains(&','));
        assert!(glyphs
            .iter()
            .any(|glyph| matches!(glyph, '-' | '_' | '`' | '.')));
    }
}

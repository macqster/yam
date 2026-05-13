use crate::render::compositor::Grid;
use crate::theme::style as theme_style;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModalFrame {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModalFooter<'a> {
    pub left: &'a str,
    pub right: &'a str,
}

impl ModalFrame {
    pub fn centered(
        terminal_width: u16,
        terminal_height: u16,
        preferred_width: u16,
        preferred_height: u16,
    ) -> Self {
        let width = terminal_width.min(preferred_width);
        let height = terminal_height.min(preferred_height);
        let x = (terminal_width.saturating_sub(width)) / 2;
        let y = (terminal_height.saturating_sub(height)) / 2;
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn body_origin(self) -> (u16, u16) {
        (self.x + 2, self.y + 3)
    }
}

pub fn paint_modal_shell(
    grid: &mut Grid,
    frame: ModalFrame,
    title: &str,
    footer: Option<ModalFooter<'_>>,
) {
    fill_panel_background(grid, frame);
    draw_border(grid, frame);
    draw_border_title(grid, frame, title);
    draw_border_footer(grid, frame, footer);
}

fn fill_panel_background(grid: &mut Grid, frame: ModalFrame) {
    let style = theme_style::modal_panel();
    for row in frame.y..frame.y.saturating_add(frame.height) {
        for col in frame.x..frame.x.saturating_add(frame.width) {
            if let Some(cell) = grid.cell_mut(col, row) {
                cell.symbol = ' ';
                cell.style = style;
            }
        }
    }
}

fn draw_border(grid: &mut Grid, frame: ModalFrame) {
    if frame.width < 2 || frame.height < 2 {
        return;
    }
    let right = frame.x + frame.width - 1;
    let bottom = frame.y + frame.height - 1;
    for cx in frame.x..=right {
        write_border_cell(
            grid,
            cx,
            frame.y,
            if cx == frame.x {
                '┌'
            } else if cx == right {
                '┐'
            } else {
                '─'
            },
        );
        write_border_cell(
            grid,
            cx,
            bottom,
            if cx == frame.x {
                '└'
            } else if cx == right {
                '┘'
            } else {
                '─'
            },
        );
    }
    for cy in frame.y + 1..bottom {
        write_border_cell(grid, frame.x, cy, '│');
        write_border_cell(grid, right, cy, '│');
    }
}

fn draw_border_title(grid: &mut Grid, frame: ModalFrame, title: &str) {
    if frame.width <= 6 || title.is_empty() {
        return;
    }
    let decorated = format!("  {}  ", title);
    let title_width = decorated.chars().count() as u16;
    if title_width >= frame.width.saturating_sub(2) {
        return;
    }
    let x = frame.x + (frame.width.saturating_sub(title_width)) / 2;
    for (offset, ch) in decorated.chars().enumerate() {
        if let Some(cell) = grid.cell_mut(x + offset as u16, frame.y) {
            cell.symbol = ch;
            cell.style = theme_style::modal_footer_symbol();
        }
    }
}

fn draw_border_footer(grid: &mut Grid, frame: ModalFrame, footer: Option<ModalFooter<'_>>) {
    let Some(footer) = footer else {
        return;
    };
    if frame.width <= 6 || (footer.left.is_empty() && footer.right.is_empty()) {
        return;
    }

    let y = frame.y + frame.height.saturating_sub(1);
    let left_start = frame.x + 3;
    let right = frame.x + frame.width - 1;
    let right_text = format!("  {}  ", footer.right);
    let right_width = right_text.chars().count() as u16;
    let right_start = right.saturating_sub(2 + right_width);

    if right_start <= left_start {
        return;
    }

    if !footer.left.is_empty() {
        let left_text = format!("  {}  ", footer.left);
        let left_available = right_start.saturating_sub(left_start) as usize;
        let left = left_text.chars().take(left_available).collect::<String>();
        write_footer_text(grid, left_start, y, &left);
    }

    let start_x = right_start;
    write_footer_text(grid, start_x, y, &right_text);
}

fn write_border_cell(grid: &mut Grid, x: u16, y: u16, ch: char) {
    if let Some(cell) = grid.cell_mut(x, y) {
        cell.symbol = ch;
        cell.style = theme_style::modal_border();
    }
}

fn write_footer_text(grid: &mut Grid, x: u16, y: u16, text: &str) {
    for (offset, ch) in text.chars().enumerate() {
        if let Some(cell) = grid.cell_mut(x + offset as u16, y) {
            cell.symbol = ch;
            cell.style = if is_footer_separator(ch) {
                theme_style::modal_border()
            } else if is_footer_symbol(ch) {
                theme_style::modal_footer_symbol()
            } else {
                theme_style::panel_text()
            };
        }
    }
}

fn is_footer_symbol(ch: char) -> bool {
    matches!(ch, '↑' | '↓' | '←' | '→' | '⇥' | '⏎' | '⌨' | '?' | '⎋')
}

fn is_footer_separator(ch: char) -> bool {
    ch == '─'
}

#[cfg(test)]
mod tests {
    use super::{paint_modal_shell, ModalFooter, ModalFrame};
    use crate::render::compositor::Grid;
    use crate::theme::{palette, style as theme_style};

    #[test]
    fn centered_modal_uses_preferred_size_with_terminal_limits() {
        let frame = ModalFrame::centered(124, 32, 68, 16);
        assert_eq!(frame.width, 68);
        assert_eq!(frame.height, 16);
        assert_eq!(frame.x, 28);
        assert_eq!(frame.y, 8);
    }

    #[test]
    fn modal_shell_uses_continuous_box_drawing_border_and_plain_title() {
        let mut grid = Grid::new(40, 12);
        let frame = ModalFrame {
            x: 4,
            y: 2,
            width: 24,
            height: 7,
        };

        paint_modal_shell(
            &mut grid,
            frame,
            "settings",
            Some(ModalFooter {
                left: "↑ ↓ ← →  ──  ⇥ ⏎ ⌨",
                right: "? ⎋",
            }),
        );

        assert_eq!(grid.cells[grid.index(4, 2)].symbol, '┌');
        assert_eq!(grid.cells[grid.index(27, 2)].symbol, '┐');
        assert_eq!(grid.cells[grid.index(4, 8)].symbol, '└');
        assert_eq!(grid.cells[grid.index(27, 8)].symbol, '┘');
        assert_eq!(grid.cells[grid.index(5, 2)].symbol, '─');
        assert_eq!(grid.cells[grid.index(4, 3)].symbol, '│');

        let title_start = frame.x + (frame.width - "  settings  ".chars().count() as u16) / 2;
        let title: String = (0.."  settings  ".chars().count() as u16)
            .map(|offset| grid.cells[grid.index(title_start + offset, frame.y)].symbol)
            .collect();
        assert_eq!(title, "  settings  ");
        assert_eq!(
            grid.cells[grid.index(title_start + 2, frame.y)].style.fg,
            Some(palette::MODAL_FOOTER_SYMBOL)
        );
        assert_eq!(
            grid.cells[grid.index(frame.x + 1, frame.y)].style,
            theme_style::modal_border()
        );
        assert_eq!(
            grid.cells[grid.index(frame.x + 1, frame.y + 1)].style.bg,
            Some(palette::MODAL_BG)
        );
        let footer_left: String = (0..22)
            .map(|offset| {
                grid.cells[grid.index(frame.x + 3 + offset, frame.y + frame.height - 1)].symbol
            })
            .collect();
        let footer_right: String = (0..7)
            .map(|offset| {
                grid.cells[grid.index(
                    frame.x + frame.width - 9 + offset,
                    frame.y + frame.height - 1,
                )]
                .symbol
            })
            .collect();
        assert!(footer_left.starts_with("  ↑ ↓"));
        assert!(footer_right.contains("?"));
        assert!(footer_right.contains("⎋"));
        assert_eq!(
            grid.cells[grid.index(frame.x + 5, frame.y + frame.height - 1)]
                .style
                .fg,
            Some(palette::MODAL_FOOTER_SYMBOL)
        );
    }

    #[test]
    fn empty_left_footer_keeps_bottom_border_continuous() {
        let mut grid = Grid::new(40, 12);
        let frame = ModalFrame {
            x: 4,
            y: 2,
            width: 24,
            height: 7,
        };

        paint_modal_shell(
            &mut grid,
            frame,
            "[?] help",
            Some(ModalFooter {
                left: "",
                right: "? ⎋",
            }),
        );

        assert_eq!(
            grid.cells[grid.index(frame.x + 3, frame.y + frame.height - 1)].symbol,
            '─'
        );
        assert_eq!(
            grid.cells[grid.index(frame.x + 6, frame.y + frame.height - 1)].symbol,
            '─'
        );
    }
}

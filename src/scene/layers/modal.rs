use crate::render::compositor::write_string;
use crate::render::compositor::Grid;
use crate::theme::style as theme_style;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModalFrame {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
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

pub fn paint_modal_shell(grid: &mut Grid, frame: ModalFrame, title: &str) {
    fill_panel_background(grid, frame);
    draw_border(grid, frame);
    write_string(
        grid,
        frame.x + 2,
        frame.y + 1,
        title,
        theme_style::panel_text(),
    );
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
            if cx == frame.x || cx == right {
                '+'
            } else {
                '-'
            },
        );
        write_border_cell(
            grid,
            cx,
            bottom,
            if cx == frame.x || cx == right {
                '+'
            } else {
                '-'
            },
        );
    }
    for cy in frame.y + 1..bottom {
        write_border_cell(grid, frame.x, cy, '|');
        write_border_cell(grid, right, cy, '|');
    }
}

fn write_border_cell(grid: &mut Grid, x: u16, y: u16, ch: char) {
    if let Some(cell) = grid.cell_mut(x, y) {
        cell.symbol = ch;
        cell.style = theme_style::panel_text();
    }
}

#[cfg(test)]
mod tests {
    use super::ModalFrame;

    #[test]
    fn centered_modal_uses_preferred_size_with_terminal_limits() {
        let frame = ModalFrame::centered(124, 32, 68, 16);
        assert_eq!(frame.width, 68);
        assert_eq!(frame.height, 16);
        assert_eq!(frame.x, 28);
        assert_eq!(frame.y, 8);
    }
}

use appcui::prelude::*;
use rand::Rng;
use std::time::Duration;

use crate::matrix_column::MatrixColumn;

#[derive(Clone)]
enum DisplayState {
    Loading,
    Running,
}

#[Window(events = TimerEvents)]
pub(super) struct Win {
    columns: Vec<MatrixColumn>,
    display_state: DisplayState,
    loading_chars_shown: usize,
    loading_start_time: u64,
    c: Handle<Canvas>,
}

impl Win {
    const LOADING_TEXT: &'static str = "Loading the matrix...";
    const LOADING_WAIT_TIME: u64 = 6500; // 6.5 seconds

    pub(super) fn new() -> Self {
        let mut w = Self {
            base: window!("Matrix,d:c,w:60,h:25"),
            columns: Vec::new(),
            display_state: DisplayState::Loading,
            loading_chars_shown: 0,
            loading_start_time: 0,
            c: Handle::None,
        };
        w.c = w.add(canvas!("d:c,w:100%,h:100%,size:60x25"));
        if let Some(timer) = w.timer() {
            timer.start(Duration::from_millis(50));
        }
        w
    }

    fn init_matrix(&mut self) {
        let c = self.c;
        if let Some(canvas) = self.control_mut(c) {
            let size = canvas.drawing_surface_mut().size();
            let width = size.width as i32;
            let height = size.height as i32;

            self.columns.clear();

            let mut rng = rand::thread_rng();
            for x in 0..width {
                if rng.gen_bool(0.8) {
                    // 80% chance to create a column at each position
                    self.columns.push(MatrixColumn::new(x, height));
                }
            }
        }
    }

    fn repaint_matrix(&mut self) {
        // Copy the data we need before borrowing canvas mutably
        let display_state = self.display_state.clone();
        let loading_chars_shown = self.loading_chars_shown;
        let columns = self.columns.clone();
        let h = self.c;

        if let Some(canvas) = self.control_mut(h) {
            let surface = canvas.drawing_surface_mut();
            surface.clear(char!("' ',black,black"));

            match display_state {
                DisplayState::Loading => {
                    // Draw the loading message with animation
                    let size = surface.size();
                    let x_pos = (size.width as i32 - Win::LOADING_TEXT.len() as i32) / 2;
                    let y_pos = size.height as i32 / 2;

                    // Only draw the characters that should be visible so far
                    if loading_chars_shown > 0 {
                        let visible_text =
                            &Win::LOADING_TEXT[0..loading_chars_shown.min(Win::LOADING_TEXT.len())];
                        surface.write_string(
                            x_pos,
                            y_pos,
                            visible_text,
                            CharAttribute::with_color(Color::Green, Color::Black),
                            false,
                        );
                    }

                    if loading_chars_shown <= Win::LOADING_TEXT.len() {
                        surface.write_char(
                            x_pos + loading_chars_shown as i32,
                            y_pos,
                            char!("' ',black,green"),
                        );
                    }
                }
                DisplayState::Running => {
                    for column in &columns {
                        for i in 0..column.length {
                            if column.active[i]
                                && column.positions[i] >= 0
                                && column.positions[i] < surface.size().height as i32
                            {
                                let x = column.column_pos;
                                let y = column.positions[i];

                                let color = if i == 0 {
                                    Color::Green
                                } else {
                                    let alpha = i as f32 / column.length as f32;
                                    if alpha > 0.5 {
                                        Color::Green
                                    } else if alpha > 0.2 {
                                        Color::DarkGreen
                                    } else {
                                        Color::Gray
                                    }
                                };

                                surface.write_char(
                                    x,
                                    y,
                                    Character::with_attributes(
                                        column.chars[i],
                                        CharAttribute::with_color(color, Color::Black),
                                    ),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

impl TimerEvents for Win {
    fn on_update(&mut self, ticks: u64) -> EventProcessStatus {
        match self.display_state {
            DisplayState::Loading => {
                // Record the start time when we begin loading
                if self.loading_start_time == 0 {
                    self.loading_start_time = ticks;
                }

                // Animate the text one character at a time (every 5 ticks = 250ms)
                if ticks % 5 == 0 && self.loading_chars_shown < Win::LOADING_TEXT.len() {
                    self.loading_chars_shown += 1;
                }

                // After the text is fully shown, wait for the loading duration
                if self.loading_chars_shown >= Win::LOADING_TEXT.len() {
                    let elapsed = (ticks - self.loading_start_time) * 50; // Convert ticks to ms
                    if elapsed >= Win::LOADING_WAIT_TIME {
                        // Transition to running state
                        self.init_matrix();
                        self.display_state = DisplayState::Running;
                    }
                }

                self.repaint_matrix();
            }
            DisplayState::Running => {
                let c = self.c;
                let mut new_columns = Vec::new();

                if let Some(canvas) = self.control_mut(c) {
                    let size = canvas.drawing_surface_mut().size();
                    let height = size.height as i32;

                    for column in &mut self.columns {
                        column.update(height);
                    }

                    let mut rng = rand::thread_rng();
                    if self.columns.len() < size.width as usize && rng.gen_bool(0.05) {
                        let x = rng.gen_range(0..size.width as i32);
                        // Check if there's already a column at this position
                        if !self.columns.iter().any(|col| col.column_pos == x) {
                            new_columns.push(MatrixColumn::new(x, height));
                        }
                    }
                }

                // Add new columns after releasing the borrow
                self.columns.extend(new_columns);
                self.repaint_matrix();
            }
        }

        EventProcessStatus::Processed
    }
}

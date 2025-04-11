use std::time::Duration;
use appcui::prelude::*;


#[derive(Copy, Clone, Default)]
struct Bar {
    value: i32,
    col: Color,
}

const COLORS: [Color; 7] = [
    Color::Red,
    Color::Green,
    Color::Blue,
    Color::Aqua,
    Color::Yellow,
    Color::Pink,
    Color::Gray,
];

#[Window(events = TimerEvents)]
pub(super) struct Win {
    bars: Vec<Bar>,
    c: Handle<Canvas>,
}

impl Win {
    pub(super) fn new() -> Self {
        let mut w = Self {
            base: window!("Animation,d:c,w:50,h:10"),
            bars: Vec::with_capacity(100),
            c: Handle::None,
        };
        w.c = w.add(canvas!("d:c,w:100%,h:100%,size:100x50"));
        if let Some(timer) = w.timer() {
            timer.start(Duration::from_millis(300));
        }
        w
    }
    fn repaint_graphic(&mut self) {
        let h = self.c;
        let line_color = self.theme().lines.normal;
        let bars_width = self.bars.len() as i32 * 2;
        let bars_count = self.bars.len();
        let sz = self.size().reduce_by(2); // window margins
        let mut arr: [Bar; 100] = [Bar::default(); 100];
        // for i in 0..bars_count.min(100) {
        //     arr[i] = self.bars[i];
        // }
        arr[..bars_count.min(100)].copy_from_slice(&self.bars[..bars_count.min(100)]);
        if let Some(canvas) = self.control_mut(h) {
            let surface = canvas.get_drawing_surface();
            surface.clear(char!("' ',w,black"));

            // draw the lines
            surface.draw_horizontal_line(0, (sz.height / 4) as i32, sz.width as i32, LineType::Single, line_color);
            surface.draw_horizontal_line(0, (sz.height / 2) as i32, sz.width as i32, LineType::Single, line_color);
            surface.draw_horizontal_line(0, (sz.height * 3 / 4) as i32, sz.width as i32, LineType::Single, line_color);

            // draw the bars
            let mut x = sz.width as i32 - bars_width;
            for b in &arr {
                surface.fill_vertical_line(
                    x,
                    sz.height as i32 - b.value,
                    sz.height as i32,
                    Character::new(' ', Color::Black, b.col, CharFlags::None),
                );
                x += 2;
            }
        }
    }
}

impl TimerEvents for Win {
    fn on_update(&mut self, _ticks: u64) -> EventProcessStatus {
        let sz = self.size().reduce_by(2); // window margins
        let h = sz.height as i32;
        let last_height = self
            .bars
            .last()
            .unwrap_or(&Bar {
                value: (rand::random::<i32>() % h).clamp(1, h),
                col: Color::Black,
            })
            .value;
        let add = match () {
            _ if last_height <= (h / 4) => ((rand::random::<i32>() % 5) - 1).min(1),
            _ if last_height >= (3 * h / 4) => ((rand::random::<i32>() % 5) - 3).max(-1),
            _ => (rand::random::<i32>() % 3) - 1,
        };
        self.bars.push(Bar {
            value: (last_height + add).clamp(1, h),
            col: COLORS[rand::random::<usize>() % COLORS.len()],
        });
        if self.bars.len() > (sz.width as usize) * 2 {
            self.bars.remove(0);
        }
        // repaint all
        self.repaint_graphic();

        EventProcessStatus::Processed
    }
}

use crate::prelude::*;


#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub start: u16,
    pub end: u16,
    pub x_pos: u16,
    pub y_pos: u16,
    pub color: Color,
    pub flags: CharFlags,
}

#[derive(Clone, Copy, Debug)]
struct Line {
    start: u16,
    end: u16,
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    fn match_delim(text: &[char]) -> Option<(usize, CharFlags)> {
        match text {
            ['*', '*', ..] => Some((2, CharFlags::Bold)),
            ['_', ..] => Some((1, CharFlags::Italic)),
            _ => None,
        }
    }

    fn toggle(flags: &mut CharFlags, f: CharFlags) {
        if flags.contains(f) {
            flags.remove(f);
        } else {
            flags.set(f);
        }
    }

    fn push_span(spans: &mut Vec<Span>, start: u16, end: u16, color: Color, flags: CharFlags) {
        if start < end {
            spans.push(Span {
                start,
                end,
                x_pos: 0,
                y_pos: 0,
                color,
                flags,
            });
        }
    }
    pub fn parse(&self, text: &[char], line_width: u16) -> Vec<Span> {
        let mut spans = Vec::new();
        let mut flags: CharFlags = CharFlags::None;
        let text_color = Color::White;
        let delim_color = Color::Gray;
        let mut i = 0;
        let mut span_start_idx = 0;

        while i < text.len() {
            if let Some((delim_len, flag)) = Self::match_delim(&text[i..]) {
                Self::push_span(&mut spans, span_start_idx, i as u16, text_color, flags);
                if flag != CharFlags::None {
                    Self::push_span(
                        &mut spans,
                        i as u16,
                        (i + delim_len) as u16,
                        delim_color,
                        flags,
                    );
                    Self::toggle(&mut flags, flag);
                }
                i += delim_len;
                span_start_idx = i as u16;
            } else {
                i += 1;
            }
        }

        Self::push_span(
            &mut spans,
            span_start_idx,
            text.len() as u16,
            text_color,
            flags,
        );

        let lines = Self::compute_lines(text, line_width);
        Self::apply_layout(spans, &lines)
    }

    fn is_word_start(text: &[char], i: u16) -> bool {
        i == 0 || text[(i - 1) as usize].is_whitespace()
    }

    fn word_len(text: &[char], i: u16) -> u16 {
        let mut len = 0u16;
        let mut j = i as usize;
        while j < text.len() && !text[j].is_whitespace() {
            len += 1;
            j += 1;
        }
        len
    }

    fn compute_lines(text: &[char], line_width: u16) -> Vec<Line> {
        let line_width = line_width.max(1);
        let mut lines = Vec::new();
        let mut line_start: u16 = 0;
        let mut x: u16 = 0;
        let mut i: u16 = 0;

        while (i as usize) < text.len() {
            let ch = text[i as usize];

            if ch == '\n' {
                lines.push(Line {
                    start: line_start,
                    end: i,
                });
                i += 1;
                line_start = i;
                x = 0;
                continue;
            }

            if !ch.is_whitespace()
                && Self::is_word_start(text, i)
                && x > 0
                && x + Self::word_len(text, i) > line_width
            {
                lines.push(Line {
                    start: line_start,
                    end: i,
                });
                line_start = i;
                x = 0;
            }

            if x >= line_width {
                lines.push(Line {
                    start: line_start,
                    end: i,
                });
                line_start = i;
                x = 0;
            }

            x += 1;
            i += 1;
        }

        lines.push(Line {
            start: line_start,
            end: text.len() as u16,
        });
        lines
    }

    fn apply_layout(spans: Vec<Span>, lines: &[Line]) -> Vec<Span> {
        let mut out = Vec::new();

        for span in spans {
            for (y, line) in lines.iter().enumerate() {
                if line.start >= span.end {
                    break;
                }
                let start = span.start.max(line.start);
                let end = span.end.min(line.end);
                if start < end {
                    out.push(Span {
                        start,
                        end,
                        x_pos: start - line.start,
                        y_pos: y as u16,
                        color: span.color,
                        flags: span.flags,
                    });
                }
            }
        }

        out
    }
}
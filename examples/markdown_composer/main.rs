use appcui::{backend, prelude::*, ui::markdown_composer::ListFlags};

#[Window(events = MarkdownComposerEvents)]
struct MyWin {
    status: Handle<Label>,
    changes: u32,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'Markdown Composer',a:c,w:60,h:20,flags:Sizeable"),
            status: Handle::None,
            changes: 0,
        };

        win.status = win.add(label!("'Write something, then press Ctrl+Enter',d:b,h:2"));

        let mut mc = markdown_composer!("'**Welcome** to the _markdown_ composer! Write to contact@company.com or visit https://example.com/a-very-long-documentation-page for details. See also www.example.org or drop a line to _another.user@mail.com._\n\nThe markers stay hidden while `show_markers` is off:\n\n* a bullet\n* another one\n\n> a quoted line\n\n```\nlet x = 1;\n```\n\nType : for emoji, @ for fruits or :B): for an emoticon.', d:t,h:16,flags:Emoticons");

        mc.add_emoji_list(':');
        mc.add_list(
            '@',
            &["apple", "apricot", "banana", "blueberry", "cherry", "grape", "grapefruit", "mango"],
            ListFlags::RemoveTrigger,
        );

        win.add(mc);
        win
    }

    fn show(&mut self, text: &str) {
        let h = self.status;
        if let Some(label) = self.control_mut(h) {
            label.set_caption(text);
        }
    }
}

impl MarkdownComposerEvents for MyWin {
    fn on_text_changed(&mut self, handle: Handle<MarkdownComposer>) -> EventProcessStatus {
        self.changes += 1;
        let len = self.control(handle).map(|mc| mc.text().len()).unwrap_or(0);
        let msg = format!("changes={} bytes={}", self.changes, len);
        self.show(&msg);
        EventProcessStatus::Processed
    }

    fn on_validate(&mut self, _handle: Handle<MarkdownComposer>, text: &str) -> EventProcessStatus {
        let msg = format!("VALIDATED ({} bytes): {}", text.len(), text.replace('\n', " | "));
        self.show(&msg);
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    #[cfg(target_os = "windows")]
    {
        App::new().backend(backend::Type::WindowsVT).color_schema(false).window(MyWin::new).run()
    }
    #[cfg(not(target_os = "windows"))]
    {
        App::new().color_schema(false).window(MyWin::new).run()
    }
}

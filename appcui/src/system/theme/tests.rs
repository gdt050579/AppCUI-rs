use AppCUIProcMacro::*;

#[test]
fn check_theme_update() {
    #[CustomControl(overwrite:OnThemeChanged+OnPaint, internal: true)]
    struct TestControl {
        attr: CharAttribute,
    }
    impl TestControl {
        fn new() -> Self {
            let mut obj = Self {
                base: ControlBase::new(Layout::new("l:1,r:1,t:1,b:1"), true),
                attr: CharAttribute::default(),
            };
            obj.attr = obj.theme().window.normal;
            obj
        }
    }
    impl OnPaint for TestControl {
        fn on_paint(&self, surface: &mut crate::prelude::Surface, _theme: &Theme) {
            surface.clear(Character::with_attributes('X', self.attr));
        }
    }
    impl OnThemeChanged for TestControl {
        fn on_theme_changed(&mut self, theme: &Theme) {
            self.attr = theme.window.normal;
        }
    }

    #[Window(events = CommandBarEvents, internal: true, commands: ChangeTheme)]
    struct TestWindow {}
    impl TestWindow {
        fn new() -> Self {
            let mut w = Self { base: window!("Test,l:2,t:2,b:2,r:2") };
            w.add(TestControl::new());
            w
        }
    }
    impl CommandBarEvents for TestWindow {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Change Theme", testwindow::Commands::ChangeTheme);
        }

        fn on_event(&mut self, command_id: testwindow::Commands) {
            if command_id == testwindow::Commands::ChangeTheme {
                let mut theme = Theme::new(Themes::Default);
                // make the window dark greem
                theme.window.normal = CharAttribute::new(Color::White, Color::DarkGreen, CharFlags::None);
                App::set_theme(theme);
            }
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xFAE7599256AB44D1) 
        Key.Pressed(F1)
        Paint('Theme changed to green')   
        CheckHash(0xFD09CB2F64B6B15) 
    ";
    let mut a = App::debug(60, 10, script).command_bar().build().unwrap();
    a.add_window(TestWindow::new());
    a.run();
}

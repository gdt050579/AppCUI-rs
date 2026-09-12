use appcui::{
    backend,
    prelude::*,
    ui::markdown_composer::{Flags, List, ListFlags},
};

#[Window(events = MarkdownComposerEvents)]
struct MyWin {}
impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'Test',a:c,w:40,h:20,flags:Sizeable"),
        };
        //let mc = MarkdownComposer::new(layout!("x:0,y:0,w:100%,h:100%"), Flags::None);
        // let mut mc = MarkdownComposer::from("**Salut** si _bine_ ai venit! Scrie-mi pe contact@firma.ro sau viziteaza https://example.com/pagina-foarte-lunga pentru detalii. Vezi si www.test.ro 😀 sau trimite pe _alt.user@mail.com._ Cuvantsuperlungfaraspatiicaretrebuierupt aici. **Link in bold www.nuidevine.ro ramane bold** gata.", layout!("x:0,y:0,w:100%,h:100%"), Flags::None);
        let mut mc = markdown_composer!("'**Salut** si _bine_ ai venit! Scrie-mi pe contact@firma.ro sau viziteaza https://example.com/pagina-foarte-lunga pentru detalii. Vezi si www.test.ro 😀 sau trimite pe _alt.user@mail.com._ Cuvantsuperlungfaraspatiicaretrebuierupt aici. **Link in bold www.nuidevine.ro ramane bold** gata.', x:0,y:0,w:100%,h:100%");
       
        mc.add_emoji_list(':');

        mc.add_list(
            '@',
            &["apple", "apricot", "banana", "blueberry", "cherry", "grape", "grapefruit", "mango"],
            ListFlags::RemoveTrigger,
        );
        
        win.add(mc);
        win
    }
}

impl Mar

fn main() -> Result<(), appcui::system::Error> {
    #[cfg(target_os = "windows")]
    { App::new().backend(backend::Type::WindowsVT).color_schema(false).window(MyWin::new).run() }
    #[cfg(not(target_os = "windows"))]
    { App::new().color_schema(false).window(MyWin::new).run() }
}
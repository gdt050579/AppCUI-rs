use appcui_proc_macro::*;
use super::structures::*;
use crate::{controlattr, graphics::*, ui::common::ControlCharAttributesState};

pub(crate) fn new() -> Theme {
    Theme {
        accordion: AccordionTheme {
            text: controlattr!("black,gray", "w,gray", "black,silver", "gray,?", "black,white"),
            hotkey: controlattr!("w,gray", "black,gray", "black,silver", "gray,?", "black,white"),
        },
        desktop: DesktopTheme {
            character: char!("' ',gray,black"),
        },
        text: TextTheme {
            normal: charattr!("black"),
            hot_key: charattr!("white"),
            inactive: charattr!("silver"),
            error: charattr!("black"),
            warning: charattr!("black"),
            hovered: charattr!("white"),
            focused: charattr!("black"),
            highlighted: charattr!("white"),
            enphasized_1: charattr!("white"),
            enphasized_2: charattr!("white"),
            enphasized_3: charattr!("white"),
        },
        symbol: SymbolTheme {
            inactive: charattr!("gray"),
            hovered: charattr!("black,white"),
            pressed: charattr!("black,silver"),
            checked: charattr!("black"),
            unchecked: charattr!("black"),
            unknown: charattr!("black"),
            arrows: charattr!("black"),
            close: charattr!("black"),
            maximized: charattr!("black"),
            resize: charattr!("black"),
        },
        tooltip: ToolTipTheme {
            text: charattr!("black,white"),
            arrow: charattr!("white,black"),
        },
        menu: MenuTheme {
            text: controlattr!("black,w", "black,w", "black,silver", "gray,w", "black,silver"),
            hotkey: controlattr!("gray,w,flags: Underline", "black,w", "black,silver", "gray,w", "black,silver"),
            shortcut: controlattr!("gray,w,flags: Underline", "black,w", "black,silver", "gray,w", "black,silver"),
            symbol: controlattr!("gray,w,flags: Underline", "black,w", "black,silver", "gray,w", "black,silver"),
        },
        parent_menu: MenuTheme {
            text: controlattr!("black,silver", "black,silver", "black,gray", "gray,silver", "w,gray"),
            hotkey: controlattr!("black,silver", "black,silver", "black,gray", "gray,silver", "w,gray"),
            shortcut: controlattr!("black,silver", "black,silver", "black,gray", "gray,silver", "w,gray"),
            symbol: controlattr!("black,silver", "black,silver", "black,gray", "gray,silver", "w,gray"),
        },
        window: WindowTheme {
            normal: charattr!("black,gray"),
            inactive: charattr!("black,black"),
            error: charattr!("black,gray"),
            warning: charattr!("black,gray"),
            info: charattr!("black,gray"),
            bar: WindowBar {
                focus: charattr!("black,w"),
                normal: charattr!("black,gray"),
                resizing: charattr!("black,silver"),
                close_button: charattr!("red,transparent"),
                maximize_button: charattr!("black,transparent"),
                tag: charattr!("silver,transparent"),
                hotkey: charattr!("dr,transparent"),                
            },            
        },
        border: controlattr!("black", "black", "black", "gray", "black,silver"),
        lines: controlattr!("black", "black", "w", "gray", "black,silver"),
        button: ButtonTheme {
            text: controlattr!("black,silver", "black,w", "black,w", "gray,black", "black,silver"),
            hotkey: controlattr!("white,silver", "black,w", "black,w", "gray,black", "white,silver"),
            shadow: charattr!("black"),
            light: charattr!("silver"),
        },
        tab: TabTheme {
            text: controlattr!("black,gray", "w,gray", "black,white", "gray,?", "black,silver"),
            hotkey: controlattr!("w,gray", "black,gray", "black,white", "gray,?", "w,silver"),
        },
        scrollbar: ScrollBarTheme {
            arrow: controlattr!("black,silver", "black,silver", "w,silver", "gray", "black,white"),
            bar: controlattr!("black,silver", "black,silver", "w,silver", "gray", "w,silver"),
            position: controlattr!("black,silver", "black,silver", "w,silver", "gray", "black,white"),
        },
        searchbar: SearchBarTheme {
            normal: charattr!("silver,black"),
            focused: charattr!("w,black"),
            count: charattr!("gray,black"),
        },
        editor: controlattr!("silver,black", "w,black", "w,black", "gray", "black,silver"),
        list_current_item: ListCurentItemTheme {
            focus: charattr!("black,w"),
            over_inactive: charattr!("gray,w"),
            over_selection: charattr!("silver,w"),
            normal: charattr!("y"),
            selected: charattr!("?,black"),
            icon: charattr!("white"),
        },
        header: HeaderTheme {
            text: controlattr!("black,silver", "black,silver", "black,white", "gray", "black,silver"),
            hotkey: controlattr!("w,silver,flags: Underline", "w,silver,flags: Underline", "w,silver,flags: Underline", "gray", "w,silver,flags: Underline"), 
            symbol: controlattr!("black,silver", "black,silver", "black,white", "gray", "black,silver"),
        },
        toggle_button: ToggleButtonTheme {
            selected: controlattr!("black", "black,silver", "black,w", "gray", "w,black"),
            unselected: controlattr!("silver", "silver,black", "black,w", "gray", "w,black"),
        },
        markdown: MarkdownTheme {
            text: charattr!("black"),
            bold: charattr!("black, flags: Bold"),
            italic: charattr!("black, flags: Italic"),
            link: charattr!("white, gray, flags: Underline"),
            code: charattr!("red, black"),
            code_block: charattr!("red, black"),
            h1: charattr!("darkred, gray"),
            h2: charattr!("darkred, gray"),
            h3: charattr!("darkred, gray"),
            ordered_list: charattr!("black"),
            unordered_list: charattr!("black"),
            table: charattr!("black"),
            table_header: charattr!("black, flags: Bold")
        },               
        progressbar: ProgressBarTheme {
            background: Color::Black,
            progress: Color::Gray,
            text: Color::White,
        },
    }
}

use super::structures::*;
use crate::{controlattr, graphics::*, ui::common::ControlCharAttributesState};
use appcui_proc_macro::*;

pub(crate) fn new() -> Theme {
    Theme {
        accordion: AccordionTheme {
            text: controlattr!("black,white", "w,white", "black,silver", "gray,?", "black,aqua"),
            hotkey: controlattr!("y,white", "dr,white", "dr,silver", "gray,?", "dr,aqua"),
        },
        desktop: DesktopTheme {
            character: char!("Block50,white,black"),
        },
        text: TextTheme {
            normal: charattr!("black"),
            hot_key: charattr!("dr"),
            inactive: charattr!("gray"),
            error: charattr!("r"),
            warning: charattr!("olive"),
            hovered: charattr!("dg"),
            focused: charattr!("black"),
            highlighted: charattr!("db"),
            enphasized_1: charattr!("dr"),
            enphasized_2: charattr!("dg"),
            enphasized_3: charattr!("olive"),
        },
        symbol: SymbolTheme {
            inactive: charattr!("gray"),
            hovered: charattr!("black,yellow"),
            pressed: charattr!("black,silver"),
            checked: charattr!("dg"),
            unchecked: charattr!("dr"),
            unknown: charattr!("olive"),
            arrows: charattr!("db"),
            close: charattr!("dr"),
            maximized: charattr!("black"),
            resize: charattr!("db"),
        },
        tooltip: ToolTipTheme {
            text: charattr!("black,aqua"),
            arrow: charattr!("green,black"),
        },
        menu: MenuTheme {
            text: controlattr!("black,white", "black,white", "black,silver", "gray,white", "y,magenta"),
            hotkey: controlattr!("dr,white", "dr,white", "dr,silver", "gray,white", "w,magenta"),
            shortcut: controlattr!("dr,white", "dr,white", "dr,silver", "gray,white", "w,magenta"),
            symbol: controlattr!("dg,white", "dg,white", "magenta,silver", "gray,white", "w,magenta"),
        },
        parent_menu: MenuTheme {
            text: controlattr!("black,silver", "black,silver", "black,gray", "gray,silver", "y,gray"),
            hotkey: controlattr!("dr,silver", "dr,silver", "dr,gray", "gray,silver", "w,gray"),
            shortcut: controlattr!("dr,silver", "dr,silver", "dr,gray", "gray,silver", "w,gray"),
            symbol: controlattr!("dg,silver", "dg,silver", "magenta,gray", "gray,silver", "w,gray"),
        },
        window: WindowTheme {
            normal: charattr!("black,white"),
            inactive: charattr!("black,silver"),
            error: charattr!("black,dr"),
            warning: charattr!("black,olive"),
            info: charattr!("black,dg"),
            bar: WindowBar {
                focus: charattr!("w,b"),
                normal: charattr!("w,gray"),
                resizing: charattr!("y,magenta"),
                close_button: charattr!("red,transparent"),
                maximize_button: charattr!("w,transparent"),
                tag: charattr!("green,transparent"),
                hotkey: charattr!("aqua,transparent"),                
            }            
        },
        border: controlattr!("black", "black", "black", "gray", "y,magenta"),
        lines: controlattr!("dg", "dg", "y", "gray", "y,magenta"),
        button: ButtonTheme {
            text: controlattr!("black,silver", "black,gray", "black,y", "gray,black", "black,olive"),
            hotkey: controlattr!("magenta,silver", "dr,gray", "magenta,y", "gray,black", "dr,olive"),
            shadow: charattr!("black"),
            light: charattr!("white"),
        },
        tab: TabTheme {
            text: controlattr!("black,white", "black,silver", "black,gray", "gray,?", "black,silver"),
            hotkey: controlattr!("dr,white", "dr,silver", "dr,gray", "gray,?", "dr,silver"),
        },
        scrollbar: ScrollBarTheme {
            arrow: controlattr!("w,silver", "w,gray", "y,gray", "gray", "w,black"),
            bar: controlattr!("w,silver", "w,gray", "y,gray", "gray", "w,black"),
            position: controlattr!("w,silver", "w,gray", "y,gray", "gray", "w,black"),
        },
        searchbar: SearchBarTheme {
            normal: charattr!("white,silver"),
            focused: charattr!("black,silver"),
            count: charattr!("gray,silver"),
        },
        editor: controlattr!("silver,black", "w,black", "y,black", "gray", "w,magenta"),
        list_current_item: ListCurentItemTheme {
            focus: charattr!("black,silver"),
            over_inactive: charattr!("gray,w"),
            over_selection: charattr!("red,w"),
            normal: charattr!("y"),
            selected: charattr!("?,gray"),
            icon: charattr!("gray"),
        },
        header: HeaderTheme {
            text: controlattr!("silver,magenta", "w,magenta", "dr,silver", "gray", "w,pink"),
            hotkey: controlattr!("y,magenta", "y,magenta", "r,silver", "gray", "y,pink"),
            symbol: controlattr!("silver,magenta", "w,magenta", "r,silver", "gray", "y,pink"),
        },
        toggle_button: ToggleButtonTheme {
            selected: controlattr!("aqua", "dr,w", "dr,y", "gray", "w,black"),
            unselected: controlattr!("silver", "black,w", "black,y", "gray", "w,black"),
        },
        markdown: MarkdownTheme {
            text: charattr!("black"),
            bold: charattr!("red, flags: Bold"),
            italic: charattr!("magenta, flags: Italic"),
            link: charattr!("aqua, flags: Underline"),
            code: charattr!("silver, black"),
            h1: charattr!("white"),
            h2: charattr!("y"),
            h3: charattr!("g"),
            code_block: charattr!("silver, black"),
            ordered_list: charattr!("black"),
            unordered_list: charattr!("black"),
            table: charattr!("black"),
            table_header: charattr!("black, flags: Bold"),
        },
        progressbar: ProgressBarTheme {
            background: Color::Black,
            progress: Color::Gray,
            text: Color::White,
        },
    }
}

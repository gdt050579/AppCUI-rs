use super::structures::*;
use crate::{controlattr, graphics::*, ui::common::ControlCharAttributesState};
use appcui_proc_macro::*;

pub(crate) fn new() -> Theme {
    Theme {
        accordion: AccordionTheme {
            text: controlattr!("black,gray", "w,gray", "black,silver", "gray,?", "black,white"),
            hotkey: controlattr!("y,gray", "dr,gray", "dr,silver", "gray,?", "dr,w"),
        },
        desktop: DesktopTheme {
            character: char!("Block50,gray,black"),
        },
        text: TextTheme {
            normal: charattr!("silver"),
            hot_key: charattr!("aqua"),
            inactive: charattr!("gray"),
            error: charattr!("red"),
            warning: charattr!("olive"),
            hovered: charattr!("y"),
            focused: charattr!("w"),
            highlighted: charattr!("y"),
            enphasized_1: charattr!("aqua"),
            enphasized_2: charattr!("green"),
            enphasized_3: charattr!("pink"),
        },
        symbol: SymbolTheme {
            inactive: charattr!("gray"),
            hovered: charattr!("black,yellow"),
            pressed: charattr!("black,silver"),
            checked: charattr!("green"),
            unchecked: charattr!("red"),
            unknown: charattr!("olive"),
            arrows: charattr!("aqua"),
            close: charattr!("red"),
            maximized: charattr!("aqua"),
            resize: charattr!("aqua"),
        },
        tooltip: ToolTipTheme {
            text: charattr!("black,aqua"),
            arrow: charattr!("green,black"),
        },
        menu: MenuTheme {
            text: controlattr!("black,w", "black,w", "black,silver", "gray,w", "y,magenta"),
            hotkey: controlattr!("dr,w", "dr,w", "dr,silver", "gray,w", "w,magenta"),
            shortcut: controlattr!("dr,w", "dr,w", "dr,silver", "gray,w", "w,magenta"),
            symbol: controlattr!("dg,w", "dg,w", "magenta,silver", "gray,w", "w,magenta"),
        },
        parent_menu: MenuTheme {
            text: controlattr!("black,silver", "black,silver", "black,gray", "gray,silver", "y,gray"),
            hotkey: controlattr!("dr,silver", "dr,silver", "dr,gray", "gray,silver", "w,gray"),
            shortcut: controlattr!("dr,silver", "dr,silver", "dr,gray", "gray,silver", "w,gray"),
            symbol: controlattr!("dg,silver", "dg,silver", "magenta,gray", "gray,silver", "w,gray"),
        },
        window: WindowTheme {
            normal: charattr!("black,db"),
            inactive: charattr!("black,black"),
            error: charattr!("black,dr"),
            warning: charattr!("black,olive"),
            info: charattr!("black,dg"),
            bar: WindowBar {
                focus: charattr!("black,w"),
                normal: charattr!("black,gray"),
                resizing: charattr!("y,magenta"),
                close_button: charattr!("red,transparent"),
                maximize_button: charattr!("black,transparent"),
                tag: charattr!("dg,transparent"),
                hotkey: charattr!("dr,transparent"),
            },
        },
        border: controlattr!("silver", "w", "y", "gray", "y,magenta"),
        lines: controlattr!("dg", "dg", "y", "gray", "y,magenta"),
        button: ButtonTheme {
            regular: RegularButtonTheme {
                text: controlattr!("black,gray", "black,w", "black,y", "gray,black", "black,olive"),
                hotkey: controlattr!("black,gray", "black,w", "black,y", "gray,black", "black,olive"),
                shadow: charattr!("black"),
            },
            bevel: BevelButtonTheme {
                text: controlattr!("black,gray", "black,w", "black,y", "gray,black", "black,olive"),
                hotkey: controlattr!("black,gray", "black,w", "black,y", "gray,black", "black,olive"),
                dark_margin: charattr!("black"),
                light_margin: charattr!("white"),
            },
        },
        tab: TabTheme {
            text: controlattr!("black,gray", "w,gray", "black,silver", "gray,?", "w,b"),
            hotkey: controlattr!("y,gray", "dr,gray", "dr,silver", "gray,?", "y,b"),
        },
        scrollbar: ScrollBarTheme {
            arrow: controlattr!("w,db", "w,teal", "y,db", "gray", "w,teal"),
            bar: controlattr!("w,db", "w,teal", "y,db", "gray", "w,teal"),
            position: controlattr!("silver,db", "g,teal", "y,db", "gray", "g,teal"),
        },
        searchbar: SearchBarTheme {
            normal: charattr!("silver,dr"),
            focused: charattr!("w,dr"),
            count: charattr!("gray,dr"),
        },
        editor: controlattr!("silver,black", "w,black", "y,black", "gray", "w,magenta"),
        list_current_item: ListCurentItemTheme {
            focus: charattr!("black,w"),
            over_inactive: charattr!("gray,w"),
            over_selection: charattr!("red,w"),
            normal: charattr!("y"),
            selected: charattr!("?,black"),
            icon: charattr!("aqua"),
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
            text: charattr!("silver"),
            bold: charattr!("red, flags: Bold"),
            italic: charattr!("magenta, flags: Italic"),
            link: charattr!("aqua, flags: Underline"),
            code: charattr!("silver, black"),
            h1: charattr!("white"),
            h2: charattr!("y"),
            h3: charattr!("g"),
            code_block: charattr!("silver, black"),
            ordered_list: charattr!("silver"),
            unordered_list: charattr!("silver"),
            table: charattr!("silver"),
            table_header: charattr!("silver, flags: Bold"),
        },

        progressbar: ProgressBarTheme {
            background: Color::Black,
            progress: Color::Teal,
            text: Color::White,
        },
    }
}

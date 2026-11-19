use super::structures::*;
use crate::{controlattr, graphics::*, ui::common::ControlCharAttributesState};
use appcui_proc_macro::*;

// A high-contrast theme: black surfaces with white text, and yellow / aqua used for
// everything that has to stand out (focus, hot keys, hover, selection).
pub(crate) fn new() -> Theme {
    Theme {
        accordion: AccordionTheme {
            text: controlattr!("black,silver", "black,y", "black,aqua", "gray,?", "black,white"),
            hotkey: controlattr!("dr,silver", "dr,y", "dr,aqua", "gray,?", "dr,white"),
        },
        desktop: DesktopTheme {
            character: char!("Block25,gray,black"),
        },
        text: TextTheme {
            normal: charattr!("white"),
            hot_key: charattr!("y"),
            inactive: charattr!("gray"),
            error: charattr!("red"),
            warning: charattr!("y"),
            hovered: charattr!("aqua"),
            focused: charattr!("w"),
            highlighted: charattr!("y"),
            enphasized_1: charattr!("aqua"),
            enphasized_2: charattr!("green"),
            enphasized_3: charattr!("pink"),
        },
        symbol: SymbolTheme {
            inactive: charattr!("gray"),
            hovered: charattr!("black,y"),
            pressed: charattr!("black,aqua"),
            checked: charattr!("green"),
            unchecked: charattr!("red"),
            unknown: charattr!("y"),
            arrows: charattr!("aqua"),
            close: charattr!("red"),
            maximized: charattr!("aqua"),
            resize: charattr!("aqua"),
        },
        tooltip: ToolTipTheme {
            text: charattr!("black,y"),
            arrow: charattr!("y,black"),
        },
        menu: MenuTheme {
            text: controlattr!("black,white", "black,y", "black,aqua", "teal,white", "black,y"),
            hotkey: controlattr!("dr,white", "dr,y", "dr,aqua", "teal,white", "dr,y"),
            shortcut: controlattr!("db,white", "db,y", "db,aqua", "teal,white", "db,y"),
            symbol: controlattr!("db,white", "db,y", "db,aqua", "teal,white", "db,y"),
        },
        parent_menu: MenuTheme {
            text: controlattr!("black,silver", "black,silver", "black,aqua", "db,silver", "black,y"),
            hotkey: controlattr!("dr,silver", "dr,silver", "dr,aqua", "db,silver", "dr,y"),
            shortcut: controlattr!("db,silver", "db,silver", "db,aqua", "db,silver", "db,y"),
            symbol: controlattr!("db,silver", "db,silver", "db,aqua", "db,silver", "db,y"),
        },
        window: WindowTheme {
            normal: charattr!("white,black"),
            inactive: charattr!("gray,black"),
            error: charattr!("white,dr"),
            warning: charattr!("black,y"),
            info: charattr!("white,db"),
            bar: WindowBar {
                focus: charattr!("black,y"),
                normal: charattr!("black,silver"),
                resizing: charattr!("black,aqua"),
                close_button: charattr!("dr,transparent"),
                maximize_button: charattr!("black,transparent"),
                tag: charattr!("db,transparent"),
                hotkey: charattr!("dr,transparent"),
            },
        },
        border: controlattr!("white", "y", "aqua", "gray", "black,y"),
        lines: controlattr!("white", "white", "y", "gray", "black,y"),
        button: ButtonTheme {
            regular: RegularButtonTheme {
                text: controlattr!("black,silver", "black,y", "black,aqua", "gray,black", "black,white"),
                hotkey: controlattr!("dr,silver", "dr,y", "dr,aqua", "gray,black", "dr,white"),
                shadow: charattr!("gray"),
            },
            bevel: BevelButtonTheme {
                text: controlattr!("white", "y", "aqua", "gray", "white"),
                hotkey: controlattr!("y,flags: underline", "w,flags: underline", "w,flags: underline", "gray,flags: underline", "y,flags: underline"),
                dark_margin: charattr!("gray"),
                light_margin: charattr!("white"),
            },
        },
        tab: TabTheme {
            text: controlattr!("black,silver", "black,white", "black,aqua", "gray,?", "black,y"),
            hotkey: controlattr!("dr,silver", "dr,white", "dr,aqua", "gray,?", "dr,y"),
        },
        scrollbar: ScrollBarTheme {
            arrow: controlattr!("black,silver", "black,y", "black,aqua", "gray", "black,white"),
            bar: controlattr!("silver,black", "y,black", "aqua,black", "gray", "w,black"),
            position: controlattr!("black,silver", "black,y", "black,aqua", "gray", "black,white"),
        },
        searchbar: SearchBarTheme {
            normal: charattr!("black,silver"),
            focused: charattr!("black,y"),
            count: charattr!("db,silver"),
        },
        editor: controlattr!("silver,black", "white,black", "y,black", "gray", "black,aqua"),
        list_current_item: ListCurentItemTheme {
            focus: charattr!("black,y"),
            over_inactive: charattr!("black,silver"),
            over_selection: charattr!("dr,y"),
            normal: charattr!("y"),
            selected: charattr!("aqua"),
            icon: charattr!("aqua"),
        },
        header: HeaderTheme {
            text: controlattr!("black,silver", "black,y", "black,aqua", "gray", "black,white"),
            hotkey: controlattr!(
                "dr,silver,flags: Underline",
                "dr,y,flags: Underline",
                "dr,aqua,flags: Underline",
                "gray",
                "dr,white,flags: Underline"
            ),
            symbol: controlattr!("black,silver", "black,y", "black,aqua", "gray", "black,white"),
        },
        toggle_button: ToggleButtonTheme {
            selected: controlattr!("black,y", "black,y", "black,aqua", "gray", "black,y"),
            unselected: controlattr!("white", "black,white", "black,aqua", "gray", "black,y"),
        },
        markdown: MarkdownTheme {
            text: charattr!("white"),
            bold: charattr!("white, flags: Bold"),
            italic: charattr!("white, flags: Italic"),
            link: charattr!("aqua, flags: Underline"),
            code: charattr!("y, black"),
            code_block: charattr!("y, black"),
            h1: charattr!("y"),
            h2: charattr!("aqua"),
            h3: charattr!("green"),
            ordered_list: charattr!("white"),
            unordered_list: charattr!("white"),
            table: charattr!("white"),
            table_header: charattr!("white, flags: Bold"),
        },
        progressbar: ProgressBarTheme {
            background: Color::Black,
            progress: Color::Teal,
            text: Color::White,
        },
        hyperlink: controlattr!("aqua,flags:Underline", "y,flags:Underline", "w,flags:Underline", "gray", "black,y,flags:Underline"),
        slider: SliderTheme {
            before_line: charattr!("aqua"),
            after_line: charattr!("gray"),
            marker: controlattr!("aqua", "y", "y", "gray", "y"),
            marker_border: controlattr!("aqua", "y", "y", "gray", "y"),
            cap: charattr!("white"),
        },
        chart: ChartTheme {
            background: charattr!("white,black"),
            axis: charattr!("white,black"),
            label: charattr!("white,black"),
            grid: charattr!("gray,black"),
            bar: [charattr!("aqua,black"), charattr!("y,black"), charattr!("green,black"), charattr!("pink,black")],
            inactive: charattr!("gray,black"),
            selection_border: charattr!("y,black"),
            hover: charattr!("black,y"),
        },
    }
}

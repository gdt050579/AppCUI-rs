use super::Alignament;
use super::Coordonate;
use super::Size;
use super::Parameter;
use crate::utils::KeyValueParser;
use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits:16)]
pub enum LayoutUsedParams {
    X = 0x0001,
    Y = 0x0002,
    LEFT = 0x0004,
    RIGHT = 0x0008,
    TOP = 0x0010,
    BOTTOM = 0x0020,
    WIDTH = 0x0040,
    HEIGHT = 0x0080,
    ALIGN = 0x0100,
    DOCK = 0x0200,
}
pub(super) struct LayoutInformation {
    pub x: Coordonate,
    pub y: Coordonate,
    pub width: Size,
    pub height: Size,
    pub a_left: Coordonate,
    pub a_right: Coordonate,
    pub a_top: Coordonate,
    pub a_bottom: Coordonate,
    pub params: LayoutUsedParams,
    pub align: Alignament,
    pub dock: Alignament,
}
impl Default for LayoutInformation {
    #[inline]
    fn default() -> Self {
        LayoutInformation {
            x: Coordonate::Absolute(0),
            y: Coordonate::Absolute(0),
            width: Size::Absolute(0),
            height: Size::Absolute(0),
            a_left: Coordonate::Absolute(0),
            a_right: Coordonate::Absolute(0),
            a_top: Coordonate::Absolute(0),
            a_bottom: Coordonate::Absolute(0),
            params: LayoutUsedParams::None,
            align: Alignament::TopLeft,
            dock: Alignament::TopLeft,
        }    
    }
}
impl LayoutInformation {

    pub (super) fn new(format: &str) -> LayoutInformation {
        let mut inf = LayoutInformation::default();
        let mut parser = KeyValueParser::new(format);

        while let Some(p) = parser.next() {
            if let Some(param) = Parameter::new(p.key_hash)
            {

            } else {
                panic!("Unknwon layout parameter: {}",p.key);
            }
        }
        //     AppCUI::Utils::KeyValueParser parser;
        //     ControlLayout::Type layoutType;

        
        //     // reset inf
        
        //     ASSERT(parser.Parse(layout), parser.GetErrorName().data());
        //     for (auto idx = 0U; idx < parser.GetCount(); idx++)
        //     {
        //         const auto& item = parser[idx];
        //         if (ControlLayout::HashToType(item.Key.hash, layoutType) == false)
        //         {
        //             error.Set("Unknwon layout item: ");
        //             error.Add((const char*) item.Key.data, item.Key.dataSize);
        //             ASSERT(false, error.GetText());
        //         }
        //         const bool isNumericalValue =
        //               (item.Value.type == KeyValuePair::Type::Number) || (item.Value.type == KeyValuePair::Type::Percentage);
        //         int32 value = item.Value.number;
        //         valueType   = (item.Value.type == KeyValuePair::Type::Percentage) ? LayoutValueType::Percentage
        //                                                                           : LayoutValueType::CharacterOffset;
        //         switch (layoutType)
        //         {
        //         case ControlLayout::Type::X:
        //             ASSERT(isNumericalValue, "Field 'X' must be a valid number or percentage");
        //             SET_LAYOUT_INFO(LAYOUT_FLAG_X, x);
        //             break;
        //         case ControlLayout::Type::Y:
        //             ASSERT(isNumericalValue, "Field 'XY' must be a valid number or percentage");
        //             SET_LAYOUT_INFO(LAYOUT_FLAG_Y, y);
        //             break;
        //         case ControlLayout::Type::Left:
        //             ASSERT(isNumericalValue, "Field 'Left' must be a valid number or percentage");
        //             SET_LAYOUT_INFO(LAYOUT_FLAG_LEFT, a_left);
        //             break;
        //         case ControlLayout::Type::Right:
        //             ASSERT(isNumericalValue, "Field 'Right' must be a valid number or percentage");
        //             SET_LAYOUT_INFO(LAYOUT_FLAG_RIGHT, a_right);
        //             break;
        //         case ControlLayout::Type::Top:
        //             ASSERT(isNumericalValue, "Field 'Top' must be a valid number or percentage");
        //             SET_LAYOUT_INFO(LAYOUT_FLAG_TOP, a_top);
        //             break;
        //         case ControlLayout::Type::Bottom:
        //             ASSERT(isNumericalValue, "Field 'Bottom' must be a valid number or percentage");
        //             SET_LAYOUT_INFO(LAYOUT_FLAG_BOTTOM, a_bottom);
        //             break;
        //         case ControlLayout::Type::Width:
        //             ASSERT(isNumericalValue, "Field 'Width' must be a valid number or percentage");
        //             SET_LAYOUT_INFO(LAYOUT_FLAG_WIDTH, width);
        //             break;
        //         case ControlLayout::Type::Height:
        //             ASSERT(isNumericalValue, "Field 'Height' must be a valid number or percentage");
        //             SET_LAYOUT_INFO(LAYOUT_FLAG_HEIGHT, height);
        //             break;
        //         case ControlLayout::Type::Align:
        //             ASSERT(item.Value.type == KeyValuePair::Type::String, "Align parameter requires a string value");
        //             ASSERT(ControlAlignament::HashToType(item.Value.hash, inf.align), "Unknwon align value");
        //             inf.flags |= LAYOUT_FLAG_ALIGN;
        //             break;
        //         case ControlLayout::Type::Dock:
        //             ASSERT(item.Value.type == KeyValuePair::Type::String, "Dock parameter requires a string value");
        //             ASSERT(ControlAlignament::HashToType(item.Value.hash, inf.dock), "Unknwon dock value");
        //             inf.flags |= LAYOUT_FLAG_DOCK;
        //             break;
        //         default:
        //             error.Set("Internal error - fail to parse item: ");
        //             error.Add((const char*) item.Key.data, item.Key.dataSize);
        //             ASSERT(false, error.GetText());
        //             return false;
        //         }
        //     }
        //     // all good
        //     return true;
        // }
        

        return inf;
    }
}

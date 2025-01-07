#[cfg(feature = "UnityEngine+TextCore+Text+MarkupTag")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkupTag {
    #[default]
    A = 65i32,
    ACTION = -1827519330i32,
    ALIGN = 75138797i32,
    ALLCAPS = 218273952i32,
    ALPHA = 75165780i32,
    ANGLE = 75347905i32,
    ANIM = 2283339i32,
    BLACK = 81074727i32,
    BLUE = 2457214i32,
    BOLD = 66i32,
    BR = 2256i32,
    CENTER = -1591113269i32,
    CHARACTER_SPACE = -1584382009i32,
    CLASS = 82115566i32,
    COLOR = 81999901i32,
    CR = 2289i32,
    DEFAULT = -620974005i32,
    EM = 2216i32,
    FALSE = 85422813i32,
    FAMILYNAME = 704251153i32,
    FLUSH = 85552164i32,
    FONT = 2586451i32,
    FONT_WEIGHT = -1889896162i32,
    FRAC = 2598518i32,
    GRADIENT = -1999759898i32,
    GREEN = 87065851i32,
    HREF = 2535353i32,
    INDENT = -1514123076i32,
    INDEX = 84268030i32,
    INVALID = 1585415185i32,
    ITALIC = 73i32,
    JUSTIFIED = 817091359i32,
    LEFT = 2660507i32,
    LIGA = 2655971i32,
    LINE_HEIGHT = -799081892i32,
    LINE_INDENT = -844305121i32,
    LINK = 2656128i32,
    LOWERCASE = -1506899689i32,
    MARGIN = -1355614050i32,
    MARGIN_LEFT = -272933656i32,
    MARGIN_RIGHT = -447416589i32,
    MARK = 2699125i32,
    MATERIAL = 825491659i32,
    MINUS = 45i32,
    MINUS_EM = 46789i32,
    MINUS_PCT = 1567082i32,
    MINUS_PERCENTAGE = 1512i32,
    MINUS_PX = 47461i32,
    MONOSPACE = -1340221943i32,
    NAME = 2875623i32,
    NBSP = 2869039i32,
    NONE = 2857034i32,
    NORMAL = -1183493901i32,
    NOTDEF = 612146780i32,
    NO_BREAK = 2856657i32,
    NO_PARSE = -408011596i32,
    ORANGE = -1108587920i32,
    PADDING = -2144568463i32,
    PAGE = 2808691i32,
    PCT = 85031i32,
    PERCENTAGE = 37i32,
    PLUS = 43i32,
    PLUS_EM = 49091i32,
    PLUS_PCT = 1634348i32,
    PLUS_PERCENTAGE = 1454i32,
    PLUS_PX = 49507i32,
    POSITION = 85420i32,
    PURPLE = -1250222130i32,
    PX = 2568i32,
    RED = 91635i32,
    REGULAR = 1291372090i32,
    RIGHT = 99937376i32,
    ROTATE = -1000007783i32,
    SCALE = 100553336i32,
    SHY = 92674i32,
    SIZE = 3061285i32,
    SLASH_A = 1614i32,
    SLASH_ACTION = -1187217679i32,
    SLASH_ALIGN = 1916026786i32,
    SLASH_ALLCAPS = -797437649i32,
    SLASH_BOLD = 1613i32,
    SLASH_CHARACTER_SPACE = -1394426712i32,
    SLASH_COLOR = 1909026194i32,
    SLASH_FONT = 57747708i32,
    SLASH_FONT_WEIGHT = -757976431i32,
    SLASH_FRAC = 57774681i32,
    SLASH_GRADIENT = -1854491959i32,
    SLASH_INDENT = -1496889389i32,
    SLASH_ITALIC = 1606i32,
    SLASH_LIGA = 57686604i32,
    SLASH_LINE_HEIGHT = 200452819i32,
    SLASH_LINE_INDENT = 93886352i32,
    SLASH_LINK = 57686191i32,
    SLASH_LOWERCASE = -1451284584i32,
    SLASH_MARGIN = -1649644303i32,
    SLASH_MARK = 57644506i32,
    SLASH_MATERIAL = -1100708252i32,
    SLASH_MONOSPACE = -1638865562i32,
    SLASH_NO_BREAK = 57477502i32,
    SLASH_NO_PARSE = -294095813i32,
    SLASH_PAGE = 58683868i32,
    SLASH_POSITION = 1777699i32,
    SLASH_ROTATE = -764695562i32,
    SLASH_SCALE = 1928413879i32,
    SLASH_SIZE = 58429962i32,
    SLASH_SMALLCAPS = 199921873i32,
    SLASH_SPACE = 1927873067i32,
    SLASH_STRIKETHROUGH = 1628i32,
    SLASH_STYLE = 1927738392i32,
    SLASH_SUBSCRIPT = 1770219i32,
    SLASH_SUPERSCRIPT = 1770233i32,
    SLASH_TABLE = -979118220i32,
    SLASH_TD = 193346074i32,
    SLASH_TH = 193346070i32,
    SLASH_TR = 193346060i32,
    SLASH_UNDERLINE = 1626i32,
    SLASH_UPPERCASE = -582368199i32,
    SLASH_VERTICAL_OFFSET = -11107948i32,
    SLASH_WIDTH = 1923459625i32,
    SMALLCAPS = -766062114i32,
    SPACE = 100083556i32,
    SPRITE = -991527447i32,
    STRIKETHROUGH = 83i32,
    STYLE = 100252951i32,
    STYLENAME = -1207081936i32,
    SUBSCRIPT = 92132i32,
    SUPERSCRIPT = 92150i32,
    TABLE = 226476955i32,
    TD = 5862485i32,
    TH = 5862489i32,
    TINT = 2960519i32,
    TR = 5862467i32,
    TRUE = 2932022i32,
    UNDERLINE = 85i32,
    UPPERCASE = -305409418i32,
    VERTICAL_OFFSET = 1952379995i32,
    WHITE = 105680263i32,
    WIDTH = 105793766i32,
    YELLOW = -882444668i32,
    ZWJ = 99623i32,
    ZWSP = 3288238i32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+MarkupTag")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::Text::MarkupTag {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "MarkupTag";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::TextCore::Text::MarkupTag {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::TextCore::Text::MarkupTag {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::TextCore::Text::MarkupTag {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::TextCore::Text::MarkupTag {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}

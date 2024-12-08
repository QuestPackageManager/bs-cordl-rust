#[cfg(feature = "System+Globalization+NumberStyles")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberStyles {
    AllowCurrencySymbol = 256i32,
    AllowDecimalPoint = 32i32,
    AllowExponent = 128i32,
    AllowHexSpecifier = 512i32,
    AllowLeadingSign = 4i32,
    AllowLeadingWhite = 1i32,
    AllowParentheses = 16i32,
    AllowThousands = 64i32,
    AllowTrailingSign = 8i32,
    AllowTrailingWhite = 2i32,
    Any = 511i32,
    Currency = 383i32,
    Float = 167i32,
    HexNumber = 515i32,
    Integer = 7i32,
    None = 0i32,
    Number = 111i32,
}
#[cfg(feature = "System+Globalization+NumberStyles")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::NumberStyles =>
    "System.Globalization"."NumberStyles"
);

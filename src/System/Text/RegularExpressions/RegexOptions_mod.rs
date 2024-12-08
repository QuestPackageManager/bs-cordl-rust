#[cfg(feature = "System+Text+RegularExpressions+RegexOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegexOptions {
    Compiled = 8i32,
    CultureInvariant = 512i32,
    ECMAScript = 256i32,
    ExplicitCapture = 4i32,
    IgnoreCase = 1i32,
    IgnorePatternWhitespace = 32i32,
    Multiline = 2i32,
    None = 0i32,
    RightToLeft = 64i32,
    Singleline = 16i32,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Text::RegularExpressions::RegexOptions
    => "System.Text.RegularExpressions"."RegexOptions"
);

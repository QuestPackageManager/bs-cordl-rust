#[cfg(feature = "System+TermInfoNumbers")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TermInfoNumbers {
    #[default]
    BitImageEntwining = 31i32,
    BitImageType = 32i32,
    BufferCapacity = 16i32,
    Buttons = 30i32,
    Columns = 0i32,
    DotHorzSpacing = 18i32,
    DotVertSpacing = 17i32,
    InitTabs = 1i32,
    LabelHeight = 9i32,
    LabelWidth = 10i32,
    Last = 33i32,
    Lines = 2i32,
    LinesOfMemory = 3i32,
    MagicCookieGlitch = 4i32,
    MaxAttributes = 11i32,
    MaxColors = 13i32,
    MaxMicroAddress = 19i32,
    MaxMicroJump = 20i32,
    MaxPairs = 14i32,
    MaximumWindows = 12i32,
    MicroColSize = 21i32,
    MicroLineSize = 22i32,
    NoColorVideo = 15i32,
    NumLabels = 8i32,
    NumberOfPins = 23i32,
    OutputResChar = 24i32,
    OutputResHorzInch = 26i32,
    OutputResLine = 25i32,
    OutputResVertInch = 27i32,
    PaddingBaudRate = 5i32,
    PrintRate = 28i32,
    VirtualTerminal = 6i32,
    WideCharSize = 29i32,
    WidthStatusLine = 7i32,
}
#[cfg(feature = "System+TermInfoNumbers")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::TermInfoNumbers => "System"
    ."TermInfoNumbers"
);

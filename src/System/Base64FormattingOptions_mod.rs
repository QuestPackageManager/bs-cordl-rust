#[cfg(feature = "System+Base64FormattingOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base64FormattingOptions {
    InsertLineBreaks = 1i32,
    None = 0i32,
}
#[cfg(feature = "System+Base64FormattingOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Base64FormattingOptions => "System"
    ."Base64FormattingOptions"
);

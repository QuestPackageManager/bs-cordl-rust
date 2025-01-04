#[cfg(feature = "System+IO+MatchType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MatchType {
    #[default]
    Simple = 0i32,
    Win32 = 1i32,
}
#[cfg(feature = "System+IO+MatchType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::MatchType => "System.IO"."MatchType"
);

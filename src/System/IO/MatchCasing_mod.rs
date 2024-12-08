#[cfg(feature = "System+IO+MatchCasing")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchCasing {
    CaseInsensitive = 2i32,
    CaseSensitive = 1i32,
    PlatformDefault = 0i32,
}
#[cfg(feature = "System+IO+MatchCasing")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::MatchCasing => "System.IO"
    ."MatchCasing"
);

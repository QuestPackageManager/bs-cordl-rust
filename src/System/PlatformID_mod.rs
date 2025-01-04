#[cfg(feature = "System+PlatformID")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlatformID {
    #[default]
    MacOSX = 6i32,
    Unix = 4i32,
    Win32NT = 2i32,
    Win32S = 0i32,
    Win32Windows = 1i32,
    WinCE = 3i32,
    Xbox = 5i32,
}
#[cfg(feature = "System+PlatformID")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::PlatformID => "System"."PlatformID"
);

#[cfg(feature = "System+Diagnostics+ProcessWindowStyle")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessWindowStyle {
    Hidden = 1i32,
    Maximized = 3i32,
    Minimized = 2i32,
    Normal = 0i32,
}
#[cfg(feature = "System+Diagnostics+ProcessWindowStyle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::ProcessWindowStyle =>
    "System.Diagnostics"."ProcessWindowStyle"
);

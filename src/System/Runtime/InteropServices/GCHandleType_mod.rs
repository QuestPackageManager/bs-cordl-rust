#[cfg(feature = "System+Runtime+InteropServices+GCHandleType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GCHandleType {
    #[default]
    Normal = 2i32,
    Pinned = 3i32,
    Weak = 0i32,
    WeakTrackResurrection = 1i32,
}
#[cfg(feature = "System+Runtime+InteropServices+GCHandleType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::InteropServices::GCHandleType
    => "System.Runtime.InteropServices"."GCHandleType"
);

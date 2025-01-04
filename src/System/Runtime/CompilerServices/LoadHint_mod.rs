#[cfg(feature = "System+Runtime+CompilerServices+LoadHint")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LoadHint {
    #[default]
    Always = 1i32,
    Default = 0i32,
    Sometimes = 2i32,
}
#[cfg(feature = "System+Runtime+CompilerServices+LoadHint")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::CompilerServices::LoadHint =>
    "System.Runtime.CompilerServices"."LoadHint"
);

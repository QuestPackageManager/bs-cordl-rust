#[cfg(feature = "FakeReflectionDynamicObjectsState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FakeReflectionDynamicObjectsState {
    Disabled = 0i32,
    Enabled = 1i32,
}
#[cfg(feature = "FakeReflectionDynamicObjectsState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::FakeReflectionDynamicObjectsState => ""
    ."FakeReflectionDynamicObjectsState"
);

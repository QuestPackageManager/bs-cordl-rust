#[cfg(feature = "MockPlayerMovementType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MockPlayerMovementType {
    AI = 0i32,
    MirrorPlayer = 1i32,
    Recording = 2i32,
}
#[cfg(feature = "MockPlayerMovementType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for MockPlayerMovementType => ""."MockPlayerMovementType"
);

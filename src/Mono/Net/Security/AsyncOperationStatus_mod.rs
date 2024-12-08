#[cfg(feature = "Mono+Net+Security+AsyncOperationStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncOperationStatus {
    Complete = 3i32,
    Continue = 1i32,
    Initialize = 0i32,
    ReadDone = 2i32,
}
#[cfg(feature = "Mono+Net+Security+AsyncOperationStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Net::Security::AsyncOperationStatus =>
    "Mono.Net.Security"."AsyncOperationStatus"
);

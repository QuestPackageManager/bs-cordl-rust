#[cfg(feature = "UnityEngine+ProBuilder+LogOutput")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogOutput {
    Console = 1i32,
    File = 2i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+LogOutput")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::LogOutput =>
    "UnityEngine.ProBuilder"."LogOutput"
);

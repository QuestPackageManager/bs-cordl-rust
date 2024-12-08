#[cfg(feature = "Unity+Burst+CompilationPriority")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilationPriority {
    Asynchronous = 1i32,
    EagerCompilationAsynchronous = 3i32,
    EagerCompilationSynchronous = 0i32,
    ILPP = 2i32,
}
#[cfg(feature = "Unity+Burst+CompilationPriority")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::CompilationPriority =>
    "Unity.Burst"."CompilationPriority"
);

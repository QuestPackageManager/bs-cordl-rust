#[cfg(feature = "Unity+Burst+BurstTargetCpu")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BurstTargetCpu {
    #[default]
    ARMV7A_NEON32 = 8i32,
    ARMV8A_AARCH64 = 9i32,
    ARMV8A_AARCH64_HALFFP = 11i32,
    ARMV9A = 12i32,
    AVX = 5i32,
    AVX2 = 6i32,
    Auto = 0i32,
    THUMB2_NEON32 = 10i32,
    WASM32 = 7i32,
    X64_SSE2 = 3i32,
    X64_SSE4 = 4i32,
    X86_SSE2 = 1i32,
    X86_SSE4 = 2i32,
}
#[cfg(feature = "Unity+Burst+BurstTargetCpu")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstTargetCpu => "Unity.Burst"
    ."BurstTargetCpu"
);

#[cfg(feature = "Unity+Burst+NativeDumpFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NativeDumpFlags {
    #[default]
    All = 1021i32,
    Analysis = 64i32,
    Asm = 16i32,
    Function = 32i32,
    IL = 1i32,
    ILPre = 256i32,
    IR = 4i32,
    IROptimized = 8i32,
    IRPassAnalysis = 128i32,
    IRPerEntryPoint = 512i32,
    None = 0i32,
    Unused = 2i32,
}
#[cfg(feature = "Unity+Burst+NativeDumpFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::NativeDumpFlags => "Unity.Burst"
    ."NativeDumpFlags"
);

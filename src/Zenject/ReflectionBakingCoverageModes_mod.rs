#[cfg(feature = "Zenject+ReflectionBakingCoverageModes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReflectionBakingCoverageModes {
    #[default]
    FallbackToDirectReflection = 0i32,
    FallbackToDirectReflectionWithWarning = 2i32,
    NoCheckAssumeFullCoverage = 1i32,
}
#[cfg(feature = "Zenject+ReflectionBakingCoverageModes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ReflectionBakingCoverageModes =>
    "Zenject"."ReflectionBakingCoverageModes"
);

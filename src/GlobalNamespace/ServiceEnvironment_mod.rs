#[cfg(feature = "ServiceEnvironment")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceEnvironment {
    Development = 4i32,
    DevelopmentA = 7i32,
    DevelopmentB = 8i32,
    InternalPlayTest = 2i32,
    Production = 0i32,
    ProductionA = 5i32,
    ProductionB = 6i32,
    ProductionC = 9i32,
    ProductionQuest1 = 10i32,
    QATesting = 3i32,
    ReleaseCandidate = 1i32,
}
#[cfg(feature = "ServiceEnvironment")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for ServiceEnvironment => ""."ServiceEnvironment"
);

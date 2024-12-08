#[cfg(feature = "UnityEngine+Rendering+ReflectionProbeSortingCriteria")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflectionProbeSortingCriteria {
    Importance = 1i32,
    ImportanceThenSize = 3i32,
    None = 0i32,
    Size = 2i32,
}
#[cfg(feature = "UnityEngine+Rendering+ReflectionProbeSortingCriteria")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::ReflectionProbeSortingCriteria => "UnityEngine.Rendering"
    ."ReflectionProbeSortingCriteria"
);

#[cfg(feature = "UnityEngine+HumanBone")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct HumanBone {
    pub m_BoneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_HumanName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub limit: crate::UnityEngine::HumanLimit,
}
#[cfg(feature = "UnityEngine+HumanBone")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::HumanBone => "UnityEngine"
    ."HumanBone"
);
#[cfg(feature = "UnityEngine+HumanBone")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::HumanBone {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+HumanBone")]
impl crate::UnityEngine::HumanBone {}

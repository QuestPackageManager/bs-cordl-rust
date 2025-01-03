#[cfg(feature = "UnityEngine+ApplicationMemoryUsageChange")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ApplicationMemoryUsageChange {
    pub _memoryUsage_k__BackingField: crate::UnityEngine::ApplicationMemoryUsage,
}
#[cfg(feature = "UnityEngine+ApplicationMemoryUsageChange")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ApplicationMemoryUsageChange =>
    "UnityEngine"."ApplicationMemoryUsageChange"
);
#[cfg(feature = "UnityEngine+ApplicationMemoryUsageChange")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ApplicationMemoryUsageChange {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ApplicationMemoryUsageChange")]
impl crate::UnityEngine::ApplicationMemoryUsageChange {
    pub fn _ctor(
        &mut self,
        usage: crate::UnityEngine::ApplicationMemoryUsage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (usage),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_memoryUsage(
        &mut self,
        value: crate::UnityEngine::ApplicationMemoryUsage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_memoryUsage",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}

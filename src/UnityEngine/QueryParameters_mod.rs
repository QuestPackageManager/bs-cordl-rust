#[cfg(feature = "UnityEngine+QueryParameters")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct QueryParameters {
    pub layerMask: i32,
    pub hitMultipleFaces: bool,
    pub hitTriggers: crate::UnityEngine::QueryTriggerInteraction,
    pub hitBackfaces: bool,
}
#[cfg(feature = "UnityEngine+QueryParameters")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::QueryParameters => "UnityEngine"
    ."QueryParameters"
);
#[cfg(feature = "UnityEngine+QueryParameters")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::QueryParameters {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+QueryParameters")]
impl crate::UnityEngine::QueryParameters {
    pub fn _ctor(
        &mut self,
        layerMask: i32,
        hitMultipleFaces: bool,
        hitTriggers: crate::UnityEngine::QueryTriggerInteraction,
        hitBackfaces: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (layerMask, hitMultipleFaces, hitTriggers, hitBackfaces),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Default() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::QueryParameters,
    > {
        let __cordl_ret: crate::UnityEngine::QueryParameters = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Default", ())?;
        Ok(__cordl_ret.into())
    }
}

#[cfg(feature = "UnityEngine+EventSystems+RaycastResult")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RaycastResult {
    pub m_GameObject: *mut crate::UnityEngine::GameObject,
    pub _cordl_module: *mut crate::UnityEngine::EventSystems::BaseRaycaster,
    pub distance: f32,
    pub index: f32,
    pub depth: i32,
    pub sortingGroupID: i32,
    pub sortingGroupOrder: i32,
    pub sortingLayer: i32,
    pub sortingOrder: i32,
    pub worldPosition: crate::UnityEngine::Vector3,
    pub worldNormal: crate::UnityEngine::Vector3,
    pub screenPosition: crate::UnityEngine::Vector2,
    pub displayIndex: i32,
}
#[cfg(feature = "UnityEngine+EventSystems+RaycastResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::RaycastResult =>
    "UnityEngine.EventSystems"."RaycastResult"
);
#[cfg(feature = "UnityEngine+EventSystems+RaycastResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::EventSystems::RaycastResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+RaycastResult")]
impl crate::UnityEngine::EventSystems::RaycastResult {
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_gameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_ret: *mut crate::UnityEngine::GameObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_gameObject",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isValid",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_gameObject(
        &mut self,
        value: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_gameObject",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}

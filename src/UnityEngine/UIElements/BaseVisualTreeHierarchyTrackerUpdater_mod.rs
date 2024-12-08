#[cfg(feature = "UnityEngine+UIElements+BaseVisualTreeHierarchyTrackerUpdater")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseVisualTreeHierarchyTrackerUpdater {
    __cordl_parent: crate::UnityEngine::UIElements::BaseVisualTreeUpdater,
    pub m_State: crate::UnityEngine::UIElements::BaseVisualTreeHierarchyTrackerUpdater_State,
    pub m_CurrentChangeElement: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_CurrentChangeParent: *mut crate::UnityEngine::UIElements::VisualElement,
}
#[cfg(feature = "UnityEngine+UIElements+BaseVisualTreeHierarchyTrackerUpdater")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::BaseVisualTreeHierarchyTrackerUpdater =>
    "UnityEngine.UIElements"."BaseVisualTreeHierarchyTrackerUpdater"
);
#[cfg(feature = "UnityEngine+UIElements+BaseVisualTreeHierarchyTrackerUpdater")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::BaseVisualTreeHierarchyTrackerUpdater {
    type Target = crate::UnityEngine::UIElements::BaseVisualTreeUpdater;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVisualTreeHierarchyTrackerUpdater")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::BaseVisualTreeHierarchyTrackerUpdater {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVisualTreeHierarchyTrackerUpdater")]
impl crate::UnityEngine::UIElements::BaseVisualTreeHierarchyTrackerUpdater {
    #[cfg(
        feature = "UnityEngine+UIElements+BaseVisualTreeHierarchyTrackerUpdater+State"
    )]
    pub type State = crate::UnityEngine::UIElements::BaseVisualTreeHierarchyTrackerUpdater_State;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnHierarchyChange(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
        _cordl_type: crate::UnityEngine::UIElements::HierarchyChangeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnHierarchyChange", (ve, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn OnVersionChanged(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
        versionChangeType: crate::UnityEngine::UIElements::VersionChangeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnVersionChanged", (ve, versionChangeType))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessAddOrMove(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAddOrMove", (ve))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessNewChange(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessNewChange", (ve))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessRemove(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessRemove", (ve))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVisualTreeHierarchyTrackerUpdater")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::BaseVisualTreeHierarchyTrackerUpdater {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVisualTreeHierarchyTrackerUpdater+State")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseVisualTreeHierarchyTrackerUpdater_State {
    TrackingAddOrMove = 1i32,
    TrackingRemove = 2i32,
    Waiting = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+BaseVisualTreeHierarchyTrackerUpdater+State")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::BaseVisualTreeHierarchyTrackerUpdater_State =>
    "UnityEngine.UIElements"."BaseVisualTreeHierarchyTrackerUpdater/State"
);

#[cfg(feature = "UnityEngine+UIElements+DetachFromPanelEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct DetachFromPanelEvent {
    __cordl_parent: crate::UnityEngine::UIElements::PanelChangedEventBase_1<
        *mut crate::UnityEngine::UIElements::DetachFromPanelEvent,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+DetachFromPanelEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DetachFromPanelEvent =>
    "UnityEngine.UIElements"."DetachFromPanelEvent"
);
#[cfg(feature = "UnityEngine+UIElements+DetachFromPanelEvent")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DetachFromPanelEvent {
    type Target = crate::UnityEngine::UIElements::PanelChangedEventBase_1<
        *mut crate::UnityEngine::UIElements::DetachFromPanelEvent,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DetachFromPanelEvent")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DetachFromPanelEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DetachFromPanelEvent")]
impl crate::UnityEngine::UIElements::DetachFromPanelEvent {
    #[cfg(feature = "UnityEngine+UIElements+DetachFromPanelEvent+__c")]
    pub type __c = crate::UnityEngine::UIElements::DetachFromPanelEvent___c;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "UnityEngine+UIElements+DetachFromPanelEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DetachFromPanelEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

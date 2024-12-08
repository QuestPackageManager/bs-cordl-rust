#[cfg(feature = "UnityEngine+UIElements+ClickEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct ClickEvent {
    __cordl_parent: crate::UnityEngine::UIElements::PointerEventBase_1<
        *mut crate::UnityEngine::UIElements::ClickEvent,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+ClickEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ClickEvent =>
    "UnityEngine.UIElements"."ClickEvent"
);
#[cfg(feature = "UnityEngine+UIElements+ClickEvent")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ClickEvent {
    type Target = crate::UnityEngine::UIElements::PointerEventBase_1<
        *mut crate::UnityEngine::UIElements::ClickEvent,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ClickEvent")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ClickEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ClickEvent")]
impl crate::UnityEngine::UIElements::ClickEvent {
    #[cfg(feature = "UnityEngine+UIElements+ClickEvent+__c")]
    pub type __c = crate::UnityEngine::UIElements::ClickEvent___c;
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret)
    }
    pub fn LocalInit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LocalInit", ())?;
        Ok(__cordl_ret)
    }
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
#[cfg(feature = "UnityEngine+UIElements+ClickEvent")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::ClickEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

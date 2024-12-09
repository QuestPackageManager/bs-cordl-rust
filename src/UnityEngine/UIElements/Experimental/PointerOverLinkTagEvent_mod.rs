#[cfg(feature = "UnityEngine+UIElements+Experimental+PointerOverLinkTagEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerOverLinkTagEvent {
    __cordl_parent: crate::UnityEngine::UIElements::PointerEventBase_1<
        *mut crate::UnityEngine::UIElements::Experimental::PointerOverLinkTagEvent,
    >,
    pub _linkID_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _linkText_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+PointerOverLinkTagEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Experimental::PointerOverLinkTagEvent =>
    "UnityEngine.UIElements.Experimental"."PointerOverLinkTagEvent"
);
#[cfg(feature = "UnityEngine+UIElements+Experimental+PointerOverLinkTagEvent")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::Experimental::PointerOverLinkTagEvent {
    type Target = crate::UnityEngine::UIElements::PointerEventBase_1<
        *mut crate::UnityEngine::UIElements::Experimental::PointerOverLinkTagEvent,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+PointerOverLinkTagEvent")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::Experimental::PointerOverLinkTagEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+PointerOverLinkTagEvent")]
impl crate::UnityEngine::UIElements::Experimental::PointerOverLinkTagEvent {
    #[cfg(feature = "UnityEngine+UIElements+Experimental+PointerOverLinkTagEvent+__c")]
    pub type __c = crate::UnityEngine::UIElements::Experimental::PointerOverLinkTagEvent___c;
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
    pub fn set_linkID(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_linkID", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_linkText(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_linkText", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+PointerOverLinkTagEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Experimental::PointerOverLinkTagEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "UnityEngine+UIElements+Experimental+PointerUpLinkTagEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerUpLinkTagEvent {
    __cordl_parent: crate::UnityEngine::UIElements::PointerEventBase_1<
        *mut crate::UnityEngine::UIElements::Experimental::PointerUpLinkTagEvent,
    >,
    pub _linkID_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _linkText_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+PointerUpLinkTagEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Experimental::PointerUpLinkTagEvent =>
    "UnityEngine.UIElements.Experimental"."PointerUpLinkTagEvent"
);
#[cfg(feature = "UnityEngine+UIElements+Experimental+PointerUpLinkTagEvent")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::Experimental::PointerUpLinkTagEvent {
    type Target = crate::UnityEngine::UIElements::PointerEventBase_1<
        *mut crate::UnityEngine::UIElements::Experimental::PointerUpLinkTagEvent,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+PointerUpLinkTagEvent")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::Experimental::PointerUpLinkTagEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+PointerUpLinkTagEvent")]
impl crate::UnityEngine::UIElements::Experimental::PointerUpLinkTagEvent {
    pub fn GetPooled(
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPointerEvent>,
        linkID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        linkText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::PointerUpLinkTagEvent,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::PointerUpLinkTagEvent,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPooled", (evt, linkID, linkText))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LocalInit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LocalInit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_linkID(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_linkID", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_linkText(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_linkText", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+PointerUpLinkTagEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Experimental::PointerUpLinkTagEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

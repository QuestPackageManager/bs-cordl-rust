#[cfg(feature = "UnityEngine+UIElements+KeyDownEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyDownEvent {
    __cordl_parent: crate::UnityEngine::UIElements::KeyboardEventBase_1<
        *mut crate::UnityEngine::UIElements::KeyDownEvent,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+KeyDownEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::KeyDownEvent =>
    "UnityEngine.UIElements"."KeyDownEvent"
);
#[cfg(feature = "UnityEngine+UIElements+KeyDownEvent")]
impl std::ops::Deref for crate::UnityEngine::UIElements::KeyDownEvent {
    type Target = crate::UnityEngine::UIElements::KeyboardEventBase_1<
        *mut crate::UnityEngine::UIElements::KeyDownEvent,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+KeyDownEvent")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::KeyDownEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+KeyDownEvent")]
impl crate::UnityEngine::UIElements::KeyDownEvent {
    #[cfg(feature = "UnityEngine+UIElements+KeyDownEvent+__c")]
    pub type __c = crate::UnityEngine::UIElements::KeyDownEvent___c;
    pub fn GetEquivalentImguiEvent(
        &mut self,
        outImguiEvent: *mut crate::UnityEngine::Event,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetEquivalentImguiEvent", (outImguiEvent))?;
        Ok(__cordl_ret)
    }
    pub fn SendEquivalentNavigationEventIfAny(
        &mut self,
        panel: *mut crate::UnityEngine::UIElements::IPanel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendEquivalentNavigationEventIfAny", (panel))?;
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
    pub fn PostDispatch(
        &mut self,
        panel: *mut crate::UnityEngine::UIElements::IPanel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PostDispatch", (panel))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+KeyDownEvent")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::KeyDownEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "UnityEngine+UIElements+KeyboardEventDispatchingStrategy")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyboardEventDispatchingStrategy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardEventDispatchingStrategy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::KeyboardEventDispatchingStrategy =>
    "UnityEngine.UIElements"."KeyboardEventDispatchingStrategy"
);
#[cfg(feature = "UnityEngine+UIElements+KeyboardEventDispatchingStrategy")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::KeyboardEventDispatchingStrategy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardEventDispatchingStrategy")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::KeyboardEventDispatchingStrategy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardEventDispatchingStrategy")]
impl crate::UnityEngine::UIElements::KeyboardEventDispatchingStrategy {
    pub fn CanDispatchEvent(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::EventBase,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanDispatchEvent", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn DispatchEvent(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::EventBase,
        panel: *mut crate::UnityEngine::UIElements::IPanel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DispatchEvent", (evt, panel))?;
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
#[cfg(feature = "UnityEngine+UIElements+KeyboardEventDispatchingStrategy")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::KeyboardEventDispatchingStrategy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

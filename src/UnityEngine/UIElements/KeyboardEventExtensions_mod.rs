#[cfg(feature = "UnityEngine+UIElements+KeyboardEventExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyboardEventExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardEventExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::KeyboardEventExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "KeyboardEventExtensions";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardEventExtensions")]
impl std::ops::Deref for crate::UnityEngine::UIElements::KeyboardEventExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardEventExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::KeyboardEventExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardEventExtensions")]
impl crate::UnityEngine::UIElements::KeyboardEventExtensions {
    pub fn ShouldSendNavigationMoveEvent(
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::KeyDownEvent>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldSendNavigationMoveEvent", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSendNavigationMoveEventRuntime(
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldSendNavigationMoveEventRuntime", (e))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardEventExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::KeyboardEventExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

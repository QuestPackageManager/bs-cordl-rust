#[cfg(feature = "UnityEngine+UIElements+DropdownMenuEventInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct DropdownMenuEventInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _modifiers_k__BackingField: crate::UnityEngine::EventModifiers,
    pub _mousePosition_k__BackingField: crate::UnityEngine::Vector2,
    pub _localMousePosition_k__BackingField: crate::UnityEngine::Vector2,
    pub _character_k__BackingField: char,
    pub _keyCode_k__BackingField: crate::UnityEngine::KeyCode,
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuEventInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::DropdownMenuEventInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "DropdownMenuEventInfo";
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
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuEventInfo")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DropdownMenuEventInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuEventInfo")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DropdownMenuEventInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuEventInfo")]
impl crate::UnityEngine::UIElements::DropdownMenuEventInfo {
    pub fn New(
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (e))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (e))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuEventInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DropdownMenuEventInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

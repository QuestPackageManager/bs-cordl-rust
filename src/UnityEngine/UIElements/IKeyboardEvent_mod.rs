#[cfg(feature = "UnityEngine+UIElements+IKeyboardEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct IKeyboardEvent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+IKeyboardEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::IKeyboardEvent =>
    "UnityEngine.UIElements"."IKeyboardEvent"
);
#[cfg(feature = "UnityEngine+UIElements+IKeyboardEvent")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IKeyboardEvent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IKeyboardEvent")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IKeyboardEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IKeyboardEvent")]
impl crate::UnityEngine::UIElements::IKeyboardEvent {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_character(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("get_character", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_keyCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::KeyCode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::KeyCode = __cordl_object
            .invoke("get_keyCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_modifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::EventModifiers> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::EventModifiers = __cordl_object
            .invoke("get_modifiers", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+IKeyboardEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IKeyboardEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "UnityEngine+Cursor")]
#[repr(C)]
#[derive(Debug)]
pub struct Cursor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Cursor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Cursor => "UnityEngine"."Cursor"
);
#[cfg(feature = "UnityEngine+Cursor")]
impl std::ops::Deref for crate::UnityEngine::Cursor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Cursor")]
impl std::ops::DerefMut for crate::UnityEngine::Cursor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Cursor")]
impl crate::UnityEngine::Cursor {}
#[cfg(feature = "UnityEngine+Cursor")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Cursor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

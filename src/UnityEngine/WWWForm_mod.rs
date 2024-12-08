#[cfg(feature = "UnityEngine+WWWForm")]
#[repr(C)]
#[derive(Debug)]
pub struct WWWForm {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+WWWForm")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::WWWForm => "UnityEngine"."WWWForm"
);
#[cfg(feature = "UnityEngine+WWWForm")]
impl std::ops::Deref for crate::UnityEngine::WWWForm {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+WWWForm")]
impl std::ops::DerefMut for crate::UnityEngine::WWWForm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+WWWForm")]
impl crate::UnityEngine::WWWForm {}
#[cfg(feature = "UnityEngine+WWWForm")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::WWWForm {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
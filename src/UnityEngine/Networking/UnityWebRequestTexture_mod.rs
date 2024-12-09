#[cfg(feature = "UnityEngine+Networking+UnityWebRequestTexture")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityWebRequestTexture {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestTexture")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Networking::UnityWebRequestTexture
    => "UnityEngine.Networking"."UnityWebRequestTexture"
);
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestTexture")]
impl std::ops::Deref for crate::UnityEngine::Networking::UnityWebRequestTexture {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestTexture")]
impl std::ops::DerefMut for crate::UnityEngine::Networking::UnityWebRequestTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestTexture")]
impl crate::UnityEngine::Networking::UnityWebRequestTexture {}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestTexture")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::UnityWebRequestTexture {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

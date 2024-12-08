#[cfg(feature = "UnityEngine+ResourcesAPIInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourcesAPIInternal {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ResourcesAPIInternal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ResourcesAPIInternal =>
    "UnityEngine"."ResourcesAPIInternal"
);
#[cfg(feature = "UnityEngine+ResourcesAPIInternal")]
impl std::ops::Deref for crate::UnityEngine::ResourcesAPIInternal {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourcesAPIInternal")]
impl std::ops::DerefMut for crate::UnityEngine::ResourcesAPIInternal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourcesAPIInternal")]
impl crate::UnityEngine::ResourcesAPIInternal {}
#[cfg(feature = "UnityEngine+ResourcesAPIInternal")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ResourcesAPIInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

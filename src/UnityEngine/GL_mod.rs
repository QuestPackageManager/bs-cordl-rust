#[cfg(feature = "UnityEngine+GL")]
#[repr(C)]
#[derive(Debug)]
pub struct GL {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+GL")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GL => "UnityEngine"."GL"
);
#[cfg(feature = "UnityEngine+GL")]
impl std::ops::Deref for crate::UnityEngine::GL {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GL")]
impl std::ops::DerefMut for crate::UnityEngine::GL {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GL")]
impl crate::UnityEngine::GL {}
#[cfg(feature = "UnityEngine+GL")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GL {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

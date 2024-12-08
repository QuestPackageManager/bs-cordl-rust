#[cfg(feature = "UnityEngine+PropertyNameUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct PropertyNameUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+PropertyNameUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PropertyNameUtils => "UnityEngine"
    ."PropertyNameUtils"
);
#[cfg(feature = "UnityEngine+PropertyNameUtils")]
impl std::ops::Deref for crate::UnityEngine::PropertyNameUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PropertyNameUtils")]
impl std::ops::DerefMut for crate::UnityEngine::PropertyNameUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PropertyNameUtils")]
impl crate::UnityEngine::PropertyNameUtils {}
#[cfg(feature = "UnityEngine+PropertyNameUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::PropertyNameUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

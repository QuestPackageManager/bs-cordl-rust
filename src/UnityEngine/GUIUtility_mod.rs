#[cfg(feature = "UnityEngine+GUIUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct GUIUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+GUIUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUIUtility => "UnityEngine"
    ."GUIUtility"
);
#[cfg(feature = "UnityEngine+GUIUtility")]
impl std::ops::Deref for crate::UnityEngine::GUIUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUIUtility")]
impl std::ops::DerefMut for crate::UnityEngine::GUIUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUIUtility")]
impl crate::UnityEngine::GUIUtility {}
#[cfg(feature = "UnityEngine+GUIUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUIUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

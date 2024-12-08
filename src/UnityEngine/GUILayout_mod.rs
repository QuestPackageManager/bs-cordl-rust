#[cfg(feature = "UnityEngine+GUILayout")]
#[repr(C)]
#[derive(Debug)]
pub struct GUILayout {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+GUILayout")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUILayout => "UnityEngine"
    ."GUILayout"
);
#[cfg(feature = "UnityEngine+GUILayout")]
impl std::ops::Deref for crate::UnityEngine::GUILayout {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayout")]
impl std::ops::DerefMut for crate::UnityEngine::GUILayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayout")]
impl crate::UnityEngine::GUILayout {}
#[cfg(feature = "UnityEngine+GUILayout")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUILayout {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

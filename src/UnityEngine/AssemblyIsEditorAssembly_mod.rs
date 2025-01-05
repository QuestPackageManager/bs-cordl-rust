#[cfg(feature = "UnityEngine+AssemblyIsEditorAssembly")]
#[repr(C)]
#[derive(Debug)]
pub struct AssemblyIsEditorAssembly {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "UnityEngine+AssemblyIsEditorAssembly")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AssemblyIsEditorAssembly =>
    "UnityEngine"."AssemblyIsEditorAssembly"
);
#[cfg(feature = "UnityEngine+AssemblyIsEditorAssembly")]
impl std::ops::Deref for crate::UnityEngine::AssemblyIsEditorAssembly {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AssemblyIsEditorAssembly")]
impl std::ops::DerefMut for crate::UnityEngine::AssemblyIsEditorAssembly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AssemblyIsEditorAssembly")]
impl crate::UnityEngine::AssemblyIsEditorAssembly {}
#[cfg(feature = "UnityEngine+AssemblyIsEditorAssembly")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AssemblyIsEditorAssembly {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

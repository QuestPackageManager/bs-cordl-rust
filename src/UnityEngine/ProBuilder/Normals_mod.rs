#[cfg(feature = "UnityEngine+ProBuilder+Normals")]
#[repr(C)]
#[derive(Debug)]
pub struct Normals {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+Normals")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Normals =>
    "UnityEngine.ProBuilder"."Normals"
);
#[cfg(feature = "UnityEngine+ProBuilder+Normals")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Normals {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Normals")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Normals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Normals")]
impl crate::UnityEngine::ProBuilder::Normals {}
#[cfg(feature = "UnityEngine+ProBuilder+Normals")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Normals {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

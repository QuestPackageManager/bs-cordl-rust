#[cfg(feature = "UnityEngine+ProBuilder+SelectPathFaces")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectPathFaces {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectPathFaces")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::SelectPathFaces =>
    "UnityEngine.ProBuilder"."SelectPathFaces"
);
#[cfg(feature = "UnityEngine+ProBuilder+SelectPathFaces")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::SelectPathFaces {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectPathFaces")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::SelectPathFaces {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectPathFaces")]
impl crate::UnityEngine::ProBuilder::SelectPathFaces {}
#[cfg(feature = "UnityEngine+ProBuilder+SelectPathFaces")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::SelectPathFaces {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "UnityEngine+ProBuilder+VectorHash")]
#[repr(C)]
#[derive(Debug)]
pub struct VectorHash {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+VectorHash")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::VectorHash =>
    "UnityEngine.ProBuilder"."VectorHash"
);
#[cfg(feature = "UnityEngine+ProBuilder+VectorHash")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::VectorHash {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+VectorHash")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::VectorHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+VectorHash")]
impl crate::UnityEngine::ProBuilder::VectorHash {
    pub const FltCompareResolution: f32 = 1000f32;
}
#[cfg(feature = "UnityEngine+ProBuilder+VectorHash")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::VectorHash {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

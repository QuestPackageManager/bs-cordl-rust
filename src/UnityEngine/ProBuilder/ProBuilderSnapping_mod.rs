#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
#[repr(C)]
#[derive(Debug)]
pub struct ProBuilderSnapping {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::ProBuilderSnapping =>
    "UnityEngine.ProBuilder"."ProBuilderSnapping"
);
#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::ProBuilderSnapping {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::ProBuilderSnapping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
impl crate::UnityEngine::ProBuilder::ProBuilderSnapping {
    pub const k_MaxRaySnapDistance: f32 = std::f32::INFINITY;
}
#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::ProBuilderSnapping {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

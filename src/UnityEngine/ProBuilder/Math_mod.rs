#[cfg(feature = "UnityEngine+ProBuilder+Math")]
#[repr(C)]
#[derive(Debug)]
pub struct Math {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+Math")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Math =>
    "UnityEngine.ProBuilder"."Math"
);
#[cfg(feature = "UnityEngine+ProBuilder+Math")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Math {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Math")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Math {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Math")]
impl crate::UnityEngine::ProBuilder::Math {
    pub const handleEpsilon: f32 = 0.0001f32;
    pub const k_FltCompareEpsilon: f32 = 0.0001f32;
    pub const k_FltEpsilon: f32 = 0.000000000000000000000000000000000000000000001f32;
    pub const phi: f32 = 1.618034f32;
}
#[cfg(feature = "UnityEngine+ProBuilder+Math")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Math {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

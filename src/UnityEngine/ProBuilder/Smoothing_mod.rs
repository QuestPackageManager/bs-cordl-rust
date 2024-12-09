#[cfg(feature = "UnityEngine+ProBuilder+Smoothing")]
#[repr(C)]
#[derive(Debug)]
pub struct Smoothing {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+Smoothing")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Smoothing =>
    "UnityEngine.ProBuilder"."Smoothing"
);
#[cfg(feature = "UnityEngine+ProBuilder+Smoothing")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Smoothing {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Smoothing")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Smoothing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Smoothing")]
impl crate::UnityEngine::ProBuilder::Smoothing {
    pub const hardRangeMax: i32 = 42i32;
    pub const hardRangeMin: i32 = 25i32;
    pub const smoothRangeMax: i32 = 24i32;
    pub const smoothRangeMin: i32 = 1i32;
    pub const smoothingGroupNone: i32 = 0i32;
    #[cfg(feature = "UnityEngine+ProBuilder+Smoothing+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::Smoothing___c;
}
#[cfg(feature = "UnityEngine+ProBuilder+Smoothing")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Smoothing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

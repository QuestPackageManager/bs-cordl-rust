#[cfg(feature = "UnityEngine+Timeline+TimeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Timeline+TimeUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TimeUtility =>
    "UnityEngine.Timeline"."TimeUtility"
);
#[cfg(feature = "UnityEngine+Timeline+TimeUtility")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimeUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeUtility")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TimeUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeUtility")]
impl crate::UnityEngine::Timeline::TimeUtility {
    #[cfg(feature = "UnityEngine+Timeline+TimeUtility+__c")]
    pub type __c = crate::UnityEngine::Timeline::TimeUtility___c;
}
#[cfg(feature = "UnityEngine+Timeline+TimeUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::TimeUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

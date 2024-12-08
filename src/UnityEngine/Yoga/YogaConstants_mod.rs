#[cfg(feature = "UnityEngine+Yoga+YogaConstants")]
#[repr(C)]
#[derive(Debug)]
pub struct YogaConstants {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Yoga+YogaConstants")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::YogaConstants =>
    "UnityEngine.Yoga"."YogaConstants"
);
#[cfg(feature = "UnityEngine+Yoga+YogaConstants")]
impl std::ops::Deref for crate::UnityEngine::Yoga::YogaConstants {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Yoga+YogaConstants")]
impl std::ops::DerefMut for crate::UnityEngine::Yoga::YogaConstants {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Yoga+YogaConstants")]
impl crate::UnityEngine::Yoga::YogaConstants {}
#[cfg(feature = "UnityEngine+Yoga+YogaConstants")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Yoga::YogaConstants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
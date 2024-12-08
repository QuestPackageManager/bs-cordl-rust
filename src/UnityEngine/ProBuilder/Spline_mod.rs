#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
#[repr(C)]
#[derive(Debug)]
pub struct Spline {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Spline =>
    "UnityEngine.ProBuilder"."Spline"
);
#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Spline {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Spline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
impl crate::UnityEngine::ProBuilder::Spline {}
#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Spline {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

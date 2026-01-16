#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Features+Interactions+EyeTrackingUsages")]
#[repr(C)]
#[derive(Debug)]
pub struct EyeTrackingUsages {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Features+Interactions+EyeTrackingUsages")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::XR::OpenXR::Features::Interactions::EyeTrackingUsages
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.OpenXR.Features.Interactions";
    const CLASS_NAME: &'static str = "EyeTrackingUsages";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+XR+OpenXR+Features+Interactions+EyeTrackingUsages")]
impl std::ops::Deref for crate::UnityEngine::XR::OpenXR::Features::Interactions::EyeTrackingUsages {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+OpenXR+Features+Interactions+EyeTrackingUsages")]
impl std::ops::DerefMut
    for crate::UnityEngine::XR::OpenXR::Features::Interactions::EyeTrackingUsages
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+OpenXR+Features+Interactions+EyeTrackingUsages")]
impl crate::UnityEngine::XR::OpenXR::Features::Interactions::EyeTrackingUsages {}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Features+Interactions+EyeTrackingUsages")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::XR::OpenXR::Features::Interactions::EyeTrackingUsages
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "UnityEngine+XR+InputTracking")]
#[repr(C)]
#[derive(Debug)]
pub struct InputTracking {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+XR+InputTracking")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::InputTracking =>
    "UnityEngine.XR"."InputTracking"
);
#[cfg(feature = "UnityEngine+XR+InputTracking")]
impl std::ops::Deref for crate::UnityEngine::XR::InputTracking {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+InputTracking")]
impl std::ops::DerefMut for crate::UnityEngine::XR::InputTracking {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+InputTracking")]
impl crate::UnityEngine::XR::InputTracking {
    #[cfg(feature = "UnityEngine+XR+InputTracking+TrackingStateEventType")]
    pub type TrackingStateEventType = crate::UnityEngine::XR::InputTracking_TrackingStateEventType;
}
#[cfg(feature = "UnityEngine+XR+InputTracking")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::XR::InputTracking {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+XR+InputTracking+TrackingStateEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputTracking_TrackingStateEventType {
    NodeAdded = 0i32,
    NodeRemoved = 1i32,
    TrackingAcquired = 2i32,
    TrackingLost = 3i32,
}
#[cfg(feature = "UnityEngine+XR+InputTracking+TrackingStateEventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::XR::InputTracking_TrackingStateEventType => "UnityEngine.XR"
    ."InputTracking/TrackingStateEventType"
);

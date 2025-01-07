#[cfg(feature = "UnityEngine+XR+XRDevice")]
#[repr(C)]
#[derive(Debug)]
pub struct XRDevice {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+XR+XRDevice")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::XR::XRDevice {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.XR";
    const CLASS_NAME: &'static str = "XRDevice";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+XR+XRDevice")]
impl std::ops::Deref for crate::UnityEngine::XR::XRDevice {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+XRDevice")]
impl std::ops::DerefMut for crate::UnityEngine::XR::XRDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+XRDevice")]
impl crate::UnityEngine::XR::XRDevice {
    pub fn DisableAutoXRCameraTracking(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        disabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DisableAutoXRCameraTracking", (camera, disabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeDeviceLoaded(
        loadedDeviceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeDeviceLoaded", (loadedDeviceName))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTrackingSpaceType(
        trackingSpaceType: crate::UnityEngine::XR::TrackingSpaceType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetTrackingSpaceType", (trackingSpaceType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+XR+XRDevice")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::XR::XRDevice {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

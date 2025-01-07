#[cfg(feature = "UnityEngine+XR+InputDevices")]
#[repr(C)]
#[derive(Debug)]
pub struct InputDevices {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+XR+InputDevices")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::XR::InputDevices {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.XR";
    const CLASS_NAME: &'static str = "InputDevices";
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
#[cfg(feature = "UnityEngine+XR+InputDevices")]
impl std::ops::Deref for crate::UnityEngine::XR::InputDevices {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+InputDevices")]
impl std::ops::DerefMut for crate::UnityEngine::XR::InputDevices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+InputDevices")]
impl crate::UnityEngine::XR::InputDevices {
    pub fn GetDeviceAtXRNode(
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::XR::InputDevice> {
        let __cordl_ret: crate::UnityEngine::XR::InputDevice = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDeviceAtXRNode", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDeviceManufacturer(
        deviceId: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDeviceManufacturer", (deviceId))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeConnectionEvent(
        deviceId: u64,
        change: crate::UnityEngine::XR::ConnectionChangeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeConnectionEvent", (deviceId, change))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDeviceValid(deviceId: u64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDeviceValid", (deviceId))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendHapticImpulse(
        deviceId: u64,
        channel: u32,
        amplitude: f32,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SendHapticImpulse", (deviceId, channel, amplitude, duration))?;
        Ok(__cordl_ret.into())
    }
    pub fn StopHaptics(
        deviceId: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StopHaptics", (deviceId))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+XR+InputDevices")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::XR::InputDevices {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

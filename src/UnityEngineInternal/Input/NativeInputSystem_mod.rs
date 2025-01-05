#[cfg(feature = "UnityEngineInternal+Input+NativeInputSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeInputSystem {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngineInternal+Input+NativeInputSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngineInternal::Input::NativeInputSystem =>
    "UnityEngineInternal.Input"."NativeInputSystem"
);
#[cfg(feature = "UnityEngineInternal+Input+NativeInputSystem")]
impl std::ops::Deref for crate::UnityEngineInternal::Input::NativeInputSystem {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngineInternal+Input+NativeInputSystem")]
impl std::ops::DerefMut for crate::UnityEngineInternal::Input::NativeInputSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngineInternal+Input+NativeInputSystem")]
impl crate::UnityEngineInternal::Input::NativeInputSystem {
    pub fn AllocateDeviceId() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllocateDeviceId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IOCTL(
        deviceId: i32,
        code: i32,
        data: crate::System::IntPtr,
        sizeInBytes: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IOCTL", (deviceId, code, data, sizeInBytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyBeforeUpdate(
        updateType: crate::UnityEngineInternal::Input::NativeInputUpdateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotifyBeforeUpdate", (updateType))?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyDeviceDiscovered(
        deviceId: i32,
        deviceDescriptor: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotifyDeviceDiscovered", (deviceId, deviceDescriptor))?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyUpdate(
        updateType: crate::UnityEngineInternal::Input::NativeInputUpdateType,
        eventBuffer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotifyUpdate", (updateType, eventBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueInputEvent(
        inputEvent: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QueueInputEvent", (inputEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPollingFrequency(
        hertz: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetPollingFrequency", (hertz))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldRunUpdate(
        updateType: crate::UnityEngineInternal::Input::NativeInputUpdateType,
        retval: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldRunUpdate", (updateType, retval))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        updateType: crate::UnityEngineInternal::Input::NativeInputUpdateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Update", (updateType))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentTime() -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_currentTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentTimeOffsetToRealtimeSinceStartup() -> quest_hook::libil2cpp::Result<
        f64,
    > {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_currentTimeOffsetToRealtimeSinceStartup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_onDeviceDiscovered() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            i32,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            i32,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_onDeviceDiscovered", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_hasDeviceDiscoveredCallback(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_hasDeviceDiscoveredCallback", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_onDeviceDiscovered(
        value: quest_hook::libil2cpp::Gc<
            i32,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_onDeviceDiscovered", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngineInternal+Input+NativeInputSystem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngineInternal::Input::NativeInputSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

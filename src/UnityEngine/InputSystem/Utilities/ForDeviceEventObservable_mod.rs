#[cfg(feature = "UnityEngine+InputSystem+Utilities+ForDeviceEventObservable+ForDevice")]
#[repr(C)]
#[derive(Debug)]
pub struct ForDeviceEventObservable_ForDevice {
    __cordl_parent: crate::System::Object,
    pub m_Observer: *mut crate::System::IObserver_1<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    >,
    pub m_Device: *mut crate::UnityEngine::InputSystem::InputDevice,
    pub m_DeviceType: *mut crate::System::Type,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ForDeviceEventObservable+ForDevice")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::ForDeviceEventObservable_ForDevice =>
    "UnityEngine.InputSystem.Utilities"."ForDeviceEventObservable/ForDevice"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ForDeviceEventObservable+ForDevice")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Utilities::ForDeviceEventObservable_ForDevice {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ForDeviceEventObservable+ForDevice")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Utilities::ForDeviceEventObservable_ForDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ForDeviceEventObservable+ForDevice")]
impl crate::UnityEngine::InputSystem::Utilities::ForDeviceEventObservable_ForDevice {
    pub fn OnCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCompleted", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnNext(
        &mut self,
        value: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNext", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        deviceType: *mut crate::System::Type,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
        observer: *mut crate::System::IObserver_1<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (deviceType, device, observer))?;
        Ok(__cordl_ret)
    }
    pub fn OnError(
        &mut self,
        error: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnError", (error))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        deviceType: *mut crate::System::Type,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
        observer: *mut crate::System::IObserver_1<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (deviceType, device, observer))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ForDeviceEventObservable+ForDevice")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::ForDeviceEventObservable_ForDevice {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ForDeviceEventObservable")]
#[repr(C)]
#[derive(Debug)]
pub struct ForDeviceEventObservable {
    __cordl_parent: crate::System::Object,
    pub m_Source: *mut crate::System::IObservable_1<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    >,
    pub m_Device: *mut crate::UnityEngine::InputSystem::InputDevice,
    pub m_DeviceType: *mut crate::System::Type,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ForDeviceEventObservable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::ForDeviceEventObservable =>
    "UnityEngine.InputSystem.Utilities"."ForDeviceEventObservable"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ForDeviceEventObservable")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Utilities::ForDeviceEventObservable {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ForDeviceEventObservable")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Utilities::ForDeviceEventObservable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ForDeviceEventObservable")]
impl crate::UnityEngine::InputSystem::Utilities::ForDeviceEventObservable {
    #[cfg(
        feature = "UnityEngine+InputSystem+Utilities+ForDeviceEventObservable+ForDevice"
    )]
    pub type ForDevice = crate::UnityEngine::InputSystem::Utilities::ForDeviceEventObservable_ForDevice;
    pub fn _ctor(
        &mut self,
        source: *mut crate::System::IObservable_1<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
        deviceType: *mut crate::System::Type,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (source, deviceType, device))?;
        Ok(__cordl_ret)
    }
    pub fn Subscribe(
        &mut self,
        observer: *mut crate::System::IObserver_1<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IDisposable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IDisposable = __cordl_object
            .invoke("Subscribe", (observer))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        source: *mut crate::System::IObservable_1<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
        deviceType: *mut crate::System::Type,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, deviceType, device))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ForDeviceEventObservable")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::ForDeviceEventObservable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

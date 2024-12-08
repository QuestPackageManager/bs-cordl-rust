#[cfg(feature = "UnityEngine+InputSystem+XR+XRDeviceDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct XRDeviceDescriptor {
    __cordl_parent: crate::System::Object,
    pub deviceName: *mut crate::System::String,
    pub manufacturer: *mut crate::System::String,
    pub serialNumber: *mut crate::System::String,
    pub characteristics: crate::UnityEngine::XR::InputDeviceCharacteristics,
    pub deviceId: i32,
    pub inputFeatures: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::InputSystem::XR::XRFeatureDescriptor,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRDeviceDescriptor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::XR::XRDeviceDescriptor
    => "UnityEngine.InputSystem.XR"."XRDeviceDescriptor"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+XRDeviceDescriptor")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::XR::XRDeviceDescriptor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRDeviceDescriptor")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::XR::XRDeviceDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRDeviceDescriptor")]
impl crate::UnityEngine::InputSystem::XR::XRDeviceDescriptor {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ToJson(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToJson", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRDeviceDescriptor")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::XR::XRDeviceDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

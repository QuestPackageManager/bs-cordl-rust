#[cfg(feature = "Unity+XR+Oculus+OculusLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusLoader {
    __cordl_parent: crate::UnityEngine::XR::Management::XRLoaderHelper,
}
#[cfg(feature = "Unity+XR+Oculus+OculusLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::OculusLoader =>
    "Unity.XR.Oculus"."OculusLoader"
);
#[cfg(feature = "Unity+XR+Oculus+OculusLoader")]
impl std::ops::Deref for crate::Unity::XR::Oculus::OculusLoader {
    type Target = crate::UnityEngine::XR::Management::XRLoaderHelper;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusLoader")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::OculusLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusLoader")]
impl crate::Unity::XR::Oculus::OculusLoader {
    #[cfg(feature = "Unity+XR+Oculus+OculusLoader+DeviceSupportedResult")]
    pub type DeviceSupportedResult = crate::Unity::XR::Oculus::OculusLoader_DeviceSupportedResult;
    pub fn CheckUnityVersionCompatibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckUnityVersionCompatibility", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Deinitialize(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Deinitialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::XR::Oculus::OculusSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::XR::Oculus::OculusSettings,
        > = __cordl_object.invoke("GetSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDeviceSupported() -> quest_hook::libil2cpp::Result<
        crate::Unity::XR::Oculus::OculusLoader_DeviceSupportedResult,
    > {
        let __cordl_ret: crate::Unity::XR::Oculus::OculusLoader_DeviceSupportedResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDeviceSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RuntimeLoadOVRPlugin() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RuntimeLoadOVRPlugin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Stop(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Stop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_displaySubsystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<Blacklisted> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: Blacklisted = __cordl_object
            .invoke("get_displaySubsystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_inputSubsystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::XRInputSubsystem>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::XR::XRInputSubsystem,
        > = __cordl_object.invoke("get_inputSubsystem", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::OculusLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusLoader+DeviceSupportedResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OculusLoader_DeviceSupportedResult {
    #[default]
    ExitApplication = 2i32,
    NotSupported = 1i32,
    Supported = 0i32,
}
#[cfg(feature = "Unity+XR+Oculus+OculusLoader+DeviceSupportedResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::XR::Oculus::OculusLoader_DeviceSupportedResult => "Unity.XR.Oculus"
    ."OculusLoader/DeviceSupportedResult"
);

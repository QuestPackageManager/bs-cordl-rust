#[cfg(feature = "OVRMixedRealityCaptureTest+CameraMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRMixedRealityCaptureTest_CameraMode {
    Normal = 0i32,
    OverrideFov = 1i32,
    ThirdPerson = 2i32,
}
#[cfg(feature = "OVRMixedRealityCaptureTest+CameraMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRMixedRealityCaptureTest_CameraMode => ""
    ."OVRMixedRealityCaptureTest/CameraMode"
);
#[cfg(feature = "OVRMixedRealityCaptureTest")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRMixedRealityCaptureTest {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub inited: bool,
    pub currentMode: crate::GlobalNamespace::OVRMixedRealityCaptureTest_CameraMode,
    pub defaultExternalCamera: *mut crate::UnityEngine::Camera,
    pub defaultFov: crate::GlobalNamespace::OVRPlugin_Fovf,
}
#[cfg(feature = "OVRMixedRealityCaptureTest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRMixedRealityCaptureTest => ""
    ."OVRMixedRealityCaptureTest"
);
#[cfg(feature = "OVRMixedRealityCaptureTest")]
impl std::ops::Deref for OVRMixedRealityCaptureTest {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMixedRealityCaptureTest")]
impl std::ops::DerefMut for OVRMixedRealityCaptureTest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMixedRealityCaptureTest")]
impl OVRMixedRealityCaptureTest {
    #[cfg(feature = "OVRMixedRealityCaptureTest+CameraMode")]
    pub type CameraMode = crate::GlobalNamespace::OVRMixedRealityCaptureTest_CameraMode;
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateDefaultExternalCamera(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDefaultExternalCamera", ())?;
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
#[cfg(feature = "OVRMixedRealityCaptureTest")]
impl quest_hook::libil2cpp::ObjectType for OVRMixedRealityCaptureTest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

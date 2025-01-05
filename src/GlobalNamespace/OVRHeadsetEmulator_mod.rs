#[cfg(feature = "OVRHeadsetEmulator")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRHeadsetEmulator {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub opMode: crate::GlobalNamespace::OVRHeadsetEmulator_OpMode,
    pub resetHmdPoseOnRelease: bool,
    pub resetHmdPoseByMiddleMouseButton: bool,
    pub activateKeys: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::KeyCode>,
    >,
    pub pitchKeys: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::KeyCode>,
    >,
    pub manager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRManager>,
    pub lastFrameEmulationActivated: bool,
    pub recordedHeadPoseRelativeOffsetTranslation: crate::UnityEngine::Vector3,
    pub recordedHeadPoseRelativeOffsetRotation: crate::UnityEngine::Vector3,
    pub hasSentEvent: bool,
    pub emulatorHasInitialized: bool,
    pub previousCursorLockMode: crate::UnityEngine::CursorLockMode,
}
#[cfg(feature = "OVRHeadsetEmulator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRHeadsetEmulator => ""
    ."OVRHeadsetEmulator"
);
#[cfg(feature = "OVRHeadsetEmulator")]
impl std::ops::Deref for crate::GlobalNamespace::OVRHeadsetEmulator {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHeadsetEmulator")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRHeadsetEmulator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHeadsetEmulator")]
impl crate::GlobalNamespace::OVRHeadsetEmulator {
    pub const MAX_ROLL: f32 = 85f32;
    pub const MOUSE_SCALE_HEIGHT: f32 = 1f32;
    pub const MOUSE_SCALE_X: f32 = -2f32;
    pub const MOUSE_SCALE_X_PITCH: f32 = -2f32;
    pub const MOUSE_SCALE_Y: f32 = 2f32;
    #[cfg(feature = "OVRHeadsetEmulator+OpMode")]
    pub type OpMode = crate::GlobalNamespace::OVRHeadsetEmulator_OpMode;
    pub fn IsEmulationActivated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEmulationActivated", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTweakingPitch(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTweakingPitch", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
}
#[cfg(feature = "OVRHeadsetEmulator")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRHeadsetEmulator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRHeadsetEmulator+OpMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRHeadsetEmulator_OpMode {
    #[default]
    AlwaysOn = 2i32,
    EditorOnly = 1i32,
    Off = 0i32,
}
#[cfg(feature = "OVRHeadsetEmulator+OpMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRHeadsetEmulator_OpMode => ""
    ."OVRHeadsetEmulator/OpMode"
);

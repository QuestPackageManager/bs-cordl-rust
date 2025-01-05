#[cfg(feature = "Unity+XR+Oculus+OculusSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusSettings {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    pub m_StereoRenderingModeDesktop: crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeDesktop,
    pub m_StereoRenderingModeAndroid: crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeAndroid,
    pub SharedDepthBuffer: bool,
    pub DepthSubmission: bool,
    pub DashSupport: bool,
    pub LowOverheadMode: bool,
    pub OptimizeBufferDiscards: bool,
    pub PhaseSync: bool,
    pub SymmetricProjection: bool,
    pub SubsampledLayout: bool,
    pub FoveatedRenderingMethod: crate::Unity::XR::Oculus::OculusSettings_FoveationMethod,
    pub LateLatching: bool,
    pub LateLatchingDebug: bool,
    pub EnableTrackingOriginStageMode: bool,
    pub SpaceWarp: bool,
    pub TargetQuest: bool,
    pub TargetQuest2: bool,
    pub TargetQuestPro: bool,
    pub SystemSplashScreen: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::OculusSettings =>
    "Unity.XR.Oculus"."OculusSettings"
);
#[cfg(feature = "Unity+XR+Oculus+OculusSettings")]
impl std::ops::Deref for crate::Unity::XR::Oculus::OculusSettings {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::OculusSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings")]
impl crate::Unity::XR::Oculus::OculusSettings {
    #[cfg(feature = "Unity+XR+Oculus+OculusSettings+FoveationMethod")]
    pub type FoveationMethod = crate::Unity::XR::Oculus::OculusSettings_FoveationMethod;
    #[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeAndroid")]
    pub type StereoRenderingModeAndroid = crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeAndroid;
    #[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeDesktop")]
    pub type StereoRenderingModeDesktop = crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeDesktop;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStereoRenderingMode(&mut self) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u16 = __cordl_object.invoke("GetStereoRenderingMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "Unity+XR+Oculus+OculusSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::OculusSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+FoveationMethod")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OculusSettings_FoveationMethod {
    #[default]
    EyeTrackedFoveatedRendering = 1i32,
    FixedFoveatedRendering = 0i32,
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+FoveationMethod")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::XR::Oculus::OculusSettings_FoveationMethod => "Unity.XR.Oculus"
    ."OculusSettings/FoveationMethod"
);
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeAndroid")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OculusSettings_StereoRenderingModeAndroid {
    #[default]
    MultiPass = 0i32,
    Multiview = 2i32,
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeAndroid")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::XR::Oculus::OculusSettings_StereoRenderingModeAndroid => "Unity.XR.Oculus"
    ."OculusSettings/StereoRenderingModeAndroid"
);
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeDesktop")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OculusSettings_StereoRenderingModeDesktop {
    #[default]
    MultiPass = 0i32,
    SinglePassInstanced = 1i32,
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeDesktop")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::XR::Oculus::OculusSettings_StereoRenderingModeDesktop => "Unity.XR.Oculus"
    ."OculusSettings/StereoRenderingModeDesktop"
);

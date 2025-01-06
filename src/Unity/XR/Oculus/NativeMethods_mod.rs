#[cfg(feature = "Unity+XR+Oculus+NativeMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::NativeMethods =>
    "Unity.XR.Oculus"."NativeMethods"
);
#[cfg(feature = "Unity+XR+Oculus+NativeMethods")]
impl std::ops::Deref for crate::Unity::XR::Oculus::NativeMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::NativeMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods")]
impl crate::Unity::XR::Oculus::NativeMethods {
    #[cfg(feature = "Unity+XR+Oculus+NativeMethods+Internal")]
    pub type Internal = crate::Unity::XR::Oculus::NativeMethods_Internal;
    #[cfg(feature = "Unity+XR+Oculus+NativeMethods+UserDefinedSettings")]
    pub type UserDefinedSettings = crate::Unity::XR::Oculus::NativeMethods_UserDefinedSettings;
    pub fn EnableAppMetrics(
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnableAppMetrics", (enable))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnablePerfMetrics(
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnablePerfMetrics", (enable))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAppShouldQuit() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAppShouldQuit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoundaryConfigured() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBoundaryConfigured", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoundaryDimensions(
        boundaryType: crate::Unity::XR::Oculus::Boundary_BoundaryType,
        dimensions: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBoundaryDimensions", (boundaryType, dimensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoundaryVisible() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBoundaryVisible", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDisplayAvailableFrequencies(
        ptr: crate::System::IntPtr,
        numFrequencies: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDisplayAvailableFrequencies", (ptr, numFrequencies))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDisplayFrequency(
        refreshRate: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDisplayFrequency", (refreshRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEyeTrackedFoveatedRenderingEnabled() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEyeTrackedFoveatedRenderingEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEyeTrackedFoveatedRenderingSupported() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEyeTrackedFoveatedRenderingSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHasInputFocus() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHasInputFocus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIsSupportedDevice() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIsSupportedDevice", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOVRPVersion(
        version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOVRPVersion", (version))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetShouldRestartSession() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetShouldRestartSession", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSystemHeadsetType() -> quest_hook::libil2cpp::Result<
        crate::Unity::XR::Oculus::SystemHeadset,
    > {
        let __cordl_ret: crate::Unity::XR::Oculus::SystemHeadset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSystemHeadsetType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTiledMultiResLevel() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTiledMultiResLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTiledMultiResSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTiledMultiResSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadOVRPlugin(
        ovrpPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadOVRPlugin", (ovrpPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBoundaryVisible(
        boundaryVisible: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetBoundaryVisible", (boundaryVisible))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCPULevel(cpuLevel: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetCPULevel", (cpuLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColorOffset(
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetColorOffset", (x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColorScale(
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetColorScale", (x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDeveloperModeStrict(active: bool) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDeveloperModeStrict", (active))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDisplayFrequency(refreshRate: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDisplayFrequency", (refreshRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetEyeTrackedFoveatedRenderingEnabled(
        isEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetEyeTrackedFoveatedRenderingEnabled", (isEnabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGPULevel(gpuLevel: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGPULevel", (gpuLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTiledMultiResDynamic(
        isDynamic: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetTiledMultiResDynamic", (isDynamic))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTiledMultiResLevel(
        level: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetTiledMultiResLevel", (level))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUserDefinedSettings(
        settings: crate::Unity::XR::Oculus::NativeMethods_UserDefinedSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetUserDefinedSettings", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadOVRPlugin() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadOVRPlugin", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::NativeMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+Internal")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeMethods_Internal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+Internal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::NativeMethods_Internal =>
    "Unity.XR.Oculus"."NativeMethods/Internal"
);
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+Internal")]
impl std::ops::Deref for crate::Unity::XR::Oculus::NativeMethods_Internal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+Internal")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::NativeMethods_Internal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+Internal")]
impl crate::Unity::XR::Oculus::NativeMethods_Internal {
    pub fn EnableAppMetrics(
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnableAppMetrics", (enable))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnablePerfMetrics(
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnablePerfMetrics", (enable))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAppHasInputFocus() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAppHasInputFocus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAppShouldQuit() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAppShouldQuit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoundaryConfigured() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBoundaryConfigured", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoundaryDimensions(
        boundaryType: crate::Unity::XR::Oculus::Boundary_BoundaryType,
        dimensions: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBoundaryDimensions", (boundaryType, dimensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoundaryVisible() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBoundaryVisible", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDisplayAvailableFrequencies(
        ptr: crate::System::IntPtr,
        numFrequencies: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDisplayAvailableFrequencies", (ptr, numFrequencies))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDisplayFrequency(
        refreshRate: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDisplayFrequency", (refreshRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEyeTrackedFoveatedRenderingEnabled() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEyeTrackedFoveatedRenderingEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEyeTrackedFoveatedRenderingSupported() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEyeTrackedFoveatedRenderingSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIsSupportedDevice() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIsSupportedDevice", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOVRPVersion(
        version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOVRPVersion", (version))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetShouldRestartSession() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetShouldRestartSession", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSystemHeadsetType() -> quest_hook::libil2cpp::Result<
        crate::Unity::XR::Oculus::SystemHeadset,
    > {
        let __cordl_ret: crate::Unity::XR::Oculus::SystemHeadset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSystemHeadsetType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTiledMultiResLevel() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTiledMultiResLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTiledMultiResSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTiledMultiResSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadOVRPlugin(
        ovrpPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadOVRPlugin", (ovrpPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBoundaryVisible(
        boundaryVisible: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetBoundaryVisible", (boundaryVisible))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCPULevel(cpuLevel: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetCPULevel", (cpuLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColorOffset(
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetColorOffset", (x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColorScale(
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetColorScale", (x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDeveloperModeStrict(active: bool) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDeveloperModeStrict", (active))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDisplayFrequency(refreshRate: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDisplayFrequency", (refreshRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetEyeTrackedFoveatedRenderingEnabled(
        isEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetEyeTrackedFoveatedRenderingEnabled", (isEnabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGPULevel(gpuLevel: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGPULevel", (gpuLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTiledMultiResDynamic(
        isDynamic: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetTiledMultiResDynamic", (isDynamic))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTiledMultiResLevel(
        level: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetTiledMultiResLevel", (level))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUserDefinedSettings(
        settings: crate::Unity::XR::Oculus::NativeMethods_UserDefinedSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetUserDefinedSettings", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadOVRPlugin() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadOVRPlugin", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+Internal")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::XR::Oculus::NativeMethods_Internal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+UserDefinedSettings")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NativeMethods_UserDefinedSettings {
    pub sharedDepthBuffer: u16,
    pub dashSupport: u16,
    pub stereoRenderingMode: u16,
    pub colorSpace: u16,
    pub lowOverheadMode: u16,
    pub optimizeBufferDiscards: u16,
    pub phaseSync: u16,
    pub symmetricProjection: u16,
    pub subsampledLayout: u16,
    pub lateLatching: u16,
    pub lateLatchingDebug: u16,
    pub enableTrackingOriginStageMode: u16,
    pub spaceWarp: u16,
    pub depthSubmission: u16,
    pub foveatedRenderingMethod: u16,
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+UserDefinedSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::XR::Oculus::NativeMethods_UserDefinedSettings => "Unity.XR.Oculus"
    ."NativeMethods/UserDefinedSettings"
);
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+UserDefinedSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::XR::Oculus::NativeMethods_UserDefinedSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+UserDefinedSettings")]
impl crate::Unity::XR::Oculus::NativeMethods_UserDefinedSettings {}

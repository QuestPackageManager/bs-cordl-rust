#[cfg(feature = "OVR+OpenVR+CVRCompositor")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRCompositor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRCompositor,
}
#[cfg(feature = "OVR+OpenVR+CVRCompositor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRCompositor => "OVR.OpenVR"
    ."CVRCompositor"
);
#[cfg(feature = "OVR+OpenVR+CVRCompositor")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRCompositor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRCompositor")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRCompositor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRCompositor")]
impl crate::OVR::OpenVR::CVRCompositor {
    pub fn CanRenderScene(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanRenderScene", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearLastSubmittedFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearLastSubmittedFrame", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearSkyboxOverride(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearSkyboxOverride", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompositorBringToFront(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompositorBringToFront", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompositorDumpImages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompositorDumpImages", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompositorGoToBack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompositorGoToBack", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompositorQuit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompositorQuit", ())?;
        Ok(__cordl_ret)
    }
    pub fn FadeGrid(
        &mut self,
        fSeconds: f32,
        bFadeIn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FadeGrid", (fSeconds, bFadeIn))?;
        Ok(__cordl_ret)
    }
    pub fn FadeToColor(
        &mut self,
        fSeconds: f32,
        fRed: f32,
        fGreen: f32,
        fBlue: f32,
        fAlpha: f32,
        bBackground: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "FadeToColor",
                (fSeconds, fRed, fGreen, fBlue, fAlpha, bBackground),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ForceInterleavedReprojectionOn(
        &mut self,
        bOverride: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ForceInterleavedReprojectionOn", (bOverride))?;
        Ok(__cordl_ret)
    }
    pub fn ForceReconnectProcess(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ForceReconnectProcess", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCumulativeStats(
        &mut self,
        pStats: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::Compositor_CumulativeStats,
        >,
        nStatsSizeInBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetCumulativeStats", (pStats, nStatsSizeInBytes))?;
        Ok(__cordl_ret)
    }
    pub fn GetCurrentFadeColor(
        &mut self,
        bBackground: bool,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdColor_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdColor_t = __cordl_object
            .invoke("GetCurrentFadeColor", (bBackground))?;
        Ok(__cordl_ret)
    }
    pub fn GetCurrentGridAlpha(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetCurrentGridAlpha", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCurrentSceneFocusProcess(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetCurrentSceneFocusProcess", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetFrameTimeRemaining(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetFrameTimeRemaining", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetFrameTiming(
        &mut self,
        pTiming: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::Compositor_FrameTiming,
        >,
        unFramesAgo: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetFrameTiming", (pTiming, unFramesAgo))?;
        Ok(__cordl_ret)
    }
    pub fn GetFrameTimings(
        &mut self,
        pTiming: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::Compositor_FrameTiming,
        >,
        nFrames: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("GetFrameTimings", (pTiming, nFrames))?;
        Ok(__cordl_ret)
    }
    pub fn GetLastFrameRenderer(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetLastFrameRenderer", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLastPoseForTrackedDeviceIndex(
        &mut self,
        unDeviceIndex: u32,
        pOutputPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        pOutputGamePose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke(
                "GetLastPoseForTrackedDeviceIndex",
                (unDeviceIndex, pOutputPose, pOutputGamePose),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetLastPoses(
        &mut self,
        pRenderPoseArray: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        pGamePoseArray: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke("GetLastPoses", (pRenderPoseArray, pGamePoseArray))?;
        Ok(__cordl_ret)
    }
    pub fn GetMirrorTextureD3D11(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        pD3D11DeviceOrResource: crate::System::IntPtr,
        ppD3D11ShaderResourceView: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke(
                "GetMirrorTextureD3D11",
                (eEye, pD3D11DeviceOrResource, ppD3D11ShaderResourceView),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetMirrorTextureGL(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        pglTextureId: quest_hook::libil2cpp::ByRefMut<u32>,
        pglSharedTextureHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke("GetMirrorTextureGL", (eEye, pglTextureId, pglSharedTextureHandle))?;
        Ok(__cordl_ret)
    }
    pub fn GetTrackingSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::ETrackingUniverseOrigin> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::ETrackingUniverseOrigin = __cordl_object
            .invoke("GetTrackingSpace", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetVulkanDeviceExtensionsRequired(
        &mut self,
        pPhysicalDevice: crate::System::IntPtr,
        pchValue: *mut crate::System::Text::StringBuilder,
        unBufferSize: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "GetVulkanDeviceExtensionsRequired",
                (pPhysicalDevice, pchValue, unBufferSize),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetVulkanInstanceExtensionsRequired(
        &mut self,
        pchValue: *mut crate::System::Text::StringBuilder,
        unBufferSize: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("GetVulkanInstanceExtensionsRequired", (pchValue, unBufferSize))?;
        Ok(__cordl_ret)
    }
    pub fn HideMirrorWindow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideMirrorWindow", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsFullscreen(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsFullscreen", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsMirrorWindowVisible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsMirrorWindowVisible", ())?;
        Ok(__cordl_ret)
    }
    pub fn LockGLSharedTextureForAccess(
        &mut self,
        glSharedTextureHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LockGLSharedTextureForAccess", (glSharedTextureHandle))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object)
    }
    pub fn PostPresentHandoff(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PostPresentHandoff", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseMirrorTextureD3D11(
        &mut self,
        pD3D11ShaderResourceView: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleaseMirrorTextureD3D11", (pD3D11ShaderResourceView))?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseSharedGLTexture(
        &mut self,
        glTextureId: u32,
        glSharedTextureHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ReleaseSharedGLTexture", (glTextureId, glSharedTextureHandle))?;
        Ok(__cordl_ret)
    }
    pub fn SetExplicitTimingMode(
        &mut self,
        eTimingMode: crate::OVR::OpenVR::EVRCompositorTimingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetExplicitTimingMode", (eTimingMode))?;
        Ok(__cordl_ret)
    }
    pub fn SetSkyboxOverride(
        &mut self,
        pTextures: *mut quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::Texture_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke("SetSkyboxOverride", (pTextures))?;
        Ok(__cordl_ret)
    }
    pub fn SetTrackingSpace(
        &mut self,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTrackingSpace", (eOrigin))?;
        Ok(__cordl_ret)
    }
    pub fn ShouldAppRenderWithLowResources(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldAppRenderWithLowResources", ())?;
        Ok(__cordl_ret)
    }
    pub fn ShowMirrorWindow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowMirrorWindow", ())?;
        Ok(__cordl_ret)
    }
    pub fn Submit(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        pTexture: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::Texture_t>,
        pBounds: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VRTextureBounds_t>,
        nSubmitFlags: crate::OVR::OpenVR::EVRSubmitFlags,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke("Submit", (eEye, pTexture, pBounds, nSubmitFlags))?;
        Ok(__cordl_ret)
    }
    pub fn SubmitExplicitTimingData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke("SubmitExplicitTimingData", ())?;
        Ok(__cordl_ret)
    }
    pub fn SuspendRendering(
        &mut self,
        bSuspend: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SuspendRendering", (bSuspend))?;
        Ok(__cordl_ret)
    }
    pub fn UnlockGLSharedTextureForAccess(
        &mut self,
        glSharedTextureHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnlockGLSharedTextureForAccess", (glSharedTextureHandle))?;
        Ok(__cordl_ret)
    }
    pub fn WaitGetPoses(
        &mut self,
        pRenderPoseArray: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        pGamePoseArray: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke("WaitGetPoses", (pRenderPoseArray, pGamePoseArray))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pInterface))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+CVRCompositor")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRCompositor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

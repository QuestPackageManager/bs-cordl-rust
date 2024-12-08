#[cfg(feature = "OVR+OpenVR+IVRCompositor")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct IVRCompositor {
    pub SetTrackingSpace: *mut crate::OVR::OpenVR::IVRCompositor__SetTrackingSpace,
    pub GetTrackingSpace: *mut crate::OVR::OpenVR::IVRCompositor__GetTrackingSpace,
    pub WaitGetPoses: *mut crate::OVR::OpenVR::IVRCompositor__WaitGetPoses,
    pub GetLastPoses: *mut crate::OVR::OpenVR::IVRCompositor__GetLastPoses,
    pub GetLastPoseForTrackedDeviceIndex: *mut crate::OVR::OpenVR::IVRCompositor__GetLastPoseForTrackedDeviceIndex,
    pub Submit: *mut crate::OVR::OpenVR::IVRCompositor__Submit,
    pub ClearLastSubmittedFrame: *mut crate::OVR::OpenVR::IVRCompositor__ClearLastSubmittedFrame,
    pub PostPresentHandoff: *mut crate::OVR::OpenVR::IVRCompositor__PostPresentHandoff,
    pub GetFrameTiming: *mut crate::OVR::OpenVR::IVRCompositor__GetFrameTiming,
    pub GetFrameTimings: *mut crate::OVR::OpenVR::IVRCompositor__GetFrameTimings,
    pub GetFrameTimeRemaining: *mut crate::OVR::OpenVR::IVRCompositor__GetFrameTimeRemaining,
    pub GetCumulativeStats: *mut crate::OVR::OpenVR::IVRCompositor__GetCumulativeStats,
    pub FadeToColor: *mut crate::OVR::OpenVR::IVRCompositor__FadeToColor,
    pub GetCurrentFadeColor: *mut crate::OVR::OpenVR::IVRCompositor__GetCurrentFadeColor,
    pub FadeGrid: *mut crate::OVR::OpenVR::IVRCompositor__FadeGrid,
    pub GetCurrentGridAlpha: *mut crate::OVR::OpenVR::IVRCompositor__GetCurrentGridAlpha,
    pub SetSkyboxOverride: *mut crate::OVR::OpenVR::IVRCompositor__SetSkyboxOverride,
    pub ClearSkyboxOverride: *mut crate::OVR::OpenVR::IVRCompositor__ClearSkyboxOverride,
    pub CompositorBringToFront: *mut crate::OVR::OpenVR::IVRCompositor__CompositorBringToFront,
    pub CompositorGoToBack: *mut crate::OVR::OpenVR::IVRCompositor__CompositorGoToBack,
    pub CompositorQuit: *mut crate::OVR::OpenVR::IVRCompositor__CompositorQuit,
    pub IsFullscreen: *mut crate::OVR::OpenVR::IVRCompositor__IsFullscreen,
    pub GetCurrentSceneFocusProcess: *mut crate::OVR::OpenVR::IVRCompositor__GetCurrentSceneFocusProcess,
    pub GetLastFrameRenderer: *mut crate::OVR::OpenVR::IVRCompositor__GetLastFrameRenderer,
    pub CanRenderScene: *mut crate::OVR::OpenVR::IVRCompositor__CanRenderScene,
    pub ShowMirrorWindow: *mut crate::OVR::OpenVR::IVRCompositor__ShowMirrorWindow,
    pub HideMirrorWindow: *mut crate::OVR::OpenVR::IVRCompositor__HideMirrorWindow,
    pub IsMirrorWindowVisible: *mut crate::OVR::OpenVR::IVRCompositor__IsMirrorWindowVisible,
    pub CompositorDumpImages: *mut crate::OVR::OpenVR::IVRCompositor__CompositorDumpImages,
    pub ShouldAppRenderWithLowResources: *mut crate::OVR::OpenVR::IVRCompositor__ShouldAppRenderWithLowResources,
    pub ForceInterleavedReprojectionOn: *mut crate::OVR::OpenVR::IVRCompositor__ForceInterleavedReprojectionOn,
    pub ForceReconnectProcess: *mut crate::OVR::OpenVR::IVRCompositor__ForceReconnectProcess,
    pub SuspendRendering: *mut crate::OVR::OpenVR::IVRCompositor__SuspendRendering,
    pub GetMirrorTextureD3D11: *mut crate::OVR::OpenVR::IVRCompositor__GetMirrorTextureD3D11,
    pub ReleaseMirrorTextureD3D11: *mut crate::OVR::OpenVR::IVRCompositor__ReleaseMirrorTextureD3D11,
    pub GetMirrorTextureGL: *mut crate::OVR::OpenVR::IVRCompositor__GetMirrorTextureGL,
    pub ReleaseSharedGLTexture: *mut crate::OVR::OpenVR::IVRCompositor__ReleaseSharedGLTexture,
    pub LockGLSharedTextureForAccess: *mut crate::OVR::OpenVR::IVRCompositor__LockGLSharedTextureForAccess,
    pub UnlockGLSharedTextureForAccess: *mut crate::OVR::OpenVR::IVRCompositor__UnlockGLSharedTextureForAccess,
    pub GetVulkanInstanceExtensionsRequired: *mut crate::OVR::OpenVR::IVRCompositor__GetVulkanInstanceExtensionsRequired,
    pub GetVulkanDeviceExtensionsRequired: *mut crate::OVR::OpenVR::IVRCompositor__GetVulkanDeviceExtensionsRequired,
    pub SetExplicitTimingMode: *mut crate::OVR::OpenVR::IVRCompositor__SetExplicitTimingMode,
    pub SubmitExplicitTimingData: *mut crate::OVR::OpenVR::IVRCompositor__SubmitExplicitTimingData,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor => "OVR.OpenVR"
    ."IVRCompositor"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::IVRCompositor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor")]
impl crate::OVR::OpenVR::IVRCompositor {
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_CanRenderScene")]
    pub type _CanRenderScene = crate::OVR::OpenVR::IVRCompositor__CanRenderScene;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_ClearLastSubmittedFrame")]
    pub type _ClearLastSubmittedFrame = crate::OVR::OpenVR::IVRCompositor__ClearLastSubmittedFrame;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_ClearSkyboxOverride")]
    pub type _ClearSkyboxOverride = crate::OVR::OpenVR::IVRCompositor__ClearSkyboxOverride;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorBringToFront")]
    pub type _CompositorBringToFront = crate::OVR::OpenVR::IVRCompositor__CompositorBringToFront;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorDumpImages")]
    pub type _CompositorDumpImages = crate::OVR::OpenVR::IVRCompositor__CompositorDumpImages;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorGoToBack")]
    pub type _CompositorGoToBack = crate::OVR::OpenVR::IVRCompositor__CompositorGoToBack;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorQuit")]
    pub type _CompositorQuit = crate::OVR::OpenVR::IVRCompositor__CompositorQuit;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_FadeGrid")]
    pub type _FadeGrid = crate::OVR::OpenVR::IVRCompositor__FadeGrid;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_FadeToColor")]
    pub type _FadeToColor = crate::OVR::OpenVR::IVRCompositor__FadeToColor;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_ForceInterleavedReprojectionOn")]
    pub type _ForceInterleavedReprojectionOn = crate::OVR::OpenVR::IVRCompositor__ForceInterleavedReprojectionOn;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_ForceReconnectProcess")]
    pub type _ForceReconnectProcess = crate::OVR::OpenVR::IVRCompositor__ForceReconnectProcess;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCumulativeStats")]
    pub type _GetCumulativeStats = crate::OVR::OpenVR::IVRCompositor__GetCumulativeStats;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentFadeColor")]
    pub type _GetCurrentFadeColor = crate::OVR::OpenVR::IVRCompositor__GetCurrentFadeColor;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentGridAlpha")]
    pub type _GetCurrentGridAlpha = crate::OVR::OpenVR::IVRCompositor__GetCurrentGridAlpha;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentSceneFocusProcess")]
    pub type _GetCurrentSceneFocusProcess = crate::OVR::OpenVR::IVRCompositor__GetCurrentSceneFocusProcess;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTimeRemaining")]
    pub type _GetFrameTimeRemaining = crate::OVR::OpenVR::IVRCompositor__GetFrameTimeRemaining;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTiming")]
    pub type _GetFrameTiming = crate::OVR::OpenVR::IVRCompositor__GetFrameTiming;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTimings")]
    pub type _GetFrameTimings = crate::OVR::OpenVR::IVRCompositor__GetFrameTimings;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastFrameRenderer")]
    pub type _GetLastFrameRenderer = crate::OVR::OpenVR::IVRCompositor__GetLastFrameRenderer;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastPoseForTrackedDeviceIndex")]
    pub type _GetLastPoseForTrackedDeviceIndex = crate::OVR::OpenVR::IVRCompositor__GetLastPoseForTrackedDeviceIndex;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastPoses")]
    pub type _GetLastPoses = crate::OVR::OpenVR::IVRCompositor__GetLastPoses;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetMirrorTextureD3D11")]
    pub type _GetMirrorTextureD3D11 = crate::OVR::OpenVR::IVRCompositor__GetMirrorTextureD3D11;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetMirrorTextureGL")]
    pub type _GetMirrorTextureGL = crate::OVR::OpenVR::IVRCompositor__GetMirrorTextureGL;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetTrackingSpace")]
    pub type _GetTrackingSpace = crate::OVR::OpenVR::IVRCompositor__GetTrackingSpace;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetVulkanDeviceExtensionsRequired")]
    pub type _GetVulkanDeviceExtensionsRequired = crate::OVR::OpenVR::IVRCompositor__GetVulkanDeviceExtensionsRequired;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetVulkanInstanceExtensionsRequired")]
    pub type _GetVulkanInstanceExtensionsRequired = crate::OVR::OpenVR::IVRCompositor__GetVulkanInstanceExtensionsRequired;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_HideMirrorWindow")]
    pub type _HideMirrorWindow = crate::OVR::OpenVR::IVRCompositor__HideMirrorWindow;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_IsFullscreen")]
    pub type _IsFullscreen = crate::OVR::OpenVR::IVRCompositor__IsFullscreen;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_IsMirrorWindowVisible")]
    pub type _IsMirrorWindowVisible = crate::OVR::OpenVR::IVRCompositor__IsMirrorWindowVisible;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_LockGLSharedTextureForAccess")]
    pub type _LockGLSharedTextureForAccess = crate::OVR::OpenVR::IVRCompositor__LockGLSharedTextureForAccess;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_PostPresentHandoff")]
    pub type _PostPresentHandoff = crate::OVR::OpenVR::IVRCompositor__PostPresentHandoff;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_ReleaseMirrorTextureD3D11")]
    pub type _ReleaseMirrorTextureD3D11 = crate::OVR::OpenVR::IVRCompositor__ReleaseMirrorTextureD3D11;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_ReleaseSharedGLTexture")]
    pub type _ReleaseSharedGLTexture = crate::OVR::OpenVR::IVRCompositor__ReleaseSharedGLTexture;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetExplicitTimingMode")]
    pub type _SetExplicitTimingMode = crate::OVR::OpenVR::IVRCompositor__SetExplicitTimingMode;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetSkyboxOverride")]
    pub type _SetSkyboxOverride = crate::OVR::OpenVR::IVRCompositor__SetSkyboxOverride;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetTrackingSpace")]
    pub type _SetTrackingSpace = crate::OVR::OpenVR::IVRCompositor__SetTrackingSpace;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_ShouldAppRenderWithLowResources")]
    pub type _ShouldAppRenderWithLowResources = crate::OVR::OpenVR::IVRCompositor__ShouldAppRenderWithLowResources;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_ShowMirrorWindow")]
    pub type _ShowMirrorWindow = crate::OVR::OpenVR::IVRCompositor__ShowMirrorWindow;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_Submit")]
    pub type _Submit = crate::OVR::OpenVR::IVRCompositor__Submit;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_SubmitExplicitTimingData")]
    pub type _SubmitExplicitTimingData = crate::OVR::OpenVR::IVRCompositor__SubmitExplicitTimingData;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_SuspendRendering")]
    pub type _SuspendRendering = crate::OVR::OpenVR::IVRCompositor__SuspendRendering;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_UnlockGLSharedTextureForAccess")]
    pub type _UnlockGLSharedTextureForAccess = crate::OVR::OpenVR::IVRCompositor__UnlockGLSharedTextureForAccess;
    #[cfg(feature = "OVR+OpenVR+IVRCompositor+_WaitGetPoses")]
    pub type _WaitGetPoses = crate::OVR::OpenVR::IVRCompositor__WaitGetPoses;
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CanRenderScene")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__CanRenderScene {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CanRenderScene")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__CanRenderScene =>
    "OVR.OpenVR"."IVRCompositor/_CanRenderScene"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CanRenderScene")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__CanRenderScene {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CanRenderScene")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__CanRenderScene {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CanRenderScene")]
impl crate::OVR::OpenVR::IVRCompositor__CanRenderScene {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CanRenderScene")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__CanRenderScene {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ClearLastSubmittedFrame")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__ClearLastSubmittedFrame {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ClearLastSubmittedFrame")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__ClearLastSubmittedFrame => "OVR.OpenVR"
    ."IVRCompositor/_ClearLastSubmittedFrame"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ClearLastSubmittedFrame")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__ClearLastSubmittedFrame {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ClearLastSubmittedFrame")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__ClearLastSubmittedFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ClearLastSubmittedFrame")]
impl crate::OVR::OpenVR::IVRCompositor__ClearLastSubmittedFrame {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ClearLastSubmittedFrame")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__ClearLastSubmittedFrame {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ClearSkyboxOverride")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__ClearSkyboxOverride {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ClearSkyboxOverride")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__ClearSkyboxOverride
    => "OVR.OpenVR"."IVRCompositor/_ClearSkyboxOverride"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ClearSkyboxOverride")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__ClearSkyboxOverride {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ClearSkyboxOverride")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__ClearSkyboxOverride {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ClearSkyboxOverride")]
impl crate::OVR::OpenVR::IVRCompositor__ClearSkyboxOverride {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ClearSkyboxOverride")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__ClearSkyboxOverride {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorBringToFront")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__CompositorBringToFront {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorBringToFront")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__CompositorBringToFront => "OVR.OpenVR"
    ."IVRCompositor/_CompositorBringToFront"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorBringToFront")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__CompositorBringToFront {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorBringToFront")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__CompositorBringToFront {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorBringToFront")]
impl crate::OVR::OpenVR::IVRCompositor__CompositorBringToFront {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorBringToFront")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__CompositorBringToFront {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorDumpImages")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__CompositorDumpImages {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorDumpImages")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__CompositorDumpImages
    => "OVR.OpenVR"."IVRCompositor/_CompositorDumpImages"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorDumpImages")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__CompositorDumpImages {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorDumpImages")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__CompositorDumpImages {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorDumpImages")]
impl crate::OVR::OpenVR::IVRCompositor__CompositorDumpImages {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorDumpImages")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__CompositorDumpImages {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorGoToBack")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__CompositorGoToBack {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorGoToBack")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__CompositorGoToBack
    => "OVR.OpenVR"."IVRCompositor/_CompositorGoToBack"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorGoToBack")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__CompositorGoToBack {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorGoToBack")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__CompositorGoToBack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorGoToBack")]
impl crate::OVR::OpenVR::IVRCompositor__CompositorGoToBack {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorGoToBack")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__CompositorGoToBack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorQuit")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__CompositorQuit {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorQuit")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__CompositorQuit =>
    "OVR.OpenVR"."IVRCompositor/_CompositorQuit"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorQuit")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__CompositorQuit {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorQuit")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__CompositorQuit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorQuit")]
impl crate::OVR::OpenVR::IVRCompositor__CompositorQuit {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_CompositorQuit")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__CompositorQuit {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_FadeGrid")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__FadeGrid {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_FadeGrid")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__FadeGrid =>
    "OVR.OpenVR"."IVRCompositor/_FadeGrid"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_FadeGrid")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__FadeGrid {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_FadeGrid")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__FadeGrid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_FadeGrid")]
impl crate::OVR::OpenVR::IVRCompositor__FadeGrid {
    pub fn BeginInvoke(
        &mut self,
        fSeconds: f32,
        bFadeIn: bool,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (fSeconds, bFadeIn, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        fSeconds: f32,
        bFadeIn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (fSeconds, bFadeIn))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_FadeGrid")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRCompositor__FadeGrid {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_FadeToColor")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__FadeToColor {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_FadeToColor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__FadeToColor =>
    "OVR.OpenVR"."IVRCompositor/_FadeToColor"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_FadeToColor")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__FadeToColor {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_FadeToColor")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__FadeToColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_FadeToColor")]
impl crate::OVR::OpenVR::IVRCompositor__FadeToColor {
    pub fn BeginInvoke(
        &mut self,
        fSeconds: f32,
        fRed: f32,
        fGreen: f32,
        fBlue: f32,
        fAlpha: f32,
        bBackground: bool,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (fSeconds, fRed, fGreen, fBlue, fAlpha, bBackground, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
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
            .invoke("Invoke", (fSeconds, fRed, fGreen, fBlue, fAlpha, bBackground))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_FadeToColor")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__FadeToColor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ForceInterleavedReprojectionOn")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__ForceInterleavedReprojectionOn {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ForceInterleavedReprojectionOn")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__ForceInterleavedReprojectionOn => "OVR.OpenVR"
    ."IVRCompositor/_ForceInterleavedReprojectionOn"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ForceInterleavedReprojectionOn")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRCompositor__ForceInterleavedReprojectionOn {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ForceInterleavedReprojectionOn")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRCompositor__ForceInterleavedReprojectionOn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ForceInterleavedReprojectionOn")]
impl crate::OVR::OpenVR::IVRCompositor__ForceInterleavedReprojectionOn {
    pub fn BeginInvoke(
        &mut self,
        bOverride: bool,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (bOverride, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        bOverride: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (bOverride))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ForceInterleavedReprojectionOn")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__ForceInterleavedReprojectionOn {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ForceReconnectProcess")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__ForceReconnectProcess {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ForceReconnectProcess")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__ForceReconnectProcess => "OVR.OpenVR"
    ."IVRCompositor/_ForceReconnectProcess"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ForceReconnectProcess")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__ForceReconnectProcess {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ForceReconnectProcess")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__ForceReconnectProcess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ForceReconnectProcess")]
impl crate::OVR::OpenVR::IVRCompositor__ForceReconnectProcess {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ForceReconnectProcess")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__ForceReconnectProcess {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCumulativeStats")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__GetCumulativeStats {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCumulativeStats")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__GetCumulativeStats
    => "OVR.OpenVR"."IVRCompositor/_GetCumulativeStats"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCumulativeStats")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__GetCumulativeStats {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCumulativeStats")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__GetCumulativeStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCumulativeStats")]
impl crate::OVR::OpenVR::IVRCompositor__GetCumulativeStats {
    pub fn BeginInvoke(
        &mut self,
        pStats: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::Compositor_CumulativeStats,
        >,
        nStatsSizeInBytes: u32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (pStats, nStatsSizeInBytes, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        pStats: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::Compositor_CumulativeStats,
        >,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (pStats, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
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
            .invoke("Invoke", (pStats, nStatsSizeInBytes))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCumulativeStats")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__GetCumulativeStats {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentFadeColor")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__GetCurrentFadeColor {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentFadeColor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__GetCurrentFadeColor
    => "OVR.OpenVR"."IVRCompositor/_GetCurrentFadeColor"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentFadeColor")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__GetCurrentFadeColor {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentFadeColor")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__GetCurrentFadeColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentFadeColor")]
impl crate::OVR::OpenVR::IVRCompositor__GetCurrentFadeColor {
    pub fn BeginInvoke(
        &mut self,
        bBackground: bool,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (bBackground, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdColor_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdColor_t = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        bBackground: bool,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdColor_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdColor_t = __cordl_object
            .invoke("Invoke", (bBackground))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentFadeColor")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__GetCurrentFadeColor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentGridAlpha")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__GetCurrentGridAlpha {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentGridAlpha")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__GetCurrentGridAlpha
    => "OVR.OpenVR"."IVRCompositor/_GetCurrentGridAlpha"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentGridAlpha")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__GetCurrentGridAlpha {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentGridAlpha")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__GetCurrentGridAlpha {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentGridAlpha")]
impl crate::OVR::OpenVR::IVRCompositor__GetCurrentGridAlpha {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentGridAlpha")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__GetCurrentGridAlpha {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentSceneFocusProcess")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__GetCurrentSceneFocusProcess {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentSceneFocusProcess")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__GetCurrentSceneFocusProcess => "OVR.OpenVR"
    ."IVRCompositor/_GetCurrentSceneFocusProcess"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentSceneFocusProcess")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__GetCurrentSceneFocusProcess {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentSceneFocusProcess")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRCompositor__GetCurrentSceneFocusProcess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentSceneFocusProcess")]
impl crate::OVR::OpenVR::IVRCompositor__GetCurrentSceneFocusProcess {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetCurrentSceneFocusProcess")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__GetCurrentSceneFocusProcess {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTimeRemaining")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__GetFrameTimeRemaining {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTimeRemaining")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__GetFrameTimeRemaining => "OVR.OpenVR"
    ."IVRCompositor/_GetFrameTimeRemaining"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTimeRemaining")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__GetFrameTimeRemaining {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTimeRemaining")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__GetFrameTimeRemaining {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTimeRemaining")]
impl crate::OVR::OpenVR::IVRCompositor__GetFrameTimeRemaining {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTimeRemaining")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__GetFrameTimeRemaining {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTiming")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__GetFrameTiming {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTiming")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__GetFrameTiming =>
    "OVR.OpenVR"."IVRCompositor/_GetFrameTiming"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTiming")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__GetFrameTiming {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTiming")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__GetFrameTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTiming")]
impl crate::OVR::OpenVR::IVRCompositor__GetFrameTiming {
    pub fn BeginInvoke(
        &mut self,
        pTiming: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::Compositor_FrameTiming,
        >,
        unFramesAgo: u32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (pTiming, unFramesAgo, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        pTiming: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::Compositor_FrameTiming,
        >,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (pTiming, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pTiming: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::Compositor_FrameTiming,
        >,
        unFramesAgo: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (pTiming, unFramesAgo))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTiming")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__GetFrameTiming {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTimings")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__GetFrameTimings {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTimings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__GetFrameTimings =>
    "OVR.OpenVR"."IVRCompositor/_GetFrameTimings"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTimings")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__GetFrameTimings {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTimings")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__GetFrameTimings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTimings")]
impl crate::OVR::OpenVR::IVRCompositor__GetFrameTimings {
    pub fn BeginInvoke(
        &mut self,
        pTiming: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::Compositor_FrameTiming,
        >,
        nFrames: u32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (pTiming, nFrames, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        pTiming: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::Compositor_FrameTiming,
        >,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (pTiming, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pTiming: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::Compositor_FrameTiming,
        >,
        nFrames: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("Invoke", (pTiming, nFrames))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetFrameTimings")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__GetFrameTimings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastFrameRenderer")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__GetLastFrameRenderer {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastFrameRenderer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__GetLastFrameRenderer
    => "OVR.OpenVR"."IVRCompositor/_GetLastFrameRenderer"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastFrameRenderer")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__GetLastFrameRenderer {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastFrameRenderer")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__GetLastFrameRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastFrameRenderer")]
impl crate::OVR::OpenVR::IVRCompositor__GetLastFrameRenderer {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastFrameRenderer")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__GetLastFrameRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastPoseForTrackedDeviceIndex")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__GetLastPoseForTrackedDeviceIndex {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastPoseForTrackedDeviceIndex")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__GetLastPoseForTrackedDeviceIndex => "OVR.OpenVR"
    ."IVRCompositor/_GetLastPoseForTrackedDeviceIndex"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastPoseForTrackedDeviceIndex")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRCompositor__GetLastPoseForTrackedDeviceIndex {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastPoseForTrackedDeviceIndex")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRCompositor__GetLastPoseForTrackedDeviceIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastPoseForTrackedDeviceIndex")]
impl crate::OVR::OpenVR::IVRCompositor__GetLastPoseForTrackedDeviceIndex {
    pub fn BeginInvoke(
        &mut self,
        unDeviceIndex: u32,
        pOutputPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        pOutputGamePose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (unDeviceIndex, pOutputPose, pOutputGamePose, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        pOutputPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        pOutputGamePose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke("EndInvoke", (pOutputPose, pOutputGamePose, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
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
            .invoke("Invoke", (unDeviceIndex, pOutputPose, pOutputGamePose))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastPoseForTrackedDeviceIndex")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__GetLastPoseForTrackedDeviceIndex {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastPoses")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__GetLastPoses {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastPoses")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__GetLastPoses =>
    "OVR.OpenVR"."IVRCompositor/_GetLastPoses"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastPoses")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__GetLastPoses {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastPoses")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__GetLastPoses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastPoses")]
impl crate::OVR::OpenVR::IVRCompositor__GetLastPoses {
    pub fn BeginInvoke(
        &mut self,
        pRenderPoseArray: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::OVR::OpenVR::TrackedDevicePose_t,
            >,
        >,
        unRenderPoseArrayCount: u32,
        pGamePoseArray: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::OVR::OpenVR::TrackedDevicePose_t,
            >,
        >,
        unGamePoseArrayCount: u32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (
                    pRenderPoseArray,
                    unRenderPoseArrayCount,
                    pGamePoseArray,
                    unGamePoseArrayCount,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pRenderPoseArray: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::OVR::OpenVR::TrackedDevicePose_t,
            >,
        >,
        unRenderPoseArrayCount: u32,
        pGamePoseArray: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::OVR::OpenVR::TrackedDevicePose_t,
            >,
        >,
        unGamePoseArrayCount: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke(
                "Invoke",
                (
                    pRenderPoseArray,
                    unRenderPoseArrayCount,
                    pGamePoseArray,
                    unGamePoseArrayCount,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetLastPoses")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__GetLastPoses {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetMirrorTextureD3D11")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__GetMirrorTextureD3D11 {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetMirrorTextureD3D11")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__GetMirrorTextureD3D11 => "OVR.OpenVR"
    ."IVRCompositor/_GetMirrorTextureD3D11"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetMirrorTextureD3D11")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__GetMirrorTextureD3D11 {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetMirrorTextureD3D11")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__GetMirrorTextureD3D11 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetMirrorTextureD3D11")]
impl crate::OVR::OpenVR::IVRCompositor__GetMirrorTextureD3D11 {
    pub fn BeginInvoke(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        pD3D11DeviceOrResource: crate::System::IntPtr,
        ppD3D11ShaderResourceView: quest_hook::libil2cpp::ByRefMut<
            crate::System::IntPtr,
        >,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (
                    eEye,
                    pD3D11DeviceOrResource,
                    ppD3D11ShaderResourceView,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        ppD3D11ShaderResourceView: quest_hook::libil2cpp::ByRefMut<
            crate::System::IntPtr,
        >,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke("EndInvoke", (ppD3D11ShaderResourceView, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
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
                "Invoke",
                (eEye, pD3D11DeviceOrResource, ppD3D11ShaderResourceView),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetMirrorTextureD3D11")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__GetMirrorTextureD3D11 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetMirrorTextureGL")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__GetMirrorTextureGL {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetMirrorTextureGL")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__GetMirrorTextureGL
    => "OVR.OpenVR"."IVRCompositor/_GetMirrorTextureGL"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetMirrorTextureGL")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__GetMirrorTextureGL {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetMirrorTextureGL")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__GetMirrorTextureGL {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetMirrorTextureGL")]
impl crate::OVR::OpenVR::IVRCompositor__GetMirrorTextureGL {
    pub fn BeginInvoke(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        pglTextureId: quest_hook::libil2cpp::ByRefMut<u32>,
        pglSharedTextureHandle: crate::System::IntPtr,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (eEye, pglTextureId, pglSharedTextureHandle, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        pglTextureId: quest_hook::libil2cpp::ByRefMut<u32>,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke("EndInvoke", (pglTextureId, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        pglTextureId: quest_hook::libil2cpp::ByRefMut<u32>,
        pglSharedTextureHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke("Invoke", (eEye, pglTextureId, pglSharedTextureHandle))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetMirrorTextureGL")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__GetMirrorTextureGL {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetTrackingSpace")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__GetTrackingSpace {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetTrackingSpace")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__GetTrackingSpace =>
    "OVR.OpenVR"."IVRCompositor/_GetTrackingSpace"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetTrackingSpace")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__GetTrackingSpace {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetTrackingSpace")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__GetTrackingSpace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetTrackingSpace")]
impl crate::OVR::OpenVR::IVRCompositor__GetTrackingSpace {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::ETrackingUniverseOrigin> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::ETrackingUniverseOrigin = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::ETrackingUniverseOrigin> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::ETrackingUniverseOrigin = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetTrackingSpace")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__GetTrackingSpace {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetVulkanDeviceExtensionsRequired")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__GetVulkanDeviceExtensionsRequired {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetVulkanDeviceExtensionsRequired")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__GetVulkanDeviceExtensionsRequired => "OVR.OpenVR"
    ."IVRCompositor/_GetVulkanDeviceExtensionsRequired"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetVulkanDeviceExtensionsRequired")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRCompositor__GetVulkanDeviceExtensionsRequired {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetVulkanDeviceExtensionsRequired")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRCompositor__GetVulkanDeviceExtensionsRequired {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetVulkanDeviceExtensionsRequired")]
impl crate::OVR::OpenVR::IVRCompositor__GetVulkanDeviceExtensionsRequired {
    pub fn BeginInvoke(
        &mut self,
        pPhysicalDevice: crate::System::IntPtr,
        pchValue: *mut crate::System::Text::StringBuilder,
        unBufferSize: u32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (pPhysicalDevice, pchValue, unBufferSize, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pPhysicalDevice: crate::System::IntPtr,
        pchValue: *mut crate::System::Text::StringBuilder,
        unBufferSize: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("Invoke", (pPhysicalDevice, pchValue, unBufferSize))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetVulkanDeviceExtensionsRequired")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__GetVulkanDeviceExtensionsRequired {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetVulkanInstanceExtensionsRequired")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__GetVulkanInstanceExtensionsRequired {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetVulkanInstanceExtensionsRequired")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__GetVulkanInstanceExtensionsRequired => "OVR.OpenVR"
    ."IVRCompositor/_GetVulkanInstanceExtensionsRequired"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetVulkanInstanceExtensionsRequired")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRCompositor__GetVulkanInstanceExtensionsRequired {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetVulkanInstanceExtensionsRequired")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRCompositor__GetVulkanInstanceExtensionsRequired {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetVulkanInstanceExtensionsRequired")]
impl crate::OVR::OpenVR::IVRCompositor__GetVulkanInstanceExtensionsRequired {
    pub fn BeginInvoke(
        &mut self,
        pchValue: *mut crate::System::Text::StringBuilder,
        unBufferSize: u32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (pchValue, unBufferSize, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pchValue: *mut crate::System::Text::StringBuilder,
        unBufferSize: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("Invoke", (pchValue, unBufferSize))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_GetVulkanInstanceExtensionsRequired")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__GetVulkanInstanceExtensionsRequired {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_HideMirrorWindow")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__HideMirrorWindow {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_HideMirrorWindow")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__HideMirrorWindow =>
    "OVR.OpenVR"."IVRCompositor/_HideMirrorWindow"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_HideMirrorWindow")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__HideMirrorWindow {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_HideMirrorWindow")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__HideMirrorWindow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_HideMirrorWindow")]
impl crate::OVR::OpenVR::IVRCompositor__HideMirrorWindow {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_HideMirrorWindow")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__HideMirrorWindow {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_IsFullscreen")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__IsFullscreen {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_IsFullscreen")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__IsFullscreen =>
    "OVR.OpenVR"."IVRCompositor/_IsFullscreen"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_IsFullscreen")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__IsFullscreen {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_IsFullscreen")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__IsFullscreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_IsFullscreen")]
impl crate::OVR::OpenVR::IVRCompositor__IsFullscreen {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_IsFullscreen")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__IsFullscreen {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_IsMirrorWindowVisible")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__IsMirrorWindowVisible {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_IsMirrorWindowVisible")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__IsMirrorWindowVisible => "OVR.OpenVR"
    ."IVRCompositor/_IsMirrorWindowVisible"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_IsMirrorWindowVisible")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__IsMirrorWindowVisible {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_IsMirrorWindowVisible")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__IsMirrorWindowVisible {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_IsMirrorWindowVisible")]
impl crate::OVR::OpenVR::IVRCompositor__IsMirrorWindowVisible {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_IsMirrorWindowVisible")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__IsMirrorWindowVisible {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_LockGLSharedTextureForAccess")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__LockGLSharedTextureForAccess {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_LockGLSharedTextureForAccess")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__LockGLSharedTextureForAccess => "OVR.OpenVR"
    ."IVRCompositor/_LockGLSharedTextureForAccess"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_LockGLSharedTextureForAccess")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRCompositor__LockGLSharedTextureForAccess {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_LockGLSharedTextureForAccess")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRCompositor__LockGLSharedTextureForAccess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_LockGLSharedTextureForAccess")]
impl crate::OVR::OpenVR::IVRCompositor__LockGLSharedTextureForAccess {
    pub fn BeginInvoke(
        &mut self,
        glSharedTextureHandle: crate::System::IntPtr,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (glSharedTextureHandle, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        glSharedTextureHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (glSharedTextureHandle))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_LockGLSharedTextureForAccess")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__LockGLSharedTextureForAccess {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_PostPresentHandoff")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__PostPresentHandoff {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_PostPresentHandoff")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__PostPresentHandoff
    => "OVR.OpenVR"."IVRCompositor/_PostPresentHandoff"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_PostPresentHandoff")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__PostPresentHandoff {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_PostPresentHandoff")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__PostPresentHandoff {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_PostPresentHandoff")]
impl crate::OVR::OpenVR::IVRCompositor__PostPresentHandoff {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_PostPresentHandoff")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__PostPresentHandoff {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ReleaseMirrorTextureD3D11")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__ReleaseMirrorTextureD3D11 {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ReleaseMirrorTextureD3D11")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__ReleaseMirrorTextureD3D11 => "OVR.OpenVR"
    ."IVRCompositor/_ReleaseMirrorTextureD3D11"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ReleaseMirrorTextureD3D11")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__ReleaseMirrorTextureD3D11 {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ReleaseMirrorTextureD3D11")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRCompositor__ReleaseMirrorTextureD3D11 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ReleaseMirrorTextureD3D11")]
impl crate::OVR::OpenVR::IVRCompositor__ReleaseMirrorTextureD3D11 {
    pub fn BeginInvoke(
        &mut self,
        pD3D11ShaderResourceView: crate::System::IntPtr,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (pD3D11ShaderResourceView, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pD3D11ShaderResourceView: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pD3D11ShaderResourceView))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ReleaseMirrorTextureD3D11")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__ReleaseMirrorTextureD3D11 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ReleaseSharedGLTexture")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__ReleaseSharedGLTexture {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ReleaseSharedGLTexture")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__ReleaseSharedGLTexture => "OVR.OpenVR"
    ."IVRCompositor/_ReleaseSharedGLTexture"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ReleaseSharedGLTexture")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__ReleaseSharedGLTexture {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ReleaseSharedGLTexture")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__ReleaseSharedGLTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ReleaseSharedGLTexture")]
impl crate::OVR::OpenVR::IVRCompositor__ReleaseSharedGLTexture {
    pub fn BeginInvoke(
        &mut self,
        glTextureId: u32,
        glSharedTextureHandle: crate::System::IntPtr,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (glTextureId, glSharedTextureHandle, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        glTextureId: u32,
        glSharedTextureHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (glTextureId, glSharedTextureHandle))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ReleaseSharedGLTexture")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__ReleaseSharedGLTexture {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetExplicitTimingMode")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__SetExplicitTimingMode {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetExplicitTimingMode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__SetExplicitTimingMode => "OVR.OpenVR"
    ."IVRCompositor/_SetExplicitTimingMode"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetExplicitTimingMode")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__SetExplicitTimingMode {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetExplicitTimingMode")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__SetExplicitTimingMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetExplicitTimingMode")]
impl crate::OVR::OpenVR::IVRCompositor__SetExplicitTimingMode {
    pub fn BeginInvoke(
        &mut self,
        eTimingMode: crate::OVR::OpenVR::EVRCompositorTimingMode,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (eTimingMode, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        eTimingMode: crate::OVR::OpenVR::EVRCompositorTimingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (eTimingMode))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetExplicitTimingMode")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__SetExplicitTimingMode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetSkyboxOverride")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__SetSkyboxOverride {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetSkyboxOverride")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__SetSkyboxOverride =>
    "OVR.OpenVR"."IVRCompositor/_SetSkyboxOverride"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetSkyboxOverride")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__SetSkyboxOverride {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetSkyboxOverride")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__SetSkyboxOverride {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetSkyboxOverride")]
impl crate::OVR::OpenVR::IVRCompositor__SetSkyboxOverride {
    pub fn BeginInvoke(
        &mut self,
        pTextures: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::Texture_t>,
        >,
        unTextureCount: u32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (pTextures, unTextureCount, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pTextures: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::Texture_t>,
        >,
        unTextureCount: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke("Invoke", (pTextures, unTextureCount))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetSkyboxOverride")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__SetSkyboxOverride {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetTrackingSpace")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__SetTrackingSpace {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetTrackingSpace")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__SetTrackingSpace =>
    "OVR.OpenVR"."IVRCompositor/_SetTrackingSpace"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetTrackingSpace")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__SetTrackingSpace {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetTrackingSpace")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__SetTrackingSpace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetTrackingSpace")]
impl crate::OVR::OpenVR::IVRCompositor__SetTrackingSpace {
    pub fn BeginInvoke(
        &mut self,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (eOrigin, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (eOrigin))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SetTrackingSpace")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__SetTrackingSpace {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ShouldAppRenderWithLowResources")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__ShouldAppRenderWithLowResources {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ShouldAppRenderWithLowResources")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__ShouldAppRenderWithLowResources => "OVR.OpenVR"
    ."IVRCompositor/_ShouldAppRenderWithLowResources"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ShouldAppRenderWithLowResources")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRCompositor__ShouldAppRenderWithLowResources {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ShouldAppRenderWithLowResources")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRCompositor__ShouldAppRenderWithLowResources {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ShouldAppRenderWithLowResources")]
impl crate::OVR::OpenVR::IVRCompositor__ShouldAppRenderWithLowResources {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ShouldAppRenderWithLowResources")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__ShouldAppRenderWithLowResources {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ShowMirrorWindow")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__ShowMirrorWindow {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ShowMirrorWindow")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__ShowMirrorWindow =>
    "OVR.OpenVR"."IVRCompositor/_ShowMirrorWindow"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ShowMirrorWindow")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__ShowMirrorWindow {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ShowMirrorWindow")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__ShowMirrorWindow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ShowMirrorWindow")]
impl crate::OVR::OpenVR::IVRCompositor__ShowMirrorWindow {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_ShowMirrorWindow")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__ShowMirrorWindow {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_Submit")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__Submit {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_Submit")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__Submit =>
    "OVR.OpenVR"."IVRCompositor/_Submit"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_Submit")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__Submit {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_Submit")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__Submit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_Submit")]
impl crate::OVR::OpenVR::IVRCompositor__Submit {
    pub fn BeginInvoke(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        pTexture: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::Texture_t>,
        pBounds: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VRTextureBounds_t>,
        nSubmitFlags: crate::OVR::OpenVR::EVRSubmitFlags,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (eEye, pTexture, pBounds, nSubmitFlags, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        pTexture: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::Texture_t>,
        pBounds: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VRTextureBounds_t>,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke("EndInvoke", (pTexture, pBounds, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
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
            .invoke("Invoke", (eEye, pTexture, pBounds, nSubmitFlags))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_Submit")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRCompositor__Submit {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SubmitExplicitTimingData")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__SubmitExplicitTimingData {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SubmitExplicitTimingData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__SubmitExplicitTimingData => "OVR.OpenVR"
    ."IVRCompositor/_SubmitExplicitTimingData"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SubmitExplicitTimingData")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__SubmitExplicitTimingData {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SubmitExplicitTimingData")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__SubmitExplicitTimingData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SubmitExplicitTimingData")]
impl crate::OVR::OpenVR::IVRCompositor__SubmitExplicitTimingData {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SubmitExplicitTimingData")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__SubmitExplicitTimingData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SuspendRendering")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__SuspendRendering {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SuspendRendering")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__SuspendRendering =>
    "OVR.OpenVR"."IVRCompositor/_SuspendRendering"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SuspendRendering")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__SuspendRendering {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SuspendRendering")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__SuspendRendering {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SuspendRendering")]
impl crate::OVR::OpenVR::IVRCompositor__SuspendRendering {
    pub fn BeginInvoke(
        &mut self,
        bSuspend: bool,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (bSuspend, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        bSuspend: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (bSuspend))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_SuspendRendering")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__SuspendRendering {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_UnlockGLSharedTextureForAccess")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__UnlockGLSharedTextureForAccess {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_UnlockGLSharedTextureForAccess")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRCompositor__UnlockGLSharedTextureForAccess => "OVR.OpenVR"
    ."IVRCompositor/_UnlockGLSharedTextureForAccess"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_UnlockGLSharedTextureForAccess")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRCompositor__UnlockGLSharedTextureForAccess {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_UnlockGLSharedTextureForAccess")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRCompositor__UnlockGLSharedTextureForAccess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_UnlockGLSharedTextureForAccess")]
impl crate::OVR::OpenVR::IVRCompositor__UnlockGLSharedTextureForAccess {
    pub fn BeginInvoke(
        &mut self,
        glSharedTextureHandle: crate::System::IntPtr,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (glSharedTextureHandle, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        glSharedTextureHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (glSharedTextureHandle))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_UnlockGLSharedTextureForAccess")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__UnlockGLSharedTextureForAccess {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_WaitGetPoses")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRCompositor__WaitGetPoses {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_WaitGetPoses")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRCompositor__WaitGetPoses =>
    "OVR.OpenVR"."IVRCompositor/_WaitGetPoses"
);
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_WaitGetPoses")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRCompositor__WaitGetPoses {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_WaitGetPoses")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRCompositor__WaitGetPoses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_WaitGetPoses")]
impl crate::OVR::OpenVR::IVRCompositor__WaitGetPoses {
    pub fn BeginInvoke(
        &mut self,
        pRenderPoseArray: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::OVR::OpenVR::TrackedDevicePose_t,
            >,
        >,
        unRenderPoseArrayCount: u32,
        pGamePoseArray: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::OVR::OpenVR::TrackedDevicePose_t,
            >,
        >,
        unGamePoseArrayCount: u32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (
                    pRenderPoseArray,
                    unRenderPoseArrayCount,
                    pGamePoseArray,
                    unGamePoseArrayCount,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pRenderPoseArray: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::OVR::OpenVR::TrackedDevicePose_t,
            >,
        >,
        unRenderPoseArrayCount: u32,
        pGamePoseArray: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::OVR::OpenVR::TrackedDevicePose_t,
            >,
        >,
        unGamePoseArrayCount: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRCompositorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRCompositorError = __cordl_object
            .invoke(
                "Invoke",
                (
                    pRenderPoseArray,
                    unRenderPoseArrayCount,
                    pGamePoseArray,
                    unGamePoseArrayCount,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRCompositor+_WaitGetPoses")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRCompositor__WaitGetPoses {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

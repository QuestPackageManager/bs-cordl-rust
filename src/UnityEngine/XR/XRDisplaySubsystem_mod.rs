#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+LateLatchNode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XRDisplaySubsystem_LateLatchNode {
    #[default]
    Head = 0i32,
    LeftHand = 1i32,
    RightHand = 2i32,
}
#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+LateLatchNode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::XR::XRDisplaySubsystem_LateLatchNode => "UnityEngine.XR"
    ."XRDisplaySubsystem/LateLatchNode"
);
#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+XRBlitParams")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct XRDisplaySubsystem_XRBlitParams {
    pub srcTex: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    pub srcTexArraySlice: i32,
    pub srcRect: crate::UnityEngine::Rect,
    pub destRect: crate::UnityEngine::Rect,
    pub foveatedRenderingInfo: crate::System::IntPtr,
    pub srcHdrEncoded: bool,
    pub srcHdrColorGamut: crate::UnityEngine::ColorGamut,
    pub srcHdrMaxLuminance: i32,
}
#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+XRBlitParams")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::XRDisplaySubsystem_XRBlitParams
    => "UnityEngine.XR"."XRDisplaySubsystem/XRBlitParams"
);
#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+XRBlitParams")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::XR::XRDisplaySubsystem_XRBlitParams {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+XRBlitParams")]
impl crate::UnityEngine::XR::XRDisplaySubsystem_XRBlitParams {}
#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+XRMirrorViewBlitDesc")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct XRDisplaySubsystem_XRMirrorViewBlitDesc {
    pub displaySubsystemInstance: crate::System::IntPtr,
    pub nativeBlitAvailable: bool,
    pub nativeBlitInvalidStates: bool,
    pub blitParamsCount: i32,
}
#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+XRMirrorViewBlitDesc")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::XR::XRDisplaySubsystem_XRMirrorViewBlitDesc => "UnityEngine.XR"
    ."XRDisplaySubsystem/XRMirrorViewBlitDesc"
);
#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+XRMirrorViewBlitDesc")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::XR::XRDisplaySubsystem_XRMirrorViewBlitDesc {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+XRMirrorViewBlitDesc")]
impl crate::UnityEngine::XR::XRDisplaySubsystem_XRMirrorViewBlitDesc {}
#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+XRRenderPass")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct XRDisplaySubsystem_XRRenderPass {
    pub displaySubsystemInstance: crate::System::IntPtr,
    pub renderPassIndex: i32,
    pub renderTarget: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    pub renderTargetDesc: crate::UnityEngine::RenderTextureDescriptor,
    pub hasMotionVectorPass: bool,
    pub motionVectorRenderTarget: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    pub motionVectorRenderTargetDesc: crate::UnityEngine::RenderTextureDescriptor,
    pub shouldFillOutDepth: bool,
    pub cullingPassIndex: i32,
    pub foveatedRenderingInfo: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+XRRenderPass")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::XRDisplaySubsystem_XRRenderPass
    => "UnityEngine.XR"."XRDisplaySubsystem/XRRenderPass"
);
#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+XRRenderPass")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderPass {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+XRRenderPass")]
impl crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderPass {}

#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+LateLatchNode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XRDisplaySubsystem_LateLatchNode {
    #[default]
    Head = 0i32,
    LeftHand = 1i32,
    RightHand = 2i32,
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+LateLatchNode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::XRDisplaySubsystem_LateLatchNode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR";
    const CLASS_NAME: &'static str = "XRDisplaySubsystem/LateLatchNode";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+LateLatchNode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::XR::XRDisplaySubsystem_LateLatchNode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+LateLatchNode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::XR::XRDisplaySubsystem_LateLatchNode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+LateLatchNode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::XR::XRDisplaySubsystem_LateLatchNode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+LateLatchNode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::XR::XRDisplaySubsystem_LateLatchNode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRBlitParams")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRBlitParams")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::XRDisplaySubsystem_XRBlitParams {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR";
    const CLASS_NAME: &'static str = "XRDisplaySubsystem/XRBlitParams";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRBlitParams")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::XR::XRDisplaySubsystem_XRBlitParams {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRBlitParams")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::XR::XRDisplaySubsystem_XRBlitParams {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRBlitParams")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::XR::XRDisplaySubsystem_XRBlitParams {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRBlitParams")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::XR::XRDisplaySubsystem_XRBlitParams {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRBlitParams")]
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
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRMirrorViewBlitDesc")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XRDisplaySubsystem_XRMirrorViewBlitDesc {
    pub displaySubsystemInstance: crate::System::IntPtr,
    pub nativeBlitAvailable: bool,
    pub nativeBlitInvalidStates: bool,
    pub blitParamsCount: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRMirrorViewBlitDesc")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::XRDisplaySubsystem_XRMirrorViewBlitDesc {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR";
    const CLASS_NAME: &'static str = "XRDisplaySubsystem/XRMirrorViewBlitDesc";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRMirrorViewBlitDesc")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::XR::XRDisplaySubsystem_XRMirrorViewBlitDesc {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRMirrorViewBlitDesc")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::XR::XRDisplaySubsystem_XRMirrorViewBlitDesc {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRMirrorViewBlitDesc")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::XR::XRDisplaySubsystem_XRMirrorViewBlitDesc {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRMirrorViewBlitDesc")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::XR::XRDisplaySubsystem_XRMirrorViewBlitDesc {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRMirrorViewBlitDesc")]
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
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRRenderPass")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRRenderPass")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderPass {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR";
    const CLASS_NAME: &'static str = "XRDisplaySubsystem/XRRenderPass";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRRenderPass")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderPass {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRRenderPass")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderPass {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRRenderPass")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderPass {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRRenderPass")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderPass {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRRenderPass")]
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

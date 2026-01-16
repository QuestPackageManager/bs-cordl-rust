#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+BindingsMarshaller")]
#[repr(C)]
#[derive(Debug)]
pub struct XRDisplaySubsystem_BindingsMarshaller {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+BindingsMarshaller")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::XRDisplaySubsystem_BindingsMarshaller {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.XR";
    const CLASS_NAME: &'static str = "XRDisplaySubsystem/BindingsMarshaller";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+BindingsMarshaller")]
impl std::ops::Deref for crate::UnityEngine::XR::XRDisplaySubsystem_BindingsMarshaller {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+BindingsMarshaller")]
impl std::ops::DerefMut
for crate::UnityEngine::XR::XRDisplaySubsystem_BindingsMarshaller {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+BindingsMarshaller")]
impl crate::UnityEngine::XR::XRDisplaySubsystem_BindingsMarshaller {
    pub fn ConvertToNative(
        xrDisplaySubsystem: Blacklisted,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (Blacklisted),
                        crate::System::IntPtr,
                        1usize,
                    >("ConvertToNative")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertToNative", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            cordl_method_info.invoke_unchecked((), (xrDisplaySubsystem))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+BindingsMarshaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::XR::XRDisplaySubsystem_BindingsMarshaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
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
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+TextureLayout")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XRDisplaySubsystem_TextureLayout {
    #[default]
    SeparateTexture2Ds = 4i32,
    SingleTexture2D = 2i32,
    Texture2DArray = 1i32,
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+TextureLayout")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::XRDisplaySubsystem_TextureLayout {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR";
    const CLASS_NAME: &'static str = "XRDisplaySubsystem/TextureLayout";
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
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+TextureLayout")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::XR::XRDisplaySubsystem_TextureLayout {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+TextureLayout")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::XR::XRDisplaySubsystem_TextureLayout {
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
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+TextureLayout")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::XR::XRDisplaySubsystem_TextureLayout {
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
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+TextureLayout")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::XR::XRDisplaySubsystem_TextureLayout {
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
impl crate::UnityEngine::XR::XRDisplaySubsystem_XRMirrorViewBlitDesc {
    pub fn GetBlitParameter(
        &mut self,
        blitParameterIndex: i32,
        blitParameter: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::XR::XRDisplaySubsystem_XRBlitParams,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::XR::XRDisplaySubsystem_XRBlitParams,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("GetBlitParameter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetBlitParameter", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (blitParameterIndex, blitParameter))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRRenderParameter")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XRDisplaySubsystem_XRRenderParameter {
    pub view: crate::UnityEngine::Matrix4x4,
    pub projection: crate::UnityEngine::Matrix4x4,
    pub viewport: crate::UnityEngine::Rect,
    pub occlusionMesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    pub textureArraySlice: i32,
    pub previousView: crate::UnityEngine::Matrix4x4,
    pub isPreviousViewValid: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRRenderParameter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderParameter {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR";
    const CLASS_NAME: &'static str = "XRDisplaySubsystem/XRRenderParameter";
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
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRRenderParameter")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderParameter {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRRenderParameter")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderParameter {
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
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRRenderParameter")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderParameter {
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
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRRenderParameter")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderParameter {
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
#[cfg(feature = "cordl_class_UnityEngine+XR+XRDisplaySubsystem+XRRenderParameter")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderParameter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+XRDisplaySubsystem+XRRenderParameter")]
impl crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderParameter {}
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
impl crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderPass {
    pub fn GetRenderParameter(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        renderParameterIndex: i32,
        renderParameter: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderParameter,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("GetRenderParameter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRenderParameter", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (camera, renderParameterIndex, renderParameter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderParameterCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetRenderParameterCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRenderParameterCount", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderParameter_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderPass,
        >,
        camera: crate::System::IntPtr,
        renderParameterIndex: i32,
        renderParameter: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderPass,
                            >,
                            crate::System::IntPtr,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::XR::XRDisplaySubsystem_XRRenderParameter,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("GetRenderParameter_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRenderParameter_Injected", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (_unity_self, camera, renderParameterIndex, renderParameter),
                )?
        };
        Ok(__cordl_ret.into())
    }
}

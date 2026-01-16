#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OcclusionCullingCommonShaderVariables {
    pub _OccluderMipBounds: crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___OccluderMipBounds_e__FixedBuffer,
    pub _ViewProjMatrix: crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewProjMatrix_e__FixedBuffer,
    pub _ViewOriginWorldSpace: crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewOriginWorldSpace_e__FixedBuffer,
    pub _FacingDirWorldSpace: crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___FacingDirWorldSpace_e__FixedBuffer,
    pub _RadialDirWorldSpace: crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___RadialDirWorldSpace_e__FixedBuffer,
    pub _DepthSizeInOccluderPixels: crate::UnityEngine::Vector4,
    pub _OccluderDepthPyramidSize: crate::UnityEngine::Vector4,
    pub _OccluderMipLayoutSizeX: u32,
    pub _OccluderMipLayoutSizeY: u32,
    pub _OcclusionTestDebugFlags: u32,
    pub _OcclusionCullingCommonPad0: u32,
    pub _OcclusionTestCount: i32,
    pub _OccluderSubviewIndices: i32,
    pub _CullingSplitIndices: i32,
    pub _CullingSplitMask: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OcclusionCullingCommonShaderVariables";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables
{
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables
{
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommonShaderVariables")]
impl crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables {
    #[cfg(
        feature = "UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__FacingDirWorldSpace_e__FixedBuffer"
    )]
    pub type __FacingDirWorldSpace_e__FixedBuffer = crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___FacingDirWorldSpace_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__OccluderMipBounds_e__FixedBuffer"
    )]
    pub type __OccluderMipBounds_e__FixedBuffer = crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___OccluderMipBounds_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__RadialDirWorldSpace_e__FixedBuffer"
    )]
    pub type __RadialDirWorldSpace_e__FixedBuffer = crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___RadialDirWorldSpace_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewOriginWorldSpace_e__FixedBuffer"
    )]
    pub type __ViewOriginWorldSpace_e__FixedBuffer = crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewOriginWorldSpace_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewProjMatrix_e__FixedBuffer"
    )]
    pub type __ViewProjMatrix_e__FixedBuffer = crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewProjMatrix_e__FixedBuffer;
    pub fn _ctor(
        &mut self,
        occluderCtx: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OccluderContext,
        >,
        subviewSettings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::InstanceOcclusionTestSubviewSettings,
        >,
        occlusionOverlayCountVisible: bool,
        overrideOcclusionTestToAlwaysPass: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::OccluderContext,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::InstanceOcclusionTestSubviewSettings,
                        >,
                        bool,
                        bool,
                    ), quest_hook::libil2cpp::Void, 4usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    occluderCtx,
                    subviewSettings,
                    occlusionOverlayCountVisible,
                    overrideOcclusionTestToAlwaysPass,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__FacingDirWorldSpace_e__FixedBuffer"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OcclusionCullingCommonShaderVariables___FacingDirWorldSpace_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__FacingDirWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___FacingDirWorldSpace_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OcclusionCullingCommonShaderVariables/<_FacingDirWorldSpace>e__FixedBuffer";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__FacingDirWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___FacingDirWorldSpace_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__FacingDirWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___FacingDirWorldSpace_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__FacingDirWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___FacingDirWorldSpace_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__FacingDirWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___FacingDirWorldSpace_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__FacingDirWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___FacingDirWorldSpace_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__FacingDirWorldSpace_e__FixedBuffer"
)]
impl crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___FacingDirWorldSpace_e__FixedBuffer {}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__OccluderMipBounds_e__FixedBuffer"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OcclusionCullingCommonShaderVariables___OccluderMipBounds_e__FixedBuffer {
    pub FixedElementField: u32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__OccluderMipBounds_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___OccluderMipBounds_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OcclusionCullingCommonShaderVariables/<_OccluderMipBounds>e__FixedBuffer";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__OccluderMipBounds_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___OccluderMipBounds_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__OccluderMipBounds_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___OccluderMipBounds_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__OccluderMipBounds_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___OccluderMipBounds_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__OccluderMipBounds_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___OccluderMipBounds_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__OccluderMipBounds_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___OccluderMipBounds_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__OccluderMipBounds_e__FixedBuffer"
)]
impl crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___OccluderMipBounds_e__FixedBuffer {}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__RadialDirWorldSpace_e__FixedBuffer"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OcclusionCullingCommonShaderVariables___RadialDirWorldSpace_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__RadialDirWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___RadialDirWorldSpace_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OcclusionCullingCommonShaderVariables/<_RadialDirWorldSpace>e__FixedBuffer";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__RadialDirWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___RadialDirWorldSpace_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__RadialDirWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___RadialDirWorldSpace_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__RadialDirWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___RadialDirWorldSpace_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__RadialDirWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___RadialDirWorldSpace_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__RadialDirWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___RadialDirWorldSpace_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__RadialDirWorldSpace_e__FixedBuffer"
)]
impl crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___RadialDirWorldSpace_e__FixedBuffer {}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewOriginWorldSpace_e__FixedBuffer"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OcclusionCullingCommonShaderVariables___ViewOriginWorldSpace_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewOriginWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewOriginWorldSpace_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OcclusionCullingCommonShaderVariables/<_ViewOriginWorldSpace>e__FixedBuffer";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewOriginWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewOriginWorldSpace_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewOriginWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewOriginWorldSpace_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewOriginWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewOriginWorldSpace_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewOriginWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewOriginWorldSpace_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewOriginWorldSpace_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewOriginWorldSpace_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewOriginWorldSpace_e__FixedBuffer"
)]
impl crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewOriginWorldSpace_e__FixedBuffer {}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewProjMatrix_e__FixedBuffer"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OcclusionCullingCommonShaderVariables___ViewProjMatrix_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewProjMatrix_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewProjMatrix_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OcclusionCullingCommonShaderVariables/<_ViewProjMatrix>e__FixedBuffer";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewProjMatrix_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewProjMatrix_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewProjMatrix_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewProjMatrix_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewProjMatrix_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewProjMatrix_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewProjMatrix_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewProjMatrix_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewProjMatrix_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewProjMatrix_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+OcclusionCullingCommonShaderVariables+__ViewProjMatrix_e__FixedBuffer"
)]
impl crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables___ViewProjMatrix_e__FixedBuffer {}

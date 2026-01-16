#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume")]
#[repr(C)]
#[derive(Debug)]
pub struct ProbeAdjustmentVolume {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub shape: crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Shape,
    pub _cordl_size: crate::UnityEngine::Vector3,
    pub radius: f32,
    pub mode: crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Mode,
    pub intensityScale: f32,
    pub overriddenDilationThreshold: f32,
    pub virtualOffsetRotation: crate::UnityEngine::Vector3,
    pub virtualOffsetDistance: f32,
    pub geometryBias: f32,
    pub virtualOffsetThreshold: f32,
    pub rayOriginBias: f32,
    pub skyDirection: crate::UnityEngine::Vector3,
    pub skyShadingDirectionRotation: crate::UnityEngine::Vector3,
    pub directSampleCount: i32,
    pub indirectSampleCount: i32,
    pub sampleCountMultiplier: i32,
    pub maxBounces: i32,
    pub skyOcclusionSampleCount: i32,
    pub skyOcclusionMaxBounces: i32,
    pub renderingLayerMaskOperation:
        crate::UnityEngine::Rendering::ProbeAdjustmentVolume_RenderingLayerMaskOperation,
    pub renderingLayerMask: u8,
    pub version: crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Version,
    pub invalidateProbes: bool,
    pub overrideDilationThreshold: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::ProbeAdjustmentVolume {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ProbeAdjustmentVolume";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+Rendering+ProbeAdjustmentVolume")]
impl std::ops::Deref for crate::UnityEngine::Rendering::ProbeAdjustmentVolume {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeAdjustmentVolume")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::ProbeAdjustmentVolume {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeAdjustmentVolume")]
impl crate::UnityEngine::Rendering::ProbeAdjustmentVolume {
    #[cfg(feature = "UnityEngine+Rendering+ProbeAdjustmentVolume+Mode")]
    pub type Mode = crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Mode;
    #[cfg(feature = "UnityEngine+Rendering+ProbeAdjustmentVolume+RenderingLayerMaskOperation")]
    pub type RenderingLayerMaskOperation =
        crate::UnityEngine::Rendering::ProbeAdjustmentVolume_RenderingLayerMaskOperation;
    #[cfg(feature = "UnityEngine+Rendering+ProbeAdjustmentVolume+Shape")]
    pub type Shape = crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Shape;
    #[cfg(feature = "UnityEngine+Rendering+ProbeAdjustmentVolume+Version")]
    pub type Version = crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Version;
    pub fn Awake(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Awake")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Awake",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnAfterDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "UnityEngine.ISerializationCallbackReceiver.OnAfterDeserialize",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.ISerializationCallbackReceiver.OnAfterDeserialize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "UnityEngine.ISerializationCallbackReceiver.OnBeforeSerialize",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.ISerializationCallbackReceiver.OnBeforeSerialize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::ProbeAdjustmentVolume {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeAdjustmentVolume")]
impl AsRef<crate::UnityEngine::ISerializationCallbackReceiver>
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume
{
    fn as_ref(&self) -> &crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeAdjustmentVolume")]
impl AsMut<crate::UnityEngine::ISerializationCallbackReceiver>
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Mode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ProbeAdjustmentVolume_Mode {
    #[default]
    ApplyVirtualOffset = 2i32,
    IntensityScale = 99i32,
    InvalidateProbes = 0i32,
    OverrideRenderingLayerMask = 6i32,
    OverrideSampleCount = 5i32,
    OverrideSkyDirection = 4i32,
    OverrideValidityThreshold = 1i32,
    OverrideVirtualOffsetSettings = 3i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Mode")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Mode
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ProbeAdjustmentVolume/Mode";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Mode")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Mode
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Mode")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Mode
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Mode")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Mode
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Mode")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Mode
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+RenderingLayerMaskOperation"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ProbeAdjustmentVolume_RenderingLayerMaskOperation {
    #[default]
    Add = 1i32,
    Override = 0i32,
    Remove = 2i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+RenderingLayerMaskOperation"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_RenderingLayerMaskOperation
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ProbeAdjustmentVolume/RenderingLayerMaskOperation";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+RenderingLayerMaskOperation"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_RenderingLayerMaskOperation
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+RenderingLayerMaskOperation"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_RenderingLayerMaskOperation
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+RenderingLayerMaskOperation"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_RenderingLayerMaskOperation
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+RenderingLayerMaskOperation"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_RenderingLayerMaskOperation
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Shape")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ProbeAdjustmentVolume_Shape {
    #[default]
    Box = 0i32,
    Sphere = 1i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Shape")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Shape
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ProbeAdjustmentVolume/Shape";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Shape")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Shape
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Shape")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Shape
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Shape")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Shape
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Shape")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Shape
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Version")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ProbeAdjustmentVolume_Version {
    #[default]
    Count = 2i32,
    Initial = 0i32,
    Mode = 1i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Version")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Version
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ProbeAdjustmentVolume/Version";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Version")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Version
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Version")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Version
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Version")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Version
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeAdjustmentVolume+Version")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::ProbeAdjustmentVolume_Version
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

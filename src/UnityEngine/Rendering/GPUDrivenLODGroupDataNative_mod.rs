#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenLODGroupDataNative")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct GPUDrivenLODGroupDataNative {
    pub lodGroupID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub lodOffset: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub lodCount: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub fadeMode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub worldSpaceReferencePoint: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub worldSpaceSize: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub renderersCount: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub lastLODIsBillboard: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub lodGroupCount: i32,
    pub invalidLODGroupID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub invalidLODGroupCount: i32,
    pub lodRenderersCount: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub lodScreenRelativeTransitionHeight:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub lodFadeTransitionWidth: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub lodDataCount: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenLODGroupDataNative")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::GPUDrivenLODGroupDataNative
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUDrivenLODGroupDataNative";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenLODGroupDataNative")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::GPUDrivenLODGroupDataNative
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenLODGroupDataNative")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::GPUDrivenLODGroupDataNative
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenLODGroupDataNative")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::GPUDrivenLODGroupDataNative
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenLODGroupDataNative")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::GPUDrivenLODGroupDataNative
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenLODGroupDataNative")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::GPUDrivenLODGroupDataNative
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUDrivenLODGroupDataNative")]
impl crate::UnityEngine::Rendering::GPUDrivenLODGroupDataNative {}

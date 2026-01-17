#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenRendererGroupDataNative")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct GPUDrivenRendererGroupDataNative {
    pub rendererGroupID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub localBounds: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub lightmapScaleOffset: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub gameObjectLayer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub renderingLayerMask: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub lodGroupID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub motionVecGenMode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub packedRendererData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub rendererPriority: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub meshIndex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub subMeshStartIndex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub materialsOffset: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub materialsCount: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub instancesOffset: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub instancesCount: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub editorData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub rendererGroupCount: i32,
    pub invalidRendererGroupID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub invalidRendererGroupIDCount: i32,
    pub localToWorldMatrix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub prevLocalToWorldMatrix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub rendererGroupIndex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub instanceCount: i32,
    pub meshID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub subMeshCount: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub subMeshDescOffset: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub meshCount: i32,
    pub subMeshDesc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub subMeshDescCount: i32,
    pub materialIndex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub materialIndexCount: i32,
    pub materialID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub packedMaterialData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub materialFilterFlags: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub materialCount: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenRendererGroupDataNative")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::GPUDrivenRendererGroupDataNative
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUDrivenRendererGroupDataNative";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenRendererGroupDataNative")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::GPUDrivenRendererGroupDataNative
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenRendererGroupDataNative")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::GPUDrivenRendererGroupDataNative
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenRendererGroupDataNative")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::GPUDrivenRendererGroupDataNative
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenRendererGroupDataNative")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::GPUDrivenRendererGroupDataNative
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenRendererGroupDataNative")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::GPUDrivenRendererGroupDataNative
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUDrivenRendererGroupDataNative")]
impl crate::UnityEngine::Rendering::GPUDrivenRendererGroupDataNative {}

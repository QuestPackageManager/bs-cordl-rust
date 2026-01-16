#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenRendererGroupData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GPUDrivenRendererGroupData {
    pub rendererGroupID: crate::Unity::Collections::NativeArray_1<i32>,
    pub localBounds: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Bounds,
    >,
    pub lightmapScaleOffset: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Vector4,
    >,
    pub gameObjectLayer: crate::Unity::Collections::NativeArray_1<i32>,
    pub renderingLayerMask: crate::Unity::Collections::NativeArray_1<u32>,
    pub lodGroupID: crate::Unity::Collections::NativeArray_1<i32>,
    pub lightmapIndex: crate::Unity::Collections::NativeArray_1<i32>,
    pub packedRendererData: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::GPUDrivenPackedRendererData,
    >,
    pub rendererPriority: crate::Unity::Collections::NativeArray_1<i32>,
    pub meshIndex: crate::Unity::Collections::NativeArray_1<i32>,
    pub subMeshStartIndex: crate::Unity::Collections::NativeArray_1<i16>,
    pub materialsOffset: crate::Unity::Collections::NativeArray_1<i32>,
    pub materialsCount: crate::Unity::Collections::NativeArray_1<i16>,
    pub instancesOffset: crate::Unity::Collections::NativeArray_1<i32>,
    pub instancesCount: crate::Unity::Collections::NativeArray_1<i32>,
    pub editorData: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::GPUDrivenRendererEditorData,
    >,
    pub invalidRendererGroupID: crate::Unity::Collections::NativeArray_1<i32>,
    pub localToWorldMatrix: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Matrix4x4,
    >,
    pub prevLocalToWorldMatrix: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Matrix4x4,
    >,
    pub rendererGroupIndex: crate::Unity::Collections::NativeArray_1<i32>,
    pub meshID: crate::Unity::Collections::NativeArray_1<i32>,
    pub subMeshCount: crate::Unity::Collections::NativeArray_1<i16>,
    pub subMeshDescOffset: crate::Unity::Collections::NativeArray_1<i32>,
    pub subMeshDesc: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::SubMeshDescriptor,
    >,
    pub materialIndex: crate::Unity::Collections::NativeArray_1<i32>,
    pub materialID: crate::Unity::Collections::NativeArray_1<i32>,
    pub packedMaterialData: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::GPUDrivenPackedMaterialData,
    >,
    pub materialFilterFlags: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenRendererGroupData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::GPUDrivenRendererGroupData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUDrivenRendererGroupData";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenRendererGroupData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::GPUDrivenRendererGroupData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenRendererGroupData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::GPUDrivenRendererGroupData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenRendererGroupData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::GPUDrivenRendererGroupData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenRendererGroupData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::GPUDrivenRendererGroupData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUDrivenRendererGroupData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::GPUDrivenRendererGroupData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUDrivenRendererGroupData")]
impl crate::UnityEngine::Rendering::GPUDrivenRendererGroupData {}

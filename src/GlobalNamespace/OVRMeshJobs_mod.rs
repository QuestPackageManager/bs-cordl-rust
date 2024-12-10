#[cfg(feature = "OVRMeshJobs")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRMeshJobs {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRMeshJobs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRMeshJobs => ""."OVRMeshJobs"
);
#[cfg(feature = "OVRMeshJobs")]
impl std::ops::Deref for crate::GlobalNamespace::OVRMeshJobs {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMeshJobs")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRMeshJobs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMeshJobs")]
impl crate::GlobalNamespace::OVRMeshJobs {
    #[cfg(feature = "OVRMeshJobs+NativeArrayHelper_1")]
    pub type NativeArrayHelper_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::OVRMeshJobs_NativeArrayHelper_1<
        T,
    >;
    #[cfg(feature = "OVRMeshJobs+TransformToUnitySpaceJob")]
    pub type TransformToUnitySpaceJob = crate::GlobalNamespace::OVRMeshJobs_TransformToUnitySpaceJob;
    #[cfg(feature = "OVRMeshJobs+TransformTrianglesJob")]
    pub type TransformTrianglesJob = crate::GlobalNamespace::OVRMeshJobs_TransformTrianglesJob;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRMeshJobs")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRMeshJobs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRMeshJobs+NativeArrayHelper_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRMeshJobs_NativeArrayHelper_1<T: quest_hook::libil2cpp::Type> {
    pub UnityNativeArray: crate::Unity::Collections::NativeArray_1<T>,
    pub _handle: crate::System::Runtime::InteropServices::GCHandle,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "OVRMeshJobs+NativeArrayHelper_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRMeshJobs_NativeArrayHelper_1
    < T > => ""."OVRMeshJobs/NativeArrayHelper`1<T>" < T >
);
#[cfg(feature = "OVRMeshJobs+NativeArrayHelper_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRMeshJobs_NativeArrayHelper_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRMeshJobs+NativeArrayHelper_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::OVRMeshJobs_NativeArrayHelper_1<T> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        ovrArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (ovrArray, length),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRMeshJobs+TransformToUnitySpaceJob")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRMeshJobs_TransformToUnitySpaceJob {
    pub Vertices: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector3>,
    pub Normals: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector3>,
    pub UV: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector2>,
    pub BoneWeights: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::BoneWeight,
    >,
    pub MeshVerticesPosition: crate::Unity::Collections::NativeArray_1<
        crate::GlobalNamespace::OVRPlugin_Vector3f,
    >,
    pub MeshNormals: crate::Unity::Collections::NativeArray_1<
        crate::GlobalNamespace::OVRPlugin_Vector3f,
    >,
    pub MeshUV: crate::Unity::Collections::NativeArray_1<
        crate::GlobalNamespace::OVRPlugin_Vector2f,
    >,
    pub MeshBoneWeights: crate::Unity::Collections::NativeArray_1<
        crate::GlobalNamespace::OVRPlugin_Vector4f,
    >,
    pub MeshBoneIndices: crate::Unity::Collections::NativeArray_1<
        crate::GlobalNamespace::OVRPlugin_Vector4s,
    >,
}
#[cfg(feature = "OVRMeshJobs+TransformToUnitySpaceJob")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRMeshJobs_TransformToUnitySpaceJob => ""
    ."OVRMeshJobs/TransformToUnitySpaceJob"
);
#[cfg(feature = "OVRMeshJobs+TransformToUnitySpaceJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRMeshJobs_TransformToUnitySpaceJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRMeshJobs+TransformToUnitySpaceJob")]
impl crate::GlobalNamespace::OVRMeshJobs_TransformToUnitySpaceJob {
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRMeshJobs+TransformTrianglesJob")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRMeshJobs_TransformTrianglesJob {
    pub Triangles: crate::Unity::Collections::NativeArray_1<u32>,
    pub MeshIndices: crate::Unity::Collections::NativeArray_1<i16>,
    pub NumIndices: i32,
}
#[cfg(feature = "OVRMeshJobs+TransformTrianglesJob")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRMeshJobs_TransformTrianglesJob => ""
    ."OVRMeshJobs/TransformTrianglesJob"
);
#[cfg(feature = "OVRMeshJobs+TransformTrianglesJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRMeshJobs_TransformTrianglesJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRMeshJobs+TransformTrianglesJob")]
impl crate::GlobalNamespace::OVRMeshJobs_TransformTrianglesJob {
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
}

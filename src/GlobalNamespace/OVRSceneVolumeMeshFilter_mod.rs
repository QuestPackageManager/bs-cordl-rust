#[cfg(feature = "OVRSceneVolumeMeshFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSceneVolumeMeshFilter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _IsCompleted_k__BackingField: bool,
    pub _mesh: *mut crate::UnityEngine::Mesh,
    pub _meshFilter: *mut crate::UnityEngine::MeshFilter,
}
#[cfg(feature = "OVRSceneVolumeMeshFilter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSceneVolumeMeshFilter => ""
    ."OVRSceneVolumeMeshFilter"
);
#[cfg(feature = "OVRSceneVolumeMeshFilter")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSceneVolumeMeshFilter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRSceneVolumeMeshFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter")]
impl crate::GlobalNamespace::OVRSceneVolumeMeshFilter {
    #[cfg(feature = "OVRSceneVolumeMeshFilter+BakeMeshJob")]
    pub type BakeMeshJob = crate::GlobalNamespace::OVRSceneVolumeMeshFilter_BakeMeshJob;
    #[cfg(feature = "OVRSceneVolumeMeshFilter+GetTriangleMeshCountsJob")]
    pub type GetTriangleMeshCountsJob = crate::GlobalNamespace::OVRSceneVolumeMeshFilter_GetTriangleMeshCountsJob;
    #[cfg(feature = "OVRSceneVolumeMeshFilter+GetTriangleMeshJob")]
    pub type GetTriangleMeshJob = crate::GlobalNamespace::OVRSceneVolumeMeshFilter_GetTriangleMeshJob;
    #[cfg(feature = "OVRSceneVolumeMeshFilter+PopulateMeshDataJob")]
    pub type PopulateMeshDataJob = crate::GlobalNamespace::OVRSceneVolumeMeshFilter_PopulateMeshDataJob;
    #[cfg(feature = "OVRSceneVolumeMeshFilter+_CreateVolumeMesh_d__7")]
    pub type _CreateVolumeMesh_d__7 = crate::GlobalNamespace::OVRSceneVolumeMeshFilter__CreateVolumeMesh_d__7;
    pub fn CreateVolumeMesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("CreateVolumeMesh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
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
    pub fn get_IsCompleted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCompleted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsCompleted(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsCompleted", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRSceneVolumeMeshFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+BakeMeshJob")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRSceneVolumeMeshFilter_BakeMeshJob {
    pub MeshID: i32,
    pub Convex: bool,
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+BakeMeshJob")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSceneVolumeMeshFilter_BakeMeshJob => ""
    ."OVRSceneVolumeMeshFilter/BakeMeshJob"
);
#[cfg(feature = "OVRSceneVolumeMeshFilter+BakeMeshJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRSceneVolumeMeshFilter_BakeMeshJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+BakeMeshJob")]
impl crate::GlobalNamespace::OVRSceneVolumeMeshFilter_BakeMeshJob {
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+BakeMeshJob")]
impl AsRef<crate::Unity::Jobs::IJob>
for crate::GlobalNamespace::OVRSceneVolumeMeshFilter_BakeMeshJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+BakeMeshJob")]
impl AsMut<crate::Unity::Jobs::IJob>
for crate::GlobalNamespace::OVRSceneVolumeMeshFilter_BakeMeshJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+GetTriangleMeshCountsJob")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRSceneVolumeMeshFilter_GetTriangleMeshCountsJob {
    pub Space: crate::GlobalNamespace::OVRSpace,
    pub Results: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+GetTriangleMeshCountsJob")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSceneVolumeMeshFilter_GetTriangleMeshCountsJob => ""
    ."OVRSceneVolumeMeshFilter/GetTriangleMeshCountsJob"
);
#[cfg(feature = "OVRSceneVolumeMeshFilter+GetTriangleMeshCountsJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRSceneVolumeMeshFilter_GetTriangleMeshCountsJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+GetTriangleMeshCountsJob")]
impl crate::GlobalNamespace::OVRSceneVolumeMeshFilter_GetTriangleMeshCountsJob {
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+GetTriangleMeshCountsJob")]
impl AsRef<crate::Unity::Jobs::IJob>
for crate::GlobalNamespace::OVRSceneVolumeMeshFilter_GetTriangleMeshCountsJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+GetTriangleMeshCountsJob")]
impl AsMut<crate::Unity::Jobs::IJob>
for crate::GlobalNamespace::OVRSceneVolumeMeshFilter_GetTriangleMeshCountsJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+GetTriangleMeshJob")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRSceneVolumeMeshFilter_GetTriangleMeshJob {
    pub Space: crate::GlobalNamespace::OVRSpace,
    pub Vertices: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector3>,
    pub Triangles: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+GetTriangleMeshJob")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSceneVolumeMeshFilter_GetTriangleMeshJob => ""
    ."OVRSceneVolumeMeshFilter/GetTriangleMeshJob"
);
#[cfg(feature = "OVRSceneVolumeMeshFilter+GetTriangleMeshJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRSceneVolumeMeshFilter_GetTriangleMeshJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+GetTriangleMeshJob")]
impl crate::GlobalNamespace::OVRSceneVolumeMeshFilter_GetTriangleMeshJob {
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+GetTriangleMeshJob")]
impl AsRef<crate::Unity::Jobs::IJob>
for crate::GlobalNamespace::OVRSceneVolumeMeshFilter_GetTriangleMeshJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+GetTriangleMeshJob")]
impl AsMut<crate::Unity::Jobs::IJob>
for crate::GlobalNamespace::OVRSceneVolumeMeshFilter_GetTriangleMeshJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+PopulateMeshDataJob")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRSceneVolumeMeshFilter_PopulateMeshDataJob {
    pub Vertices: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector3>,
    pub Triangles: crate::Unity::Collections::NativeArray_1<i32>,
    pub MeshData: crate::UnityEngine::Mesh_MeshData,
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+PopulateMeshDataJob")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSceneVolumeMeshFilter_PopulateMeshDataJob => ""
    ."OVRSceneVolumeMeshFilter/PopulateMeshDataJob"
);
#[cfg(feature = "OVRSceneVolumeMeshFilter+PopulateMeshDataJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRSceneVolumeMeshFilter_PopulateMeshDataJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+PopulateMeshDataJob")]
impl crate::GlobalNamespace::OVRSceneVolumeMeshFilter_PopulateMeshDataJob {
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+PopulateMeshDataJob")]
impl AsRef<crate::Unity::Jobs::IJob>
for crate::GlobalNamespace::OVRSceneVolumeMeshFilter_PopulateMeshDataJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "OVRSceneVolumeMeshFilter+PopulateMeshDataJob")]
impl AsMut<crate::Unity::Jobs::IJob>
for crate::GlobalNamespace::OVRSceneVolumeMeshFilter_PopulateMeshDataJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}

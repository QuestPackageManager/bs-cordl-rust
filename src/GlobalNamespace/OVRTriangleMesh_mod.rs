#[cfg(feature = "OVRTriangleMesh")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRTriangleMesh {
    pub _Handle_k__BackingField: u64,
}
#[cfg(feature = "OVRTriangleMesh")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTriangleMesh => ""
    ."OVRTriangleMesh"
);
#[cfg(feature = "OVRTriangleMesh")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRTriangleMesh {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRTriangleMesh")]
impl crate::GlobalNamespace::OVRTriangleMesh {
    #[cfg(feature = "OVRTriangleMesh+FlipTriangleWindingJob")]
    pub type FlipTriangleWindingJob = crate::GlobalNamespace::OVRTriangleMesh_FlipTriangleWindingJob;
    #[cfg(feature = "OVRTriangleMesh+GetMeshJob")]
    pub type GetMeshJob = crate::GlobalNamespace::OVRTriangleMesh_GetMeshJob;
    #[cfg(feature = "OVRTriangleMesh+NegateXJob")]
    pub type NegateXJob = crate::GlobalNamespace::OVRTriangleMesh_NegateXJob;
    #[cfg(feature = "OVRTriangleMesh+Triangle")]
    pub type Triangle = crate::GlobalNamespace::OVRTriangleMesh_Triangle;
    pub fn Equals_OVRTriangleMesh0(
        &mut self,
        other: crate::GlobalNamespace::OVRTriangleMesh,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IOVRAnchorComponent_OVRTriangleMesh__FromAnchor(
        &mut self,
        anchor: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTriangleMesh> {
        let __cordl_ret: crate::GlobalNamespace::OVRTriangleMesh = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRTriangleMesh>.FromAnchor",
            (anchor),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IOVRAnchorComponent_OVRTriangleMesh__SetEnabledAsync(
        &mut self,
        enabled: bool,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRTriangleMesh>.SetEnabledAsync",
            (enabled, timeout),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IOVRAnchorComponent_OVRTriangleMesh__get_Handle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRTriangleMesh>.get_Handle",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IOVRAnchorComponent_OVRTriangleMesh__get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRTriangleMesh>.get_Type",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ScheduleGetMeshJob(
        &mut self,
        positions: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector3>,
        indices: crate::Unity::Collections::NativeArray_1<i32>,
        dependencies: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        let __cordl_ret: crate::Unity::Jobs::JobHandle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ScheduleGetMeshJob",
            (positions, indices, dependencies),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TryGetCounts(
        &mut self,
        vertexCount: quest_hook::libil2cpp::ByRefMut<i32>,
        triangleCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetCounts",
            (vertexCount, triangleCount),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TryGetMesh(
        &mut self,
        positions: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector3>,
        indices: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetMesh",
            (positions, indices),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TryGetMeshRawUntransformed(
        &mut self,
        positions: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector3>,
        indices: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetMeshRawUntransformed",
            (positions, indices),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        anchor: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (anchor),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Handle(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Handle",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsEnabled",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsNull",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Type",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRTriangleMesh+FlipTriangleWindingJob")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRTriangleMesh_FlipTriangleWindingJob {
    pub Triangles: crate::Unity::Collections::NativeArray_1<
        crate::GlobalNamespace::OVRTriangleMesh_Triangle,
    >,
}
#[cfg(feature = "OVRTriangleMesh+FlipTriangleWindingJob")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRTriangleMesh_FlipTriangleWindingJob => ""
    ."OVRTriangleMesh/FlipTriangleWindingJob"
);
#[cfg(feature = "OVRTriangleMesh+FlipTriangleWindingJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRTriangleMesh_FlipTriangleWindingJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRTriangleMesh+FlipTriangleWindingJob")]
impl crate::GlobalNamespace::OVRTriangleMesh_FlipTriangleWindingJob {
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (index),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRTriangleMesh+GetMeshJob")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRTriangleMesh_GetMeshJob {
    pub Space: u64,
    pub Positions: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector3>,
    pub Indices: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(feature = "OVRTriangleMesh+GetMeshJob")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTriangleMesh_GetMeshJob =>
    ""."OVRTriangleMesh/GetMeshJob"
);
#[cfg(feature = "OVRTriangleMesh+GetMeshJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRTriangleMesh_GetMeshJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRTriangleMesh+GetMeshJob")]
impl crate::GlobalNamespace::OVRTriangleMesh_GetMeshJob {
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRTriangleMesh+NegateXJob")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRTriangleMesh_NegateXJob {
    pub Positions: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector3>,
}
#[cfg(feature = "OVRTriangleMesh+NegateXJob")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTriangleMesh_NegateXJob =>
    ""."OVRTriangleMesh/NegateXJob"
);
#[cfg(feature = "OVRTriangleMesh+NegateXJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRTriangleMesh_NegateXJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRTriangleMesh+NegateXJob")]
impl crate::GlobalNamespace::OVRTriangleMesh_NegateXJob {
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (index),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRTriangleMesh+Triangle")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRTriangleMesh_Triangle {
    pub A: i32,
    pub B: i32,
    pub C: i32,
}
#[cfg(feature = "OVRTriangleMesh+Triangle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTriangleMesh_Triangle => ""
    ."OVRTriangleMesh/Triangle"
);
#[cfg(feature = "OVRTriangleMesh+Triangle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRTriangleMesh_Triangle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRTriangleMesh+Triangle")]
impl crate::GlobalNamespace::OVRTriangleMesh_Triangle {}

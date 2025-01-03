#[cfg(feature = "TMPro+TMP_MeshInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TMP_MeshInfo {
    pub mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    pub vertexCount: i32,
    pub vertices: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    >,
    pub normals: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    >,
    pub tangents: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    >,
    pub uvs0: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
    >,
    pub uvs2: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
    >,
    pub colors32: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
    >,
    pub triangles: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
}
#[cfg(feature = "TMPro+TMP_MeshInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_MeshInfo => "TMPro"."TMP_MeshInfo"
);
#[cfg(feature = "TMPro+TMP_MeshInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::TMP_MeshInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_MeshInfo")]
impl crate::TMPro::TMP_MeshInfo {
    pub fn ClearUnusedVertices_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ClearUnusedVertices",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearUnusedVertices_i32_1(
        &mut self,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ClearUnusedVertices",
            (startIndex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearUnusedVertices_i32__cordl_bool2(
        &mut self,
        startIndex: i32,
        updateMesh: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ClearUnusedVertices",
            (startIndex, updateMesh),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear__cordl_bool1(
        &mut self,
        uploadChanges: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (uploadChanges),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ResizeMeshInfo__cordl_bool1(
        &mut self,
        _cordl_size: i32,
        isVolumetric: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ResizeMeshInfo",
            (_cordl_size, isVolumetric),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ResizeMeshInfo_i32_0(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ResizeMeshInfo",
            (_cordl_size),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SortGeometry_IList_1_1(
        &mut self,
        sortingOrder: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SortGeometry",
            (sortingOrder),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SortGeometry_VertexSortingOrder0(
        &mut self,
        order: crate::TMPro::VertexSortingOrder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SortGeometry",
            (order),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SwapVertexData(
        &mut self,
        src: i32,
        dst: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SwapVertexData",
            (src, dst),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Mesh_i32_0(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (mesh, _cordl_size),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        _cordl_size: i32,
        isVolumetric: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (mesh, _cordl_size, isVolumetric),
        )?;
        Ok(__cordl_ret.into())
    }
}

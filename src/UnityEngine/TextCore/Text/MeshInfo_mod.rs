#[cfg(feature = "UnityEngine+TextCore+Text+MeshInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MeshInfo {
    pub vertexCount: i32,
    pub vertices: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    pub normals: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    pub tangents: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    pub uvs0: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    pub uvs2: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
    pub colors32: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
    pub triangles: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub material: *mut crate::UnityEngine::Material,
    pub glyphRenderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
}
#[cfg(feature = "UnityEngine+TextCore+Text+MeshInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::MeshInfo =>
    "UnityEngine.TextCore.Text"."MeshInfo"
);
#[cfg(feature = "UnityEngine+TextCore+Text+MeshInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::MeshInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+MeshInfo")]
impl crate::UnityEngine::TextCore::Text::MeshInfo {
    pub fn _ctor(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_size),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ResizeMeshInfo(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ResizeMeshInfo",
            (_cordl_size),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
        uploadChanges: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (uploadChanges),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ClearUnusedVertices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ClearUnusedVertices",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SortGeometry(
        &mut self,
        order: crate::UnityEngine::TextCore::Text::VertexSortingOrder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SortGeometry",
            (order),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}

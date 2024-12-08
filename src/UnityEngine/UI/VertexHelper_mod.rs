#[cfg(feature = "UnityEngine+UI+VertexHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct VertexHelper {
    __cordl_parent: crate::System::Object,
    pub m_Positions: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Vector3,
    >,
    pub m_Colors: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Color32,
    >,
    pub m_Uv0S: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Vector4,
    >,
    pub m_Uv1S: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Vector4,
    >,
    pub m_Uv2S: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Vector4,
    >,
    pub m_Uv3S: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Vector4,
    >,
    pub m_Normals: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Vector3,
    >,
    pub m_Tangents: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Vector4,
    >,
    pub m_Indices: *mut crate::System::Collections::Generic::List_1<i32>,
    pub m_ListsInitalized: bool,
}
#[cfg(feature = "UnityEngine+UI+VertexHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::VertexHelper =>
    "UnityEngine.UI"."VertexHelper"
);
#[cfg(feature = "UnityEngine+UI+VertexHelper")]
impl std::ops::Deref for crate::UnityEngine::UI::VertexHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+VertexHelper")]
impl std::ops::DerefMut for crate::UnityEngine::UI::VertexHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+VertexHelper")]
impl crate::UnityEngine::UI::VertexHelper {
    pub fn InitializeListIfRequired(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeListIfRequired", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_currentVertCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_currentVertCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Mesh1(
        &mut self,
        m: *mut crate::UnityEngine::Mesh,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (m))?;
        Ok(__cordl_ret)
    }
    pub fn PopulateUIVertex(
        &mut self,
        vertex: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIVertex>,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateUIVertex", (vertex, i))?;
        Ok(__cordl_ret)
    }
    pub fn get_currentIndexCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_currentIndexCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddVert_Vector3_Color32_Vector4_Vector4_Vector4_Vector4_Vector3_Vector4_0(
        &mut self,
        position: crate::UnityEngine::Vector3,
        color: crate::UnityEngine::Color32,
        uv0: crate::UnityEngine::Vector4,
        uv1: crate::UnityEngine::Vector4,
        uv2: crate::UnityEngine::Vector4,
        uv3: crate::UnityEngine::Vector4,
        normal: crate::UnityEngine::Vector3,
        tangent: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddVert", (position, color, uv0, uv1, uv2, uv3, normal, tangent))?;
        Ok(__cordl_ret)
    }
    pub fn AddVert_Vector3_Color32_Vector4_Vector4_Vector3_Vector4_1(
        &mut self,
        position: crate::UnityEngine::Vector3,
        color: crate::UnityEngine::Color32,
        uv0: crate::UnityEngine::Vector4,
        uv1: crate::UnityEngine::Vector4,
        normal: crate::UnityEngine::Vector3,
        tangent: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddVert", (position, color, uv0, uv1, normal, tangent))?;
        Ok(__cordl_ret)
    }
    pub fn AddVert_Vector3_Color32_Vector4_2(
        &mut self,
        position: crate::UnityEngine::Vector3,
        color: crate::UnityEngine::Color32,
        uv0: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddVert", (position, color, uv0))?;
        Ok(__cordl_ret)
    }
    pub fn AddVert_UIVertex3(
        &mut self,
        v: crate::UnityEngine::UIVertex,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddVert", (v))?;
        Ok(__cordl_ret)
    }
    pub fn AddUIVertexStream(
        &mut self,
        verts: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIVertex,
        >,
        indices: *mut crate::System::Collections::Generic::List_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddUIVertexStream", (verts, indices))?;
        Ok(__cordl_ret)
    }
    pub fn GetUIVertexStream(
        &mut self,
        stream: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIVertex,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetUIVertexStream", (stream))?;
        Ok(__cordl_ret)
    }
    pub fn SetUIVertex(
        &mut self,
        vertex: crate::UnityEngine::UIVertex,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUIVertex", (vertex, i))?;
        Ok(__cordl_ret)
    }
    pub fn AddTriangle(
        &mut self,
        idx0: i32,
        idx1: i32,
        idx2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTriangle", (idx0, idx1, idx2))?;
        Ok(__cordl_ret)
    }
    pub fn FillMesh(
        &mut self,
        mesh: *mut crate::UnityEngine::Mesh,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillMesh", (mesh))?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddUIVertexQuad(
        &mut self,
        verts: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::UIVertex>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddUIVertexQuad", (verts))?;
        Ok(__cordl_ret)
    }
    pub fn AddUIVertexTriangleStream(
        &mut self,
        verts: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIVertex,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddUIVertexTriangleStream", (verts))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Mesh1(
        m: *mut crate::UnityEngine::Mesh,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (m))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UI+VertexHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::VertexHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "UnityEngine+ProBuilder+SceneSelection")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneSelection {
    __cordl_parent: crate::System::Object,
    pub gameObject: *mut crate::UnityEngine::GameObject,
    pub mesh: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
    pub m_Vertices: *mut crate::System::Collections::Generic::List_1<i32>,
    pub m_Edges: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::ProBuilder::Edge,
    >,
    pub m_Faces: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::ProBuilder::Face,
    >,
    pub vertex: i32,
    pub edge: crate::UnityEngine::ProBuilder::Edge,
    pub face: *mut crate::UnityEngine::ProBuilder::Face,
}
#[cfg(feature = "UnityEngine+ProBuilder+SceneSelection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::SceneSelection =>
    "UnityEngine.ProBuilder"."SceneSelection"
);
#[cfg(feature = "UnityEngine+ProBuilder+SceneSelection")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::SceneSelection {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SceneSelection")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::SceneSelection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SceneSelection")]
impl crate::UnityEngine::ProBuilder::SceneSelection {
    pub fn CopyTo(
        &mut self,
        dst: *mut crate::UnityEngine::ProBuilder::SceneSelection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (dst))?;
        Ok(__cordl_ret)
    }
    pub fn get_edges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::ProBuilder::Edge,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::ProBuilder::Edge,
        > = __cordl_object.invoke("get_edges", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetSingleFace(
        &mut self,
        face: *mut crate::UnityEngine::ProBuilder::Face,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSingleFace", (face))?;
        Ok(__cordl_ret)
    }
    pub fn set_vertexes(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_vertexes", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_faces(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::ProBuilder::Face,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_faces", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_vertexes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<i32> = __cordl_object
            .invoke("get_vertexes", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetSingleEdge(
        &mut self,
        edge: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSingleEdge", (edge))?;
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
    pub fn set_edges(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::ProBuilder::Edge,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_edges", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetSingleVertex(
        &mut self,
        vertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSingleVertex", (vertex))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_GameObject0(
        &mut self,
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (gameObject))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ProBuilderMesh_i32_1(
        &mut self,
        mesh: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
        vertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (mesh, vertex))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ProBuilderMesh_Edge2(
        &mut self,
        mesh: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
        edge: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (mesh, edge))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ProBuilderMesh_Face3(
        &mut self,
        mesh: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
        face: *mut crate::UnityEngine::ProBuilder::Face,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (mesh, face))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ProBuilderMesh_List_1_4(
        &mut self,
        mesh: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
        vertexes: *mut crate::System::Collections::Generic::List_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (mesh, vertexes))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ProBuilderMesh_List_1_5(
        &mut self,
        mesh: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
        edges: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::ProBuilder::Edge,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (mesh, edges))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ProBuilderMesh_List_1_6(
        &mut self,
        mesh: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
        faces: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::ProBuilder::Face,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (mesh, faces))?;
        Ok(__cordl_ret)
    }
    pub fn get_faces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::ProBuilder::Face,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::ProBuilder::Face,
        > = __cordl_object.invoke("get_faces", ())?;
        Ok(__cordl_ret)
    }
    pub fn Equals_SceneSelection0(
        &mut self,
        other: *mut crate::UnityEngine::ProBuilder::SceneSelection,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_GameObject0(
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (gameObject))?;
        Ok(__cordl_object)
    }
    pub fn New_ProBuilderMesh_i32_1(
        mesh: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
        vertex: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mesh, vertex))?;
        Ok(__cordl_object)
    }
    pub fn New_ProBuilderMesh_Edge2(
        mesh: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
        edge: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mesh, edge))?;
        Ok(__cordl_object)
    }
    pub fn New_ProBuilderMesh_Face3(
        mesh: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
        face: *mut crate::UnityEngine::ProBuilder::Face,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mesh, face))?;
        Ok(__cordl_object)
    }
    pub fn New_ProBuilderMesh_List_1_4(
        mesh: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
        vertexes: *mut crate::System::Collections::Generic::List_1<i32>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mesh, vertexes))?;
        Ok(__cordl_object)
    }
    pub fn New_ProBuilderMesh_List_1_5(
        mesh: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
        edges: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::ProBuilder::Edge,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mesh, edges))?;
        Ok(__cordl_object)
    }
    pub fn New_ProBuilderMesh_List_1_6(
        mesh: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
        faces: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::ProBuilder::Face,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mesh, faces))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SceneSelection")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::SceneSelection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

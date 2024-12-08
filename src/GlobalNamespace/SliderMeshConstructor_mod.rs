#[cfg(feature = "SliderMeshConstructor")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderMeshConstructor {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _meshFilter: *mut crate::UnityEngine::MeshFilter,
    pub reusableVerts: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector3,
    >,
    pub reusableUvs: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector2,
    >,
    pub reusableNormals: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector3,
    >,
    pub reusableTriangles: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _mesh: *mut crate::UnityEngine::Mesh,
}
#[cfg(feature = "SliderMeshConstructor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SliderMeshConstructor => ""."SliderMeshConstructor"
);
#[cfg(feature = "SliderMeshConstructor")]
impl std::ops::Deref for SliderMeshConstructor {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderMeshConstructor")]
impl std::ops::DerefMut for SliderMeshConstructor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderMeshConstructor")]
impl SliderMeshConstructor {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Mesh> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Mesh = __cordl_object
            .invoke("get_mesh", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateSliderMesh(
        &mut self,
        path: *mut VertexPath,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateSliderMesh", (path))?;
        Ok(__cordl_ret)
    }
    pub fn CreateSliderMeshInternal(
        &mut self,
        path: *mut VertexPath,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateSliderMeshInternal", (path))?;
        Ok(__cordl_ret)
    }
    pub fn GetVertexCount(
        &mut self,
        path: *mut VertexPath,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetVertexCount", (path))?;
        Ok(__cordl_ret)
    }
    pub fn CreateMeshIfNonExisting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateMeshIfNonExisting", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTrianglesCount(
        &mut self,
        path: *mut VertexPath,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetTrianglesCount", (path))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "SliderMeshConstructor")]
impl quest_hook::libil2cpp::ObjectType for SliderMeshConstructor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

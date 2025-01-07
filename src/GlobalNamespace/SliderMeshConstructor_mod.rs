#[cfg(feature = "SliderMeshConstructor")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderMeshConstructor {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _meshFilter: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshFilter>,
    pub reusableVerts: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    >,
    pub reusableUvs: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    >,
    pub reusableNormals: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    >,
    pub reusableTriangles: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub _mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
}
#[cfg(feature = "SliderMeshConstructor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SliderMeshConstructor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SliderMeshConstructor";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "SliderMeshConstructor")]
impl std::ops::Deref for crate::GlobalNamespace::SliderMeshConstructor {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderMeshConstructor")]
impl std::ops::DerefMut for crate::GlobalNamespace::SliderMeshConstructor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderMeshConstructor")]
impl crate::GlobalNamespace::SliderMeshConstructor {
    pub fn CreateMeshIfNonExisting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateMeshIfNonExisting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSliderMesh(
        &mut self,
        path: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VertexPath>,
        zDistanceBetweenNotes: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateSliderMesh", (path, zDistanceBetweenNotes))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSliderMeshInternal(
        &mut self,
        path: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VertexPath>,
        zDistanceBetweenNotes: f32,
        bounds: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateSliderMeshInternal", (path, zDistanceBetweenNotes, bounds))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrianglesCount(
        &mut self,
        path: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VertexPath>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetTrianglesCount", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexCount(
        &mut self,
        path: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VertexPath>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetVertexCount", (path))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn get_mesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = __cordl_object
            .invoke("get_mesh", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SliderMeshConstructor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SliderMeshConstructor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

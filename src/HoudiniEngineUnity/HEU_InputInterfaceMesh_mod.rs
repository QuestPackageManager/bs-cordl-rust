#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputInterfaceMesh {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_InputInterface,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InputInterfaceMesh =>
    "HoudiniEngineUnity"."HEU_InputInterfaceMesh"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InputInterfaceMesh {
    type Target = quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_InputInterface,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_InputInterfaceMesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh")]
impl crate::HoudiniEngineUnity::HEU_InputInterfaceMesh {
    #[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh+HEU_InputDataMesh")]
    pub type HEU_InputDataMesh = crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMesh;
    #[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh+HEU_InputDataMeshes")]
    pub type HEU_InputDataMeshes = crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMeshes;
    pub fn CreateInputNodeWithDataUpload(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        connectNodeID: i32,
        inputObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        inputNodeID: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CreateInputNodeWithDataUpload",
                (session, connectNodeID, inputObject, inputNodeID),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSingleMeshData(
        meshGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMesh,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateSingleMeshData", (meshGameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateMeshDatasFromGameObject(
        &mut self,
        inputObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMeshes,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMeshes,
        > = __cordl_object.invoke("GenerateMeshDatasFromGameObject", (inputObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMeshFromObject(
        meshGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMeshFromObject", (meshGameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUVsFromMesh(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        srcUVs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        destUVs: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUVsFromMesh", (mesh, srcUVs, destUVs, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsThisInputObjectSupported(
        &mut self,
        inputObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsThisInputObjectSupported", (inputObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UploadData(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        inputNodeID: i32,
        inputData: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputData>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UploadData", (session, inputNodeID, inputData))?;
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
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InputInterfaceMesh {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh+HEU_InputDataMesh")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputInterfaceMesh_HEU_InputDataMesh {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    pub _materials: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        >,
    >,
    pub _meshPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _meshName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _numVertices: i32,
    pub _numSubMeshes: i32,
    pub _indexStart: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    pub _indexCount: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    pub _LODScreenTransition: f32,
    pub _transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh+HEU_InputDataMesh")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMesh =>
    "HoudiniEngineUnity"."HEU_InputInterfaceMesh/HEU_InputDataMesh"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh+HEU_InputDataMesh")]
impl std::ops::Deref
for crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMesh {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh+HEU_InputDataMesh")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh+HEU_InputDataMesh")]
impl crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMesh {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh+HEU_InputDataMesh")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMesh {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh+HEU_InputDataMeshes")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputInterfaceMesh_HEU_InputDataMeshes {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputData>,
    pub _inputMeshes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMesh,
        >,
    >,
    pub _hasLOD: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh+HEU_InputDataMeshes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMeshes =>
    "HoudiniEngineUnity"."HEU_InputInterfaceMesh/HEU_InputDataMeshes"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh+HEU_InputDataMeshes")]
impl std::ops::Deref
for crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMeshes {
    type Target = quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputData>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh+HEU_InputDataMeshes")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMeshes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh+HEU_InputDataMeshes")]
impl crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMeshes {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh+HEU_InputDataMeshes")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMeshes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

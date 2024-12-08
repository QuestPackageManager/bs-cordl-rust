#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh+HEU_InputDataMesh")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputInterfaceMesh_HEU_InputDataMesh {
    __cordl_parent: crate::System::Object,
    pub _mesh: *mut crate::UnityEngine::Mesh,
    pub _materials: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Material,
    >,
    pub _meshPath: *mut crate::System::String,
    pub _meshName: *mut crate::System::String,
    pub _numVertices: i32,
    pub _numSubMeshes: i32,
    pub _indexStart: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub _indexCount: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub _LODScreenTransition: f32,
    pub _transform: *mut crate::UnityEngine::Transform,
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
    type Target = crate::System::Object;
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
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
    __cordl_parent: crate::HoudiniEngineUnity::HEU_InputData,
    pub _inputMeshes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMesh,
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
    type Target = crate::HoudiniEngineUnity::HEU_InputData;
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
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
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputInterfaceMesh {
    __cordl_parent: crate::HoudiniEngineUnity::HEU_InputInterface,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InputInterfaceMesh =>
    "HoudiniEngineUnity"."HEU_InputInterfaceMesh"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceMesh")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InputInterfaceMesh {
    type Target = crate::HoudiniEngineUnity::HEU_InputInterface;
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
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        connectNodeID: i32,
        inputObject: *mut crate::UnityEngine::GameObject,
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
        Ok(__cordl_ret)
    }
    pub fn GenerateMeshDatasFromGameObject(
        &mut self,
        inputObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMeshes,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_InputInterfaceMesh_HEU_InputDataMeshes = __cordl_object
            .invoke("GenerateMeshDatasFromGameObject", (inputObject))?;
        Ok(__cordl_ret)
    }
    pub fn IsThisInputObjectSupported(
        &mut self,
        inputObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsThisInputObjectSupported", (inputObject))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn UploadData(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        inputNodeID: i32,
        inputData: *mut crate::HoudiniEngineUnity::HEU_InputData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UploadData", (session, inputNodeID, inputData))?;
        Ok(__cordl_ret)
    }
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

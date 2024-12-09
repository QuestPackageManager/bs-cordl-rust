#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshImporter")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshImporter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_SourceMesh: *mut crate::UnityEngine::Mesh,
    pub m_SourceMaterials: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Material,
    >,
    pub m_Destination: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
    pub m_Vertices: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::ProBuilder::Vertex,
    >,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshImporter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::MeshImporter =>
    "UnityEngine.ProBuilder.MeshOperations"."MeshImporter"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshImporter")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::MeshImporter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshImporter")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::MeshImporter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshImporter")]
impl crate::UnityEngine::ProBuilder::MeshOperations::MeshImporter {
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshImporter+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::MeshOperations::MeshImporter___c;
    pub fn Import_GameObject_MeshImportSettings0(
        &mut self,
        go: *mut crate::UnityEngine::GameObject,
        importSettings: *mut crate::UnityEngine::ProBuilder::MeshOperations::MeshImportSettings,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Import", (go, importSettings))?;
        Ok(__cordl_ret)
    }
    pub fn Import_MeshImportSettings1(
        &mut self,
        importSettings: *mut crate::UnityEngine::ProBuilder::MeshOperations::MeshImportSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Import", (importSettings))?;
        Ok(__cordl_ret)
    }
    pub fn New_GameObject0(
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (gameObject))?;
        Ok(__cordl_object)
    }
    pub fn New_Mesh_Il2CppArray_ProBuilderMesh1(
        sourceMesh: *mut crate::UnityEngine::Mesh,
        sourceMaterials: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Material,
        >,
        destination: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sourceMesh, sourceMaterials, destination))?;
        Ok(__cordl_object)
    }
    pub fn New_ProBuilderMesh2(
        destination: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (destination))?;
        Ok(__cordl_object)
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
    pub fn _ctor_Mesh_Il2CppArray_ProBuilderMesh1(
        &mut self,
        sourceMesh: *mut crate::UnityEngine::Mesh,
        sourceMaterials: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Material,
        >,
        destination: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sourceMesh, sourceMaterials, destination))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ProBuilderMesh2(
        &mut self,
        destination: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (destination))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshImporter")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::MeshImporter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

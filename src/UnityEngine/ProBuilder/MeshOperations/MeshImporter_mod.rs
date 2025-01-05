#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshImporter")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshImporter {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_SourceMesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    pub m_SourceMaterials: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        >,
    >,
    pub m_Destination: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::ProBuilderMesh,
    >,
    pub m_Vertices: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        >,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn Import_Gc0(
        &mut self,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        importSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::MeshOperations::MeshImportSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Import", (go, importSettings))?;
        Ok(__cordl_ret.into())
    }
    pub fn Import_Gc1(
        &mut self,
        importSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::MeshOperations::MeshImportSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Import", (importSettings))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Gc0(
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (gameObject))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Gc2(
        destination: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (destination))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Gc_Gc1(
        sourceMesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        sourceMaterials: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
            >,
        >,
        destination: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sourceMesh, sourceMaterials, destination))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Gc0(
        &mut self,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (gameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc2(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (destination))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc_Gc1(
        &mut self,
        sourceMesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        sourceMaterials: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
            >,
        >,
        destination: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sourceMesh, sourceMaterials, destination))?;
        Ok(__cordl_ret.into())
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

#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_MaterialFactory {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_MaterialFactory =>
    "HoudiniEngineUnity"."HEU_MaterialFactory"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialFactory")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_MaterialFactory {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialFactory")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_MaterialFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialFactory")]
impl crate::HoudiniEngineUnity::HEU_MaterialFactory {
    pub fn CopyMaterial(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyMaterial", (material))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHoudiniMaterialData(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        assetID: i32,
        materialID: i32,
        geoID: i32,
        partID: i32,
        materialCache: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
        >,
        assetCacheFolderPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_MaterialData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateHoudiniMaterialData",
                (
                    session,
                    assetID,
                    materialID,
                    geoID,
                    partID,
                    materialCache,
                    assetCacheFolderPath,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMaterialInCache(
        materialKey: i32,
        materialName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sourceType: crate::HoudiniEngineUnity::HEU_MaterialData_Source,
        bWriteToFile: bool,
        materialCache: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
        >,
        assetCacheFolderPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_MaterialData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateMaterialInCache",
                (
                    materialKey,
                    materialName,
                    sourceType,
                    bWriteToFile,
                    materialCache,
                    assetCacheFolderPath,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateNewHoudiniStandardMaterial(
        assetCacheFolderPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        materialName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bWriteToFile: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateNewHoudiniStandardMaterial",
                (assetCacheFolderPath, materialName, bWriteToFile),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUnitySubstanceMaterialData(
        materialKey: i32,
        materialPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        substanceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        substanceIndex: i32,
        materialCache: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
        >,
        assetCacheFolderPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_MaterialData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateUnitySubstanceMaterialData",
                (
                    materialKey,
                    materialPath,
                    substanceName,
                    substanceIndex,
                    materialCache,
                    assetCacheFolderPath,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteAssetMaterial(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeleteAssetMaterial", (material))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyNonAssetMaterial(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        bRegisterUndo: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyNonAssetMaterial", (material, bRegisterUndo))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoesMaterialExistInAssetCache(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoesMaterialExistInAssetCache", (material))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnableGPUInstancing(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnableGPUInstancing", (material))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractHoudiniImageToTextureFile(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        materialInfo: crate::HoudiniEngineUnity::HAPI_MaterialInfo,
        imagePlanes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assetCacheFolderPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ExtractHoudiniImageToTextureFile",
                (session, materialInfo, imagePlanes, assetCacheFolderPath),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractHoudiniImageToTexturePNGJPEG(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        materialInfo: crate::HoudiniEngineUnity::HAPI_MaterialInfo,
        imagePlanes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ExtractHoudiniImageToTexturePNGJPEG",
                (session, materialInfo, imagePlanes),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractHoudiniImageToTextureRaw(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        materialInfo: crate::HoudiniEngineUnity::HAPI_MaterialInfo,
        imagePlanes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ExtractHoudiniImageToTextureRaw",
                (session, materialInfo, imagePlanes),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FindPluginShader(
        shaderName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindPluginShader", (shaderName))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindShader(
        shaderName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindShader", (shaderName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateDefaultMaterialName(
        geoID: i32,
        partID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateDefaultMaterialName", (geoID, partID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultStandardMaterial() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultStandardMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaterialDataFromCache(
        materialKey: i32,
        materialCache: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_MaterialData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMaterialDataFromCache", (materialKey, materialCache))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaterialDataMapFromCache(
        materialCache: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            i32,
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            i32,
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMaterialDataMapFromCache", (materialCache))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNewMaterialWithShader(
        assetCacheFolderPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        shaderName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        materialName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bWriteToFile: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetNewMaterialWithShader",
                (assetCacheFolderPath, shaderName, materialName, bWriteToFile),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreateDefaultMaterialInCache(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        bWriteToFile: bool,
        materialCache: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
        >,
        assetCacheFolderPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_MaterialData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetOrCreateDefaultMaterialInCache",
                (
                    session,
                    geoID,
                    partID,
                    bWriteToFile,
                    materialCache,
                    assetCacheFolderPath,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnitySubstanceMaterialKey(
        unityMaterialPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        substanceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        substanceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetUnitySubstanceMaterialKey",
                (unityMaterialPath, substanceName, substanceIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSubstanceMaterialWithIndex(
        materialPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        substanceMaterialIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "LoadSubstanceMaterialWithIndex",
                (materialPath, substanceMaterialIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSubstanceMaterialWithName(
        materialPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        substanceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadSubstanceMaterialWithName", (materialPath, substanceName))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadTexture(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadTexture", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadUnityMaterial(
        materialPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadUnityMaterial", (materialPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn MaterialHasGPUInstancingEnabled(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MaterialHasGPUInstancingEnabled", (material))?;
        Ok(__cordl_ret.into())
    }
    pub fn MaterialNameToKey(
        materialName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MaterialNameToKey", (materialName))?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderAndExtractImageToTexture(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        materialInfo: crate::HoudiniEngineUnity::HAPI_MaterialInfo,
        textureParmID: i32,
        textureName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assetCacheFolderPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        isNormalMap: bool,
        invertTexture: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RenderAndExtractImageToTexture",
                (
                    session,
                    materialInfo,
                    textureParmID,
                    textureName,
                    assetCacheFolderPath,
                    isNormalMap,
                    invertTexture,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WhiteTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WhiteTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteMaterialToAssetCache(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        assetCacheFolderPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        materialName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bOverwriteExisting: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WriteMaterialToAssetCache",
                (material, assetCacheFolderPath, materialName, bOverwriteExisting),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MaterialFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_MaterialFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

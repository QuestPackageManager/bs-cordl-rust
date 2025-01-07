#[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_AssetDatabase {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_AssetDatabase {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_AssetDatabase";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_AssetDatabase {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_AssetDatabase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase")]
impl crate::HoudiniEngineUnity::HEU_AssetDatabase {
    #[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase+HEU_ImportAssetOptions")]
    pub type HEU_ImportAssetOptions = crate::HoudiniEngineUnity::HEU_AssetDatabase_HEU_ImportAssetOptions;
    pub fn AddObjectToAsset(
        objectToAdd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        assetObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddObjectToAsset", (objectToAdd, assetObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendMaterialsPathToAssetFolder(
        inAssetCacheFolder: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendMaterialsPathToAssetFolder", (inAssetCacheFolder))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendMeshesAssetFileName(
        assetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendMeshesAssetFileName", (assetName))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendMeshesPathToAssetFolder(
        inAssetCacheFolder: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendMeshesPathToAssetFolder", (inAssetCacheFolder))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendPrefabPath(
        inAssetCacheFolder: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        assetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendPrefabPath", (inAssetCacheFolder, assetName))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendTerrainPathToAssetFolder(
        inAssetCacheFolder: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendTerrainPathToAssetFolder", (inAssetCacheFolder))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendTexturesPathToAssetFolder(
        inAssetCacheFolder: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendTexturesPathToAssetFolder", (inAssetCacheFolder))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsAsset(
        assetObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsAsset", (assetObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyAndLoadAssetAtAnyPath(
        srcAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        copyPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        bOverwriteExisting: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CopyAndLoadAssetAtAnyPath",
                (srcAsset, copyPath, _cordl_type, bOverwriteExisting),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyAndLoadAssetAtGivenPath(
        srcAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        targetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyAndLoadAssetAtGivenPath", (srcAsset, targetPath, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyAndLoadAssetFromAssetCachePath(
        srcAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        copyPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        bOverwriteExisting: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CopyAndLoadAssetFromAssetCachePath",
                (srcAsset, copyPath, _cordl_type, bOverwriteExisting),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyAndLoadAssetWithRelativePath(
        srcAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        copyAssetFolder: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        relativePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        bOverwriteExisting: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CopyAndLoadAssetWithRelativePath",
                (
                    srcAsset,
                    copyAssetFolder,
                    relativePath,
                    _cordl_type,
                    bOverwriteExisting,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyAsset(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyAsset", (path, newPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyUniqueAndLoadAssetAtAnyPath(
        srcAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        copyPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CopyUniqueAndLoadAssetAtAnyPath",
                (srcAsset, copyPath, _cordl_type),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateAddObjectInAssetCacheFolder(
        assetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assetObjectFileName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        objectToAdd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        relativeFolderPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        exportRootPath: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        assetDBObject: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateAddObjectInAssetCacheFolder",
                (
                    assetName,
                    assetObjectFileName,
                    objectToAdd,
                    relativeFolderPath,
                    exportRootPath,
                    assetDBObject,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateAsset(
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateAsset", (asset, path))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateAssetCacheFolder(
        suggestedAssetPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        hash: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateAssetCacheFolder", (suggestedAssetPath, hash))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateObjectInAssetCacheFolder(
        objectToCreate: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        assetCacheRoot: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        relativeFolderPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        assetFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        bOverwriteExisting: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateObjectInAssetCacheFolder",
                (
                    objectToCreate,
                    assetCacheRoot,
                    relativeFolderPath,
                    assetFileName,
                    _cordl_type,
                    bOverwriteExisting,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePathWithFolders(
        inPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePathWithFolders", (inPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUniqueBakePath(
        assetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateUniqueBakePath", (assetName))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteAsset(
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeleteAsset", (asset))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteAssetAtPath(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeleteAssetAtPath", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteAssetCacheFolder(
        assetCacheFolderPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeleteAssetCacheFolder", (assetCacheFolderPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteAssetIfInBakedFolder(
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeleteAssetIfInBakedFolder", (asset))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetBakedPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetBakedPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetBakedPathWithAssetName(
        assetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetBakedPathWithAssetName", (assetName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetCachePath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetCachePath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetFullPath(
        inPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetFullPath", (inPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetOrScenePath(
        inputObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetOrScenePath", (inputObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetPath(
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetPath", (asset))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetPathWithSubAssetSupport(
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetPathWithSubAssetSupport", (asset))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetPathsFromAssetBundle(
        assetBundleFileName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetPathsFromAssetBundle", (assetBundleFileName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetRelativePath(
        inFullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetRelativePath", (inFullPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetRelativePathStart() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetRelativePathStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetRootPath(
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetRootPath", (asset))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetSubFolders() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetSubFolders", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetWorkingPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetWorkingPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBuiltinExtraResource<T>(
        resourceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBuiltinExtraResource", (resourceName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPackagesRelativePath(
        inFullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPackagesRelativePath", (inFullPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPackagesRelativePathStart() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPackagesRelativePathStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSubAssetPathFromPath(
        fullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mainPath: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        subPath: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSubAssetPathFromPath", (fullPath, mainPath, subPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUniqueAssetPath(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUniqueAssetPath", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUniqueAssetPathForUnityAsset(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUniqueAssetPathForUnityAsset", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnityProjectPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnityProjectPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValidAssetPath(
        inPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetValidAssetPath", (inPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImportAsset(
        assetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        heuOptions: crate::HoudiniEngineUnity::HEU_AssetDatabase_HEU_ImportAssetOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImportAsset", (assetPath, heuOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAssetInAssetCacheBakedFolder(
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAssetInAssetCacheBakedFolder", (asset))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAssetInAssetCacheWorkingFolder(
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAssetInAssetCacheWorkingFolder", (asset))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAssetSavedInScene(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAssetSavedInScene", (go))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPathInAssetCache(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPathInAssetCache", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPathInAssetCacheBakedFolder(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPathInAssetCacheBakedFolder", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPathInAssetCacheWorkingFolder(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPathInAssetCacheWorkingFolder", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPathRelativeToAssets(
        inPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPathRelativeToAssets", (inPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPathRelativeToPackages(
        inPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPathRelativeToPackages", (inPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSubAsset(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSubAsset", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAllAssetRepresentationsAtPath(
        assetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAllAssetRepresentationsAtPath", (assetPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAllAssetsAtPath(
        assetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAllAssetsAtPath", (assetPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetAtPath(
        assetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAssetAtPath", (assetPath, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSubAssetAtPath(
        mainPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subAssetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadSubAssetAtPath", (mainPath, subAssetPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadUnityAssetFromUniqueAssetPath<T>(
        assetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadUnityAssetFromUniqueAssetPath", (assetPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrintDependencies(
        targetGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrintDependencies", (targetGO))?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshAssetDatabase() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RefreshAssetDatabase", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveAndRefreshDatabase() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveAndRefreshDatabase", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveAssetDatabase() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveAssetDatabase", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectAssetAtPath(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SelectAssetAtPath", (path))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_AssetDatabase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase+HEU_ImportAssetOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_AssetDatabase_HEU_ImportAssetOptions {
    #[default]
    Default = 0i32,
    DontDownloadFromCacheServer = 8192i32,
    ForceSynchronousImport = 8i32,
    ForceUncompressedImport = 16384i32,
    ForceUpdate = 1i32,
    ImportRecursive = 256i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase+HEU_ImportAssetOptions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_AssetDatabase_HEU_ImportAssetOptions {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_ImportAssetOptions";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::HoudiniEngineUnity::HEU_AssetDatabase_HEU_ImportAssetOptions {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HEU_AssetDatabase_HEU_ImportAssetOptions {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::HoudiniEngineUnity::HEU_AssetDatabase_HEU_ImportAssetOptions {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::HoudiniEngineUnity::HEU_AssetDatabase_HEU_ImportAssetOptions {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}

#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_EditorUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_EditorUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_EditorUtility";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_EditorUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_EditorUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility")]
impl crate::HoudiniEngineUnity::HEU_EditorUtility {
    #[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility+HEU_ReplacePrefabOptions")]
    pub type HEU_ReplacePrefabOptions = crate::HoudiniEngineUnity::HEU_EditorUtility_HEU_ReplacePrefabOptions;
    pub fn AddComponent<T>(
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        bRegisterUndo: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddComponent", (target, bRegisterUndo))?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeAndReplaceAllInScene() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BakeAndReplaceAllInScene", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeAndReplaceAssets(
        rootAssets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_HoudiniAssetRoot,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BakeAndReplaceAssets", (rootAssets))?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeAndReplaceSelectedInScene() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BakeAndReplaceSelectedInScene", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearProgressBar() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearProgressBar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CollectDependencies(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
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
            .invoke("CollectDependencies", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn CookAll() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CookAll", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CookAssets(
        rootAssets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_HoudiniAssetRoot,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CookAssets", (rootAssets))?;
        Ok(__cordl_ret.into())
    }
    pub fn CookSelected() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CookSelected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DisconnectPrefabInstance(
        instance: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DisconnectPrefabInstance", (instance))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisplayDialog(
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ok: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DisplayDialog", (title, message, ok, cancel))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisplayErrorDialog(
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ok: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DisplayErrorDialog", (title, message, ok, cancel))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisplayProgressBar(
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        info: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        progress: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DisplayProgressBar", (title, info, progress))?;
        Ok(__cordl_ret.into())
    }
    pub fn EditorSaveFolderPanel(
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        folder: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EditorSaveFolderPanel", (title, folder, defaultName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExportAllAssetsToGeoFiles() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExportAllAssetsToGeoFiles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExportAssetsToGeoFiles(
        rootAssets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_HoudiniAssetRoot,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExportAssetsToGeoFiles", (rootAssets))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExportSelectedAssetsToGeoFiles() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExportSelectedAssetsToGeoFiles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllAssetRoots() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_HoudiniAssetRoot,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_HoudiniAssetRoot,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllAssetRoots", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectParentFolder(
        parentObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        generatedMaterials: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetObjectParentFolder", (parentObject, generatedMaterials))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectParentFolderHelper(
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetObjectParentFolderHelper", (instanceID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPrefabAsset(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPrefabAsset", (go))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPrefabAssetPath(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPrefabAssetPath", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSelectedAssetRoots() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_HoudiniAssetRoot,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_HoudiniAssetRoot,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSelectedAssetRoots", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSelectedObjects() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSelectedObjects", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSelectedObjectsMeanPosition() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Vector3,
    > {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSelectedObjectsMeanPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSelectedObjectsMeanTransform() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Matrix4x4,
    > {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSelectedObjectsMeanTransform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUniqueNameForSibling(
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUniqueNameForSibling", (parentTransform, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateGameObject(
        sourceGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        instantiateInWorldSpace: bool,
        bRegisterUndo: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InstantiateGameObject",
                (
                    sourceGameObject,
                    parentTransform,
                    instantiateInWorldSpace,
                    bRegisterUndo,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefab(
        prefabOriginal: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiatePrefab", (prefabOriginal))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDisconnectedPrefabInstance(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDisconnectedPrefabInstance", (go))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEditingInPrefabMode(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsEditingInPrefabMode", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEditorNotInPlayModeAndNotGoingToPlayMode() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsEditorNotInPlayModeAndNotGoingToPlayMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEditorPlaying() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsEditorPlaying", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPersistant(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPersistant", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPrefabAsset(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPrefabAsset", (go))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPrefabInstance(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPrefabInstance", (go))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkSceneDirty() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MarkSceneDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PrefabIsAddedComponentOverride(
        comp: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrefabIsAddedComponentOverride", (comp))?;
        Ok(__cordl_ret.into())
    }
    pub fn QuerySelectedMeshTopology() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QuerySelectedMeshTopology", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RebuildAll() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RebuildAll", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RebuildAssets(
        rootAssets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_HoudiniAssetRoot,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RebuildAssets", (rootAssets))?;
        Ok(__cordl_ret.into())
    }
    pub fn RebuildSelected() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RebuildSelected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleasedMouse() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReleasedMouse", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RepaintScene() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RepaintScene", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReplacePrefab(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        targetPrefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        heuOptions: crate::HoudiniEngineUnity::HEU_EditorUtility_HEU_ReplacePrefabOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReplacePrefab", (go, targetPrefab, heuOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn RevealInFinder(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RevealInFinder", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveAsPrefabAsset(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveAsPrefabAsset", (path, go))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectObject(
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SelectObject", (gameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectObjects(
        gameObjects: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SelectObjects", (gameObjects))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIsHidden(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        isHidden: bool,
        bIncludeChildren: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetIsHidden", (go, isHidden, bIncludeChildren))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetObjectDirtyForEditorUpdate(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetObjectDirtyForEditorUpdate", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStatic(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        bStatic: bool,
        bIncludeChildren: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetStatic", (go, bStatic, bIncludeChildren))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTextureToNormalMap(
        filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetTextureToNormalMap", (filename))?;
        Ok(__cordl_ret.into())
    }
    pub fn UndoCollapseCurrentGroup() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UndoCollapseCurrentGroup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UndoRecordObject(
        objectToUndo: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UndoRecordObject", (objectToUndo, name))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_EditorUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility+HEU_ReplacePrefabOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_EditorUtility_HEU_ReplacePrefabOptions {
    #[default]
    ConnectToPrefab = 1i32,
    Default = 0i32,
    ReplaceNameBased = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility+HEU_ReplacePrefabOptions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_EditorUtility_HEU_ReplacePrefabOptions {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_ReplacePrefabOptions";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility+HEU_ReplacePrefabOptions")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::HoudiniEngineUnity::HEU_EditorUtility_HEU_ReplacePrefabOptions {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility+HEU_ReplacePrefabOptions")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HEU_EditorUtility_HEU_ReplacePrefabOptions {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility+HEU_ReplacePrefabOptions")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::HoudiniEngineUnity::HEU_EditorUtility_HEU_ReplacePrefabOptions {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility+HEU_ReplacePrefabOptions")]
unsafe impl quest_hook::libil2cpp::Return
for crate::HoudiniEngineUnity::HEU_EditorUtility_HEU_ReplacePrefabOptions {
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

#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfTools")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ShelfTools {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfTools")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_ShelfTools =>
    "HoudiniEngineUnity"."HEU_ShelfTools"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfTools")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_ShelfTools {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfTools")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_ShelfTools {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfTools")]
impl crate::HoudiniEngineUnity::HEU_ShelfTools {
    pub const TARGET_ALL: &'static str = "all";
    pub const TARGET_UNITY: &'static str = "unity";
    pub fn AddShelf(
        shelfName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        shelfPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Shelf>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_Shelf,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddShelf", (shelfName, shelfPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn AreShelvesLoaded() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreShelvesLoaded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearShelves() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearShelves", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteTool(
        toolSlot: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExecuteTool", (toolSlot))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteToolBatch(
        toolName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        toolPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        batchObjects: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GameObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExecuteToolBatch", (toolName, toolPath, batchObjects))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteToolGenerator(
        toolName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        toolPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        targetPosition: crate::UnityEngine::Vector3,
        targetRotation: crate::UnityEngine::Quaternion,
        targetScale: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ExecuteToolGenerator",
                (toolName, toolPath, targetPosition, targetRotation, targetScale),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteToolNoInput(
        toolName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        toolPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExecuteToolNoInput", (toolName, toolPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteToolOperatorMultiple(
        toolName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        toolPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        inputObjects: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GameObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExecuteToolOperatorMultiple", (toolName, toolPath, inputObjects))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteToolOperatorSingle(
        toolName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        toolPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        inputObjects: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GameObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExecuteToolOperatorSingle", (toolName, toolPath, inputObjects))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentShelfIndex() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentShelfIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNumShelves() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNumShelves", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetShelfStorageEntry(
        shelfName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        shelfPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetShelfStorageEntry", (shelfName, shelfPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetShelf_Il2CppString1(
        shelfName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Shelf>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_Shelf,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetShelf", (shelfName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetShelf_i32_0(
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Shelf>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_Shelf,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetShelf", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSplitShelfEntry(
        shelfEntry: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        shelfName: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        shelfPath: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSplitShelfEntry", (shelfEntry, shelfName, shelfPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetToolAssetPath(
        tool: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ShelfToolData>,
        inPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetToolAssetPath", (tool, inPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetToolIconPath(
        tool: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ShelfToolData>,
        inPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetToolIconPath", (tool, inPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetToolResourcePath(
        tool: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ShelfToolData>,
        inPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetToolResourcePath", (tool, inPath, ext))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidInput(
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidInput", (gameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadShelves() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadShelves", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadToolFromJsonFile(
        jsonFilePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ShelfToolData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ShelfToolData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadToolFromJsonFile", (jsonFilePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadToolFromJsonString(
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        jsonFilePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ShelfToolData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ShelfToolData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadToolFromJsonString", (json, jsonFilePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadToolsFromDirectory(
        folderPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tools: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::List_1<
                *mut crate::HoudiniEngineUnity::HEU_ShelfToolData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadToolsFromDirectory", (folderPath, tools))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveShelf(
        shelfIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveShelf", (shelfIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveShelf() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveShelf", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCurrentShelf(
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetCurrentShelf", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetReloadShelves() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetReloadShelves", ())?;
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfTools")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_ShelfTools {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

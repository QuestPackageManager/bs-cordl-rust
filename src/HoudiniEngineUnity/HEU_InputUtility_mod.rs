#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputUtility {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InputUtility =>
    "HoudiniEngineUnity"."HEU_InputUtility"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InputUtility {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_InputUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
impl crate::HoudiniEngineUnity::HEU_InputUtility {
    pub fn CreateInputNodeWithMultiAssets(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        parentAsset: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        >,
        connectMergeID: quest_hook::libil2cpp::ByRefMut<i32>,
        inputAssetInfos: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputHDAInfo>,
            >,
        >,
        bKeepWorldTransform: bool,
        mergeParentID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateInputNodeWithMultiAssets",
                (
                    session,
                    parentAsset,
                    connectMergeID,
                    inputAssetInfos,
                    bKeepWorldTransform,
                    mergeParentID,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInputNodeWithMultiObjects(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        assetID: i32,
        connectMergeID: quest_hook::libil2cpp::ByRefMut<i32>,
        inputObjects: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputObjectInfo>,
            >,
        >,
        inputObjectsConnectedAssetIDs: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<i32>,
        >,
        inputNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputNode>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateInputNodeWithMultiObjects",
                (
                    session,
                    assetID,
                    connectMergeID,
                    inputObjects,
                    inputObjectsConnectedAssetIDs,
                    inputNode,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHighestPriority() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHighestPriority", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInputInterfaceByType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputInterface>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterface,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInputInterfaceByType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInputInterface_Gc0(
        inputObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputInterface>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterface,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInputInterface", (inputObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInputInterface_Gc1(
        inputObjectInfo: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputObjectInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputInterface>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterface,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInputInterface", (inputObjectInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterInputInterface(
        inputInterface: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterInputInterface", (inputInterface))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterInputInterface(
        inputInterface: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnregisterInputInterface", (inputInterface))?;
        Ok(__cordl_ret.into())
    }
    pub fn UploadInputObjectTransform(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        inputObject: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputObjectInfo,
        >,
        inputNodeID: i32,
        bKeepWorldTransform: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "UploadInputObjectTransform",
                (session, inputObject, inputNodeID, bKeepWorldTransform),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_InputUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

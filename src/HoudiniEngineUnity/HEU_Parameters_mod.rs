#[cfg(feature = "HoudiniEngineUnity+HEU_Parameters")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_Parameters {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _nodeID: i32,
    pub _uiLabel: *mut crate::System::String,
    pub _paramInts: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _paramFloats: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _paramStrings: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub _paramChoices: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::HoudiniEngineUnity::HAPI_ParmChoiceInfo,
    >,
    pub _rootParameters: *mut crate::System::Collections::Generic::List_1<i32>,
    pub _parameterList: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_ParameterData,
    >,
    pub _parameterModifiers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_ParameterModifier,
    >,
    pub _regenerateParameters: bool,
    pub _presetData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _defaultPresetData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _validParameters: bool,
    pub _showParameters: bool,
    pub _recacheUI: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Parameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_Parameters =>
    "HoudiniEngineUnity"."HEU_Parameters"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_Parameters")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_Parameters {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Parameters")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_Parameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Parameters")]
impl crate::HoudiniEngineUnity::HEU_Parameters {
    #[cfg(feature = "HoudiniEngineUnity+HEU_Parameters+__c__DisplayClass35_0")]
    pub type __c__DisplayClass35_0 = crate::HoudiniEngineUnity::HEU_Parameters___c__DisplayClass35_0;
    pub fn CleanUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanUp", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasModifiersPending(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasModifiersPending", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveParameter(
        &mut self,
        listIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveParameter", (listIndex))?;
        Ok(__cordl_ret)
    }
    pub fn SetStringToParameter(
        &mut self,
        paramName: *mut crate::System::String,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStringToParameter", (paramName, value))?;
        Ok(__cordl_ret)
    }
    pub fn GetParameter_i32_0(
        &mut self,
        listIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_ParameterData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_ParameterData = __cordl_object
            .invoke("GetParameter", (listIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetParameter_String1(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_ParameterData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_ParameterData = __cordl_object
            .invoke("GetParameter", (name))?;
        Ok(__cordl_ret)
    }
    pub fn get_RequiresRegeneration(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_RequiresRegeneration", ())?;
        Ok(__cordl_ret)
    }
    pub fn InsertInstanceToMultiParm(
        &mut self,
        unityParamIndex: i32,
        instanceIndex: i32,
        numInstancesToAdd: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InsertInstanceToMultiParm",
                (unityParamIndex, instanceIndex, numInstancesToAdd),
            )?;
        Ok(__cordl_ret)
    }
    pub fn set_RootParameters(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RootParameters", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetPresetData(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPresetData", (data))?;
        Ok(__cordl_ret)
    }
    pub fn AreParametersValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AreParametersValid", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetupRampParameter(
        &mut self,
        rampParameter: *mut crate::HoudiniEngineUnity::HEU_ParameterData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupRampParameter", (rampParameter))?;
        Ok(__cordl_ret)
    }
    pub fn set_ShowParameters(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ShowParameters", (value))?;
        Ok(__cordl_ret)
    }
    pub fn DownloadAsDefaultPresetData(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DownloadAsDefaultPresetData", (session))?;
        Ok(__cordl_ret)
    }
    pub fn ResetAllToDefault(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetAllToDefault", (session))?;
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
    pub fn get_RootParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<i32> = __cordl_object
            .invoke("get_RootParameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn UploadParameterInputs(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        parentAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        bForceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UploadParameterInputs", (session, parentAsset, bForceUpdate))?;
        Ok(__cordl_ret)
    }
    pub fn get_RecacheUI(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_RecacheUI", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPresetData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetPresetData", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetStringFromParameter(
        &mut self,
        paramName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetStringFromParameter", (paramName))?;
        Ok(__cordl_ret)
    }
    pub fn GetParameterWithParmID(
        &mut self,
        parmID: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_ParameterData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_ParameterData = __cordl_object
            .invoke("GetParameterWithParmID", (parmID))?;
        Ok(__cordl_ret)
    }
    pub fn DownloadPresetData(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DownloadPresetData", (session))?;
        Ok(__cordl_ret)
    }
    pub fn UploadPresetData(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UploadPresetData", (session))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateTransformParameters(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        HAPITransform: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_TransformEuler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateTransformParameters", (session, HAPITransform))?;
        Ok(__cordl_ret)
    }
    pub fn SyncParameterFromHoudini(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        parameterName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncParameterFromHoudini", (session, parameterName))?;
        Ok(__cordl_ret)
    }
    pub fn IsEquivalentTo(
        &mut self,
        other: *mut crate::HoudiniEngineUnity::HEU_Parameters,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret)
    }
    pub fn GetParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_ParameterData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_ParameterData,
        > = __cordl_object.invoke("GetParameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveInstancesFromMultiParm(
        &mut self,
        unityParamIndex: i32,
        instanceIndex: i32,
        numInstancesToRemove: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RemoveInstancesFromMultiParm",
                (unityParamIndex, instanceIndex, numInstancesToRemove),
            )?;
        Ok(__cordl_ret)
    }
    pub fn set_ParameterModifiers(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_ParameterModifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ParameterModifiers", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HaveParametersChanged(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HaveParametersChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParameterModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_ParameterModifier,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_ParameterModifier,
        > = __cordl_object.invoke("get_ParameterModifiers", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_RequiresRegeneration(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RequiresRegeneration", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_ShowParameters(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ShowParameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn UploadValuesToHoudini(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        parentAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        bDoCheck: bool,
        bForceUploadInputs: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UploadValuesToHoudini",
                (session, parentAsset, bDoCheck, bForceUploadInputs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetInputNodeConnectionObjects(
        &mut self,
        inputNodeObjects: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetInputNodeConnectionObjects", (inputNodeObjects))?;
        Ok(__cordl_ret)
    }
    pub fn set_RecacheUI(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RecacheUI", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        nodeID: i32,
        nodeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_NodeInfo,
        >,
        previousParamFolders: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::HoudiniEngineUnity::HEU_ParameterData,
        >,
        previousParamInputNodes: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::HoudiniEngineUnity::HEU_InputNode,
        >,
        parentAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "Initialize",
                (
                    session,
                    nodeID,
                    nodeInfo,
                    previousParamFolders,
                    previousParamInputNodes,
                    parentAsset,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ClearInstancesFromMultiParm(
        &mut self,
        unityParamIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearInstancesFromMultiParm", (unityParamIndex))?;
        Ok(__cordl_ret)
    }
    pub fn SyncInternalParametersForUndoCompare(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncInternalParametersForUndoCompare", (session))?;
        Ok(__cordl_ret)
    }
    pub fn GetParameterDataForUIRestore(
        &mut self,
        folderParams: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::HoudiniEngineUnity::HEU_ParameterData,
        >,
        inputNodeParams: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::HoudiniEngineUnity::HEU_InputNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetParameterDataForUIRestore", (folderParams, inputNodeParams))?;
        Ok(__cordl_ret)
    }
    pub fn GetDefaultPresetData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetDefaultPresetData", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessModifiers(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessModifiers", (session))?;
        Ok(__cordl_ret)
    }
    pub fn GetChosenIndexFromChoiceList(
        &mut self,
        inChoiceParameter: *mut crate::HoudiniEngineUnity::HEU_ParameterData,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetChosenIndexFromChoiceList", (inChoiceParameter))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Parameters")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_Parameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

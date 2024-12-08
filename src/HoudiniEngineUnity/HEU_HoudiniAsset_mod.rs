#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+AssetBuildAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_HoudiniAsset_AssetBuildAction {
    COOK = 2i32,
    DUPLICATE = 5i32,
    INVALID = 3i32,
    NONE = 0i32,
    RELOAD = 1i32,
    RESET_PARAMS = 6i32,
    STRIP_HEDATA = 4i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+AssetBuildAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_HoudiniAsset_AssetBuildAction => "HoudiniEngineUnity"
    ."HEU_HoudiniAsset/AssetBuildAction"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+AssetCookResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_HoudiniAsset_AssetCookResult {
    ERRORED = 2i32,
    NONE = 0i32,
    SUCCESS = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+AssetCookResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_HoudiniAsset_AssetCookResult => "HoudiniEngineUnity"
    ."HEU_HoudiniAsset/AssetCookResult"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+AssetCookStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_HoudiniAsset_AssetCookStatus {
    COOKING = 1i32,
    LOADING = 3i32,
    NONE = 0i32,
    POSTCOOK = 2i32,
    POSTLOAD = 4i32,
    PRELOAD = 5i32,
    SELECT_SUBASSET = 6i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+AssetCookStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_HoudiniAsset_AssetCookStatus => "HoudiniEngineUnity"
    ."HEU_HoudiniAsset/AssetCookStatus"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+AssetInstantiationMethod")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_HoudiniAsset_AssetInstantiationMethod {
    DEFAULT = 0i32,
    DUPLICATED = 1i32,
    UNDO = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+AssetInstantiationMethod")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_HoudiniAsset_AssetInstantiationMethod =>
    "HoudiniEngineUnity"."HEU_HoudiniAsset/AssetInstantiationMethod"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+HEU_AssetType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_HoudiniAsset_HEU_AssetType {
    TYPE_CURVE = 2i32,
    TYPE_HDA = 1i32,
    TYPE_INPUT = 3i32,
    TYPE_INVALID = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+HEU_AssetType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_HoudiniAsset_HEU_AssetType => "HoudiniEngineUnity"
    ."HEU_HoudiniAsset/HEU_AssetType"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_HoudiniAsset {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _assetType: crate::HoudiniEngineUnity::HEU_HoudiniAsset_HEU_AssetType,
    pub _assetInfo: crate::HoudiniEngineUnity::HAPI_AssetInfo,
    pub _nodeInfo: crate::HoudiniEngineUnity::HAPI_NodeInfo,
    pub _assetName: *mut crate::System::String,
    pub _assetOpName: *mut crate::System::String,
    pub _assetHelp: *mut crate::System::String,
    pub _assetID: i32,
    pub _assetPath: *mut crate::System::String,
    pub _loadAssetFromMemory: bool,
    pub _alwaysOverwriteOnLoad: bool,
    pub _assetFileObject: *mut crate::UnityEngine::Object,
    pub _objectNodes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_ObjectNode,
    >,
    pub _rootGameObject: *mut crate::UnityEngine::GameObject,
    pub _materialCache: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_MaterialData,
    >,
    pub _parameters: *mut crate::HoudiniEngineUnity::HEU_Parameters,
    pub _lastSyncedTransformMatrix: crate::UnityEngine::Matrix4x4,
    pub _assetCacheFolderPath: *mut crate::System::String,
    pub _subassetNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub _selectedSubassetIndex: i32,
    pub _savedAssetPreset: *mut crate::HoudiniEngineUnity::HEU_AssetPreset,
    pub _recookPreset: *mut crate::HoudiniEngineUnity::HEU_RecookPreset,
    pub _totalCookCount: i32,
    pub _requestBuildAction: crate::HoudiniEngineUnity::HEU_HoudiniAsset_AssetBuildAction,
    pub _checkParameterChangeForCook: bool,
    pub _skipCookCheck: bool,
    pub _uploadParameters: bool,
    pub _forceUploadInputs: bool,
    pub _upstreamCookChanged: bool,
    pub _cookStatus: crate::HoudiniEngineUnity::HEU_HoudiniAsset_AssetCookStatus,
    pub _lastCookResult: crate::HoudiniEngineUnity::HEU_HoudiniAsset_AssetCookResult,
    pub _isCookingAssetReloaded: bool,
    pub _bForceUpdate: bool,
    pub _sessionID: i64,
    pub _WarnedPrefabNotSupported_k__BackingField: bool,
    pub _uiLocked: bool,
    pub _showHDAOptions: bool,
    pub _showGenerateSection: bool,
    pub _showBakeSection: bool,
    pub _showEventsSection: bool,
    pub _showCurvesSection: bool,
    pub _showInputNodesSection: bool,
    pub _showToolsSection: bool,
    pub _showTerrainSection: bool,
    pub _instanceInputUIState: *mut crate::HoudiniEngineUnity::HEU_InstanceInputUIState,
    pub _reloadEvent: *mut crate::HoudiniEngineUnity::ReloadEvent,
    pub _cookedEvent: *mut crate::HoudiniEngineUnity::CookedEvent,
    pub _bakedEvent: *mut crate::HoudiniEngineUnity::BakedEvent,
    pub _reloadDataEvent: *mut crate::HoudiniEngineUnity::HEU_ReloadDataEvent,
    pub _cookedDataEvent: *mut crate::HoudiniEngineUnity::HEU_CookedDataEvent,
    pub _bakedDataEvent: *mut crate::HoudiniEngineUnity::HEU_BakedDataEvent,
    pub _preAssetEvent: *mut crate::HoudiniEngineUnity::HEU_PreAssetEvent,
    pub _refreshUIDelegate: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset_UpdateUIDelegate,
    pub _downstreamConnectionCookedEvent: *mut crate::HoudiniEngineUnity::CookedEvent,
    pub _generateUVs: bool,
    pub _generateTangents: bool,
    pub _generateNormals: bool,
    pub _pushTransformToHoudini: bool,
    pub _transformChangeTriggersCooks: bool,
    pub _cookingTriggersDownCooks: bool,
    pub _autoCookOnParameterChange: bool,
    pub _ignoreNonDisplayNodes: bool,
    pub _generateMeshUsingPoints: bool,
    pub _useLODGroups: bool,
    pub _splitGeosByGroup: bool,
    pub _sessionSyncAutoCook: bool,
    pub _bakeUpdateKeepPreviousTransformValues: bool,
    pub _pauseCooking: bool,
    pub _curveEditorEnabled: bool,
    pub _curves: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_Curve,
    >,
    pub _curveDrawCollision: crate::HoudiniEngineUnity::HEU_Curve_CurveDrawCollision,
    pub _curveDrawColliders: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Collider,
    >,
    pub _curveDrawLayerMask: crate::UnityEngine::LayerMask,
    pub _curveProjectMaxDistance: f32,
    pub _curveProjectDirection: crate::UnityEngine::Vector3,
    pub _curveDisableScaleRotation: bool,
    pub _curveCookOnDrag: bool,
    pub _curveFrameSelectedNodes: bool,
    pub _curveFrameSelectedNodeDistance: f32,
    pub _inputNodes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_InputNode,
    >,
    pub _handles: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_Handle,
    >,
    pub _handlesEnabled: bool,
    pub _volumeCaches: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_VolumeCache,
    >,
    pub _attributeStores: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_AttributesStore,
    >,
    pub _editableNodesToolsEnabled: bool,
    pub _toolsInfo: *mut crate::HoudiniEngineUnity::HEU_ToolsInfo,
    pub _serializedMetaData: *mut crate::HoudiniEngineUnity::HEU_AssetSerializedMetaData,
    pub _pendingAutoCookOnMouseRelease: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_HoudiniAsset =>
    "HoudiniEngineUnity"."HEU_HoudiniAsset"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_HoudiniAsset {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_HoudiniAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset")]
impl crate::HoudiniEngineUnity::HEU_HoudiniAsset {
    #[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+AssetCookResult")]
    pub type AssetCookResult = crate::HoudiniEngineUnity::HEU_HoudiniAsset_AssetCookResult;
    #[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+__c__DisplayClass365_0")]
    pub type __c__DisplayClass365_0 = crate::HoudiniEngineUnity::HEU_HoudiniAsset___c__DisplayClass365_0;
    #[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+UpdateUIDelegate")]
    pub type UpdateUIDelegate = crate::HoudiniEngineUnity::HEU_HoudiniAsset_UpdateUIDelegate;
    #[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+AssetBuildAction")]
    pub type AssetBuildAction = crate::HoudiniEngineUnity::HEU_HoudiniAsset_AssetBuildAction;
    #[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+AssetInstantiationMethod")]
    pub type AssetInstantiationMethod = crate::HoudiniEngineUnity::HEU_HoudiniAsset_AssetInstantiationMethod;
    #[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+__c")]
    pub type __c = crate::HoudiniEngineUnity::HEU_HoudiniAsset___c;
    #[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+__c__DisplayClass281_0")]
    pub type __c__DisplayClass281_0 = crate::HoudiniEngineUnity::HEU_HoudiniAsset___c__DisplayClass281_0;
    #[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+AssetCookStatus")]
    pub type AssetCookStatus = crate::HoudiniEngineUnity::HEU_HoudiniAsset_AssetCookStatus;
    #[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+HEU_AssetType")]
    pub type HEU_AssetType = crate::HoudiniEngineUnity::HEU_HoudiniAsset_HEU_AssetType;
    pub fn AddAttributeStore(
        &mut self,
        attributeStore: *mut crate::HoudiniEngineUnity::HEU_AttributesStore,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAttributeStore", (attributeStore))?;
        Ok(__cordl_ret)
    }
    pub fn AddCurve(
        &mut self,
        curve: *mut crate::HoudiniEngineUnity::HEU_Curve,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCurve", (curve))?;
        Ok(__cordl_ret)
    }
    pub fn AddCurveDrawCollider(
        &mut self,
        newCollider: *mut crate::UnityEngine::Collider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCurveDrawCollider", (newCollider))?;
        Ok(__cordl_ret)
    }
    pub fn AddDownstreamConnection(
        &mut self,
        receiver: *mut crate::UnityEngine::Events::UnityAction_3<
            *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
            bool,
            *mut crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::GameObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDownstreamConnection", (receiver))?;
        Ok(__cordl_ret)
    }
    pub fn AddInputNode(
        &mut self,
        node: *mut crate::HoudiniEngineUnity::HEU_InputNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddInputNode", (node))?;
        Ok(__cordl_ret)
    }
    pub fn AddToAssetDBCache(
        &mut self,
        assetObjectFileName: *mut crate::System::String,
        objectToAdd: *mut crate::UnityEngine::Object,
        relativeFolderPath: *mut crate::System::String,
        targetAssetDBObject: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddToAssetDBCache",
                (
                    assetObjectFileName,
                    objectToAdd,
                    relativeFolderPath,
                    targetAssetDBObject,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddVolumeCache(
        &mut self,
        cache: *mut crate::HoudiniEngineUnity::HEU_VolumeCache,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddVolumeCache", (cache))?;
        Ok(__cordl_ret)
    }
    pub fn ApplyInputPresets(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        inputPresets: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_InputPreset,
        >,
        bAddMissingInputsToRecookPreset: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ApplyInputPresets",
                (session, inputPresets, bAddMissingInputsToRecookPreset),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ApplyRecookPreset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyRecookPreset", ())?;
        Ok(__cordl_ret)
    }
    pub fn ApplyVolumeCachePresets(
        &mut self,
        volumeCachePresets: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_VolumeCachePreset,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ApplyVolumeCachePresets", (volumeCachePresets))?;
        Ok(__cordl_ret)
    }
    pub fn AssetUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AssetUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn BakeToExistingPrefab(
        &mut self,
        bakeTargetGO: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BakeToExistingPrefab", (bakeTargetGO))?;
        Ok(__cordl_ret)
    }
    pub fn BakeToExistingStandalone(
        &mut self,
        bakeTargetGO: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BakeToExistingStandalone", (bakeTargetGO))?;
        Ok(__cordl_ret)
    }
    pub fn BakeToNewPrefab(
        &mut self,
        destinationPrefabPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("BakeToNewPrefab", (destinationPrefabPath))?;
        Ok(__cordl_ret)
    }
    pub fn BakeToNewStandalone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("BakeToNewStandalone", ())?;
        Ok(__cordl_ret)
    }
    pub fn CalculateColliderState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateColliderState", ())?;
        Ok(__cordl_ret)
    }
    pub fn CalculateVisibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateVisibility", ())?;
        Ok(__cordl_ret)
    }
    pub fn CleanUpAndDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanUpAndDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn CleanUpHandles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanUpHandles", ())?;
        Ok(__cordl_ret)
    }
    pub fn CleanUpInputNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanUpInputNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearAllUpstreamConnections(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearAllUpstreamConnections", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearBuildRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearBuildRequest", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearCurveDrawColliders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearCurveDrawColliders", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearInvalidCurves(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearInvalidCurves", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearInvalidLists(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearInvalidLists", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearMaterialCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearMaterialCache", ())?;
        Ok(__cordl_ret)
    }
    pub fn CloneAssetWithoutHDA(
        &mut self,
        bakedAssetPath: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
        bWriteMeshesToAssetDatabase: bool,
        bReconnectPrefabInstances: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke(
                "CloneAssetWithoutHDA",
                (bakedAssetPath, bWriteMeshesToAssetDatabase, bReconnectPrefabInstances),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ConnectToUpstream(
        &mut self,
        upstreamAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConnectToUpstream", (upstreamAsset))?;
        Ok(__cordl_ret)
    }
    pub fn CopyPropertiesTo(
        &mut self,
        newAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyPropertiesTo", (newAsset))?;
        Ok(__cordl_ret)
    }
    pub fn CreateAndCookAsset(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        subassetIndex: i32,
        newAssetID: quest_hook::libil2cpp::ByRefMut<i32>,
        bCookTemplatedGeos: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CreateAndCookAsset",
                (session, subassetIndex, newAssetID, bCookTemplatedGeos),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateAssetInputs(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateAssetInputs", (session))?;
        Ok(__cordl_ret)
    }
    pub fn CreateObjectNode(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        objectInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_ObjectInfo,
        >,
        objectTranform: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_Transform,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_ObjectNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_ObjectNode = __cordl_object
            .invoke("CreateObjectNode", (session, objectInfo, objectTranform))?;
        Ok(__cordl_ret)
    }
    pub fn CreateObjects(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CreateObjects", (session))?;
        Ok(__cordl_ret)
    }
    pub fn DeleteAllGeneratedData(
        &mut self,
        bIsRebuild: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteAllGeneratedData", (bIsRebuild))?;
        Ok(__cordl_ret)
    }
    pub fn DeleteAssetCacheData(
        &mut self,
        bRegisterUndo: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteAssetCacheData", (bRegisterUndo))?;
        Ok(__cordl_ret)
    }
    pub fn DeleteSessionDataOnly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteSessionDataOnly", ())?;
        Ok(__cordl_ret)
    }
    pub fn DisableAllColliders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableAllColliders", ())?;
        Ok(__cordl_ret)
    }
    pub fn DisconnectFromUpstream(
        &mut self,
        upstreamAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectFromUpstream", (upstreamAsset))?;
        Ok(__cordl_ret)
    }
    pub fn DoPostCookWork(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoPostCookWork", (session))?;
        Ok(__cordl_ret)
    }
    pub fn DoesAssetRequireRecook(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DoesAssetRequireRecook", ())?;
        Ok(__cordl_ret)
    }
    pub fn DownloadParameterPresetFromHoudini(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DownloadParameterPresetFromHoudini", (session))?;
        Ok(__cordl_ret)
    }
    pub fn DuplicateAsset(
        &mut self,
        newRootGameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("DuplicateAsset", (newRootGameObject))?;
        Ok(__cordl_ret)
    }
    pub fn ExecutePostCookCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecutePostCookCallbacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn FinishRebuild(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("FinishRebuild", ())?;
        Ok(__cordl_ret)
    }
    pub fn GenerateAttributesStore(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateAttributesStore", (session))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateHandles(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateHandles", (session))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateInstances(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateInstances", (session))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateObjectsGeometry(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        bRebuild: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateObjectsGeometry", (session, bRebuild))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateParameters(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateParameters", (session))?;
        Ok(__cordl_ret)
    }
    pub fn GetAssetInputNode(
        &mut self,
        inputName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_InputNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_InputNode = __cordl_object
            .invoke("GetAssetInputNode", (inputName))?;
        Ok(__cordl_ret)
    }
    pub fn GetAssetPreset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_AssetPreset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_AssetPreset = __cordl_object
            .invoke("GetAssetPreset", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAssetSession(
        &mut self,
        bCreateIfInvalid: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_SessionBase> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_SessionBase = __cordl_object
            .invoke("GetAssetSession", (bCreateIfInvalid))?;
        Ok(__cordl_ret)
    }
    pub fn GetAttributeStore(
        &mut self,
        geoName: *mut crate::System::String,
        partID: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_AttributesStore,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_AttributesStore = __cordl_object
            .invoke("GetAttributeStore", (geoName, partID))?;
        Ok(__cordl_ret)
    }
    pub fn GetAttributesStores(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_AttributesStore,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_AttributesStore,
        > = __cordl_object.invoke("GetAttributesStores", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetClonableParts(
        &mut self,
        clonableParts: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_PartData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetClonableParts", (clonableParts))?;
        Ok(__cordl_ret)
    }
    pub fn GetCookStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::HEU_HoudiniAsset_AssetCookStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HEU_HoudiniAsset_AssetCookStatus = __cordl_object
            .invoke("GetCookStatus", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCurve(
        &mut self,
        curveName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_Curve> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_Curve = __cordl_object
            .invoke("GetCurve", (curveName))?;
        Ok(__cordl_ret)
    }
    pub fn GetCurveDrawColliders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Collider,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Collider,
        > = __cordl_object.invoke("GetCurveDrawColliders", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCurveDrawLayerMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LayerMask = __cordl_object
            .invoke("GetCurveDrawLayerMask", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCurves(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_Curve,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_Curve,
        > = __cordl_object.invoke("GetCurves", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEditableCurveCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetEditableCurveCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHandleByName(
        &mut self,
        handleName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_Handle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_Handle = __cordl_object
            .invoke("GetHandleByName", (handleName))?;
        Ok(__cordl_ret)
    }
    pub fn GetHandles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_Handle,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_Handle,
        > = __cordl_object.invoke("GetHandles", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHoudiniTransformAndApply(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetHoudiniTransformAndApply", (session))?;
        Ok(__cordl_ret)
    }
    pub fn GetInputNode(
        &mut self,
        inputName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_InputNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_InputNode = __cordl_object
            .invoke("GetInputNode", (inputName))?;
        Ok(__cordl_ret)
    }
    pub fn GetInputNodeByIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_InputNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_InputNode = __cordl_object
            .invoke("GetInputNodeByIndex", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetInputNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_InputNode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_InputNode,
        > = __cordl_object.invoke("GetInputNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetInstantiatedObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset = __cordl_object
            .invoke("GetInstantiatedObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetInstantiationMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::HEU_HoudiniAsset_AssetInstantiationMethod,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HEU_HoudiniAsset_AssetInstantiationMethod = __cordl_object
            .invoke("GetInstantiationMethod", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetInternalHDAPartWithGameObject(
        &mut self,
        outputGameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_PartData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_PartData = __cordl_object
            .invoke("GetInternalHDAPartWithGameObject", (outputGameObject))?;
        Ok(__cordl_ret)
    }
    pub fn GetMaterialCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_MaterialData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_MaterialData,
        > = __cordl_object.invoke("GetMaterialCache", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMaterialData(
        &mut self,
        material: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_MaterialData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_MaterialData = __cordl_object
            .invoke("GetMaterialData", (material))?;
        Ok(__cordl_ret)
    }
    pub fn GetNonParameterInputNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_InputNode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_InputNode,
        > = __cordl_object.invoke("GetNonParameterInputNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetObjectNodeByName(
        &mut self,
        objName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_ObjectNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_ObjectNode = __cordl_object
            .invoke("GetObjectNodeByName", (objName))?;
        Ok(__cordl_ret)
    }
    pub fn GetObjectTransform(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        objectID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::HAPI_Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HAPI_Transform = __cordl_object
            .invoke("GetObjectTransform", (session, objectID))?;
        Ok(__cordl_ret)
    }
    pub fn GetObjectWithID(
        &mut self,
        objId: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_ObjectNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_ObjectNode = __cordl_object
            .invoke("GetObjectWithID", (objId))?;
        Ok(__cordl_ret)
    }
    pub fn GetOutput(
        &mut self,
        outputs: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_GeneratedOutput,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetOutput", (outputs))?;
        Ok(__cordl_ret)
    }
    pub fn GetOutputGameObjects(
        &mut self,
        outputObjects: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetOutputGameObjects", (outputObjects))?;
        Ok(__cordl_ret)
    }
    pub fn GetOutputGeoNodes(
        &mut self,
        outputGeoNodes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_GeoNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetOutputGeoNodes", (outputGeoNodes))?;
        Ok(__cordl_ret)
    }
    pub fn GetValidAssetCacheFolderPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetValidAssetCacheFolderPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetVolumeCacheCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetVolumeCacheCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetVolumeCachePreset(
        &mut self,
        objName: *mut crate::System::String,
        geoName: *mut crate::System::String,
        tile: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_VolumeCachePreset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_VolumeCachePreset = __cordl_object
            .invoke("GetVolumeCachePreset", (objName, geoName, tile))?;
        Ok(__cordl_ret)
    }
    pub fn GetVolumeCaches(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_VolumeCache,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_VolumeCache,
        > = __cordl_object.invoke("GetVolumeCaches", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasInputNodeTransformChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasInputNodeTransformChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasTransformChangedSinceLastUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasTransformChangedSinceLastUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasValidAssetPath(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasValidAssetPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn HideAllGeometry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideAllGeometry", ())?;
        Ok(__cordl_ret)
    }
    pub fn InputNodeNotifyRemoved(
        &mut self,
        node: *mut crate::HoudiniEngineUnity::HEU_InputNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InputNodeNotifyRemoved", (node))?;
        Ok(__cordl_ret)
    }
    pub fn InternalSetAssetID(
        &mut self,
        assetID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalSetAssetID", (assetID))?;
        Ok(__cordl_ret)
    }
    pub fn InternalStartRecook(
        &mut self,
        bCheckParamsChanged: bool,
        bSkipCookCheck: bool,
        bUploadParameters: bool,
        bUploadParameterPreset: bool,
        bForceUploadInputs: bool,
        bCookingSessionSync: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "InternalStartRecook",
                (
                    bCheckParamsChanged,
                    bSkipCookCheck,
                    bUploadParameters,
                    bUploadParameterPreset,
                    bForceUploadInputs,
                    bCookingSessionSync,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InvalidateAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvalidateAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn InvokeBakedEvent(
        &mut self,
        bSuccess: bool,
        outputObjects: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::GameObject,
        >,
        isNewBake: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeBakedEvent", (bSuccess, outputObjects, isNewBake))?;
        Ok(__cordl_ret)
    }
    pub fn InvokePostCookEvent(
        &mut self,
        bCookSuccess: bool,
        outputObjects: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokePostCookEvent", (bCookSuccess, outputObjects))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeReloadEvent(
        &mut self,
        bCookSuccess: bool,
        outputObjects: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeReloadEvent", (bCookSuccess, outputObjects))?;
        Ok(__cordl_ret)
    }
    pub fn IsAssetSavedInScene(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAssetSavedInScene", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsAssetValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAssetValid", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsAssetValidInHoudini(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsAssetValidInHoudini", (session))?;
        Ok(__cordl_ret)
    }
    pub fn IsEquivalentTo(
        &mut self,
        asset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (asset))?;
        Ok(__cordl_ret)
    }
    pub fn IsValidForInteraction(
        &mut self,
        errorMessage: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsValidForInteraction", (errorMessage))?;
        Ok(__cordl_ret)
    }
    pub fn LoadAssetFileWithSubasset(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        bPromptForSubasset: bool,
        desiredSubassetIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "LoadAssetFileWithSubasset",
                (session, bPromptForSubasset, desiredSubassetIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LoadAssetPresetAndCook(
        &mut self,
        assetPreset: *mut crate::HoudiniEngineUnity::HEU_AssetPreset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadAssetPresetAndCook", (assetPreset))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn NotifyInputNodesCookFinished(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyInputNodesCookFinished", ())?;
        Ok(__cordl_ret)
    }
    pub fn NotifyUpstreamCooked(
        &mut self,
        upstreamAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        bSuccess: bool,
        outputs: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyUpstreamCooked", (upstreamAsset, bSuccess, outputs))?;
        Ok(__cordl_ret)
    }
    pub fn NumAttributeStores(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NumAttributeStores", ())?;
        Ok(__cordl_ret)
    }
    pub fn NumHandles(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NumHandles", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnValidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValidate", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopulateObjectInstanceInfos(
        &mut self,
        objInstanceInfos: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateObjectInstanceInfos", (objInstanceInfos))?;
        Ok(__cordl_ret)
    }
    pub fn PostAssetUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PostAssetUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessHoudiniCookStatus(
        &mut self,
        bAsync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessHoudiniCookStatus", (bAsync))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessPoskCook(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessPoskCook", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessRebuild(
        &mut self,
        bPromptForSubasset: bool,
        desiredSubassetIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessRebuild", (bPromptForSubasset, desiredSubassetIndex))?;
        Ok(__cordl_ret)
    }
    pub fn ReconnectInputsUpstreamNotifications(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReconnectInputsUpstreamNotifications", ())?;
        Ok(__cordl_ret)
    }
    pub fn RecookAsync(
        &mut self,
        bCheckParamsChanged: bool,
        bSkipCookCheck: bool,
        bUploadParameters: bool,
        bUploadParameterPreset: bool,
        bForceUploadInputs: bool,
        bCookingSessionSync: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "RecookAsync",
                (
                    bCheckParamsChanged,
                    bSkipCookCheck,
                    bUploadParameters,
                    bUploadParameterPreset,
                    bForceUploadInputs,
                    bCookingSessionSync,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RecookBlocking(
        &mut self,
        bCheckParamsChanged: bool,
        bSkipCookCheck: bool,
        bUploadParameters: bool,
        bUploadParameterPreset: bool,
        bForceUploadInputs: bool,
        bCookingSessionSync: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "RecookBlocking",
                (
                    bCheckParamsChanged,
                    bSkipCookCheck,
                    bUploadParameters,
                    bUploadParameterPreset,
                    bForceUploadInputs,
                    bCookingSessionSync,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RemoveAttributeStore(
        &mut self,
        attributeStore: *mut crate::HoudiniEngineUnity::HEU_AttributesStore,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAttributeStore", (attributeStore))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveCurve(
        &mut self,
        curve: *mut crate::HoudiniEngineUnity::HEU_Curve,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveCurve", (curve))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveCurveDrawCollider(
        &mut self,
        collider: *mut crate::UnityEngine::Collider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveCurveDrawCollider", (collider))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveDownstreamConnection(
        &mut self,
        receiver: *mut crate::UnityEngine::Events::UnityAction_3<
            *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
            bool,
            *mut crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::GameObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveDownstreamConnection", (receiver))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveInputNode(
        &mut self,
        node: *mut crate::HoudiniEngineUnity::HEU_InputNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveInputNode", (node))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveMaterial(
        &mut self,
        material: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveMaterial", (material))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveUnusedMaterials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveUnusedMaterials", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveVolumeCache(
        &mut self,
        cache: *mut crate::HoudiniEngineUnity::HEU_VolumeCache,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveVolumeCache", (cache))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveVolumeCachePreset(
        &mut self,
        preset: *mut crate::HoudiniEngineUnity::HEU_VolumeCachePreset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveVolumeCachePreset", (preset))?;
        Ok(__cordl_ret)
    }
    pub fn ReorderAttributeStore(
        &mut self,
        oldIndex: i32,
        newIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReorderAttributeStore", (oldIndex, newIndex))?;
        Ok(__cordl_ret)
    }
    pub fn RequestBakeInPlace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestBakeInPlace", ())?;
        Ok(__cordl_ret)
    }
    pub fn RequestCook(
        &mut self,
        bCheckParametersChanged: bool,
        bAsync: bool,
        bSkipCookCheck: bool,
        bUploadParameters: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RequestCook",
                (bCheckParametersChanged, bAsync, bSkipCookCheck, bUploadParameters),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RequestReload(
        &mut self,
        bAsync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestReload", (bAsync))?;
        Ok(__cordl_ret)
    }
    pub fn RequestResetParameters(
        &mut self,
        bAsync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestResetParameters", (bAsync))?;
        Ok(__cordl_ret)
    }
    pub fn ResetAndCopyInstantiatedProperties(
        &mut self,
        newAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetAndCopyInstantiatedProperties", (newAsset))?;
        Ok(__cordl_ret)
    }
    pub fn ResetMaterialOverrides(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetMaterialOverrides", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResetParametersToDefault(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetParametersToDefault", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetCookStatus(
        &mut self,
        status: crate::HoudiniEngineUnity::HEU_HoudiniAsset_AssetCookStatus,
        result: crate::HoudiniEngineUnity::HEU_HoudiniAsset_AssetCookResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCookStatus", (status, result))?;
        Ok(__cordl_ret)
    }
    pub fn SetCurveDrawLayerMask(
        &mut self,
        mask: crate::UnityEngine::LayerMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCurveDrawLayerMask", (mask))?;
        Ok(__cordl_ret)
    }
    pub fn SetSoftDeleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSoftDeleted", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetupAsset(
        &mut self,
        assetType: crate::HoudiniEngineUnity::HEU_HoudiniAsset_HEU_AssetType,
        filePath: *mut crate::System::String,
        rootGameObject: *mut crate::UnityEngine::GameObject,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupAsset", (assetType, filePath, rootGameObject, session))?;
        Ok(__cordl_ret)
    }
    pub fn StartHoudiniCookNode(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("StartHoudiniCookNode", (session))?;
        Ok(__cordl_ret)
    }
    pub fn StartRebuild(
        &mut self,
        bPromptForSubasset: bool,
        desiredSubassetIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("StartRebuild", (bPromptForSubasset, desiredSubassetIndex))?;
        Ok(__cordl_ret)
    }
    pub fn SyncDirtyAttributesToHoudini(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncDirtyAttributesToHoudini", (session))?;
        Ok(__cordl_ret)
    }
    pub fn SyncInternalParametersForUndoCompare(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncInternalParametersForUndoCompare", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateAllObjectNodes(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAllObjectNodes", (session))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateHoudiniMaterials(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateHoudiniMaterials", (session))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateInputsOnAssetRecreation(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateInputsOnAssetRecreation", (session))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateParameterInputsToHoudini(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        bForceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateParameterInputsToHoudini", (session, bForceUpdate))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateSessionSync(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("UpdateSessionSync", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateTotalCookCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateTotalCookCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn UploadAttributeValues(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UploadAttributeValues", (session))?;
        Ok(__cordl_ret)
    }
    pub fn UploadCurvesParameters(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        bCheckParamsChanged: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UploadCurvesParameters", (session, bCheckParamsChanged))?;
        Ok(__cordl_ret)
    }
    pub fn UploadInputNodes(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        bForceUpdate: bool,
        bUpdateAll: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UploadInputNodes", (session, bForceUpdate, bUpdateAll))?;
        Ok(__cordl_ret)
    }
    pub fn UploadParameterPresetToHoudini(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UploadParameterPresetToHoudini", (session))?;
        Ok(__cordl_ret)
    }
    pub fn UploadUnityTransform(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        bOnlySendIfChangedFromLastSync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UploadUnityTransform", (session, bOnlySendIfChangedFromLastSync))?;
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
    pub fn get_AlwaysOverwriteOnLoad(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AlwaysOverwriteOnLoad", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AssetHelp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AssetHelp", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AssetID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_AssetID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AssetInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::HAPI_AssetInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HAPI_AssetInfo = __cordl_object
            .invoke("get_AssetInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AssetName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AssetName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AssetOpName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AssetOpName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AssetPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AssetPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AssetType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::HEU_HoudiniAsset_HEU_AssetType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HEU_HoudiniAsset_HEU_AssetType = __cordl_object
            .invoke("get_AssetType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AutoCookOnParameterChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_AutoCookOnParameterChange", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_BakeUpdateKeepPreviousTransformValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_BakeUpdateKeepPreviousTransformValues", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CookingTriggersDownCooks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_CookingTriggersDownCooks", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CurveCookOnDrag(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CurveCookOnDrag", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CurveDisableScaleRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_CurveDisableScaleRotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CurveDrawCollision(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::HEU_Curve_CurveDrawCollision,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HEU_Curve_CurveDrawCollision = __cordl_object
            .invoke("get_CurveDrawCollision", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CurveEditorEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CurveEditorEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CurveFrameSelectedNodeDistance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_CurveFrameSelectedNodeDistance", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CurveFrameSelectedNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_CurveFrameSelectedNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EditableNodesToolsEnabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_EditableNodesToolsEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GenerateMeshUsingPoints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_GenerateMeshUsingPoints", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GenerateNormals(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_GenerateNormals", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GenerateTangents(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_GenerateTangents", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GenerateUVs(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_GenerateUVs", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GeoInputCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_GeoInputCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HandleCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_HandleCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HandlesEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HandlesEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IgnoreNonDisplayNodes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IgnoreNonDisplayNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InstanceInputUIState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_InstanceInputUIState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_InstanceInputUIState = __cordl_object
            .invoke("get_InstanceInputUIState", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LoadAssetFromMemory(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_LoadAssetFromMemory", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NodeInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::HAPI_NodeInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HAPI_NodeInfo = __cordl_object
            .invoke("get_NodeInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OwnerGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_OwnerGameObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Parameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_Parameters> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_Parameters = __cordl_object
            .invoke("get_Parameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PauseCooking(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_PauseCooking", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PendingAutoCookOnMouseRelease(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_PendingAutoCookOnMouseRelease", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PushTransformToHoudini(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_PushTransformToHoudini", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RootGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_RootGameObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SerializedMetaData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_AssetSerializedMetaData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_AssetSerializedMetaData = __cordl_object
            .invoke("get_SerializedMetaData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SessionID(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_SessionID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SessionSyncAutoCook(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SessionSyncAutoCook", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SplitGeosByGroup(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SplitGeosByGroup", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SubassetNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_SubassetNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ToolsInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_ToolsInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_ToolsInfo = __cordl_object
            .invoke("get_ToolsInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TransformChangeTriggersCooks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_TransformChangeTriggersCooks", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TransformInputCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_TransformInputCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UseLODGroups(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseLODGroups", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_WarnedPrefabNotSupported(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_WarnedPrefabNotSupported", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_AlwaysOverwriteOnLoad(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AlwaysOverwriteOnLoad", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_AutoCookOnParameterChange(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AutoCookOnParameterChange", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_BakeUpdateKeepPreviousTransformValues(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BakeUpdateKeepPreviousTransformValues", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_CookingTriggersDownCooks(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CookingTriggersDownCooks", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_CurveCookOnDrag(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CurveCookOnDrag", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_CurveDisableScaleRotation(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CurveDisableScaleRotation", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_CurveDrawCollision(
        &mut self,
        value: crate::HoudiniEngineUnity::HEU_Curve_CurveDrawCollision,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CurveDrawCollision", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_CurveEditorEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CurveEditorEnabled", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_CurveFrameSelectedNodeDistance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CurveFrameSelectedNodeDistance", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_CurveFrameSelectedNodes(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CurveFrameSelectedNodes", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_EditableNodesToolsEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EditableNodesToolsEnabled", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_GenerateMeshUsingPoints(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GenerateMeshUsingPoints", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_GenerateNormals(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GenerateNormals", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_GenerateTangents(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GenerateTangents", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_GenerateUVs(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GenerateUVs", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_HandlesEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_HandlesEnabled", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_IgnoreNonDisplayNodes(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IgnoreNonDisplayNodes", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_InstanceInputUIState(
        &mut self,
        value: *mut crate::HoudiniEngineUnity::HEU_InstanceInputUIState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InstanceInputUIState", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_LoadAssetFromMemory(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LoadAssetFromMemory", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_PauseCooking(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PauseCooking", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_PendingAutoCookOnMouseRelease(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PendingAutoCookOnMouseRelease", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_PushTransformToHoudini(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PushTransformToHoudini", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SessionSyncAutoCook(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SessionSyncAutoCook", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SplitGeosByGroup(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SplitGeosByGroup", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_TransformChangeTriggersCooks(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TransformChangeTriggersCooks", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_UseLODGroups(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UseLODGroups", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_WarnedPrefabNotSupported(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_WarnedPrefabNotSupported", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_HoudiniAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+UpdateUIDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_HoudiniAsset_UpdateUIDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+UpdateUIDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_HoudiniAsset_UpdateUIDelegate => "HoudiniEngineUnity"
    ."HEU_HoudiniAsset/UpdateUIDelegate"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+UpdateUIDelegate")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_HoudiniAsset_UpdateUIDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+UpdateUIDelegate")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_HoudiniAsset_UpdateUIDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+UpdateUIDelegate")]
impl crate::HoudiniEngineUnity::HEU_HoudiniAsset_UpdateUIDelegate {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniAsset+UpdateUIDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_HoudiniAsset_UpdateUIDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
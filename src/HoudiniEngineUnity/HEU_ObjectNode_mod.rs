#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectNode")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ObjectNode {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _objName: *mut crate::System::String,
    pub _parentAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    pub _objectInfo: crate::HoudiniEngineUnity::HAPI_ObjectInfo,
    pub _geoNodes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_GeoNode,
    >,
    pub _objectTransform: crate::HoudiniEngineUnity::HAPI_Transform,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_ObjectNode =>
    "HoudiniEngineUnity"."HEU_ObjectNode"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectNode")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_ObjectNode {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectNode")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_ObjectNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectNode")]
impl crate::HoudiniEngineUnity::HEU_ObjectNode {
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
    pub fn UpdateObject(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        bForceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateObject", (session, bForceUpdate))?;
        Ok(__cordl_ret)
    }
    pub fn IsUsingMaterial(
        &mut self,
        materialData: *mut crate::HoudiniEngineUnity::HEU_MaterialData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsUsingMaterial", (materialData))?;
        Ok(__cordl_ret)
    }
    pub fn ClearObjectInstances(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearObjectInstances", (session))?;
        Ok(__cordl_ret)
    }
    pub fn DestroyAllData(
        &mut self,
        bIsRebuild: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyAllData", (bIsRebuild))?;
        Ok(__cordl_ret)
    }
    pub fn GetCurves(
        &mut self,
        curves: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_Curve,
        >,
        bEditableOnly: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetCurves", (curves, bEditableOnly))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        objectInfo: crate::HoudiniEngineUnity::HAPI_ObjectInfo,
        objectTranform: crate::HoudiniEngineUnity::HAPI_Transform,
        parentAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (session, objectInfo, objectTranform, parentAsset))?;
        Ok(__cordl_ret)
    }
    pub fn SetObjectInfo(
        &mut self,
        newObjectInfo: crate::HoudiniEngineUnity::HAPI_ObjectInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetObjectInfo", (newObjectInfo))?;
        Ok(__cordl_ret)
    }
    pub fn IsInstancer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsInstancer", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHDAPartWithGameObject(
        &mut self,
        outputGameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_PartData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_PartData = __cordl_object
            .invoke("GetHDAPartWithGameObject", (outputGameObject))?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ObjectID", ())?;
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
    pub fn IsInstanced(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsInstanced", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDebugInfo(
        &mut self,
        sb: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetDebugInfo", (sb))?;
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
    pub fn ApplyObjectTransformToGeoNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyObjectTransformToGeoNodes", ())?;
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
    pub fn get_ObjectName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ObjectName", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsVisible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsVisible", ())?;
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
    pub fn GetGeoNode(
        &mut self,
        geoName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_GeoNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_GeoNode = __cordl_object
            .invoke("GetGeoNode", (geoName))?;
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
    pub fn ProcessUnityScriptAttributes(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessUnityScriptAttributes", (session))?;
        Ok(__cordl_ret)
    }
    pub fn SyncWithObjectInfo(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncWithObjectInfo", (session))?;
        Ok(__cordl_ret)
    }
    pub fn IsEquivalentTo(
        &mut self,
        other: *mut crate::HoudiniEngineUnity::HEU_ObjectNode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret)
    }
    pub fn GetOutputGeoNodes(
        &mut self,
        outGeoNodes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_GeoNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetOutputGeoNodes", (outGeoNodes))?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParentAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset = __cordl_object
            .invoke("get_ParentAsset", ())?;
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
    pub fn GenerateObjectInstances(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateObjectInstances", (session))?;
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
    pub fn GeneratePartInstances(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GeneratePartInstances", (session))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateGeometry(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        bRebuild: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateGeometry", (session, bRebuild))?;
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
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateGeoNode(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        geoInfo: crate::HoudiniEngineUnity::HAPI_GeoInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_GeoNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_GeoNode = __cordl_object
            .invoke("CreateGeoNode", (session, geoInfo))?;
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectNode")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_ObjectNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

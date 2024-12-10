#[cfg(feature = "HoudiniEngineUnity+HEU_AttributesStore")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_AttributesStore {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _geoID: i32,
    pub _partID: i32,
    pub _geoName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _attributeDatas: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_AttributeData,
    >,
    pub _hasColorAttribute: bool,
    pub _localMaterial: *mut crate::UnityEngine::Material,
    pub _outputTransform: *mut crate::UnityEngine::Transform,
    pub _positionAttributeValues: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector3,
    >,
    pub _vertexIndices: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _outputGameObject: *mut crate::UnityEngine::GameObject,
    pub _outputMesh: *mut crate::UnityEngine::Mesh,
    pub _outputMaterials: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Material,
    >,
    pub _outputCollider: *mut crate::UnityEngine::MeshCollider,
    pub _outputColliderMesh: *mut crate::UnityEngine::Mesh,
    pub _outputMeshCollider: *mut crate::UnityEngine::MeshCollider,
    pub _localMeshCollider: *mut crate::UnityEngine::MeshCollider,
    pub _outputMeshRendererInitiallyEnabled: bool,
    pub _outputMeshColliderInitiallyEnabled: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributesStore")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_AttributesStore =>
    "HoudiniEngineUnity"."HEU_AttributesStore"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributesStore")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_AttributesStore {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributesStore")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_AttributesStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributesStore")]
impl crate::HoudiniEngineUnity::HEU_AttributesStore {
    #[cfg(feature = "HoudiniEngineUnity+HEU_AttributesStore+SetAttributeValueFunc")]
    pub type SetAttributeValueFunc = crate::HoudiniEngineUnity::HEU_AttributesStore_SetAttributeValueFunc;
    pub fn AreAttributesDirty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AreAttributesDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyAttributeValuesTo(
        &mut self,
        destAttrStore: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_AttributesStore,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyAttributeValuesTo", (destAttrStore))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateAttribute(
        &mut self,
        attributeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attributeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_AttributeData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_AttributeData,
        > = __cordl_object.invoke("CreateAttribute", (attributeName, attributeInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyAllData(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyAllData", (asset))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisablePaintCollider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisablePaintCollider", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnablePaintCollider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnablePaintCollider", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FillAttribute(
        &mut self,
        attributeData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_AttributeData,
        >,
        sourceTools: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ToolsInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillAttribute", (attributeData, sourceTools))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributeData_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_AttributeData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_AttributeData,
        > = __cordl_object.invoke("GetAttributeData", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributeData_i32_1(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_AttributeData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_AttributeData,
        > = __cordl_object.invoke("GetAttributeData", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributeNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("GetAttributeNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributesList(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        attributesList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::HoudiniEngineUnity::HEU_AttributeData,
            >,
        >,
        ownerType: crate::HoudiniEngineUnity::HAPI_AttributeOwner,
        attributeCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetAttributesList",
                (session, geoID, partID, attributesList, ownerType, attributeCount),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPaintMeshCollider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshCollider>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshCollider> = __cordl_object
            .invoke("GetPaintMeshCollider", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPositionAttributeValues(
        &mut self,
        positionArray: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetPositionAttributeValues", (positionArray))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexIndices(
        &mut self,
        indices: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetVertexIndices", (indices))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasColorAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasColorAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasDirtyAttributes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasDirtyAttributes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasMeshForPainting(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasMeshForPainting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HidePaintMesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HidePaintMesh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_AttributesStore>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidStore(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValidStore", (session))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PaintAttribute(
        &mut self,
        attributeData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_AttributeData,
        >,
        sourceTools: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ToolsInfo>,
        attributeIndex: i32,
        paintFactor: f32,
        setAttrFunc: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_AttributesStore_SetAttributeValueFunc,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PaintAttribute",
                (attributeData, sourceTools, attributeIndex, paintFactor, setAttrFunc),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateAttributeData(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        attributeData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_AttributeData,
        >,
        attributeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PopulateAttributeData",
                (session, geoID, partID, attributeData, attributeInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshUpstreamInputs(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshUpstreamInputs", (session))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupMeshAndMaterials(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        partType: crate::HoudiniEngineUnity::HAPI_PartType,
        outputGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupMeshAndMaterials", (asset, partType, outputGameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowPaintMesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowPaintMesh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SyncAllAttributesFrom(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        geoID: i32,
        partInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_PartInfo,
        >,
        outputGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SyncAllAttributesFrom",
                (session, asset, geoID, partInfo, outputGameObject),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SyncDirtyAttributesToHoudini(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncDirtyAttributesToHoudini", (session))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAttribute(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        attributeData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_AttributeData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAttribute", (session, geoID, partID, attributeData))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAttributeList(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        attributeDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::HoudiniEngineUnity::HEU_AttributeData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAttributeList", (session, geoID, partID, attributeDataList))?;
        Ok(__cordl_ret.into())
    }
    pub fn UploadAttributeViaMeshInput(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UploadAttributeViaMeshInput", (session, geoID, partID))?;
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
    pub fn get_GeoID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_GeoID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GeoName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_GeoName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OutputMesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = __cordl_object
            .invoke("get_OutputMesh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OutputTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = __cordl_object
            .invoke("get_OutputTransform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PartID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_PartID", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributesStore")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_AttributesStore {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributesStore")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_AttributesStore,
    >,
> for crate::HoudiniEngineUnity::HEU_AttributesStore {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_AttributesStore,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributesStore")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_AttributesStore,
    >,
> for crate::HoudiniEngineUnity::HEU_AttributesStore {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_AttributesStore,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributesStore+SetAttributeValueFunc")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_AttributesStore_SetAttributeValueFunc {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributesStore+SetAttributeValueFunc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_AttributesStore_SetAttributeValueFunc =>
    "HoudiniEngineUnity"."HEU_AttributesStore/SetAttributeValueFunc"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributesStore+SetAttributeValueFunc")]
impl std::ops::Deref
for crate::HoudiniEngineUnity::HEU_AttributesStore_SetAttributeValueFunc {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributesStore+SetAttributeValueFunc")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_AttributesStore_SetAttributeValueFunc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributesStore+SetAttributeValueFunc")]
impl crate::HoudiniEngineUnity::HEU_AttributesStore_SetAttributeValueFunc {
    pub fn BeginInvoke(
        &mut self,
        attributeData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_AttributeData,
        >,
        targetIndex: i32,
        sourceTools: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ToolsInfo>,
        sourceIndex: i32,
        factor: f32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (
                    attributeData,
                    targetIndex,
                    sourceTools,
                    sourceIndex,
                    factor,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        attributeData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_AttributeData,
        >,
        targetIndex: i32,
        sourceTools: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ToolsInfo>,
        sourceIndex: i32,
        factor: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Invoke",
                (attributeData, targetIndex, sourceTools, sourceIndex, factor),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AttributesStore+SetAttributeValueFunc")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_AttributesStore_SetAttributeValueFunc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "HoudiniEngineUnity+HEU_Curve")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_Curve {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _geoID: i32,
    pub _curveNodeData: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::CurveNodeData,
    >,
    pub _vertices: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    pub _isEditable: bool,
    pub _parameters: *mut crate::HoudiniEngineUnity::HEU_Parameters,
    pub _bUploadParameterPreset: bool,
    pub _curveName: *mut crate::System::String,
    pub _targetGameObject: *mut crate::UnityEngine::GameObject,
    pub _isGeoCurve: bool,
    pub _editState: crate::HoudiniEngineUnity::HEU_Curve_CurveEditState,
    pub _parentAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Curve")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_Curve =>
    "HoudiniEngineUnity"."HEU_Curve"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_Curve")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_Curve {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Curve")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_Curve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Curve")]
impl crate::HoudiniEngineUnity::HEU_Curve {
    #[cfg(feature = "HoudiniEngineUnity+HEU_Curve+CurveDrawCollision")]
    pub type CurveDrawCollision = crate::HoudiniEngineUnity::HEU_Curve_CurveDrawCollision;
    #[cfg(feature = "HoudiniEngineUnity+HEU_Curve+CurveEditState")]
    pub type CurveEditState = crate::HoudiniEngineUnity::HEU_Curve_CurveEditState;
    #[cfg(feature = "HoudiniEngineUnity+HEU_Curve+Interaction")]
    pub type Interaction = crate::HoudiniEngineUnity::HEU_Curve_Interaction;
    #[cfg(feature = "HoudiniEngineUnity+HEU_Curve+__c__DisplayClass38_0")]
    pub type __c__DisplayClass38_0 = crate::HoudiniEngineUnity::HEU_Curve___c__DisplayClass38_0;
    #[cfg(feature = "HoudiniEngineUnity+HEU_Curve+__c__DisplayClass40_0")]
    pub type __c__DisplayClass40_0 = crate::HoudiniEngineUnity::HEU_Curve___c__DisplayClass40_0;
    #[cfg(feature = "HoudiniEngineUnity+HEU_Curve+__c__DisplayClass48_0")]
    pub type __c__DisplayClass48_0 = crate::HoudiniEngineUnity::HEU_Curve___c__DisplayClass48_0;
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
    pub fn DuplicateCurveNodeData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::CurveNodeData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::CurveNodeData,
        > = __cordl_object.invoke("DuplicateCurveNodeData", ())?;
        Ok(__cordl_ret)
    }
    pub fn GenerateMesh(
        &mut self,
        inGameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateMesh", (inGameObject))?;
        Ok(__cordl_ret)
    }
    pub fn GetAllPointTransforms(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::CurveNodeData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::CurveNodeData,
        > = __cordl_object.invoke("GetAllPointTransforms", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAllPoints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Vector3,
        > = __cordl_object.invoke("GetAllPoints", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCurvePoint(
        &mut self,
        pointIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetCurvePoint", (pointIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetInvertedTransformedDirection(
        &mut self,
        inPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetInvertedTransformedDirection", (inPosition))?;
        Ok(__cordl_ret)
    }
    pub fn GetInvertedTransformedPosition(
        &mut self,
        inPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetInvertedTransformedPosition", (inPosition))?;
        Ok(__cordl_ret)
    }
    pub fn GetNumPoints(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetNumPoints", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTransformedPoint(
        &mut self,
        pointIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetTransformedPoint", (pointIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetTransformedPosition(
        &mut self,
        inPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetTransformedPosition", (inPosition))?;
        Ok(__cordl_ret)
    }
    pub fn GetVertices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Vector3,
        > = __cordl_object.invoke("GetVertices", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsEditable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEditable", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsEquivalentTo(
        &mut self,
        other: *mut crate::HoudiniEngineUnity::HEU_Curve,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret)
    }
    pub fn IsGeoCurve(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsGeoCurve", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ProjectToColliders(
        &mut self,
        parentAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        rayDirection: crate::UnityEngine::Vector3,
        rayDistance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProjectToColliders", (parentAsset, rayDirection, rayDistance))?;
        Ok(__cordl_ret)
    }
    pub fn ResetCurveParameters(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        parentAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetCurveParameters", (session, parentAsset))?;
        Ok(__cordl_ret)
    }
    pub fn SetCurveGeometryVisibility(
        &mut self,
        bVisible: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCurveGeometryVisibility", (bVisible))?;
        Ok(__cordl_ret)
    }
    pub fn SetCurveName(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCurveName", (name))?;
        Ok(__cordl_ret)
    }
    pub fn SetCurveNodeData(
        &mut self,
        curveNodeData: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::CurveNodeData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCurveNodeData", (curveNodeData))?;
        Ok(__cordl_ret)
    }
    pub fn SetCurveParameterPreset(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        parentAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        parameterPreset: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCurveParameterPreset", (session, parentAsset, parameterPreset))?;
        Ok(__cordl_ret)
    }
    pub fn SetCurvePoint(
        &mut self,
        pointIndex: i32,
        newPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCurvePoint", (pointIndex, newPosition))?;
        Ok(__cordl_ret)
    }
    pub fn SetEditState(
        &mut self,
        editState: crate::HoudiniEngineUnity::HEU_Curve_CurveEditState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetEditState", (editState))?;
        Ok(__cordl_ret)
    }
    pub fn SetUploadParameterPreset(
        &mut self,
        bValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUploadParameterPreset", (bValue))?;
        Ok(__cordl_ret)
    }
    pub fn SyncFromParameters(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        parentAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncFromParameters", (session, parentAsset))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateCurve(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        partID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCurve", (session, partID))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateCurveInputForCustomAttributes(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        parentAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UpdateCurveInputForCustomAttributes", (session, parentAsset))?;
        Ok(__cordl_ret)
    }
    pub fn UpdatePoints(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        partID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePoints", (session, partID))?;
        Ok(__cordl_ret)
    }
    pub fn UploadParameterPreset(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        geoID: i32,
        parentAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UploadParameterPreset", (session, geoID, parentAsset))?;
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
    pub fn get_CurveName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_CurveName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CurveNodeData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::CurveNodeData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::CurveNodeData,
        > = __cordl_object.invoke("get_CurveNodeData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EditState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::HEU_Curve_CurveEditState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HEU_Curve_CurveEditState = __cordl_object
            .invoke("get_EditState", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GeoID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_GeoID", ())?;
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
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Curve")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_Curve {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Curve+CurveDrawCollision")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_Curve_CurveDrawCollision {
    COLLIDERS = 0i32,
    LAYERMASK = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Curve+CurveDrawCollision")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_Curve_CurveDrawCollision
    => "HoudiniEngineUnity"."HEU_Curve/CurveDrawCollision"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_Curve+CurveEditState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_Curve_CurveEditState {
    EDITING = 2i32,
    GENERATED = 1i32,
    INVALID = 0i32,
    REQUIRES_GENERATION = 3i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Curve+CurveEditState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_Curve_CurveEditState =>
    "HoudiniEngineUnity"."HEU_Curve/CurveEditState"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_Curve+Interaction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_Curve_Interaction {
    ADD = 1i32,
    EDIT = 2i32,
    VIEW = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Curve+Interaction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_Curve_Interaction =>
    "HoudiniEngineUnity"."HEU_Curve/Interaction"
);

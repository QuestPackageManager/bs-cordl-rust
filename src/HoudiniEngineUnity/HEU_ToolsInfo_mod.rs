#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ToolsInfo {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _paintBrushSize: f32,
    pub _paintBrushOpacity: f32,
    pub _paintIntValue: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _paintFloatValue: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _paintStringValue: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _lastAttributesGeoID: i32,
    pub _lastAttributesPartID: i32,
    pub _lastAttributeNodeName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _lastAttributeName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _brushHandleColor: crate::UnityEngine::Color,
    pub _affectedAreaPaintColor: crate::UnityEngine::Color,
    pub _liveUpdate: bool,
    pub _isPainting: bool,
    pub _editPointBoxSize: f32,
    pub _editPointBoxUnselectedColor: crate::UnityEngine::Color,
    pub _editPointBoxSelectedColor: crate::UnityEngine::Color,
    pub _recacheRequired: bool,
    pub _paintMergeMode: crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMergeMode,
    pub _showOnlyEditGeometry: bool,
    pub _alwaysCookUpstream: bool,
    pub _paintMeshVisiblity: crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMeshVisibility,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_ToolsInfo =>
    "HoudiniEngineUnity"."HEU_ToolsInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_ToolsInfo {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_ToolsInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo")]
impl crate::HoudiniEngineUnity::HEU_ToolsInfo {
    #[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMergeMode")]
    pub type PaintMergeMode = crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMergeMode;
    #[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMeshVisibility")]
    pub type PaintMeshVisibility = crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMeshVisibility;
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ToolsInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_ToolsInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMergeMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_ToolsInfo_PaintMergeMode {
    ADD = 1i32,
    MULTIPLY = 3i32,
    REPLACE = 0i32,
    SUBTRACT = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMergeMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_ToolsInfo_PaintMergeMode
    => "HoudiniEngineUnity"."HEU_ToolsInfo/PaintMergeMode"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMeshVisibility")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_ToolsInfo_PaintMeshVisibility {
    AUTO = 0i32,
    HIDE = 2i32,
    SHOW = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMeshVisibility")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_ToolsInfo_PaintMeshVisibility => "HoudiniEngineUnity"
    ."HEU_ToolsInfo/PaintMeshVisibility"
);

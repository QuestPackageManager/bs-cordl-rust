#[cfg(feature = "HoudiniEngineUnity+HEU_PluginSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_PluginSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginSettings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_PluginSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_PluginSettings";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginSettings")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_PluginSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginSettings")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_PluginSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginSettings")]
impl crate::HoudiniEngineUnity::HEU_PluginSettings {
    pub fn get_AssetCachePath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_AssetCachePath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CollisionGroupName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CollisionGroupName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CookOptionSplitGeosByGroup() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CookOptionSplitGeosByGroup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CookTemplatedGeos() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CookTemplatedGeos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CookingEnabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CookingEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CookingTriggersDownstreamCooks() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CookingTriggersDownstreamCooks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Curves_ShowInSceneView() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Curves_ShowInSceneView", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultCurveShader() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultCurveShader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultStandardShader() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultStandardShader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultTerrainMaterial() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultTerrainMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultTransparentShader() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultTransparentShader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultVertexColorShader() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultVertexColorShader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EditorOnly_Tag() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_EditorOnly_Tag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HDAData_Name() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_HDAData_Name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HEngineShelfSelectedIndex() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_HEngineShelfSelectedIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HEngineToolsShelves() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_HEngineToolsShelves", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HoudiniDebugLaunchPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_HoudiniDebugLaunchPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HoudiniEngineEnvFilePath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_HoudiniEngineEnvFilePath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HoudiniInstallPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_HoudiniInstallPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ImageGamma() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ImageGamma", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InputSelectionFilterLocation() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InputSelectionFilterLocation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InputSelectionFilterName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InputSelectionFilterName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InputSelectionFilterRoots() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InputSelectionFilterRoots", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InputSelectionFilterState() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InputSelectionFilterState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InstanceAttr() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InstanceAttr", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LastExportPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_LastExportPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LastHoudiniVersion() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_LastHoudiniVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LastLoadHDAPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_LastLoadHDAPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LastLoadHIPPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_LastLoadHIPPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LineColor() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_LineColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxVerticesPerPrimitive() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_MaxVerticesPerPrimitive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NormalGenerationThresholdAngle() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_NormalGenerationThresholdAngle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PushUnityTransformToHoudini() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_PushUnityTransformToHoudini", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RenderedCollisionGroupName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_RenderedCollisionGroupName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RenderedConvexCollisionGroupName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_RenderedConvexCollisionGroupName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SessionSyncAutoCook() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_SessionSyncAutoCook", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Session_AutoClose() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Session_AutoClose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Session_Localhost() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Session_Localhost", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Session_Mode() -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::SessionMode,
    > {
        let __cordl_ret: crate::HoudiniEngineUnity::SessionMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Session_Mode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Session_PipeName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Session_PipeName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Session_Port() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Session_Port", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Session_Timeout() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Session_Timeout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SetCurrentThreadToInvariantCulture() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_SetCurrentThreadToInvariantCulture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ShortenFolderPaths() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ShortenFolderPaths", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SupportHoudiniBoxType() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_SupportHoudiniBoxType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SupportHoudiniSphereType() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_SupportHoudiniSphereType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TerrainSplatTextureDefault() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_TerrainSplatTextureDefault", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TransformChangeTriggersCooks() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_TransformChangeTriggersCooks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnityInputMeshAttr() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UnityInputMeshAttr", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnityInstanceAttr() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UnityInstanceAttr", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnityLayerAttributeName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UnityLayerAttributeName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnityMaterialAttribName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UnityMaterialAttribName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnityScriptAttributeName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UnityScriptAttributeName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnityStaticAttributeName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UnityStaticAttributeName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnitySubMaterialAttribName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UnitySubMaterialAttribName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnitySubMaterialIndexAttribName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UnitySubMaterialIndexAttribName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnityTagAttributeName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UnityTagAttributeName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UseFullPathNamesForOutput() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UseFullPathNamesForOutput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UseHDRColor() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UseHDRColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UseLegacyShaders() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UseLegacyShaders", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UseSpecularShader() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UseSpecularShader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WriteCookLogs() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_WriteCookLogs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AssetCachePath(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_AssetCachePath", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CollisionGroupName(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_CollisionGroupName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CookOptionSplitGeosByGroup(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_CookOptionSplitGeosByGroup", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CookTemplatedGeos(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_CookTemplatedGeos", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CookingEnabled(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_CookingEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CookingTriggersDownstreamCooks(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_CookingTriggersDownstreamCooks", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Curves_ShowInSceneView(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_Curves_ShowInSceneView", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DefaultCurveShader(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_DefaultCurveShader", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DefaultStandardShader(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_DefaultStandardShader", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DefaultTerrainMaterial(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_DefaultTerrainMaterial", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DefaultTransparentShader(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_DefaultTransparentShader", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DefaultVertexColorShader(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_DefaultVertexColorShader", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_EditorOnly_Tag(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_EditorOnly_Tag", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_HDAData_Name(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_HDAData_Name", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_HEngineShelfSelectedIndex(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_HEngineShelfSelectedIndex", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_HEngineToolsShelves(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_HEngineToolsShelves", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_HoudiniDebugLaunchPath(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_HoudiniDebugLaunchPath", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_HoudiniEngineEnvFilePath(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_HoudiniEngineEnvFilePath", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_HoudiniInstallPath(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_HoudiniInstallPath", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ImageGamma(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ImageGamma", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_InputSelectionFilterLocation(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_InputSelectionFilterLocation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_InputSelectionFilterName(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_InputSelectionFilterName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_InputSelectionFilterRoots(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_InputSelectionFilterRoots", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_InputSelectionFilterState(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_InputSelectionFilterState", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_InstanceAttr(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_InstanceAttr", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LastExportPath(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_LastExportPath", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LastHoudiniVersion(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_LastHoudiniVersion", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LastLoadHDAPath(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_LastLoadHDAPath", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LastLoadHIPPath(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_LastLoadHIPPath", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LineColor(
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_LineColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MaxVerticesPerPrimitive(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_MaxVerticesPerPrimitive", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_NormalGenerationThresholdAngle(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_NormalGenerationThresholdAngle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PushUnityTransformToHoudini(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_PushUnityTransformToHoudini", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RenderedCollisionGroupName(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_RenderedCollisionGroupName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RenderedConvexCollisionGroupName(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_RenderedConvexCollisionGroupName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SessionSyncAutoCook(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_SessionSyncAutoCook", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Session_AutoClose(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_Session_AutoClose", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Session_Localhost(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_Session_Localhost", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Session_Mode(
        value: crate::HoudiniEngineUnity::SessionMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_Session_Mode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Session_PipeName(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_Session_PipeName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Session_Port(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_Session_Port", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Session_Timeout(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_Session_Timeout", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SetCurrentThreadToInvariantCulture(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_SetCurrentThreadToInvariantCulture", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ShortenFolderPaths(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ShortenFolderPaths", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SupportHoudiniBoxType(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_SupportHoudiniBoxType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SupportHoudiniSphereType(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_SupportHoudiniSphereType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_TerrainSplatTextureDefault(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_TerrainSplatTextureDefault", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_TransformChangeTriggersCooks(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_TransformChangeTriggersCooks", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UnityInputMeshAttr(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_UnityInputMeshAttr", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UnityInstanceAttr(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_UnityInstanceAttr", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UnityLayerAttributeName(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_UnityLayerAttributeName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UnityMaterialAttribName(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_UnityMaterialAttribName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UnityScriptAttributeName(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_UnityScriptAttributeName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UnityStaticAttributeName(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_UnityStaticAttributeName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UnitySubMaterialAttribName(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_UnitySubMaterialAttribName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UnitySubMaterialIndexAttribName(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_UnitySubMaterialIndexAttribName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UnityTagAttributeName(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_UnityTagAttributeName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UseFullPathNamesForOutput(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_UseFullPathNamesForOutput", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UseHDRColor(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_UseHDRColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UseLegacyShaders(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_UseLegacyShaders", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UseSpecularShader(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_UseSpecularShader", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_WriteCookLogs(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_WriteCookLogs", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_PluginSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

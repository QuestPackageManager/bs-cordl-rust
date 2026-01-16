#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+PreferenceKeys")]
#[repr(C)]
#[derive(Debug)]
pub struct PreferenceKeys {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+PreferenceKeys")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ProBuilder::PreferenceKeys {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "PreferenceKeys";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+ProBuilder+PreferenceKeys")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::PreferenceKeys {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+PreferenceKeys")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::PreferenceKeys {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+PreferenceKeys")]
impl crate::UnityEngine::ProBuilder::PreferenceKeys {
    pub const CMD_ALT: char = '\u{2387}';
    pub const CMD_DELETE: char = '\u{232b}';
    pub const CMD_OPTION: char = '\u{2325}';
    pub const CMD_SHIFT: char = '\u{21e7}';
    pub const CMD_SUPER: char = '\u{2318}';
    pub const DEGREE_SYMBOL: char = '\u{b0}';
    pub const defaultUnwrapParameters: &'static str = "pbDefaultUnwrapParameters";
    pub const k_MaxPointDistanceFromControl: f32 = 20f32;
    pub const menuActions: i32 = 300i32;
    pub const menuEditor: i32 = 100i32;
    pub const menuExport: i32 = 800i32;
    pub const menuGeometry: i32 = 200i32;
    pub const menuMaterialColors: i32 = 400i32;
    pub const menuMisc: i32 = 600i32;
    pub const menuRepair: i32 = 600i32;
    pub const menuSelection: i32 = 200i32;
    pub const menuVertexColors: i32 = 400i32;
    pub const pbBevelAmount: &'static str = "pbBevelAmount";
    pub const pbCheckForProBuilderUpdates: &'static str = "pbCheckForProBuilderUpdates";
    pub const pbCloseShapeWindow: &'static str = "pbCloseShapeWindow";
    pub const pbCollapseVertexToFirst: &'static str = "pbCollapseVertexToFirst";
    pub const pbCurrentMaterialPalette: &'static str = "pbCurrentMaterialPalette";
    pub const pbDefaultCollider: &'static str = "pbDefaultCollider";
    pub const pbDefaultEditLevel: &'static str = "pbDefaultEditLevel";
    pub const pbDefaultEntity: &'static str = "pbDefaultEntity";
    pub const pbDefaultMaterial: &'static str = "pbDefaultMaterial";
    pub const pbDefaultOpenInDockableWindow: &'static str = "pbDefaultOpenInDockableWindow";
    pub const pbDefaultSelectionMode: &'static str = "pbDefaultSelectionMode";
    pub const pbDefaultShortcuts: &'static str = "pbDefaultShortcuts";
    pub const pbDefaultStaticFlags: &'static str = "pbDefaultStaticFlags";
    pub const pbDetachToNewObject: &'static str = "pbDetachToNewObject";
    pub const pbDisableAutoUV2Generation: &'static str = "pbDisableAutoUV2Generation";
    pub const pbDragCheckLimit: &'static str = "pbDragCheckLimit";
    pub const pbDragSelectMode: &'static str = "pbDragSelectMode";
    pub const pbDragSelectWholeElement: &'static str = "pbDragSelectWholeElement";
    pub const pbDrawAxisLines: &'static str = "pbDrawAxisLines";
    pub const pbEdgeSubdivisions: &'static str = "pbEdgeSubdivisions";
    pub const pbEditorPrefVersion: &'static str = "pbEditorPrefVersion";
    pub const pbEditorShortcutsVersion: &'static str = "pbEditorShortcutsVersion";
    pub const pbElementSelectIsHamFisted: &'static str = "pbElementSelectIsHamFisted";
    pub const pbEnableBackfaceSelection: &'static str = "pbEnableBackfaceSelection";
    pub const pbEnableExperimental: &'static str = "pbEnableExperimental";
    pub const pbExtrudeAsGroup: &'static str = "pbExtrudeAsGroup";
    pub const pbExtrudeDistance: &'static str = "pbExtrudeDistance";
    pub const pbExtrudeMethod: &'static str = "pbExtrudeMethod";
    pub const pbFillHoleSelectsEntirePath: &'static str = "pbFillHoleSelectsEntirePath";
    pub const pbForceConvex: &'static str = "pbForceConvex";
    pub const pbForceGridPivot: &'static str = "pbForceGridPivot";
    pub const pbForceVertexPivot: &'static str = "pbForceVertexPivot";
    pub const pbGrowSelectionAngle: &'static str = "pbGrowSelectionAngle";
    pub const pbGrowSelectionAngleIterative: &'static str = "pbGrowSelectionAngleIterative";
    pub const pbGrowSelectionUsingAngle: &'static str = "pbGrowSelectionUsingAngle";
    pub const pbHandleAlignment: &'static str = "pbHandleAlignment";
    pub const pbIconGUI: &'static str = "pbIconGUI";
    pub const pbLineHandleSize: &'static str = "pbLineHandleSize";
    pub const pbManageLightmappingStaticFlag: &'static str = "pbManageLightmappingStaticFlag";
    pub const pbManifoldEdgeExtrusion: &'static str = "pbManifoldEdgeExtrusion";
    pub const pbMaterialEditorFloating: &'static str = "pbMaterialEditorFloating";
    pub const pbMeshesAreAssets: &'static str = "pbMeshesAreAssets";
    pub const pbNormalizeUVsOnPlanarProjection: &'static str = "pbNormalizeUVsOnPlanarProjection";
    pub const pbPBOSelectionOnly: &'static str = "pbPBOSelectionOnly";
    pub const pbPerimeterEdgeBridgeOnly: &'static str = "pbPerimeterEdgeBridgeOnly";
    pub const pbPreselectionColor: &'static str = "pbPreselectionColor";
    pub const pbPreserveFaces: &'static str = "pbPreserveFaces";
    pub const pbRectSelectMode: &'static str = "pbRectSelectMode";
    pub const pbSelectedEdgeColor: &'static str = "pbSelectedEdgeColor";
    pub const pbSelectedFaceColor: &'static str = "pbDefaultFaceColor";
    pub const pbSelectedFaceDither: &'static str = "pbSelectedFaceDither";
    pub const pbSelectedVertexColor: &'static str = "pbDefaultSelectedVertexColor";
    pub const pbShadowCastingMode: &'static str = "pbShadowCastingMode";
    pub const pbShapeWindowFloating: &'static str = "pbShapeWindowFloating";
    pub const pbShiftOnlyTooltips: &'static str = "pbShiftOnlyTooltips";
    pub const pbShowCollider: &'static str = "pbShowCollider";
    pub const pbShowDetail: &'static str = "pbShowDetail";
    pub const pbShowEditorNotifications: &'static str = "pbShowEditorNotifications";
    pub const pbShowMissingLightmapUvWarning: &'static str =
        "pb_Lightmapping::showMissingLightmapUvWarning";
    pub const pbShowMover: &'static str = "pbShowMover";
    pub const pbShowNoDraw: &'static str = "pbShowNoDraw";
    pub const pbShowOccluder: &'static str = "pbShowOccluder";
    pub const pbShowPreselectionHighlight: &'static str = "pbShowPreselectionHighlight";
    pub const pbShowSceneInfo: &'static str = "pbShowSceneInfo";
    pub const pbShowSceneToolbar: &'static str = "pbShowSceneToolbar";
    pub const pbShowTrigger: &'static str = "pbShowTrigger";
    pub const pbStripProBuilderOnBuild: &'static str = "pbStripProBuilderOnBuild";
    pub const pbToolbarLocation: &'static str = "pbToolbarLocation";
    pub const pbUVEditorFloating: &'static str = "pbUVEditorFloating";
    pub const pbUVGridSnapValue: &'static str = "pbUVGridSnapValue";
    pub const pbUVMaterialPreview: &'static str = "pbUVMaterialPreview";
    pub const pbUVWeldDistance: &'static str = "pbUVWeldDistance";
    pub const pbUniqueModeShortcuts: &'static str = "pbUniqueModeShortcuts";
    pub const pbUnselectedEdgeColor: &'static str = "pbUnselectedEdgeColor";
    pub const pbUnselectedVertexColor: &'static str = "pbDefaultVertexColor";
    pub const pbUseUnityColors: &'static str = "pbUseUnityColors";
    pub const pbVertexColorPrefs: &'static str = "pbVertexColorPrefs";
    pub const pbVertexColorTool: &'static str = "pbVertexColorTool";
    pub const pbVertexHandleSize: &'static str = "pbVertexHandleSize";
    pub const pbVertexPaletteDockable: &'static str = "pbVertexPaletteDockable";
    pub const pbWeldDistance: &'static str = "pbWeldDistance";
    pub const pbWireframeColor: &'static str = "pbDefaultEdgeColor";
    pub const pbWireframeSize: &'static str = "pbWireframeSize";
    pub const pluginTitle: &'static str = "ProBuilder";
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+PreferenceKeys")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::PreferenceKeys {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

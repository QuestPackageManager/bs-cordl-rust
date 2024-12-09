#[cfg(feature = "OVRTelemetryConstants+OVRManager+AnnotationTypes")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRManager_OVRTelemetryConstants_AnnotationTypes {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager+AnnotationTypes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRManager_OVRTelemetryConstants_AnnotationTypes => ""
    ."OVRTelemetryConstants/OVRManager/AnnotationTypes"
);
#[cfg(feature = "OVRTelemetryConstants+OVRManager+AnnotationTypes")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_AnnotationTypes {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager+AnnotationTypes")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_AnnotationTypes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager+AnnotationTypes")]
impl crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_AnnotationTypes {
    pub const Origin: &'static str = "Origin";
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager+AnnotationTypes")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_AnnotationTypes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager+ConsentOrigins")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_OVRTelemetryConstants_ConsentOrigins {
    Legacy = 2i32,
    Popup = 0i32,
    Settings = 1i32,
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager+ConsentOrigins")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRManager_OVRTelemetryConstants_ConsentOrigins => ""
    ."OVRTelemetryConstants/OVRManager/ConsentOrigins"
);
#[cfg(feature = "OVRTelemetryConstants+OVRManager+MarkerId")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRManager_OVRTelemetryConstants_MarkerId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager+MarkerId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRManager_OVRTelemetryConstants_MarkerId => ""
    ."OVRTelemetryConstants/OVRManager/MarkerId"
);
#[cfg(feature = "OVRTelemetryConstants+OVRManager+MarkerId")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_MarkerId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager+MarkerId")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_MarkerId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager+MarkerId")]
impl crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_MarkerId {
    pub const Consent: i32 = 163056770i32;
    pub const Init: i32 = 163069401i32;
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager+MarkerId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_MarkerId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRTelemetryConstants")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetryConstants {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRTelemetryConstants")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTelemetryConstants => ""
    ."OVRTelemetryConstants"
);
#[cfg(feature = "OVRTelemetryConstants")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetryConstants {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetryConstants {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants")]
impl crate::GlobalNamespace::OVRTelemetryConstants {
    #[cfg(feature = "OVRTelemetryConstants+Editor")]
    pub type Editor = crate::GlobalNamespace::OVRTelemetryConstants_Editor;
    #[cfg(feature = "OVRTelemetryConstants+OVRManager")]
    pub type OVRManager = crate::GlobalNamespace::OVRTelemetryConstants_OVRManager;
    #[cfg(feature = "OVRTelemetryConstants+SBB")]
    pub type SBB = crate::GlobalNamespace::OVRTelemetryConstants_SBB;
    #[cfg(feature = "OVRTelemetryConstants+Scene")]
    pub type Scene = crate::GlobalNamespace::OVRTelemetryConstants_Scene;
}
#[cfg(feature = "OVRTelemetryConstants")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRTelemetryConstants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRTelemetryConstants+Editor")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetryConstants_Editor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRTelemetryConstants+Editor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTelemetryConstants_Editor =>
    ""."OVRTelemetryConstants/Editor"
);
#[cfg(feature = "OVRTelemetryConstants+Editor")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetryConstants_Editor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Editor")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetryConstants_Editor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Editor")]
impl crate::GlobalNamespace::OVRTelemetryConstants_Editor {
    pub const Start: i32 = 163067235i32;
}
#[cfg(feature = "OVRTelemetryConstants+Editor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRTelemetryConstants_Editor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetryConstants_OVRManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRTelemetryConstants_OVRManager => ""
    ."OVRTelemetryConstants/OVRManager"
);
#[cfg(feature = "OVRTelemetryConstants+OVRManager")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetryConstants_OVRManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetryConstants_OVRManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager")]
impl crate::GlobalNamespace::OVRTelemetryConstants_OVRManager {
    #[cfg(feature = "OVRTelemetryConstants+OVRManager+AnnotationTypes")]
    pub type AnnotationTypes = crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_AnnotationTypes;
    #[cfg(feature = "OVRTelemetryConstants+OVRManager+ConsentOrigins")]
    pub type ConsentOrigins = crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_ConsentOrigins;
    #[cfg(feature = "OVRTelemetryConstants+OVRManager+MarkerId")]
    pub type MarkerId = crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_MarkerId;
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRTelemetryConstants_OVRManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRTelemetryConstants+SBB")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetryConstants_SBB {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRTelemetryConstants+SBB")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTelemetryConstants_SBB => ""
    ."OVRTelemetryConstants/SBB"
);
#[cfg(feature = "OVRTelemetryConstants+SBB")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetryConstants_SBB {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+SBB")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetryConstants_SBB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+SBB")]
impl crate::GlobalNamespace::OVRTelemetryConstants_SBB {
    #[cfg(feature = "OVRTelemetryConstants+SBB+AnnotationType")]
    pub type AnnotationType = crate::GlobalNamespace::SBB_OVRTelemetryConstants_AnnotationType;
    #[cfg(feature = "OVRTelemetryConstants+SBB+MarkerId")]
    pub type MarkerId = crate::GlobalNamespace::SBB_OVRTelemetryConstants_MarkerId;
}
#[cfg(feature = "OVRTelemetryConstants+SBB")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRTelemetryConstants_SBB {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRTelemetryConstants+Scene")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetryConstants_Scene {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRTelemetryConstants+Scene")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTelemetryConstants_Scene =>
    ""."OVRTelemetryConstants/Scene"
);
#[cfg(feature = "OVRTelemetryConstants+Scene")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetryConstants_Scene {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Scene")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetryConstants_Scene {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Scene")]
impl crate::GlobalNamespace::OVRTelemetryConstants_Scene {
    #[cfg(feature = "OVRTelemetryConstants+Scene+MarkerId")]
    pub type MarkerId = crate::GlobalNamespace::Scene_OVRTelemetryConstants_MarkerId;
}
#[cfg(feature = "OVRTelemetryConstants+Scene")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRTelemetryConstants_Scene {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRTelemetryConstants+SBB+AnnotationType")]
#[repr(C)]
#[derive(Debug)]
pub struct SBB_OVRTelemetryConstants_AnnotationType {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRTelemetryConstants+SBB+AnnotationType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SBB_OVRTelemetryConstants_AnnotationType => ""
    ."OVRTelemetryConstants/SBB/AnnotationType"
);
#[cfg(feature = "OVRTelemetryConstants+SBB+AnnotationType")]
impl std::ops::Deref
for crate::GlobalNamespace::SBB_OVRTelemetryConstants_AnnotationType {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+SBB+AnnotationType")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SBB_OVRTelemetryConstants_AnnotationType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+SBB+AnnotationType")]
impl crate::GlobalNamespace::SBB_OVRTelemetryConstants_AnnotationType {
    pub const BlockId: &'static str = "BlockId";
}
#[cfg(feature = "OVRTelemetryConstants+SBB+AnnotationType")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SBB_OVRTelemetryConstants_AnnotationType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRTelemetryConstants+SBB+MarkerId")]
#[repr(C)]
#[derive(Debug)]
pub struct SBB_OVRTelemetryConstants_MarkerId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRTelemetryConstants+SBB+MarkerId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SBB_OVRTelemetryConstants_MarkerId => ""
    ."OVRTelemetryConstants/SBB/MarkerId"
);
#[cfg(feature = "OVRTelemetryConstants+SBB+MarkerId")]
impl std::ops::Deref for crate::GlobalNamespace::SBB_OVRTelemetryConstants_MarkerId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+SBB+MarkerId")]
impl std::ops::DerefMut for crate::GlobalNamespace::SBB_OVRTelemetryConstants_MarkerId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+SBB+MarkerId")]
impl crate::GlobalNamespace::SBB_OVRTelemetryConstants_MarkerId {
    pub const AddBlock: i32 = 163060420i32;
    pub const InstallSDK: i32 = 163067801i32;
    pub const OpenWindow: i32 = 163062905i32;
    pub const RemoveSDK: i32 = 163067560i32;
    pub const RunBlock: i32 = 163063912i32;
}
#[cfg(feature = "OVRTelemetryConstants+SBB+MarkerId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SBB_OVRTelemetryConstants_MarkerId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRTelemetryConstants+Scene+MarkerId")]
#[repr(C)]
#[derive(Debug)]
pub struct Scene_OVRTelemetryConstants_MarkerId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRTelemetryConstants+Scene+MarkerId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::Scene_OVRTelemetryConstants_MarkerId => ""
    ."OVRTelemetryConstants/Scene/MarkerId"
);
#[cfg(feature = "OVRTelemetryConstants+Scene+MarkerId")]
impl std::ops::Deref for crate::GlobalNamespace::Scene_OVRTelemetryConstants_MarkerId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Scene+MarkerId")]
impl std::ops::DerefMut
for crate::GlobalNamespace::Scene_OVRTelemetryConstants_MarkerId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Scene+MarkerId")]
impl crate::GlobalNamespace::Scene_OVRTelemetryConstants_MarkerId {
    pub const SpatialAnchorCreate: i32 = 163068641i32;
    pub const SpatialAnchorErase: i32 = 163059334i32;
    pub const SpatialAnchorQuery: i32 = 163057870i32;
    pub const SpatialAnchorSave: i32 = 163056007i32;
    pub const SpatialAnchorSetComponentStatus: i32 = 163055742i32;
}
#[cfg(feature = "OVRTelemetryConstants+Scene+MarkerId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::Scene_OVRTelemetryConstants_MarkerId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

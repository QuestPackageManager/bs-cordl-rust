#[cfg(feature = "OVRPermissionsRequester")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPermissionsRequester {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPermissionsRequester")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRPermissionsRequester => ""."OVRPermissionsRequester"
);
#[cfg(feature = "OVRPermissionsRequester")]
impl std::ops::Deref for OVRPermissionsRequester {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPermissionsRequester")]
impl std::ops::DerefMut for OVRPermissionsRequester {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPermissionsRequester")]
impl OVRPermissionsRequester {
    pub const BodyTrackingPermission: &'static str = "com.oculus.permission.BODY_TRACKING";
    pub const EyeTrackingPermission: &'static str = "com.oculus.permission.EYE_TRACKING";
    pub const FaceTrackingPermission: &'static str = "com.oculus.permission.FACE_TRACKING";
    pub const ScenePermission: &'static str = "com.oculus.permission.USE_SCENE";
    #[cfg(feature = "OVRPermissionsRequester+__c")]
    pub type __c = crate::GlobalNamespace::OVRPermissionsRequester___c;
    #[cfg(feature = "OVRPermissionsRequester+Permission")]
    pub type Permission = crate::GlobalNamespace::OVRPermissionsRequester_Permission;
}
#[cfg(feature = "OVRPermissionsRequester")]
impl quest_hook::libil2cpp::ObjectType for OVRPermissionsRequester {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPermissionsRequester+Permission")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPermissionsRequester_Permission {
    BodyTracking = 1i32,
    EyeTracking = 2i32,
    FaceTracking = 0i32,
    Scene = 3i32,
}
#[cfg(feature = "OVRPermissionsRequester+Permission")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPermissionsRequester_Permission => ""
    ."OVRPermissionsRequester/Permission"
);

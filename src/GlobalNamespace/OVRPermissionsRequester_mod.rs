#[cfg(feature = "OVRPermissionsRequester")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPermissionsRequester {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPermissionsRequester")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRPermissionsRequester {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRPermissionsRequester";
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
#[cfg(feature = "OVRPermissionsRequester")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPermissionsRequester {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPermissionsRequester")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPermissionsRequester {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPermissionsRequester")]
impl crate::GlobalNamespace::OVRPermissionsRequester {
    pub const BodyTrackingPermission: &'static str = "com.oculus.permission.BODY_TRACKING";
    pub const EyeTrackingPermission: &'static str = "com.oculus.permission.EYE_TRACKING";
    pub const FaceTrackingPermission: &'static str = "com.oculus.permission.FACE_TRACKING";
    pub const ScenePermission: &'static str = "com.oculus.permission.USE_SCENE";
    #[cfg(feature = "OVRPermissionsRequester+Permission")]
    pub type Permission = crate::GlobalNamespace::OVRPermissionsRequester_Permission;
    pub fn BuildPermissionCallbacks() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Android::PermissionCallbacks>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Android::PermissionCallbacks,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildPermissionCallbacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPermissionId(
        permission: crate::GlobalNamespace::OVRPermissionsRequester_Permission,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPermissionId", (permission))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPermissionGranted(
        permission: crate::GlobalNamespace::OVRPermissionsRequester_Permission,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPermissionGranted", (permission))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPermissionSupportedByPlatform(
        permission: crate::GlobalNamespace::OVRPermissionsRequester_Permission,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPermissionSupportedByPlatform", (permission))?;
        Ok(__cordl_ret.into())
    }
    pub fn Request(
        permissions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::GlobalNamespace::OVRPermissionsRequester_Permission,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Request", (permissions))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestPermissions(
        permissions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::GlobalNamespace::OVRPermissionsRequester_Permission,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RequestPermissions", (permissions))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldRequestPermission(
        permission: crate::GlobalNamespace::OVRPermissionsRequester_Permission,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldRequestPermission", (permission))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_PermissionGranted(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_PermissionGranted", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_PermissionGranted(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_PermissionGranted", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPermissionsRequester")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPermissionsRequester {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPermissionsRequester+Permission")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPermissionsRequester_Permission {
    #[default]
    BodyTracking = 1i32,
    EyeTracking = 2i32,
    FaceTracking = 0i32,
    Scene = 3i32,
}
#[cfg(feature = "OVRPermissionsRequester+Permission")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRPermissionsRequester_Permission {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "Permission";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVRPermissionsRequester+Permission")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRPermissionsRequester_Permission {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRPermissionsRequester+Permission")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRPermissionsRequester_Permission {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVRPermissionsRequester+Permission")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRPermissionsRequester_Permission {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVRPermissionsRequester+Permission")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRPermissionsRequester_Permission {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}

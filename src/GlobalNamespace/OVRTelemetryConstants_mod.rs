#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+AnnotationTypes")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRManager_OVRTelemetryConstants_AnnotationTypes {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+AnnotationTypes")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_AnnotationTypes {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/OVRManager/AnnotationTypes";
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
#[cfg(feature = "OVRTelemetryConstants+OVRManager+AnnotationTypes")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_AnnotationTypes {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager+AnnotationTypes")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_AnnotationTypes {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager+AnnotationTypes")]
impl crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_AnnotationTypes {
    pub const Origin: &'static str = "Origin";
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+AnnotationTypes")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_AnnotationTypes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+ConsentOrigins")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRManager_OVRTelemetryConstants_ConsentOrigins {
    #[default]
    Legacy = 2i32,
    Popup = 0i32,
    Settings = 1i32,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+ConsentOrigins")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_ConsentOrigins {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/OVRManager/ConsentOrigins";
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+ConsentOrigins")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_ConsentOrigins {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+ConsentOrigins")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_ConsentOrigins {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+ConsentOrigins")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_ConsentOrigins {
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+ConsentOrigins")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_ConsentOrigins {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+MarkerId")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRManager_OVRTelemetryConstants_MarkerId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+MarkerId")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_MarkerId {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/OVRManager/MarkerId";
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
#[cfg(feature = "OVRTelemetryConstants+OVRManager+MarkerId")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_MarkerId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager+MarkerId")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_MarkerId {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager+MarkerId")]
impl crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_MarkerId {
    pub const Consent: i32 = 163056770i32;
    pub const Init: i32 = 163069401i32;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+MarkerId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_MarkerId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetryConstants {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRTelemetryConstants {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants";
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
#[cfg(feature = "OVRTelemetryConstants")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetryConstants {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetryConstants {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRTelemetryConstants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Editor")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetryConstants_Editor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Editor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRTelemetryConstants_Editor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/Editor";
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
#[cfg(feature = "OVRTelemetryConstants+Editor")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetryConstants_Editor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Editor")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetryConstants_Editor {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Editor")]
impl crate::GlobalNamespace::OVRTelemetryConstants_Editor {
    pub const Start: i32 = 163067235i32;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Editor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRTelemetryConstants_Editor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetryConstants_OVRManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRTelemetryConstants_OVRManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/OVRManager";
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
#[cfg(feature = "OVRTelemetryConstants+OVRManager")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetryConstants_OVRManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetryConstants_OVRManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRTelemetryConstants_OVRManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+SBB")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetryConstants_SBB {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+SBB")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRTelemetryConstants_SBB {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/SBB";
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
#[cfg(feature = "OVRTelemetryConstants+SBB")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetryConstants_SBB {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+SBB")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetryConstants_SBB {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+SBB")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRTelemetryConstants_SBB {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Scene")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetryConstants_Scene {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Scene")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRTelemetryConstants_Scene {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/Scene";
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
#[cfg(feature = "OVRTelemetryConstants+Scene")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetryConstants_Scene {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Scene")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetryConstants_Scene {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Scene")]
impl crate::GlobalNamespace::OVRTelemetryConstants_Scene {
    #[cfg(feature = "OVRTelemetryConstants+Scene+MarkerId")]
    pub type MarkerId = crate::GlobalNamespace::Scene_OVRTelemetryConstants_MarkerId;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Scene")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRTelemetryConstants_Scene {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+SBB+AnnotationType")]
#[repr(C)]
#[derive(Debug)]
pub struct SBB_OVRTelemetryConstants_AnnotationType {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+SBB+AnnotationType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SBB_OVRTelemetryConstants_AnnotationType {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/SBB/AnnotationType";
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
#[cfg(feature = "OVRTelemetryConstants+SBB+AnnotationType")]
impl std::ops::Deref
for crate::GlobalNamespace::SBB_OVRTelemetryConstants_AnnotationType {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+SBB+AnnotationType")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SBB_OVRTelemetryConstants_AnnotationType {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+SBB+AnnotationType")]
impl crate::GlobalNamespace::SBB_OVRTelemetryConstants_AnnotationType {
    pub const BlockId: &'static str = "BlockId";
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+SBB+AnnotationType")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SBB_OVRTelemetryConstants_AnnotationType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+SBB+MarkerId")]
#[repr(C)]
#[derive(Debug)]
pub struct SBB_OVRTelemetryConstants_MarkerId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+SBB+MarkerId")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SBB_OVRTelemetryConstants_MarkerId {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/SBB/MarkerId";
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
#[cfg(feature = "OVRTelemetryConstants+SBB+MarkerId")]
impl std::ops::Deref for crate::GlobalNamespace::SBB_OVRTelemetryConstants_MarkerId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+SBB+MarkerId")]
impl std::ops::DerefMut for crate::GlobalNamespace::SBB_OVRTelemetryConstants_MarkerId {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+SBB+MarkerId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SBB_OVRTelemetryConstants_MarkerId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Scene+MarkerId")]
#[repr(C)]
#[derive(Debug)]
pub struct Scene_OVRTelemetryConstants_MarkerId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Scene+MarkerId")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::Scene_OVRTelemetryConstants_MarkerId {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/Scene/MarkerId";
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
#[cfg(feature = "OVRTelemetryConstants+Scene+MarkerId")]
impl std::ops::Deref for crate::GlobalNamespace::Scene_OVRTelemetryConstants_MarkerId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Scene+MarkerId")]
impl std::ops::DerefMut
for crate::GlobalNamespace::Scene_OVRTelemetryConstants_MarkerId {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Scene+MarkerId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::Scene_OVRTelemetryConstants_MarkerId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

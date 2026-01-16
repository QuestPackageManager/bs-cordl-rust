#[cfg(feature = "cordl_class_OVRTelemetryConstants+BB+AnnotationType")]
#[repr(C)]
#[derive(Debug)]
pub struct BB_OVRTelemetryConstants_AnnotationType {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+BB+AnnotationType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::BB_OVRTelemetryConstants_AnnotationType
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/BB/AnnotationType";
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
#[cfg(feature = "OVRTelemetryConstants+BB+AnnotationType")]
impl std::ops::Deref for crate::GlobalNamespace::BB_OVRTelemetryConstants_AnnotationType {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+BB+AnnotationType")]
impl std::ops::DerefMut for crate::GlobalNamespace::BB_OVRTelemetryConstants_AnnotationType {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+BB+AnnotationType")]
impl crate::GlobalNamespace::BB_OVRTelemetryConstants_AnnotationType {
    pub const ActionTrigger: &'static str = "action_trigger";
    pub const BlockId: &'static str = "BlockId";
    pub const BlockName: &'static str = "BlockName";
    pub const BlocksCount: &'static str = "BlocksCount";
    pub const Error: &'static str = "error";
    pub const InstallationRoutineData: &'static str = "InstallationRoutineData";
    pub const InstallationRoutineId: &'static str = "InstallationRoutineId";
    pub const InstanceId: &'static str = "InstanceId";
    pub const SceneSizeInB: &'static str = "SceneSizeInB";
    pub const Version: &'static str = "Version";
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+BB+AnnotationType")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::BB_OVRTelemetryConstants_AnnotationType
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+BB+MarkerId")]
#[repr(C)]
#[derive(Debug)]
pub struct BB_OVRTelemetryConstants_MarkerId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+BB+MarkerId")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::BB_OVRTelemetryConstants_MarkerId
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/BB/MarkerId";
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
#[cfg(feature = "OVRTelemetryConstants+BB+MarkerId")]
impl std::ops::Deref for crate::GlobalNamespace::BB_OVRTelemetryConstants_MarkerId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+BB+MarkerId")]
impl std::ops::DerefMut for crate::GlobalNamespace::BB_OVRTelemetryConstants_MarkerId {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+BB+MarkerId")]
impl crate::GlobalNamespace::BB_OVRTelemetryConstants_MarkerId {
    pub const AddBlock: i32 = 163060420i32;
    pub const InstallBlockData: i32 = 163065449i32;
    pub const InstallSDK: i32 = 163067801i32;
    pub const OpenSceneWithBlock: i32 = 163063649i32;
    pub const OpenWindow: i32 = 163062905i32;
    pub const RemoveSDK: i32 = 163067560i32;
    pub const RunBlock: i32 = 163063912i32;
    pub const UpdateBlock: i32 = 163064521i32;
    pub const VariantsWindowFlow: i32 = 163065580i32;
    pub const VariantsWindowOpen: i32 = 163068476i32;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+BB+MarkerId")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::BB_OVRTelemetryConstants_MarkerId
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+BB+Origins")]
#[repr(C)]
#[derive(Debug)]
pub struct BB_OVRTelemetryConstants_Origins {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+BB+Origins")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::BB_OVRTelemetryConstants_Origins
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/BB/Origins";
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
#[cfg(feature = "OVRTelemetryConstants+BB+Origins")]
impl std::ops::Deref for crate::GlobalNamespace::BB_OVRTelemetryConstants_Origins {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+BB+Origins")]
impl std::ops::DerefMut for crate::GlobalNamespace::BB_OVRTelemetryConstants_Origins {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+BB+Origins")]
impl crate::GlobalNamespace::BB_OVRTelemetryConstants_Origins {}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+BB+Origins")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::BB_OVRTelemetryConstants_Origins
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Editor+AnnotationType")]
#[repr(C)]
#[derive(Debug)]
pub struct Editor_OVRTelemetryConstants_AnnotationType {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Editor+AnnotationType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::Editor_OVRTelemetryConstants_AnnotationType
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/Editor/AnnotationType";
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
#[cfg(feature = "OVRTelemetryConstants+Editor+AnnotationType")]
impl std::ops::Deref for crate::GlobalNamespace::Editor_OVRTelemetryConstants_AnnotationType {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Editor+AnnotationType")]
impl std::ops::DerefMut for crate::GlobalNamespace::Editor_OVRTelemetryConstants_AnnotationType {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Editor+AnnotationType")]
impl crate::GlobalNamespace::Editor_OVRTelemetryConstants_AnnotationType {
    pub const AssemblyName: &'static str = "AssemblyName";
    pub const ComponentName: &'static str = "ComponentName";
    pub const Origin: &'static str = "Origin";
    pub const UsesProSkin: &'static str = "UsesProSkin";
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Editor+AnnotationType")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::Editor_OVRTelemetryConstants_AnnotationType
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Editor+AnnotationVariant")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum Editor_OVRTelemetryConstants_AnnotationVariant {
    #[default]
    Optional = 1i32,
    Required = 0i32,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Editor+AnnotationVariant")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::Editor_OVRTelemetryConstants_AnnotationVariant
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/Editor/AnnotationVariant";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Editor+AnnotationVariant")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::Editor_OVRTelemetryConstants_AnnotationVariant
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Editor+AnnotationVariant")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::Editor_OVRTelemetryConstants_AnnotationVariant
{
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Editor+AnnotationVariant")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::Editor_OVRTelemetryConstants_AnnotationVariant
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Editor+AnnotationVariant")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::Editor_OVRTelemetryConstants_AnnotationVariant
{
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Editor+MarkerId")]
#[repr(C)]
#[derive(Debug)]
pub struct Editor_OVRTelemetryConstants_MarkerId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Editor+MarkerId")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::Editor_OVRTelemetryConstants_MarkerId
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/Editor/MarkerId";
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
#[cfg(feature = "OVRTelemetryConstants+Editor+MarkerId")]
impl std::ops::Deref for crate::GlobalNamespace::Editor_OVRTelemetryConstants_MarkerId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Editor+MarkerId")]
impl std::ops::DerefMut for crate::GlobalNamespace::Editor_OVRTelemetryConstants_MarkerId {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Editor+MarkerId")]
impl crate::GlobalNamespace::Editor_OVRTelemetryConstants_MarkerId {
    pub const Build: i32 = 163066733i32;
    pub const ComponentAdd: i32 = 163060094i32;
    pub const FeaturesInScene: i32 = 163069415i32;
    pub const Start: i32 = 163067235i32;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Editor+MarkerId")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::Editor_OVRTelemetryConstants_MarkerId
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Feedback+AnnotationType")]
#[repr(C)]
#[derive(Debug)]
pub struct Feedback_OVRTelemetryConstants_AnnotationType {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Feedback+AnnotationType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::Feedback_OVRTelemetryConstants_AnnotationType
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/Feedback/AnnotationType";
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
#[cfg(feature = "OVRTelemetryConstants+Feedback+AnnotationType")]
impl std::ops::Deref for crate::GlobalNamespace::Feedback_OVRTelemetryConstants_AnnotationType {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Feedback+AnnotationType")]
impl std::ops::DerefMut for crate::GlobalNamespace::Feedback_OVRTelemetryConstants_AnnotationType {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Feedback+AnnotationType")]
impl crate::GlobalNamespace::Feedback_OVRTelemetryConstants_AnnotationType {
    pub const ToolName: &'static str = "ToolName";
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Feedback+AnnotationType")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::Feedback_OVRTelemetryConstants_AnnotationType
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Feedback+MarkerId")]
#[repr(C)]
#[derive(Debug)]
pub struct Feedback_OVRTelemetryConstants_MarkerId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Feedback+MarkerId")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::Feedback_OVRTelemetryConstants_MarkerId
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/Feedback/MarkerId";
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
#[cfg(feature = "OVRTelemetryConstants+Feedback+MarkerId")]
impl std::ops::Deref for crate::GlobalNamespace::Feedback_OVRTelemetryConstants_MarkerId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Feedback+MarkerId")]
impl std::ops::DerefMut for crate::GlobalNamespace::Feedback_OVRTelemetryConstants_MarkerId {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Feedback+MarkerId")]
impl crate::GlobalNamespace::Feedback_OVRTelemetryConstants_MarkerId {
    pub const SubmitFeedback: i32 = 163059978i32;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Feedback+MarkerId")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::Feedback_OVRTelemetryConstants_MarkerId
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+GuidedSetup+AnnotationType")]
#[repr(C)]
#[derive(Debug)]
pub struct GuidedSetup_OVRTelemetryConstants_AnnotationType {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+GuidedSetup+AnnotationType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::GuidedSetup_OVRTelemetryConstants_AnnotationType
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/GuidedSetup/AnnotationType";
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
#[cfg(feature = "OVRTelemetryConstants+GuidedSetup+AnnotationType")]
impl std::ops::Deref for crate::GlobalNamespace::GuidedSetup_OVRTelemetryConstants_AnnotationType {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+GuidedSetup+AnnotationType")]
impl std::ops::DerefMut
    for crate::GlobalNamespace::GuidedSetup_OVRTelemetryConstants_AnnotationType
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+GuidedSetup+AnnotationType")]
impl crate::GlobalNamespace::GuidedSetup_OVRTelemetryConstants_AnnotationType {
    pub const HasAppId: &'static str = "app_id_exist";
    pub const HasNewVersionAvailable: &'static str = "new_version_available";
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+GuidedSetup+AnnotationType")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::GuidedSetup_OVRTelemetryConstants_AnnotationType
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+GuidedSetup+MarkerId")]
#[repr(C)]
#[derive(Debug)]
pub struct GuidedSetup_OVRTelemetryConstants_MarkerId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+GuidedSetup+MarkerId")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::GuidedSetup_OVRTelemetryConstants_MarkerId
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/GuidedSetup/MarkerId";
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
#[cfg(feature = "OVRTelemetryConstants+GuidedSetup+MarkerId")]
impl std::ops::Deref for crate::GlobalNamespace::GuidedSetup_OVRTelemetryConstants_MarkerId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+GuidedSetup+MarkerId")]
impl std::ops::DerefMut for crate::GlobalNamespace::GuidedSetup_OVRTelemetryConstants_MarkerId {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+GuidedSetup+MarkerId")]
impl crate::GlobalNamespace::GuidedSetup_OVRTelemetryConstants_MarkerId {
    pub const SetAppIdFromGuidedSetup: i32 = 163061548i32;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+GuidedSetup+MarkerId")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::GuidedSetup_OVRTelemetryConstants_MarkerId
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+AnnotationTypes")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRManager_OVRTelemetryConstants_AnnotationTypes {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+AnnotationTypes")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_AnnotationTypes
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/OVRManager/AnnotationTypes";
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
#[cfg(feature = "OVRTelemetryConstants+OVRManager+AnnotationTypes")]
impl std::ops::Deref for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_AnnotationTypes {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager+AnnotationTypes")]
impl std::ops::DerefMut
    for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_AnnotationTypes
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager+AnnotationTypes")]
impl crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_AnnotationTypes {
    pub const BatchMode: &'static str = "BatchMode";
    pub const Origin: &'static str = "Origin";
    pub const ProcessorType: &'static str = "ProcessorType";
    pub const ProjectGuid: &'static str = "ProjectGuid";
    pub const ProjectName: &'static str = "ProjectName";
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+AnnotationTypes")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_AnnotationTypes
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+ConsentOrigins")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum OVRManager_OVRTelemetryConstants_ConsentOrigins {
    #[default]
    Legacy = 2i32,
    Popup = 0i32,
    Settings = 1i32,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+ConsentOrigins")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_ConsentOrigins
{
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+ConsentOrigins")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_ConsentOrigins
{
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
    for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_ConsentOrigins
{
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
    for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_ConsentOrigins
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager+ConsentOrigins")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_ConsentOrigins
{
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
    for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_MarkerId
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/OVRManager/MarkerId";
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
#[cfg(feature = "OVRTelemetryConstants+OVRManager+MarkerId")]
impl std::ops::Deref for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_MarkerId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+OVRManager+MarkerId")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_MarkerId {
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
    for crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_MarkerId
{
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
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRTelemetryConstants {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants";
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
    #[cfg(feature = "OVRTelemetryConstants+BB")]
    pub type BB = crate::GlobalNamespace::OVRTelemetryConstants_BB;
    #[cfg(feature = "OVRTelemetryConstants+Editor")]
    pub type Editor = crate::GlobalNamespace::OVRTelemetryConstants_Editor;
    #[cfg(feature = "OVRTelemetryConstants+Feedback")]
    pub type Feedback = crate::GlobalNamespace::OVRTelemetryConstants_Feedback;
    #[cfg(feature = "OVRTelemetryConstants+GuidedSetup")]
    pub type GuidedSetup = crate::GlobalNamespace::OVRTelemetryConstants_GuidedSetup;
    #[cfg(feature = "OVRTelemetryConstants+OVRManager")]
    pub type OVRManager = crate::GlobalNamespace::OVRTelemetryConstants_OVRManager;
    #[cfg(feature = "OVRTelemetryConstants+ProjectSettings")]
    pub type ProjectSettings = crate::GlobalNamespace::OVRTelemetryConstants_ProjectSettings;
    #[cfg(feature = "OVRTelemetryConstants+Scene")]
    pub type Scene = crate::GlobalNamespace::OVRTelemetryConstants_Scene;
    #[cfg(feature = "OVRTelemetryConstants+Utils")]
    pub type Utils = crate::GlobalNamespace::OVRTelemetryConstants_Utils;
    #[cfg(feature = "OVRTelemetryConstants+XRSim")]
    pub type XRSim = crate::GlobalNamespace::OVRTelemetryConstants_XRSim;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRTelemetryConstants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+BB")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetryConstants_BB {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+BB")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRTelemetryConstants_BB {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/BB";
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
#[cfg(feature = "OVRTelemetryConstants+BB")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetryConstants_BB {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+BB")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetryConstants_BB {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+BB")]
impl crate::GlobalNamespace::OVRTelemetryConstants_BB {
    #[cfg(feature = "OVRTelemetryConstants+BB+AnnotationType")]
    pub type AnnotationType = crate::GlobalNamespace::BB_OVRTelemetryConstants_AnnotationType;
    #[cfg(feature = "OVRTelemetryConstants+BB+MarkerId")]
    pub type MarkerId = crate::GlobalNamespace::BB_OVRTelemetryConstants_MarkerId;
    #[cfg(feature = "OVRTelemetryConstants+BB+Origins")]
    pub type Origins = crate::GlobalNamespace::BB_OVRTelemetryConstants_Origins;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+BB")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRTelemetryConstants_BB {
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
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRTelemetryConstants_Editor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/Editor";
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
    #[cfg(feature = "OVRTelemetryConstants+Editor+AnnotationType")]
    pub type AnnotationType = crate::GlobalNamespace::Editor_OVRTelemetryConstants_AnnotationType;
    #[cfg(feature = "OVRTelemetryConstants+Editor+AnnotationVariant")]
    pub type AnnotationVariant =
        crate::GlobalNamespace::Editor_OVRTelemetryConstants_AnnotationVariant;
    #[cfg(feature = "OVRTelemetryConstants+Editor+MarkerId")]
    pub type MarkerId = crate::GlobalNamespace::Editor_OVRTelemetryConstants_MarkerId;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Editor")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRTelemetryConstants_Editor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Feedback")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetryConstants_Feedback {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Feedback")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRTelemetryConstants_Feedback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/Feedback";
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
#[cfg(feature = "OVRTelemetryConstants+Feedback")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetryConstants_Feedback {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Feedback")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetryConstants_Feedback {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Feedback")]
impl crate::GlobalNamespace::OVRTelemetryConstants_Feedback {
    #[cfg(feature = "OVRTelemetryConstants+Feedback+AnnotationType")]
    pub type AnnotationType = crate::GlobalNamespace::Feedback_OVRTelemetryConstants_AnnotationType;
    #[cfg(feature = "OVRTelemetryConstants+Feedback+MarkerId")]
    pub type MarkerId = crate::GlobalNamespace::Feedback_OVRTelemetryConstants_MarkerId;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Feedback")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRTelemetryConstants_Feedback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+GuidedSetup")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetryConstants_GuidedSetup {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+GuidedSetup")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::OVRTelemetryConstants_GuidedSetup
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/GuidedSetup";
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
#[cfg(feature = "OVRTelemetryConstants+GuidedSetup")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetryConstants_GuidedSetup {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+GuidedSetup")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetryConstants_GuidedSetup {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+GuidedSetup")]
impl crate::GlobalNamespace::OVRTelemetryConstants_GuidedSetup {
    #[cfg(feature = "OVRTelemetryConstants+GuidedSetup+AnnotationType")]
    pub type AnnotationType =
        crate::GlobalNamespace::GuidedSetup_OVRTelemetryConstants_AnnotationType;
    #[cfg(feature = "OVRTelemetryConstants+GuidedSetup+MarkerId")]
    pub type MarkerId = crate::GlobalNamespace::GuidedSetup_OVRTelemetryConstants_MarkerId;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+GuidedSetup")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::OVRTelemetryConstants_GuidedSetup
{
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
    for crate::GlobalNamespace::OVRTelemetryConstants_OVRManager
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/OVRManager";
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
    pub type AnnotationTypes =
        crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_AnnotationTypes;
    #[cfg(feature = "OVRTelemetryConstants+OVRManager+ConsentOrigins")]
    pub type ConsentOrigins =
        crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_ConsentOrigins;
    #[cfg(feature = "OVRTelemetryConstants+OVRManager+MarkerId")]
    pub type MarkerId = crate::GlobalNamespace::OVRManager_OVRTelemetryConstants_MarkerId;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+OVRManager")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::OVRTelemetryConstants_OVRManager
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetryConstants_ProjectSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::OVRTelemetryConstants_ProjectSettings
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/ProjectSettings";
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
#[cfg(feature = "OVRTelemetryConstants+ProjectSettings")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetryConstants_ProjectSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+ProjectSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetryConstants_ProjectSettings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+ProjectSettings")]
impl crate::GlobalNamespace::OVRTelemetryConstants_ProjectSettings {
    #[cfg(feature = "OVRTelemetryConstants+ProjectSettings+AnnotationType")]
    pub type AnnotationType =
        crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_AnnotationType;
    #[cfg(feature = "OVRTelemetryConstants+ProjectSettings+DepthSubmissionMode")]
    pub type DepthSubmissionMode =
        crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_DepthSubmissionMode;
    #[cfg(feature = "OVRTelemetryConstants+ProjectSettings+FoveatedRenderingAPI")]
    pub type FoveatedRenderingAPI =
        crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_FoveatedRenderingAPI;
    #[cfg(feature = "OVRTelemetryConstants+ProjectSettings+FoveatedRenderingMode")]
    pub type FoveatedRenderingMode =
        crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_FoveatedRenderingMode;
    #[cfg(feature = "OVRTelemetryConstants+ProjectSettings+MarkerId")]
    pub type MarkerId = crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_MarkerId;
    #[cfg(feature = "OVRTelemetryConstants+ProjectSettings+RenderThreadingMode")]
    pub type RenderThreadingMode =
        crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_RenderThreadingMode;
    #[cfg(feature = "OVRTelemetryConstants+ProjectSettings+RenderingPath")]
    pub type RenderingPath =
        crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_RenderingPath;
    #[cfg(feature = "OVRTelemetryConstants+ProjectSettings+XrPlugin")]
    pub type XrPlugin = crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_XrPlugin;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::OVRTelemetryConstants_ProjectSettings
{
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
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRTelemetryConstants_Scene {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/Scene";
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
    #[cfg(feature = "OVRTelemetryConstants+Scene+AnnotationType")]
    pub type AnnotationType = crate::GlobalNamespace::Scene_OVRTelemetryConstants_AnnotationType;
    #[cfg(feature = "OVRTelemetryConstants+Scene+MarkerId")]
    pub type MarkerId = crate::GlobalNamespace::Scene_OVRTelemetryConstants_MarkerId;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Scene")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRTelemetryConstants_Scene {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Utils")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetryConstants_Utils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Utils")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRTelemetryConstants_Utils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/Utils";
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
#[cfg(feature = "OVRTelemetryConstants+Utils")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetryConstants_Utils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Utils")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetryConstants_Utils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Utils")]
impl crate::GlobalNamespace::OVRTelemetryConstants_Utils {
    #[cfg(feature = "OVRTelemetryConstants+Utils+AnnotationType")]
    pub type AnnotationType = crate::GlobalNamespace::Utils_OVRTelemetryConstants_AnnotationType;
    #[cfg(feature = "OVRTelemetryConstants+Utils+MarkerId")]
    pub type MarkerId = crate::GlobalNamespace::Utils_OVRTelemetryConstants_MarkerId;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Utils")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRTelemetryConstants_Utils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+XRSim")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetryConstants_XRSim {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+XRSim")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRTelemetryConstants_XRSim {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/XRSim";
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
#[cfg(feature = "OVRTelemetryConstants+XRSim")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetryConstants_XRSim {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+XRSim")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetryConstants_XRSim {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+XRSim")]
impl crate::GlobalNamespace::OVRTelemetryConstants_XRSim {
    #[cfg(feature = "OVRTelemetryConstants+XRSim+AnnotationType")]
    pub type AnnotationType = crate::GlobalNamespace::XRSim_OVRTelemetryConstants_AnnotationType;
    #[cfg(feature = "OVRTelemetryConstants+XRSim+MarkerId")]
    pub type MarkerId = crate::GlobalNamespace::XRSim_OVRTelemetryConstants_MarkerId;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+XRSim")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRTelemetryConstants_XRSim {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+AnnotationType")]
#[repr(C)]
#[derive(Debug)]
pub struct ProjectSettings_OVRTelemetryConstants_AnnotationType {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+AnnotationType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_AnnotationType
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/ProjectSettings/AnnotationType";
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
#[cfg(feature = "OVRTelemetryConstants+ProjectSettings+AnnotationType")]
impl std::ops::Deref
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_AnnotationType
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+ProjectSettings+AnnotationType")]
impl std::ops::DerefMut
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_AnnotationType
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+ProjectSettings+AnnotationType")]
impl crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_AnnotationType {
    pub const RenderThreadingMode: &'static str = "render_threading_mode";
    pub const RenderingPath: &'static str = "rendering_path";
    pub const XrPluginType: &'static str = "xr_plugin_type";
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+AnnotationType")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_AnnotationType
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+DepthSubmissionMode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ProjectSettings_OVRTelemetryConstants_DepthSubmissionMode {
    #[default]
    Depth16Bit = 2i32,
    Depth24Bit = 3i32,
    None = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+DepthSubmissionMode")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_DepthSubmissionMode
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/ProjectSettings/DepthSubmissionMode";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+DepthSubmissionMode")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_DepthSubmissionMode
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+DepthSubmissionMode")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_DepthSubmissionMode
{
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+DepthSubmissionMode")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_DepthSubmissionMode
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+DepthSubmissionMode")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_DepthSubmissionMode
{
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+FoveatedRenderingAPI")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ProjectSettings_OVRTelemetryConstants_FoveatedRenderingAPI {
    #[default]
    Legacy = 1i32,
    SRP = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+FoveatedRenderingAPI")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_FoveatedRenderingAPI
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/ProjectSettings/FoveatedRenderingAPI";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+FoveatedRenderingAPI")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_FoveatedRenderingAPI
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+FoveatedRenderingAPI")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_FoveatedRenderingAPI
{
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+FoveatedRenderingAPI")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_FoveatedRenderingAPI
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+FoveatedRenderingAPI")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_FoveatedRenderingAPI
{
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+FoveatedRenderingMode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ProjectSettings_OVRTelemetryConstants_FoveatedRenderingMode {
    #[default]
    EyeTrackedFoveatedRendering = 2i32,
    FixedFoveatedRendering = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+FoveatedRenderingMode")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_FoveatedRenderingMode
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/ProjectSettings/FoveatedRenderingMode";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+FoveatedRenderingMode")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_FoveatedRenderingMode
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+FoveatedRenderingMode")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_FoveatedRenderingMode
{
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+FoveatedRenderingMode")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_FoveatedRenderingMode
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+FoveatedRenderingMode")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_FoveatedRenderingMode
{
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+MarkerId")]
#[repr(C)]
#[derive(Debug)]
pub struct ProjectSettings_OVRTelemetryConstants_MarkerId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+MarkerId")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_MarkerId
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/ProjectSettings/MarkerId";
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
#[cfg(feature = "OVRTelemetryConstants+ProjectSettings+MarkerId")]
impl std::ops::Deref for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_MarkerId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+ProjectSettings+MarkerId")]
impl std::ops::DerefMut for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_MarkerId {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+ProjectSettings+MarkerId")]
impl crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_MarkerId {
    pub const RenderThreadingMode: i32 = 163060994i32;
    pub const RenderingPath: i32 = 163068301i32;
    pub const XrPluginType: i32 = 163069107i32;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+MarkerId")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_MarkerId
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+RenderThreadingMode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ProjectSettings_OVRTelemetryConstants_RenderThreadingMode {
    #[default]
    LegacyGraphicsJobs = 2i32,
    Multithreaded = 1i32,
    NativeGraphicsJobs = 3i32,
    Unknown = 0i32,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+RenderThreadingMode")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_RenderThreadingMode
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/ProjectSettings/RenderThreadingMode";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+RenderThreadingMode")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_RenderThreadingMode
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+RenderThreadingMode")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_RenderThreadingMode
{
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+RenderThreadingMode")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_RenderThreadingMode
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+RenderThreadingMode")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_RenderThreadingMode
{
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+RenderingPath")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ProjectSettings_OVRTelemetryConstants_RenderingPath {
    #[default]
    Deferred = 3i32,
    Forward = 1i32,
    ForwardPlus = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+RenderingPath")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_RenderingPath
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/ProjectSettings/RenderingPath";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+RenderingPath")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_RenderingPath
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+RenderingPath")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_RenderingPath
{
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+RenderingPath")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_RenderingPath
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+RenderingPath")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_RenderingPath
{
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+XrPlugin")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ProjectSettings_OVRTelemetryConstants_XrPlugin {
    #[default]
    Oculus = 1i32,
    OpenXR = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+XrPlugin")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_XrPlugin
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/ProjectSettings/XrPlugin";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+XrPlugin")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_XrPlugin
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+XrPlugin")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_XrPlugin
{
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+XrPlugin")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_XrPlugin
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+ProjectSettings+XrPlugin")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::ProjectSettings_OVRTelemetryConstants_XrPlugin
{
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
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Scene+AnnotationType")]
#[repr(C)]
#[derive(Debug)]
pub struct Scene_OVRTelemetryConstants_AnnotationType {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Scene+AnnotationType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::Scene_OVRTelemetryConstants_AnnotationType
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/Scene/AnnotationType";
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
#[cfg(feature = "OVRTelemetryConstants+Scene+AnnotationType")]
impl std::ops::Deref for crate::GlobalNamespace::Scene_OVRTelemetryConstants_AnnotationType {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Scene+AnnotationType")]
impl std::ops::DerefMut for crate::GlobalNamespace::Scene_OVRTelemetryConstants_AnnotationType {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Scene+AnnotationType")]
impl crate::GlobalNamespace::Scene_OVRTelemetryConstants_AnnotationType {
    pub const ActiveRoomsOnly: &'static str = "active_rooms_only";
    pub const BuildTarget: &'static str = "BuildTarget";
    pub const EnabledSettings: &'static str = "FeaturesSupportInSettings";
    pub const Features: &'static str = "Features";
    pub const Guid: &'static str = "Guid";
    pub const RuntimePlatform: &'static str = "RuntimePlatform";
    pub const UsingBasicPrefabs: &'static str = "basic_prefabs";
    pub const UsingPrefabOverrides: &'static str = "prefab_overrides";
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Scene+AnnotationType")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::Scene_OVRTelemetryConstants_AnnotationType
{
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
    for crate::GlobalNamespace::Scene_OVRTelemetryConstants_MarkerId
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/Scene/MarkerId";
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
#[cfg(feature = "OVRTelemetryConstants+Scene+MarkerId")]
impl std::ops::Deref for crate::GlobalNamespace::Scene_OVRTelemetryConstants_MarkerId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Scene+MarkerId")]
impl std::ops::DerefMut for crate::GlobalNamespace::Scene_OVRTelemetryConstants_MarkerId {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Scene+MarkerId")]
impl crate::GlobalNamespace::Scene_OVRTelemetryConstants_MarkerId {
    pub const SceneClose: i32 = 163068562i32;
    pub const SceneOpen: i32 = 163063049i32;
    pub const UseDefaultSceneModelLoader: i32 = 163059869i32;
    pub const UseOVRSceneManager: i32 = 163061745i32;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Scene+MarkerId")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::Scene_OVRTelemetryConstants_MarkerId
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Utils+AnnotationType")]
#[repr(C)]
#[derive(Debug)]
pub struct Utils_OVRTelemetryConstants_AnnotationType {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Utils+AnnotationType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::Utils_OVRTelemetryConstants_AnnotationType
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/Utils/AnnotationType";
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
#[cfg(feature = "OVRTelemetryConstants+Utils+AnnotationType")]
impl std::ops::Deref for crate::GlobalNamespace::Utils_OVRTelemetryConstants_AnnotationType {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Utils+AnnotationType")]
impl std::ops::DerefMut for crate::GlobalNamespace::Utils_OVRTelemetryConstants_AnnotationType {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Utils+AnnotationType")]
impl crate::GlobalNamespace::Utils_OVRTelemetryConstants_AnnotationType {
    pub const ContentType: &'static str = "content_type";
    pub const ErrorMessage: &'static str = "error_message";
    pub const StatusCode: &'static str = "status_code";
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Utils+AnnotationType")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::Utils_OVRTelemetryConstants_AnnotationType
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Utils+MarkerId")]
#[repr(C)]
#[derive(Debug)]
pub struct Utils_OVRTelemetryConstants_MarkerId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Utils+MarkerId")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::Utils_OVRTelemetryConstants_MarkerId
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/Utils/MarkerId";
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
#[cfg(feature = "OVRTelemetryConstants+Utils+MarkerId")]
impl std::ops::Deref for crate::GlobalNamespace::Utils_OVRTelemetryConstants_MarkerId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Utils+MarkerId")]
impl std::ops::DerefMut for crate::GlobalNamespace::Utils_OVRTelemetryConstants_MarkerId {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+Utils+MarkerId")]
impl crate::GlobalNamespace::Utils_OVRTelemetryConstants_MarkerId {
    pub const DownloadContent: i32 = 163067281i32;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+Utils+MarkerId")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::Utils_OVRTelemetryConstants_MarkerId
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+XRSim+AnnotationType")]
#[repr(C)]
#[derive(Debug)]
pub struct XRSim_OVRTelemetryConstants_AnnotationType {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+XRSim+AnnotationType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::XRSim_OVRTelemetryConstants_AnnotationType
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/XRSim/AnnotationType";
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
#[cfg(feature = "OVRTelemetryConstants+XRSim+AnnotationType")]
impl std::ops::Deref for crate::GlobalNamespace::XRSim_OVRTelemetryConstants_AnnotationType {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+XRSim+AnnotationType")]
impl std::ops::DerefMut for crate::GlobalNamespace::XRSim_OVRTelemetryConstants_AnnotationType {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+XRSim+AnnotationType")]
impl crate::GlobalNamespace::XRSim_OVRTelemetryConstants_AnnotationType {
    pub const Action: &'static str = "action";
    pub const EngineXRSimSession: &'static str = "engine_xrsim_session";
    pub const IsActive: &'static str = "active";
    pub const XRSimEnabled: &'static str = "xrsimenabled";
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+XRSim+AnnotationType")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::XRSim_OVRTelemetryConstants_AnnotationType
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+XRSim+MarkerId")]
#[repr(C)]
#[derive(Debug)]
pub struct XRSim_OVRTelemetryConstants_MarkerId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+XRSim+MarkerId")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::XRSim_OVRTelemetryConstants_MarkerId
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryConstants/XRSim/MarkerId";
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
#[cfg(feature = "OVRTelemetryConstants+XRSim+MarkerId")]
impl std::ops::Deref for crate::GlobalNamespace::XRSim_OVRTelemetryConstants_MarkerId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+XRSim+MarkerId")]
impl std::ops::DerefMut for crate::GlobalNamespace::XRSim_OVRTelemetryConstants_MarkerId {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetryConstants+XRSim+MarkerId")]
impl crate::GlobalNamespace::XRSim_OVRTelemetryConstants_MarkerId {
    pub const EditorRun: i32 = 163063015i32;
    pub const SESInteraction: i32 = 163056472i32;
    pub const ToggleState: i32 = 163059165i32;
}
#[cfg(feature = "cordl_class_OVRTelemetryConstants+XRSim+MarkerId")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::XRSim_OVRTelemetryConstants_MarkerId
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

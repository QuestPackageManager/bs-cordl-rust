#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+BuildSpecificMetadata")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct RunScreenshotLevelData_ScreenshotLevelStarter_BuildSpecificMetadata {
    pub gameVersion: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+BuildSpecificMetadata")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_BuildSpecificMetadata
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str =
        "ScreenshotLevelStarter/RunScreenshotLevelData/BuildSpecificMetadata";
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+BuildSpecificMetadata")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_BuildSpecificMetadata
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+BuildSpecificMetadata")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_BuildSpecificMetadata
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+BuildSpecificMetadata")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_BuildSpecificMetadata
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+BuildSpecificMetadata")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_BuildSpecificMetadata
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+BuildSpecificMetadata")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_BuildSpecificMetadata
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ScreenshotLevelStarter+RunScreenshotLevelData+BuildSpecificMetadata")]
impl crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_BuildSpecificMetadata {}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+Command")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct RunScreenshotLevelData_ScreenshotLevelStarter_Command {
    pub beatmapKey:
        crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_SerializedBeatmapKey,
    pub screenshotTimes:
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<f32>>,
    pub overrideGlobalSettings: bool,
    pub settings: crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Settings,
    pub differenceThreshold: f32,
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+Command")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Command
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ScreenshotLevelStarter/RunScreenshotLevelData/Command";
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+Command")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Command
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+Command")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Command
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+Command")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Command
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+Command")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Command
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+Command")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Command
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ScreenshotLevelStarter+RunScreenshotLevelData+Command")]
impl crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Command {}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+DeviceMetadata")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct RunScreenshotLevelData_ScreenshotLevelStarter_DeviceMetadata {
    pub deviceUID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub deviceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub deviceType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub devicePlatform: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub deviceModel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+DeviceMetadata")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_DeviceMetadata
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ScreenshotLevelStarter/RunScreenshotLevelData/DeviceMetadata";
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+DeviceMetadata")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_DeviceMetadata
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+DeviceMetadata")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_DeviceMetadata
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+DeviceMetadata")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_DeviceMetadata
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+DeviceMetadata")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_DeviceMetadata
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+DeviceMetadata")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_DeviceMetadata
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ScreenshotLevelStarter+RunScreenshotLevelData+DeviceMetadata")]
impl crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_DeviceMetadata {}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+FolderMetadata")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct RunScreenshotLevelData_ScreenshotLevelStarter_FolderMetadata {
    pub command: crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Command,
    pub gameSettings: crate::BeatSaber::Settings::Settings,
    pub playerSpecificSettings:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSpecificSettings>,
    pub gameplayModifiers: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+FolderMetadata")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_FolderMetadata
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ScreenshotLevelStarter/RunScreenshotLevelData/FolderMetadata";
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+FolderMetadata")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_FolderMetadata
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+FolderMetadata")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_FolderMetadata
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+FolderMetadata")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_FolderMetadata
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+FolderMetadata")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_FolderMetadata
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+FolderMetadata")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_FolderMetadata
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ScreenshotLevelStarter+RunScreenshotLevelData+FolderMetadata")]
impl crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_FolderMetadata {}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+SerializedBeatmapKey")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct RunScreenshotLevelData_ScreenshotLevelStarter_SerializedBeatmapKey {
    pub levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub characteristic: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub difficulty: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+SerializedBeatmapKey")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_SerializedBeatmapKey
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str =
        "ScreenshotLevelStarter/RunScreenshotLevelData/SerializedBeatmapKey";
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+SerializedBeatmapKey")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_SerializedBeatmapKey
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+SerializedBeatmapKey")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_SerializedBeatmapKey
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+SerializedBeatmapKey")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_SerializedBeatmapKey
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+SerializedBeatmapKey")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_SerializedBeatmapKey
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+SerializedBeatmapKey")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_SerializedBeatmapKey
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ScreenshotLevelStarter+RunScreenshotLevelData+SerializedBeatmapKey")]
impl crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_SerializedBeatmapKey {
    pub fn ToBeatmapKey(
        &mut self,
        beatmapLevels: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelsModel>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapKey> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapLevelsModel,
                        >),
                        crate::GlobalNamespace::BeatmapKey,
                        1usize,
                    >("ToBeatmapKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToBeatmapKey", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::BeatmapKey =
            unsafe { cordl_method_info.invoke_unchecked(self, (beatmapLevels))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        characteristic: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficulty: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (levelId, characteristic, difficulty))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+Settings")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct RunScreenshotLevelData_ScreenshotLevelStarter_Settings {
    pub framerate: i32,
    pub screenshotPeriod: f32,
    pub overwriteExistingFolder: bool,
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+Settings")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Settings
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ScreenshotLevelStarter/RunScreenshotLevelData/Settings";
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+Settings")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Settings
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+Settings")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Settings
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+Settings")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Settings
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+Settings")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Settings
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData+Settings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Settings
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ScreenshotLevelStarter+RunScreenshotLevelData+Settings")]
impl crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Settings {}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter")]
#[repr(C)]
#[derive(Debug)]
pub struct ScreenshotLevelStarter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _gameScenesManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameScenesManager>,
    pub _menuTransitionsHelper:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuTransitionsHelper>,
    pub _beatmapLevels: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelsModel>,
    pub _gameVersionProvider:
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersionProvider>,
    pub _settingsManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SettingsManager>,
    pub _determinismConfig: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::DeterminismConfig>,
    pub _coroutineStarter: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ICoroutineStarter>,
    pub _renderingParamsApplicator:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IRenderingParamsApplicator>,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerDataModel>,
    pub _timeHelper: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::TimeHelper>,
    pub _commandQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Queue_1<
            crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Command,
        >,
    >,
    pub _screenshotQueueCoroutine: quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine>,
    pub _cachedGameVersion: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::ScreenshotLevelStarter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ScreenshotLevelStarter";
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
#[cfg(feature = "ScreenshotLevelStarter")]
impl std::ops::Deref for crate::GlobalNamespace::ScreenshotLevelStarter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScreenshotLevelStarter")]
impl std::ops::DerefMut for crate::GlobalNamespace::ScreenshotLevelStarter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScreenshotLevelStarter")]
impl crate::GlobalNamespace::ScreenshotLevelStarter {
    pub const kCameraFieldOfView: f32 = 70f32;
    pub const kCancelledGettingLevelDataVersion: &'static str =
        "ScreenshotLevelStarter: Getting level data version was cancelled.";
    pub const kFmtErrorGettingLevelDataVerison: &'static str =
        "ScreenshotLevelStarter: Failed getting level data version: {0}";
    pub const kFmtMessageDownloadFolder: &'static str =
        "ScreenshotLevelStarter: Screenshots for level found under: {0}";
    pub const kFmtMessageErrorCharacteristicNotFound: &'static str =
        "ScreenshotLevelStarter: Level {0} does not have characteristic {1}";
    pub const kFmtMessageErrorDifficultyNotFound: &'static str =
        "ScreenshotLevelStarter: Level {0} with characteristic {1} does not have difficulty {2}";
    pub const kFmtMessageErrorMapNotFound: &'static str =
        "ScreenshotLevelStarter: Could not find beatmap level {0}";
    pub const kFmtMessageErrorParsingDifficulty: &'static str =
        "ScreenshotLevelStarter: Unknown difficulty {0}, falling back to ExpertPlus";
    pub const kFmtMessageErrorSavingMetadata: &'static str =
        "ScreenshotLevelStarter: Error while saving metadata to file {0}: {1}";
    pub const kFmtMessageErrorSavingScreenshot: &'static str =
        "ScreenshotLevelStarter: Error while saving screenshot to path {0}: {1}";
    pub const kFmtMessageSavingMetadata: &'static str =
        "ScreenshotLevelStarter: Writing metadata to path {0}";
    pub const kFmtMessageScreenshotTaken: &'static str =
        "ScreenshotLevelStarter: Writing screenshot to path {0}";
    pub const kFmtMessageScreenshottingFinished: &'static str =
        "ScreenshotLevelStarter: Finished screenshots for level {0}";
    pub const kFmtMessageScreenshottingStarted: &'static str =
        "ScreenshotLevelStarter: Starting screenshot test for level {0}, will be stored in {1}";
    pub const kFmtMessageSongStarted: &'static str =
        "ScreenshotLevelStarter: Starting screenshotting of {0}";
    pub const kFmtMessageSongsRemaining: &'static str =
        "ScreenshotLevelStarter: -- There are {0} songs in queue...";
    pub const kFmtMessaggeWontPlayError: &'static str =
        "ScreenshotLevelStarter: Failed launching screenshotting for {0} without retrying!";
    pub const kFullHDHeight: i32 = 1080i32;
    pub const kFullHDWidth: i32 = 1920i32;
    pub const kLogPrefix: &'static str = "ScreenshotLevelStarter: ";
    pub const kMessageDefaultingToFrameRate: &'static str = "ScreenshotLevelStarter: Defaulting to frame rate of 30 FPS, no valid command.framerate set.";
    pub const kMessagePlayerSensitivity: &'static str =
        "ScreenshotLevelStarter: Player not allowed to play explicit songs";
    pub const kMessageQueueFinished: &'static str =
        "ScreenshotLevelStarter: ---Finished the screenshot level queue---";
    pub const kMessageQueueStarted: &'static str =
        "ScreenshotLevelStarter: ---Starting the screenshot level queue---";
    pub const kMessageSongFinished: &'static str = "ScreenshotLevelStarter: -- Finished song";
    pub const kMessageSongStarted: &'static str = "ScreenshotLevelStarter: -- Started song";
    #[cfg(feature = "ScreenshotLevelStarter+RunScreenshotLevelData")]
    pub type RunScreenshotLevelData =
        crate::GlobalNamespace::ScreenshotLevelStarter_RunScreenshotLevelData;
    pub fn EnqueueScreenshotLevel_Il2CppString_Il2CppString_Il2CppString_f32_List_1_i32_0(
        &mut self,
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        characteristic: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficulty: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        screenshotPeriod: f32,
        screenshotTimes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<f32>,
        >,
        captureFramerate: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<f32>>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "EnqueueScreenshotLevel"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnqueueScreenshotLevel",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    levelID,
                    characteristic,
                    difficulty,
                    screenshotPeriod,
                    screenshotTimes,
                    captureFramerate,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnqueueScreenshotLevel_RunScreenshotLevelData_ScreenshotLevelStarter_Command1(
        &mut self,
        command: crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Command,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Command),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EnqueueScreenshotLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnqueueScreenshotLevel", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (command))? };
        Ok(__cordl_ret.into())
    }
    pub fn HandlePauseControllerCanPause(
        &mut self,
        canPause: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HandlePauseControllerCanPause")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandlePauseControllerCanPause", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (canPause))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RunScreenshotLevel(
        &mut self,
        command: crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Command,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Command),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerator,
                        >,
                        1usize,
                    >("RunScreenshotLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RunScreenshotLevel", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> =
            unsafe { cordl_method_info.invoke_unchecked(self, (command))? };
        Ok(__cordl_ret.into())
    }
    pub fn RunScreenshotLevelQueue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerator,
                        >,
                        0usize,
                    >("RunScreenshotLevelQueue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RunScreenshotLevelQueue", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SaveBuildSpecificMetadata(
        metadataFolderPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gameVersion: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SaveBuildSpecificMetadata"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SaveBuildSpecificMetadata",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (metadataFolderPath, gameVersion))? };
        Ok(__cordl_ret.into())
    }
    pub fn SaveDeviceMetadata(
        metadataFolderPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SaveDeviceMetadata")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SaveDeviceMetadata", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (metadataFolderPath))? };
        Ok(__cordl_ret.into())
    }
    pub fn SaveMetadata(
        command: crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Command,
        metadataFolderPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        playerSpecificSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSpecificSettings>,
        >,
        gameplayModifiers: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Command,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::BeatSaber::Settings::Settings,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::PlayerSpecificSettings,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::GameplayModifiers,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SaveMetadata")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SaveMetadata", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    command,
                    metadataFolderPath,
                    settings,
                    playerSpecificSettings,
                    gameplayModifiers,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SaveMetadataObject(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SaveMetadataObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SaveMetadataObject",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (obj, path))? };
        Ok(__cordl_ret.into())
    }
    pub fn StopScreenshotting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("StopScreenshotting")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "StopScreenshotting",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn TakeScreenshotFromCamera(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        screenshotTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "TakeScreenshotFromCamera"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TakeScreenshotFromCamera",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (camera, screenshotTexture))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Screenshotting(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_Screenshotting")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Screenshotting",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ScreenshotLevelStarter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct ScreenshotLevelStarter_RunScreenshotLevelData {
    pub globalScreenshotSettings:
        crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Settings,
    pub screenshotLevels: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Command,
        >,
    >,
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::ScreenshotLevelStarter_RunScreenshotLevelData
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ScreenshotLevelStarter/RunScreenshotLevelData";
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::ScreenshotLevelStarter_RunScreenshotLevelData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::ScreenshotLevelStarter_RunScreenshotLevelData
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::ScreenshotLevelStarter_RunScreenshotLevelData
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::ScreenshotLevelStarter_RunScreenshotLevelData
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
#[cfg(feature = "cordl_class_ScreenshotLevelStarter+RunScreenshotLevelData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::ScreenshotLevelStarter_RunScreenshotLevelData
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ScreenshotLevelStarter+RunScreenshotLevelData")]
impl crate::GlobalNamespace::ScreenshotLevelStarter_RunScreenshotLevelData {
    #[cfg(feature = "ScreenshotLevelStarter+RunScreenshotLevelData+BuildSpecificMetadata")]
    pub type BuildSpecificMetadata =
        crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_BuildSpecificMetadata;
    #[cfg(feature = "ScreenshotLevelStarter+RunScreenshotLevelData+Command")]
    pub type Command =
        crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Command;
    #[cfg(feature = "ScreenshotLevelStarter+RunScreenshotLevelData+DeviceMetadata")]
    pub type DeviceMetadata =
        crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_DeviceMetadata;
    #[cfg(feature = "ScreenshotLevelStarter+RunScreenshotLevelData+FolderMetadata")]
    pub type FolderMetadata =
        crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_FolderMetadata;
    #[cfg(feature = "ScreenshotLevelStarter+RunScreenshotLevelData+SerializedBeatmapKey")]
    pub type SerializedBeatmapKey =
        crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_SerializedBeatmapKey;
    #[cfg(feature = "ScreenshotLevelStarter+RunScreenshotLevelData+Settings")]
    pub type Settings =
        crate::GlobalNamespace::RunScreenshotLevelData_ScreenshotLevelStarter_Settings;
}

#[cfg(feature = "OVRDeserialize")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRDeserialize {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRDeserialize")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRDeserialize {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize";
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
#[cfg(feature = "OVRDeserialize")]
impl std::ops::Deref for crate::GlobalNamespace::OVRDeserialize {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRDeserialize")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRDeserialize {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRDeserialize")]
impl crate::GlobalNamespace::OVRDeserialize {
    #[cfg(feature = "OVRDeserialize+DisplayRefreshRateChangedData")]
    pub type DisplayRefreshRateChangedData = crate::GlobalNamespace::OVRDeserialize_DisplayRefreshRateChangedData;
    #[cfg(feature = "OVRDeserialize+SceneCaptureCompleteData")]
    pub type SceneCaptureCompleteData = crate::GlobalNamespace::OVRDeserialize_SceneCaptureCompleteData;
    #[cfg(feature = "OVRDeserialize+SpaceEraseCompleteData")]
    pub type SpaceEraseCompleteData = crate::GlobalNamespace::OVRDeserialize_SpaceEraseCompleteData;
    #[cfg(feature = "OVRDeserialize+SpaceListSaveResultData")]
    pub type SpaceListSaveResultData = crate::GlobalNamespace::OVRDeserialize_SpaceListSaveResultData;
    #[cfg(feature = "OVRDeserialize+SpaceQueryCompleteData")]
    pub type SpaceQueryCompleteData = crate::GlobalNamespace::OVRDeserialize_SpaceQueryCompleteData;
    #[cfg(feature = "OVRDeserialize+SpaceQueryResultsData")]
    pub type SpaceQueryResultsData = crate::GlobalNamespace::OVRDeserialize_SpaceQueryResultsData;
    #[cfg(feature = "OVRDeserialize+SpaceSaveCompleteData")]
    pub type SpaceSaveCompleteData = crate::GlobalNamespace::OVRDeserialize_SpaceSaveCompleteData;
    #[cfg(feature = "OVRDeserialize+SpaceSetComponentStatusCompleteData")]
    pub type SpaceSetComponentStatusCompleteData = crate::GlobalNamespace::OVRDeserialize_SpaceSetComponentStatusCompleteData;
    #[cfg(feature = "OVRDeserialize+SpaceShareResultData")]
    pub type SpaceShareResultData = crate::GlobalNamespace::OVRDeserialize_SpaceShareResultData;
    #[cfg(feature = "OVRDeserialize+SpatialAnchorCreateCompleteData")]
    pub type SpatialAnchorCreateCompleteData = crate::GlobalNamespace::OVRDeserialize_SpatialAnchorCreateCompleteData;
    pub fn ByteArrayToStructure<T>(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >),
                        T,
                        1usize,
                    >("ByteArrayToStructure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ByteArrayToStructure", 1usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked((), (bytes))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRDeserialize")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRDeserialize {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRDeserialize+DisplayRefreshRateChangedData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_DisplayRefreshRateChangedData {
    pub FromRefreshRate: f32,
    pub ToRefreshRate: f32,
}
#[cfg(feature = "OVRDeserialize+DisplayRefreshRateChangedData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_DisplayRefreshRateChangedData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/DisplayRefreshRateChangedData";
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
#[cfg(feature = "OVRDeserialize+DisplayRefreshRateChangedData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_DisplayRefreshRateChangedData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRDeserialize+DisplayRefreshRateChangedData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_DisplayRefreshRateChangedData {
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
#[cfg(feature = "OVRDeserialize+DisplayRefreshRateChangedData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_DisplayRefreshRateChangedData {
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
#[cfg(feature = "OVRDeserialize+DisplayRefreshRateChangedData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_DisplayRefreshRateChangedData {
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
#[cfg(feature = "OVRDeserialize+DisplayRefreshRateChangedData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_DisplayRefreshRateChangedData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+DisplayRefreshRateChangedData")]
impl crate::GlobalNamespace::OVRDeserialize_DisplayRefreshRateChangedData {}
#[cfg(feature = "OVRDeserialize+SceneCaptureCompleteData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SceneCaptureCompleteData {
    pub RequestId: u64,
    pub Result: i32,
}
#[cfg(feature = "OVRDeserialize+SceneCaptureCompleteData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_SceneCaptureCompleteData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/SceneCaptureCompleteData";
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
#[cfg(feature = "OVRDeserialize+SceneCaptureCompleteData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_SceneCaptureCompleteData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRDeserialize+SceneCaptureCompleteData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_SceneCaptureCompleteData {
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
#[cfg(feature = "OVRDeserialize+SceneCaptureCompleteData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_SceneCaptureCompleteData {
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
#[cfg(feature = "OVRDeserialize+SceneCaptureCompleteData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_SceneCaptureCompleteData {
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
#[cfg(feature = "OVRDeserialize+SceneCaptureCompleteData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SceneCaptureCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SceneCaptureCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_SceneCaptureCompleteData {}
#[cfg(feature = "OVRDeserialize+SpaceEraseCompleteData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpaceEraseCompleteData {
    pub RequestId: u64,
    pub Result: i32,
    pub Uuid: crate::System::Guid,
    pub Location: crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
}
#[cfg(feature = "OVRDeserialize+SpaceEraseCompleteData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_SpaceEraseCompleteData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/SpaceEraseCompleteData";
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
#[cfg(feature = "OVRDeserialize+SpaceEraseCompleteData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_SpaceEraseCompleteData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRDeserialize+SpaceEraseCompleteData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_SpaceEraseCompleteData {
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
#[cfg(feature = "OVRDeserialize+SpaceEraseCompleteData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_SpaceEraseCompleteData {
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
#[cfg(feature = "OVRDeserialize+SpaceEraseCompleteData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_SpaceEraseCompleteData {
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
#[cfg(feature = "OVRDeserialize+SpaceEraseCompleteData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpaceEraseCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpaceEraseCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_SpaceEraseCompleteData {}
#[cfg(feature = "OVRDeserialize+SpaceListSaveResultData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpaceListSaveResultData {
    pub RequestId: u64,
    pub Result: i32,
}
#[cfg(feature = "OVRDeserialize+SpaceListSaveResultData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_SpaceListSaveResultData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/SpaceListSaveResultData";
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
#[cfg(feature = "OVRDeserialize+SpaceListSaveResultData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_SpaceListSaveResultData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRDeserialize+SpaceListSaveResultData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_SpaceListSaveResultData {
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
#[cfg(feature = "OVRDeserialize+SpaceListSaveResultData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_SpaceListSaveResultData {
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
#[cfg(feature = "OVRDeserialize+SpaceListSaveResultData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_SpaceListSaveResultData {
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
#[cfg(feature = "OVRDeserialize+SpaceListSaveResultData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpaceListSaveResultData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpaceListSaveResultData")]
impl crate::GlobalNamespace::OVRDeserialize_SpaceListSaveResultData {}
#[cfg(feature = "OVRDeserialize+SpaceQueryCompleteData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpaceQueryCompleteData {
    pub RequestId: u64,
    pub Result: i32,
}
#[cfg(feature = "OVRDeserialize+SpaceQueryCompleteData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_SpaceQueryCompleteData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/SpaceQueryCompleteData";
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
#[cfg(feature = "OVRDeserialize+SpaceQueryCompleteData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_SpaceQueryCompleteData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRDeserialize+SpaceQueryCompleteData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_SpaceQueryCompleteData {
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
#[cfg(feature = "OVRDeserialize+SpaceQueryCompleteData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_SpaceQueryCompleteData {
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
#[cfg(feature = "OVRDeserialize+SpaceQueryCompleteData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_SpaceQueryCompleteData {
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
#[cfg(feature = "OVRDeserialize+SpaceQueryCompleteData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpaceQueryCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpaceQueryCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_SpaceQueryCompleteData {}
#[cfg(feature = "OVRDeserialize+SpaceQueryResultsData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpaceQueryResultsData {
    pub RequestId: u64,
}
#[cfg(feature = "OVRDeserialize+SpaceQueryResultsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_SpaceQueryResultsData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/SpaceQueryResultsData";
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
#[cfg(feature = "OVRDeserialize+SpaceQueryResultsData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_SpaceQueryResultsData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRDeserialize+SpaceQueryResultsData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_SpaceQueryResultsData {
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
#[cfg(feature = "OVRDeserialize+SpaceQueryResultsData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_SpaceQueryResultsData {
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
#[cfg(feature = "OVRDeserialize+SpaceQueryResultsData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_SpaceQueryResultsData {
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
#[cfg(feature = "OVRDeserialize+SpaceQueryResultsData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpaceQueryResultsData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpaceQueryResultsData")]
impl crate::GlobalNamespace::OVRDeserialize_SpaceQueryResultsData {}
#[cfg(feature = "OVRDeserialize+SpaceSaveCompleteData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpaceSaveCompleteData {
    pub RequestId: u64,
    pub Space: u64,
    pub Result: i32,
    pub Uuid: crate::System::Guid,
}
#[cfg(feature = "OVRDeserialize+SpaceSaveCompleteData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_SpaceSaveCompleteData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/SpaceSaveCompleteData";
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
#[cfg(feature = "OVRDeserialize+SpaceSaveCompleteData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_SpaceSaveCompleteData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRDeserialize+SpaceSaveCompleteData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_SpaceSaveCompleteData {
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
#[cfg(feature = "OVRDeserialize+SpaceSaveCompleteData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_SpaceSaveCompleteData {
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
#[cfg(feature = "OVRDeserialize+SpaceSaveCompleteData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_SpaceSaveCompleteData {
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
#[cfg(feature = "OVRDeserialize+SpaceSaveCompleteData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpaceSaveCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpaceSaveCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_SpaceSaveCompleteData {}
#[cfg(feature = "OVRDeserialize+SpaceSetComponentStatusCompleteData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpaceSetComponentStatusCompleteData {
    pub RequestId: u64,
    pub Result: i32,
    pub Space: u64,
    pub Uuid: crate::System::Guid,
    pub ComponentType: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    pub Enabled: i32,
}
#[cfg(feature = "OVRDeserialize+SpaceSetComponentStatusCompleteData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_SpaceSetComponentStatusCompleteData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/SpaceSetComponentStatusCompleteData";
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
#[cfg(feature = "OVRDeserialize+SpaceSetComponentStatusCompleteData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_SpaceSetComponentStatusCompleteData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRDeserialize+SpaceSetComponentStatusCompleteData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_SpaceSetComponentStatusCompleteData {
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
#[cfg(feature = "OVRDeserialize+SpaceSetComponentStatusCompleteData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_SpaceSetComponentStatusCompleteData {
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
#[cfg(feature = "OVRDeserialize+SpaceSetComponentStatusCompleteData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_SpaceSetComponentStatusCompleteData {
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
#[cfg(feature = "OVRDeserialize+SpaceSetComponentStatusCompleteData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpaceSetComponentStatusCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpaceSetComponentStatusCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_SpaceSetComponentStatusCompleteData {}
#[cfg(feature = "OVRDeserialize+SpaceShareResultData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpaceShareResultData {
    pub RequestId: u64,
    pub Result: i32,
}
#[cfg(feature = "OVRDeserialize+SpaceShareResultData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_SpaceShareResultData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/SpaceShareResultData";
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
#[cfg(feature = "OVRDeserialize+SpaceShareResultData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_SpaceShareResultData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRDeserialize+SpaceShareResultData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_SpaceShareResultData {
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
#[cfg(feature = "OVRDeserialize+SpaceShareResultData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_SpaceShareResultData {
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
#[cfg(feature = "OVRDeserialize+SpaceShareResultData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_SpaceShareResultData {
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
#[cfg(feature = "OVRDeserialize+SpaceShareResultData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpaceShareResultData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpaceShareResultData")]
impl crate::GlobalNamespace::OVRDeserialize_SpaceShareResultData {}
#[cfg(feature = "OVRDeserialize+SpatialAnchorCreateCompleteData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpatialAnchorCreateCompleteData {
    pub RequestId: u64,
    pub Result: i32,
    pub Space: u64,
    pub Uuid: crate::System::Guid,
}
#[cfg(feature = "OVRDeserialize+SpatialAnchorCreateCompleteData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_SpatialAnchorCreateCompleteData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/SpatialAnchorCreateCompleteData";
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
#[cfg(feature = "OVRDeserialize+SpatialAnchorCreateCompleteData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_SpatialAnchorCreateCompleteData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRDeserialize+SpatialAnchorCreateCompleteData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_SpatialAnchorCreateCompleteData {
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
#[cfg(feature = "OVRDeserialize+SpatialAnchorCreateCompleteData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_SpatialAnchorCreateCompleteData {
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
#[cfg(feature = "OVRDeserialize+SpatialAnchorCreateCompleteData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_SpatialAnchorCreateCompleteData {
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
#[cfg(feature = "OVRDeserialize+SpatialAnchorCreateCompleteData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpatialAnchorCreateCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpatialAnchorCreateCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_SpatialAnchorCreateCompleteData {}

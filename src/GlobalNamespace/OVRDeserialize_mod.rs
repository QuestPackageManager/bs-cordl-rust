#[cfg(
    feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryResultData+_AdvertisementMetadata_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ColocationSessionDiscoveryResultData_OVRDeserialize__AdvertisementMetadata_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryResultData+_AdvertisementMetadata_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ColocationSessionDiscoveryResultData_OVRDeserialize__AdvertisementMetadata_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/ColocationSessionDiscoveryResultData/<AdvertisementMetadata>e__FixedBuffer";
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
#[cfg(
    feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryResultData+_AdvertisementMetadata_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::ColocationSessionDiscoveryResultData_OVRDeserialize__AdvertisementMetadata_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryResultData+_AdvertisementMetadata_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::ColocationSessionDiscoveryResultData_OVRDeserialize__AdvertisementMetadata_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryResultData+_AdvertisementMetadata_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::ColocationSessionDiscoveryResultData_OVRDeserialize__AdvertisementMetadata_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryResultData+_AdvertisementMetadata_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::ColocationSessionDiscoveryResultData_OVRDeserialize__AdvertisementMetadata_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryResultData+_AdvertisementMetadata_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::ColocationSessionDiscoveryResultData_OVRDeserialize__AdvertisementMetadata_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "OVRDeserialize+ColocationSessionDiscoveryResultData+_AdvertisementMetadata_e__FixedBuffer"
)]
impl crate::GlobalNamespace::ColocationSessionDiscoveryResultData_OVRDeserialize__AdvertisementMetadata_e__FixedBuffer {}
#[cfg(feature = "cordl_class_OVRDeserialize")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRDeserialize {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRDeserialize")]
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
    #[cfg(feature = "OVRDeserialize+BoundaryVisibilityChangedData")]
    pub type BoundaryVisibilityChangedData = crate::GlobalNamespace::OVRDeserialize_BoundaryVisibilityChangedData;
    #[cfg(feature = "OVRDeserialize+ColocationSessionAdvertisementCompleteData")]
    pub type ColocationSessionAdvertisementCompleteData = crate::GlobalNamespace::OVRDeserialize_ColocationSessionAdvertisementCompleteData;
    #[cfg(feature = "OVRDeserialize+ColocationSessionDiscoveryCompleteData")]
    pub type ColocationSessionDiscoveryCompleteData = crate::GlobalNamespace::OVRDeserialize_ColocationSessionDiscoveryCompleteData;
    #[cfg(feature = "OVRDeserialize+ColocationSessionDiscoveryResultData")]
    pub type ColocationSessionDiscoveryResultData = crate::GlobalNamespace::OVRDeserialize_ColocationSessionDiscoveryResultData;
    #[cfg(feature = "OVRDeserialize+CreateDynamicObjectTrackerResultData")]
    pub type CreateDynamicObjectTrackerResultData = crate::GlobalNamespace::OVRDeserialize_CreateDynamicObjectTrackerResultData;
    #[cfg(feature = "OVRDeserialize+DisplayRefreshRateChangedData")]
    pub type DisplayRefreshRateChangedData = crate::GlobalNamespace::OVRDeserialize_DisplayRefreshRateChangedData;
    #[cfg(feature = "OVRDeserialize+PassthroughLayerResumedData")]
    pub type PassthroughLayerResumedData = crate::GlobalNamespace::OVRDeserialize_PassthroughLayerResumedData;
    #[cfg(feature = "OVRDeserialize+SceneCaptureCompleteData")]
    pub type SceneCaptureCompleteData = crate::GlobalNamespace::OVRDeserialize_SceneCaptureCompleteData;
    #[cfg(feature = "OVRDeserialize+SetDynamicObjectTrackedClassesResultData")]
    pub type SetDynamicObjectTrackedClassesResultData = crate::GlobalNamespace::OVRDeserialize_SetDynamicObjectTrackedClassesResultData;
    #[cfg(feature = "OVRDeserialize+ShareSpacesToGroupsCompleteData")]
    pub type ShareSpacesToGroupsCompleteData = crate::GlobalNamespace::OVRDeserialize_ShareSpacesToGroupsCompleteData;
    #[cfg(feature = "OVRDeserialize+SpaceDiscoveryCompleteData")]
    pub type SpaceDiscoveryCompleteData = crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryCompleteData;
    #[cfg(feature = "OVRDeserialize+SpaceDiscoveryResultsData")]
    pub type SpaceDiscoveryResultsData = crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryResultsData;
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
    #[cfg(feature = "OVRDeserialize+SpacesEraseResultData")]
    pub type SpacesEraseResultData = crate::GlobalNamespace::OVRDeserialize_SpacesEraseResultData;
    #[cfg(feature = "OVRDeserialize+SpacesSaveResultData")]
    pub type SpacesSaveResultData = crate::GlobalNamespace::OVRDeserialize_SpacesSaveResultData;
    #[cfg(feature = "OVRDeserialize+SpatialAnchorCreateCompleteData")]
    pub type SpatialAnchorCreateCompleteData = crate::GlobalNamespace::OVRDeserialize_SpatialAnchorCreateCompleteData;
    #[cfg(feature = "OVRDeserialize+StartColocationSessionAdvertisementCompleteData")]
    pub type StartColocationSessionAdvertisementCompleteData = crate::GlobalNamespace::OVRDeserialize_StartColocationSessionAdvertisementCompleteData;
    #[cfg(feature = "OVRDeserialize+StartColocationSessionDiscoveryCompleteData")]
    pub type StartColocationSessionDiscoveryCompleteData = crate::GlobalNamespace::OVRDeserialize_StartColocationSessionDiscoveryCompleteData;
    #[cfg(feature = "OVRDeserialize+StopColocationSessionAdvertisementCompleteData")]
    pub type StopColocationSessionAdvertisementCompleteData = crate::GlobalNamespace::OVRDeserialize_StopColocationSessionAdvertisementCompleteData;
    #[cfg(feature = "OVRDeserialize+StopColocationSessionDiscoveryCompleteData")]
    pub type StopColocationSessionDiscoveryCompleteData = crate::GlobalNamespace::OVRDeserialize_StopColocationSessionDiscoveryCompleteData;
    pub fn ByteArrayToStructure<T>(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
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
        let __cordl_ret: T = unsafe { cordl_method_info.invoke_unchecked((), (bytes))? };
        Ok(__cordl_ret.into())
    }
    pub fn MarshalEntireStructAs<T>(
        eventDataBuffer: crate::GlobalNamespace::OVRPlugin_EventDataBuffer,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::OVRPlugin_EventDataBuffer,
                            crate::Unity::Collections::Allocator,
                        ),
                        T,
                        2usize,
                    >("MarshalEntireStructAs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MarshalEntireStructAs", 2usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            cordl_method_info.invoke_unchecked((), (eventDataBuffer, allocator))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRDeserialize")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRDeserialize {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRDeserialize+BoundaryVisibilityChangedData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_BoundaryVisibilityChangedData {
    pub BoundaryVisibility: crate::GlobalNamespace::OVRPlugin_BoundaryVisibility,
}
#[cfg(feature = "cordl_class_OVRDeserialize+BoundaryVisibilityChangedData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_BoundaryVisibilityChangedData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/BoundaryVisibilityChangedData";
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
#[cfg(feature = "cordl_class_OVRDeserialize+BoundaryVisibilityChangedData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_BoundaryVisibilityChangedData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRDeserialize+BoundaryVisibilityChangedData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_BoundaryVisibilityChangedData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+BoundaryVisibilityChangedData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_BoundaryVisibilityChangedData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+BoundaryVisibilityChangedData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_BoundaryVisibilityChangedData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+BoundaryVisibilityChangedData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_BoundaryVisibilityChangedData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+BoundaryVisibilityChangedData")]
impl crate::GlobalNamespace::OVRDeserialize_BoundaryVisibilityChangedData {}
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionAdvertisementCompleteData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_ColocationSessionAdvertisementCompleteData {
    pub EventType: crate::GlobalNamespace::OVRPlugin_EventType,
    pub RequestId: u64,
    pub Result: crate::GlobalNamespace::OVRPlugin_Result,
}
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionAdvertisementCompleteData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionAdvertisementCompleteData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/ColocationSessionAdvertisementCompleteData";
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
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionAdvertisementCompleteData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionAdvertisementCompleteData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionAdvertisementCompleteData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionAdvertisementCompleteData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionAdvertisementCompleteData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionAdvertisementCompleteData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionAdvertisementCompleteData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionAdvertisementCompleteData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionAdvertisementCompleteData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionAdvertisementCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+ColocationSessionAdvertisementCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_ColocationSessionAdvertisementCompleteData {}
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryCompleteData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_ColocationSessionDiscoveryCompleteData {
    pub EventType: crate::GlobalNamespace::OVRPlugin_EventType,
    pub RequestId: u64,
    pub Result: crate::GlobalNamespace::OVRPlugin_Result,
}
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionDiscoveryCompleteData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/ColocationSessionDiscoveryCompleteData";
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
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionDiscoveryCompleteData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionDiscoveryCompleteData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionDiscoveryCompleteData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionDiscoveryCompleteData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionDiscoveryCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+ColocationSessionDiscoveryCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_ColocationSessionDiscoveryCompleteData {}
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryResultData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_ColocationSessionDiscoveryResultData {
    pub EventType: crate::GlobalNamespace::OVRPlugin_EventType,
    pub RequestId: u64,
    pub AdvertisementUuid: crate::System::Guid,
    pub AdvertisementMetadataCount: u32,
    pub AdvertisementMetadata: crate::GlobalNamespace::ColocationSessionDiscoveryResultData_OVRDeserialize__AdvertisementMetadata_e__FixedBuffer,
}
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryResultData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionDiscoveryResultData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/ColocationSessionDiscoveryResultData";
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
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryResultData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionDiscoveryResultData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryResultData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionDiscoveryResultData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryResultData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionDiscoveryResultData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryResultData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionDiscoveryResultData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+ColocationSessionDiscoveryResultData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_ColocationSessionDiscoveryResultData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+ColocationSessionDiscoveryResultData")]
impl crate::GlobalNamespace::OVRDeserialize_ColocationSessionDiscoveryResultData {
    #[cfg(
        feature = "OVRDeserialize+ColocationSessionDiscoveryResultData+_AdvertisementMetadata_e__FixedBuffer"
    )]
    pub type _AdvertisementMetadata_e__FixedBuffer = crate::GlobalNamespace::ColocationSessionDiscoveryResultData_OVRDeserialize__AdvertisementMetadata_e__FixedBuffer;
}
#[cfg(feature = "cordl_class_OVRDeserialize+CreateDynamicObjectTrackerResultData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_CreateDynamicObjectTrackerResultData {
    pub EventType: crate::GlobalNamespace::OVRPlugin_EventType,
    pub Tracker: u64,
    pub Result: crate::GlobalNamespace::OVRPlugin_Result,
}
#[cfg(feature = "cordl_class_OVRDeserialize+CreateDynamicObjectTrackerResultData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_CreateDynamicObjectTrackerResultData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/CreateDynamicObjectTrackerResultData";
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
#[cfg(feature = "cordl_class_OVRDeserialize+CreateDynamicObjectTrackerResultData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_CreateDynamicObjectTrackerResultData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRDeserialize+CreateDynamicObjectTrackerResultData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_CreateDynamicObjectTrackerResultData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+CreateDynamicObjectTrackerResultData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_CreateDynamicObjectTrackerResultData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+CreateDynamicObjectTrackerResultData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_CreateDynamicObjectTrackerResultData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+CreateDynamicObjectTrackerResultData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_CreateDynamicObjectTrackerResultData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+CreateDynamicObjectTrackerResultData")]
impl crate::GlobalNamespace::OVRDeserialize_CreateDynamicObjectTrackerResultData {}
#[cfg(feature = "cordl_class_OVRDeserialize+DisplayRefreshRateChangedData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_DisplayRefreshRateChangedData {
    pub FromRefreshRate: f32,
    pub ToRefreshRate: f32,
}
#[cfg(feature = "cordl_class_OVRDeserialize+DisplayRefreshRateChangedData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+DisplayRefreshRateChangedData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+DisplayRefreshRateChangedData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+DisplayRefreshRateChangedData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+DisplayRefreshRateChangedData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+DisplayRefreshRateChangedData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+PassthroughLayerResumedData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_PassthroughLayerResumedData {
    pub LayerId: i32,
}
#[cfg(feature = "cordl_class_OVRDeserialize+PassthroughLayerResumedData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_PassthroughLayerResumedData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/PassthroughLayerResumedData";
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
#[cfg(feature = "cordl_class_OVRDeserialize+PassthroughLayerResumedData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_PassthroughLayerResumedData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRDeserialize+PassthroughLayerResumedData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_PassthroughLayerResumedData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+PassthroughLayerResumedData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_PassthroughLayerResumedData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+PassthroughLayerResumedData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_PassthroughLayerResumedData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+PassthroughLayerResumedData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_PassthroughLayerResumedData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+PassthroughLayerResumedData")]
impl crate::GlobalNamespace::OVRDeserialize_PassthroughLayerResumedData {}
#[cfg(feature = "cordl_class_OVRDeserialize+SceneCaptureCompleteData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SceneCaptureCompleteData {
    pub RequestId: u64,
    pub Result: i32,
}
#[cfg(feature = "cordl_class_OVRDeserialize+SceneCaptureCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SceneCaptureCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SceneCaptureCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SceneCaptureCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SceneCaptureCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SceneCaptureCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SetDynamicObjectTrackedClassesResultData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SetDynamicObjectTrackedClassesResultData {
    pub EventType: crate::GlobalNamespace::OVRPlugin_EventType,
    pub Tracker: u64,
    pub Result: crate::GlobalNamespace::OVRPlugin_Result,
}
#[cfg(feature = "cordl_class_OVRDeserialize+SetDynamicObjectTrackedClassesResultData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_SetDynamicObjectTrackedClassesResultData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/SetDynamicObjectTrackedClassesResultData";
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
#[cfg(feature = "cordl_class_OVRDeserialize+SetDynamicObjectTrackedClassesResultData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_SetDynamicObjectTrackedClassesResultData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRDeserialize+SetDynamicObjectTrackedClassesResultData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_SetDynamicObjectTrackedClassesResultData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+SetDynamicObjectTrackedClassesResultData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_SetDynamicObjectTrackedClassesResultData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+SetDynamicObjectTrackedClassesResultData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_SetDynamicObjectTrackedClassesResultData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+SetDynamicObjectTrackedClassesResultData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SetDynamicObjectTrackedClassesResultData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SetDynamicObjectTrackedClassesResultData")]
impl crate::GlobalNamespace::OVRDeserialize_SetDynamicObjectTrackedClassesResultData {}
#[cfg(feature = "cordl_class_OVRDeserialize+ShareSpacesToGroupsCompleteData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_ShareSpacesToGroupsCompleteData {
    pub EventType: crate::GlobalNamespace::OVRPlugin_EventType,
    pub RequestId: u64,
    pub Result: crate::GlobalNamespace::OVRPlugin_Result,
}
#[cfg(feature = "cordl_class_OVRDeserialize+ShareSpacesToGroupsCompleteData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_ShareSpacesToGroupsCompleteData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/ShareSpacesToGroupsCompleteData";
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
#[cfg(feature = "cordl_class_OVRDeserialize+ShareSpacesToGroupsCompleteData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_ShareSpacesToGroupsCompleteData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRDeserialize+ShareSpacesToGroupsCompleteData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_ShareSpacesToGroupsCompleteData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+ShareSpacesToGroupsCompleteData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_ShareSpacesToGroupsCompleteData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+ShareSpacesToGroupsCompleteData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_ShareSpacesToGroupsCompleteData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+ShareSpacesToGroupsCompleteData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_ShareSpacesToGroupsCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+ShareSpacesToGroupsCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_ShareSpacesToGroupsCompleteData {}
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceDiscoveryCompleteData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpaceDiscoveryCompleteData {
    pub RequestId: u64,
    pub Result: i32,
}
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryCompleteData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/SpaceDiscoveryCompleteData";
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryCompleteData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryCompleteData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryCompleteData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryCompleteData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpaceDiscoveryCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryCompleteData {}
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceDiscoveryResultsData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpaceDiscoveryResultsData {
    pub RequestId: u64,
}
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceDiscoveryResultsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryResultsData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/SpaceDiscoveryResultsData";
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceDiscoveryResultsData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryResultsData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceDiscoveryResultsData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryResultsData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceDiscoveryResultsData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryResultsData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceDiscoveryResultsData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryResultsData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceDiscoveryResultsData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryResultsData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpaceDiscoveryResultsData")]
impl crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryResultsData {}
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceEraseCompleteData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpaceEraseCompleteData {
    pub RequestId: u64,
    pub Result: i32,
    pub Uuid: crate::System::Guid,
    pub Location: crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
}
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceEraseCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceEraseCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceEraseCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceEraseCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceEraseCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceEraseCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceListSaveResultData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpaceListSaveResultData {
    pub RequestId: u64,
    pub Result: i32,
}
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceListSaveResultData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceListSaveResultData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceListSaveResultData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceListSaveResultData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceListSaveResultData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceListSaveResultData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceQueryCompleteData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpaceQueryCompleteData {
    pub RequestId: u64,
    pub Result: i32,
}
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceQueryCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceQueryCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceQueryCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceQueryCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceQueryCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceQueryCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceQueryResultsData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpaceQueryResultsData {
    pub RequestId: u64,
}
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceQueryResultsData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceQueryResultsData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceQueryResultsData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceQueryResultsData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceQueryResultsData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceQueryResultsData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceSaveCompleteData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpaceSaveCompleteData {
    pub RequestId: u64,
    pub Space: u64,
    pub Result: i32,
    pub Uuid: crate::System::Guid,
}
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceSaveCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceSaveCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceSaveCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceSaveCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceSaveCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceSaveCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceSetComponentStatusCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceSetComponentStatusCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceSetComponentStatusCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceSetComponentStatusCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceSetComponentStatusCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceSetComponentStatusCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceSetComponentStatusCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceShareResultData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpaceShareResultData {
    pub RequestId: u64,
    pub Result: i32,
}
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceShareResultData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceShareResultData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceShareResultData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceShareResultData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceShareResultData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpaceShareResultData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpacesEraseResultData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpacesEraseResultData {
    pub RequestId: u64,
    pub Result: crate::GlobalNamespace::OVRAnchor_EraseResult,
}
#[cfg(feature = "cordl_class_OVRDeserialize+SpacesEraseResultData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_SpacesEraseResultData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/SpacesEraseResultData";
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpacesEraseResultData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_SpacesEraseResultData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRDeserialize+SpacesEraseResultData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_SpacesEraseResultData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpacesEraseResultData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_SpacesEraseResultData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpacesEraseResultData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_SpacesEraseResultData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpacesEraseResultData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpacesEraseResultData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpacesEraseResultData")]
impl crate::GlobalNamespace::OVRDeserialize_SpacesEraseResultData {}
#[cfg(feature = "cordl_class_OVRDeserialize+SpacesSaveResultData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpacesSaveResultData {
    pub RequestId: u64,
    pub Result: crate::GlobalNamespace::OVRAnchor_SaveResult,
}
#[cfg(feature = "cordl_class_OVRDeserialize+SpacesSaveResultData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_SpacesSaveResultData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/SpacesSaveResultData";
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpacesSaveResultData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_SpacesSaveResultData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRDeserialize+SpacesSaveResultData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_SpacesSaveResultData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpacesSaveResultData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_SpacesSaveResultData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpacesSaveResultData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_SpacesSaveResultData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpacesSaveResultData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_SpacesSaveResultData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+SpacesSaveResultData")]
impl crate::GlobalNamespace::OVRDeserialize_SpacesSaveResultData {}
#[cfg(feature = "cordl_class_OVRDeserialize+SpatialAnchorCreateCompleteData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_SpatialAnchorCreateCompleteData {
    pub RequestId: u64,
    pub Result: i32,
    pub Space: u64,
    pub Uuid: crate::System::Guid,
}
#[cfg(feature = "cordl_class_OVRDeserialize+SpatialAnchorCreateCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpatialAnchorCreateCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpatialAnchorCreateCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpatialAnchorCreateCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpatialAnchorCreateCompleteData")]
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
#[cfg(feature = "cordl_class_OVRDeserialize+SpatialAnchorCreateCompleteData")]
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
#[cfg(
    feature = "cordl_class_OVRDeserialize+StartColocationSessionAdvertisementCompleteData"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_StartColocationSessionAdvertisementCompleteData {
    pub EventType: crate::GlobalNamespace::OVRPlugin_EventType,
    pub RequestId: u64,
    pub Result: crate::GlobalNamespace::OVRPlugin_Result,
    pub AdvertisementUuid: crate::System::Guid,
}
#[cfg(
    feature = "cordl_class_OVRDeserialize+StartColocationSessionAdvertisementCompleteData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_StartColocationSessionAdvertisementCompleteData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/StartColocationSessionAdvertisementCompleteData";
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
#[cfg(
    feature = "cordl_class_OVRDeserialize+StartColocationSessionAdvertisementCompleteData"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_StartColocationSessionAdvertisementCompleteData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_OVRDeserialize+StartColocationSessionAdvertisementCompleteData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_StartColocationSessionAdvertisementCompleteData {
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
#[cfg(
    feature = "cordl_class_OVRDeserialize+StartColocationSessionAdvertisementCompleteData"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_StartColocationSessionAdvertisementCompleteData {
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
#[cfg(
    feature = "cordl_class_OVRDeserialize+StartColocationSessionAdvertisementCompleteData"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_StartColocationSessionAdvertisementCompleteData {
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
#[cfg(
    feature = "cordl_class_OVRDeserialize+StartColocationSessionAdvertisementCompleteData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_StartColocationSessionAdvertisementCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+StartColocationSessionAdvertisementCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_StartColocationSessionAdvertisementCompleteData {}
#[cfg(
    feature = "cordl_class_OVRDeserialize+StartColocationSessionDiscoveryCompleteData"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_StartColocationSessionDiscoveryCompleteData {
    pub EventType: crate::GlobalNamespace::OVRPlugin_EventType,
    pub RequestId: u64,
    pub Result: crate::GlobalNamespace::OVRPlugin_Result,
}
#[cfg(
    feature = "cordl_class_OVRDeserialize+StartColocationSessionDiscoveryCompleteData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_StartColocationSessionDiscoveryCompleteData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/StartColocationSessionDiscoveryCompleteData";
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
#[cfg(
    feature = "cordl_class_OVRDeserialize+StartColocationSessionDiscoveryCompleteData"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_StartColocationSessionDiscoveryCompleteData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_OVRDeserialize+StartColocationSessionDiscoveryCompleteData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_StartColocationSessionDiscoveryCompleteData {
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
#[cfg(
    feature = "cordl_class_OVRDeserialize+StartColocationSessionDiscoveryCompleteData"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_StartColocationSessionDiscoveryCompleteData {
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
#[cfg(
    feature = "cordl_class_OVRDeserialize+StartColocationSessionDiscoveryCompleteData"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_StartColocationSessionDiscoveryCompleteData {
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
#[cfg(
    feature = "cordl_class_OVRDeserialize+StartColocationSessionDiscoveryCompleteData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_StartColocationSessionDiscoveryCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+StartColocationSessionDiscoveryCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_StartColocationSessionDiscoveryCompleteData {}
#[cfg(
    feature = "cordl_class_OVRDeserialize+StopColocationSessionAdvertisementCompleteData"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_StopColocationSessionAdvertisementCompleteData {
    pub EventType: crate::GlobalNamespace::OVRPlugin_EventType,
    pub RequestId: u64,
    pub Result: crate::GlobalNamespace::OVRPlugin_Result,
}
#[cfg(
    feature = "cordl_class_OVRDeserialize+StopColocationSessionAdvertisementCompleteData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_StopColocationSessionAdvertisementCompleteData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/StopColocationSessionAdvertisementCompleteData";
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
#[cfg(
    feature = "cordl_class_OVRDeserialize+StopColocationSessionAdvertisementCompleteData"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_StopColocationSessionAdvertisementCompleteData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_OVRDeserialize+StopColocationSessionAdvertisementCompleteData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_StopColocationSessionAdvertisementCompleteData {
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
#[cfg(
    feature = "cordl_class_OVRDeserialize+StopColocationSessionAdvertisementCompleteData"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_StopColocationSessionAdvertisementCompleteData {
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
#[cfg(
    feature = "cordl_class_OVRDeserialize+StopColocationSessionAdvertisementCompleteData"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_StopColocationSessionAdvertisementCompleteData {
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
#[cfg(
    feature = "cordl_class_OVRDeserialize+StopColocationSessionAdvertisementCompleteData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_StopColocationSessionAdvertisementCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+StopColocationSessionAdvertisementCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_StopColocationSessionAdvertisementCompleteData {}
#[cfg(feature = "cordl_class_OVRDeserialize+StopColocationSessionDiscoveryCompleteData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRDeserialize_StopColocationSessionDiscoveryCompleteData {
    pub EventType: crate::GlobalNamespace::OVRPlugin_EventType,
    pub RequestId: u64,
    pub Result: crate::GlobalNamespace::OVRPlugin_Result,
}
#[cfg(feature = "cordl_class_OVRDeserialize+StopColocationSessionDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRDeserialize_StopColocationSessionDiscoveryCompleteData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRDeserialize/StopColocationSessionDiscoveryCompleteData";
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
#[cfg(feature = "cordl_class_OVRDeserialize+StopColocationSessionDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRDeserialize_StopColocationSessionDiscoveryCompleteData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRDeserialize+StopColocationSessionDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRDeserialize_StopColocationSessionDiscoveryCompleteData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+StopColocationSessionDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRDeserialize_StopColocationSessionDiscoveryCompleteData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+StopColocationSessionDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRDeserialize_StopColocationSessionDiscoveryCompleteData {
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
#[cfg(feature = "cordl_class_OVRDeserialize+StopColocationSessionDiscoveryCompleteData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDeserialize_StopColocationSessionDiscoveryCompleteData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDeserialize+StopColocationSessionDiscoveryCompleteData")]
impl crate::GlobalNamespace::OVRDeserialize_StopColocationSessionDiscoveryCompleteData {}

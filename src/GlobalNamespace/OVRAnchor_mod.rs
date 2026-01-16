#[cfg(feature = "cordl_class_OVRAnchor")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OVRAnchor {
    pub _Handle_k__BackingField: u64,
    pub _Uuid_k__BackingField: crate::System::Guid,
}
#[cfg(feature = "cordl_class_OVRAnchor")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRAnchor {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor";
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
#[cfg(feature = "cordl_class_OVRAnchor")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRAnchor {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRAnchor")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRAnchor {
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
#[cfg(feature = "cordl_class_OVRAnchor")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRAnchor {
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
#[cfg(feature = "cordl_class_OVRAnchor")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRAnchor {
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
#[cfg(feature = "cordl_class_OVRAnchor")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::OVRAnchor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRAnchor")]
impl crate::GlobalNamespace::OVRAnchor {
    #[cfg(feature = "OVRAnchor+ConfigureTrackerResult")]
    pub type ConfigureTrackerResult = crate::GlobalNamespace::OVRAnchor_ConfigureTrackerResult;
    #[cfg(feature = "OVRAnchor+DeferredKey")]
    pub type DeferredKey = crate::GlobalNamespace::OVRAnchor_DeferredKey;
    #[cfg(feature = "OVRAnchor+DeferredValue")]
    pub type DeferredValue = crate::GlobalNamespace::OVRAnchor_DeferredValue;
    #[cfg(feature = "OVRAnchor+EraseResult")]
    pub type EraseResult = crate::GlobalNamespace::OVRAnchor_EraseResult;
    #[cfg(feature = "OVRAnchor+FetchOptions")]
    pub type FetchOptions = crate::GlobalNamespace::OVRAnchor_FetchOptions;
    #[cfg(feature = "OVRAnchor+FetchResult")]
    pub type FetchResult = crate::GlobalNamespace::OVRAnchor_FetchResult;
    #[cfg(feature = "OVRAnchor+FetchTaskData")]
    pub type FetchTaskData = crate::GlobalNamespace::OVRAnchor_FetchTaskData;
    #[cfg(feature = "OVRAnchor+FilterUnion")]
    pub type FilterUnion = crate::GlobalNamespace::OVRAnchor_FilterUnion;
    #[cfg(feature = "OVRAnchor+SaveResult")]
    pub type SaveResult = crate::GlobalNamespace::OVRAnchor_SaveResult;
    #[cfg(feature = "OVRAnchor+ShareResult")]
    pub type ShareResult = crate::GlobalNamespace::OVRAnchor_ShareResult;
    #[cfg(feature = "OVRAnchor+Telemetry")]
    pub type Telemetry = crate::GlobalNamespace::OVRAnchor_Telemetry;
    #[cfg(feature = "OVRAnchor+TrackableType")]
    pub type TrackableType = crate::GlobalNamespace::OVRAnchor_TrackableType;
    #[cfg(feature = "OVRAnchor+Tracker")]
    pub type Tracker = crate::GlobalNamespace::OVRAnchor_Tracker;
    #[cfg(feature = "OVRAnchor+TrackerConfiguration")]
    pub type TrackerConfiguration = crate::GlobalNamespace::OVRAnchor_TrackerConfiguration;
    #[cfg(feature = "OVRAnchor+__FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d")]
    pub type __FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d = crate::GlobalNamespace::OVRAnchor___FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d;
    pub fn CreateDeferredSpaceComponentStatusTask(
        space: u64,
        componentType: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
        enabledDesired: bool,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u64,
                        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                        bool,
                        f64,
                    ), crate::GlobalNamespace::OVRTask_1<bool>, 4usize>(
                        "CreateDeferredSpaceComponentStatusTask",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateDeferredSpaceComponentStatusTask",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = unsafe {
            cordl_method_info
                .invoke_unchecked((), (space, componentType, enabledDesired, timeout))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSpatialAnchorAsync_Pose0(
        trackingSpacePose: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::GlobalNamespace::OVRAnchor>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Pose),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::GlobalNamespace::OVRAnchor,
                        >,
                        1usize,
                    >("CreateSpatialAnchorAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateSpatialAnchorAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<crate::GlobalNamespace::OVRAnchor> =
            unsafe { cordl_method_info.invoke_unchecked((), (trackingSpacePose))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSpatialAnchorAsync_Transform_Camera1(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        centerEyeCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::GlobalNamespace::OVRAnchor>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        ),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::GlobalNamespace::OVRAnchor,
                        >,
                        2usize,
                    >("CreateSpatialAnchorAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateSpatialAnchorAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<crate::GlobalNamespace::OVRAnchor> =
            unsafe { cordl_method_info.invoke_unchecked((), (transform, centerEyeCamera))? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_OVRAnchor0(
        &mut self,
        other: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::GlobalNamespace::OVRAnchor), bool, 1usize>("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn EraseAsync_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_EraseResult>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::GlobalNamespace::OVRTask_1<
                        crate::GlobalNamespace::OVRResult_1<
                            crate::GlobalNamespace::OVRAnchor_EraseResult,
                        >,
                    >, 0usize>("EraseAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EraseAsync",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_EraseResult>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn EraseAsync_IEnumerable_1_IEnumerable_1_1(
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::GlobalNamespace::OVRAnchor>,
        >,
        uuids: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_EraseResult>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                crate::GlobalNamespace::OVRAnchor,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
                        >,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::GlobalNamespace::OVRResult_1<
                            crate::GlobalNamespace::OVRAnchor_EraseResult,
                        >,
                    >, 2usize>("EraseAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EraseAsync",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_EraseResult>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (anchors, uuids))? };
        Ok(__cordl_ret.into())
    }
    pub fn EraseSpace(
        space: u64,
        location: crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u64,
                        crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
                        quest_hook::libil2cpp::ByRefMut<u64>,
                    ), crate::GlobalNamespace::OVRPlugin_Result, 3usize>(
                        "EraseSpace"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EraseSpace",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result =
            unsafe { cordl_method_info.invoke_unchecked((), (space, location, requestId))? };
        Ok(__cordl_ret.into())
    }
    pub fn EraseSpacesAsync(
        spaces: crate::System::ReadOnlySpan_1<u64>,
        uuids: crate::System::ReadOnlySpan_1<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_EraseResult>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<u64>,
                        crate::System::ReadOnlySpan_1<crate::System::Guid>,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::GlobalNamespace::OVRResult_1<
                            crate::GlobalNamespace::OVRAnchor_EraseResult,
                        >,
                    >, 2usize>("EraseSpacesAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EraseSpacesAsync",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_EraseResult>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (spaces, uuids))? };
        Ok(__cordl_ret.into())
    }
    pub fn FetchAnchors(
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::GlobalNamespace::OVRAnchor>,
        >,
        queryInfo: crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::GlobalNamespace::OVRPlugin_Result>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    crate::GlobalNamespace::OVRAnchor,
                                >,
                            >,
                            crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2,
                        ),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::GlobalNamespace::OVRPlugin_Result,
                        >,
                        2usize,
                    >("FetchAnchors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FetchAnchors", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRPlugin_Result,
        > = unsafe { cordl_method_info.invoke_unchecked((), (anchors, queryInfo))? };
        Ok(__cordl_ret.into())
    }
    pub fn FetchAnchorsAsync_IEnumerable_1_IList_1_OVRSpace_StorageLocation_f64_2(
        uuids: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
        >,
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::GlobalNamespace::OVRAnchor>,
        >,
        location: crate::GlobalNamespace::OVRSpace_StorageLocation,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IList_1<
                                crate::GlobalNamespace::OVRAnchor,
                            >,
                        >,
                        crate::GlobalNamespace::OVRSpace_StorageLocation,
                        f64,
                    ), crate::GlobalNamespace::OVRTask_1<bool>, 4usize>(
                        "FetchAnchorsAsync"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FetchAnchorsAsync",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> =
            unsafe { cordl_method_info.invoke_unchecked((), (uuids, anchors, location, timeout))? };
        Ok(__cordl_ret.into())
    }
    pub fn FetchAnchorsAsync_IList_1_OVRSpace_StorageLocation_i32_f64_1<T>(
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::GlobalNamespace::OVRAnchor>,
        >,
        location: crate::GlobalNamespace::OVRSpace_StorageLocation,
        maxResults: i32,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IList_1<
                                crate::GlobalNamespace::OVRAnchor,
                            >,
                        >,
                        crate::GlobalNamespace::OVRSpace_StorageLocation,
                        i32,
                        f64,
                    ), crate::GlobalNamespace::OVRTask_1<bool>, 4usize>(
                        "FetchAnchorsAsync"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FetchAnchorsAsync",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = unsafe {
            cordl_method_info.invoke_unchecked((), (anchors, location, maxResults, timeout))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FetchAnchorsAsync_List_1_OVRAnchor_FetchOptions_Action_2_0(
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
        >,
        options: crate::GlobalNamespace::OVRAnchor_FetchOptions,
        incrementalResultsCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
                >,
                i32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
                >,
                crate::GlobalNamespace::OVRAnchor_FetchResult,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::GlobalNamespace::OVRAnchor,
                            >,
                        >,
                        crate::GlobalNamespace::OVRAnchor_FetchOptions,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::List_1<
                                        crate::GlobalNamespace::OVRAnchor,
                                    >,
                                >,
                                i32,
                            >,
                        >,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::GlobalNamespace::OVRResult_2<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::GlobalNamespace::OVRAnchor,
                                >,
                            >,
                            crate::GlobalNamespace::OVRAnchor_FetchResult,
                        >,
                    >, 3usize>("FetchAnchorsAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FetchAnchorsAsync",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
                >,
                crate::GlobalNamespace::OVRAnchor_FetchResult,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (anchors, options, incrementalResultsCallback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FetchAnchorsAsync_OVRPlugin_SpaceComponentType_IList_1_OVRSpace_StorageLocation_i32_f64_3(
        _cordl_type: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::GlobalNamespace::OVRAnchor>,
        >,
        location: crate::GlobalNamespace::OVRSpace_StorageLocation,
        maxResults: i32,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IList_1<
                                crate::GlobalNamespace::OVRAnchor,
                            >,
                        >,
                        crate::GlobalNamespace::OVRSpace_StorageLocation,
                        i32,
                        f64,
                    ), crate::GlobalNamespace::OVRTask_1<bool>, 5usize>(
                        "FetchAnchorsAsync"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FetchAnchorsAsync",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_cordl_type, anchors, location, maxResults, timeout))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FetchSharedAnchorsAsync_IEnumerable_1_List_1_1(
        groupUuid: crate::System::Guid,
        allowedAnchorUuids: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
        >,
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
                >,
                crate::GlobalNamespace::OVRAnchor_FetchResult,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::Guid,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::GlobalNamespace::OVRAnchor,
                            >,
                        >,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::GlobalNamespace::OVRResult_2<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::GlobalNamespace::OVRAnchor,
                                >,
                            >,
                            crate::GlobalNamespace::OVRAnchor_FetchResult,
                        >,
                    >, 3usize>("FetchSharedAnchorsAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FetchSharedAnchorsAsync",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
                >,
                crate::GlobalNamespace::OVRAnchor_FetchResult,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (groupUuid, allowedAnchorUuids, anchors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FetchSharedAnchorsAsync_List_1_0(
        groupUuid: crate::System::Guid,
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
                >,
                crate::GlobalNamespace::OVRAnchor_FetchResult,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::Guid,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::GlobalNamespace::OVRAnchor,
                            >,
                        >,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::GlobalNamespace::OVRResult_2<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::GlobalNamespace::OVRAnchor,
                                >,
                            >,
                            crate::GlobalNamespace::OVRAnchor_FetchResult,
                        >,
                    >, 2usize>("FetchSharedAnchorsAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FetchSharedAnchorsAsync",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
                >,
                crate::GlobalNamespace::OVRAnchor_FetchResult,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (groupUuid, anchors))? };
        Ok(__cordl_ret.into())
    }
    pub fn FetchTrackablesAsync(
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
        >,
        trackableTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::GlobalNamespace::OVRAnchor_TrackableType,
            >,
        >,
        incrementalResultsCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
                >,
                i32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
                >,
                crate::GlobalNamespace::OVRAnchor_FetchResult,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::GlobalNamespace::OVRAnchor,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                crate::GlobalNamespace::OVRAnchor_TrackableType,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::List_1<
                                        crate::GlobalNamespace::OVRAnchor,
                                    >,
                                >,
                                i32,
                            >,
                        >,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::GlobalNamespace::OVRResult_2<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::GlobalNamespace::OVRAnchor,
                                >,
                            >,
                            crate::GlobalNamespace::OVRAnchor_FetchResult,
                        >,
                    >, 3usize>("FetchTrackablesAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FetchTrackablesAsync",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
                >,
                crate::GlobalNamespace::OVRAnchor_FetchResult,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (anchors, trackableTypes, incrementalResultsCallback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetComponent<T>(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), T, 0usize>("GetComponent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetComponent",
                            0usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHashCode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRequiredComponents(
        trackableTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::GlobalNamespace::OVRAnchor_TrackableType,
            >,
        >,
        trackableTypesOut: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                crate::GlobalNamespace::OVRAnchor_TrackableType,
            >,
        >,
        requiredComponentsOut: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                crate::GlobalNamespace::OVRAnchor_TrackableType,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::HashSet_1<
                                crate::GlobalNamespace::OVRAnchor_TrackableType,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::HashSet_1<
                                crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "GetRequiredComponents"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetRequiredComponents",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (trackableTypes, trackableTypesOut, requiredComponentsOut),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSupportedComponents(
        &mut self,
        components: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                        >,
                    >), bool, 1usize>("GetSupportedComponents")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetSupportedComponents",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (components))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackableType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRAnchor_TrackableType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::GlobalNamespace::OVRAnchor_TrackableType, 0usize>(
                        "GetTrackableType",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTrackableType",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRAnchor_TrackableType =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Init() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Init",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OnEraseSpacesResult(
        eventData: crate::GlobalNamespace::OVRDeserialize_SpacesEraseResultData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRDeserialize_SpacesEraseResultData),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnEraseSpacesResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnEraseSpacesResult", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnSaveSpacesResult(
        eventData: crate::GlobalNamespace::OVRDeserialize_SpacesSaveResultData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRDeserialize_SpacesSaveResultData),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnSaveSpacesResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnSaveSpacesResult", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnShareAnchorsToGroupsComplete(
        requestId: u64,
        result: crate::GlobalNamespace::OVRPlugin_Result,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u64, crate::GlobalNamespace::OVRPlugin_Result),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("OnShareAnchorsToGroupsComplete")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnShareAnchorsToGroupsComplete", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (requestId, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnSpaceDiscoveryComplete(
        data: crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryCompleteData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryCompleteData),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnSpaceDiscoveryComplete")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnSpaceDiscoveryComplete", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (data))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnSpaceDiscoveryResultsAvailable(
        data: crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryResultsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRDeserialize_SpaceDiscoveryResultsData),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnSpaceDiscoveryResultsAvailable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnSpaceDiscoveryResultsAvailable", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (data))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnSpaceEraseComplete(
        eventData: crate::GlobalNamespace::OVRDeserialize_SpaceEraseCompleteData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRDeserialize_SpaceEraseCompleteData),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnSpaceEraseComplete")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnSpaceEraseComplete", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnSpaceListSaveResult(
        eventData: crate::GlobalNamespace::OVRDeserialize_SpaceListSaveResultData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRDeserialize_SpaceListSaveResultData),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnSpaceListSaveResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnSpaceListSaveResult", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnSpaceQueryComplete(
        data: crate::GlobalNamespace::OVRDeserialize_SpaceQueryCompleteData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRDeserialize_SpaceQueryCompleteData),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnSpaceQueryComplete")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnSpaceQueryComplete", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (data))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnSpaceSetComponentStatusComplete(
        eventData: crate::GlobalNamespace::OVRDeserialize_SpaceSetComponentStatusCompleteData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRDeserialize_SpaceSetComponentStatusCompleteData),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnSpaceSetComponentStatusComplete")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnSpaceSetComponentStatusComplete", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn SaveAsync_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_SaveResult>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::GlobalNamespace::OVRTask_1<
                        crate::GlobalNamespace::OVRResult_1<
                            crate::GlobalNamespace::OVRAnchor_SaveResult,
                        >,
                    >, 0usize>("SaveAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SaveAsync",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_SaveResult>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SaveAsync_IEnumerable_1_1(
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::GlobalNamespace::OVRAnchor>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_SaveResult>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            crate::GlobalNamespace::OVRAnchor,
                        >,
                    >), crate::GlobalNamespace::OVRTask_1<
                        crate::GlobalNamespace::OVRResult_1<
                            crate::GlobalNamespace::OVRAnchor_SaveResult,
                        >,
                    >, 1usize>("SaveAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SaveAsync",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_SaveResult>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (anchors))? };
        Ok(__cordl_ret.into())
    }
    pub fn SaveSpaceList(
        spaces: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        numSpaces: u32,
        location: crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        u32,
                        crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
                        quest_hook::libil2cpp::ByRefMut<u64>,
                    ), crate::GlobalNamespace::OVRPlugin_Result, 4usize>(
                        "SaveSpaceList"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SaveSpaceList",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = unsafe {
            cordl_method_info.invoke_unchecked((), (spaces, numSpaces, location, requestId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SaveSpacesAsync(
        spaces: crate::System::ReadOnlySpan_1<u64>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_SaveResult>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::ReadOnlySpan_1<u64>),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::GlobalNamespace::OVRResult_1<
                                crate::GlobalNamespace::OVRAnchor_SaveResult,
                            >,
                        >,
                        1usize,
                    >("SaveSpacesAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SaveSpacesAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_SaveResult>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (spaces))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShareAsyncInternal(
        anchors: crate::System::ReadOnlySpan_1<u64>,
        groupUuids: crate::System::ReadOnlySpan_1<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_ShareResult>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<u64>,
                        crate::System::ReadOnlySpan_1<crate::System::Guid>,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::GlobalNamespace::OVRResult_1<
                            crate::GlobalNamespace::OVRAnchor_ShareResult,
                        >,
                    >, 2usize>("ShareAsyncInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ShareAsyncInternal",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_ShareResult>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (anchors, groupUuids))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShareAsync_Guid2(
        &mut self,
        groupUuid: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_ShareResult>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::System::Guid), crate::GlobalNamespace::OVRTask_1<
                        crate::GlobalNamespace::OVRResult_1<
                            crate::GlobalNamespace::OVRAnchor_ShareResult,
                        >,
                    >, 1usize>("ShareAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ShareAsync",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_ShareResult>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (groupUuid))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShareAsync_IEnumerable_1_0(
        &mut self,
        users: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::GlobalNamespace::OVRSpaceUser,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_ShareResult>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            crate::GlobalNamespace::OVRSpaceUser,
                        >,
                    >), crate::GlobalNamespace::OVRTask_1<
                        crate::GlobalNamespace::OVRResult_1<
                            crate::GlobalNamespace::OVRAnchor_ShareResult,
                        >,
                    >, 1usize>("ShareAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ShareAsync",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_ShareResult>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (users))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShareAsync_IEnumerable_1_Guid3(
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::GlobalNamespace::OVRAnchor>,
        >,
        groupUuid: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_ShareResult>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                crate::GlobalNamespace::OVRAnchor,
                            >,
                        >,
                        crate::System::Guid,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::GlobalNamespace::OVRResult_1<
                            crate::GlobalNamespace::OVRAnchor_ShareResult,
                        >,
                    >, 2usize>("ShareAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ShareAsync",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_ShareResult>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (anchors, groupUuid))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShareAsync_IEnumerable_1_IEnumerable_1_1(
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::GlobalNamespace::OVRAnchor>,
        >,
        users: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::GlobalNamespace::OVRSpaceUser,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_ShareResult>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                crate::GlobalNamespace::OVRAnchor,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                crate::GlobalNamespace::OVRSpaceUser,
                            >,
                        >,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::GlobalNamespace::OVRResult_1<
                            crate::GlobalNamespace::OVRAnchor_ShareResult,
                        >,
                    >, 2usize>("ShareAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ShareAsync",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_ShareResult>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (anchors, users))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShareSpacesAsync(
        spaces: crate::System::ReadOnlySpan_1<u64>,
        users: crate::System::ReadOnlySpan_1<u64>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_ShareResult>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<u64>,
                        crate::System::ReadOnlySpan_1<u64>,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::GlobalNamespace::OVRResult_1<
                            crate::GlobalNamespace::OVRAnchor_ShareResult,
                        >,
                    >, 2usize>("ShareSpacesAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ShareSpacesAsync",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_ShareResult>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (spaces, users))? };
        Ok(__cordl_ret.into())
    }
    pub fn SupportsComponent<T>(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("SupportsComponent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SupportsComponent",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetComponent<T>(
        &mut self,
        component: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<T>), bool, 1usize>(
                        "TryGetComponent",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetComponent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (component))? };
        Ok(__cordl_ret.into())
    }
    pub fn _FetchTrackablesAsync_g__DoesComponentMatchTrackableType_66_1(
        trackableTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                crate::GlobalNamespace::OVRAnchor_TrackableType,
            >,
        >,
        anchor: crate::GlobalNamespace::OVRAnchor,
        componentType: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::HashSet_1<
                                crate::GlobalNamespace::OVRAnchor_TrackableType,
                            >,
                        >,
                        crate::GlobalNamespace::OVRAnchor,
                        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                    ), bool, 3usize>(
                        "<FetchTrackablesAsync>g__DoesComponentMatchTrackableType|66_1",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "<FetchTrackablesAsync>g__DoesComponentMatchTrackableType|66_1",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (trackableTypes, anchor, componentType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0(
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
        >,
        trackableTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                crate::GlobalNamespace::OVRAnchor_TrackableType,
            >,
        >,
        componentType: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
        incrementalResultsCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
                >,
                i32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::GlobalNamespace::OVRPlugin_Result>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::GlobalNamespace::OVRAnchor,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::HashSet_1<
                                    crate::GlobalNamespace::OVRAnchor_TrackableType,
                                >,
                            >,
                            crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_2<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Collections::Generic::List_1<
                                            crate::GlobalNamespace::OVRAnchor,
                                        >,
                                    >,
                                    i32,
                                >,
                            >,
                        ),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::GlobalNamespace::OVRPlugin_Result,
                        >,
                        4usize,
                    >("<FetchTrackablesAsync>g__QuerySingleComponentAsync|66_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<FetchTrackablesAsync>g__QuerySingleComponentAsync|66_0",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRPlugin_Result,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    anchors,
                    trackableTypes,
                    componentType,
                    incrementalResultsCallback,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        handle: u64,
        uuid: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u64, crate::System::Guid), quest_hook::libil2cpp::Void, 2usize>(
                        ".ctor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (handle, uuid))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Handle(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u64, 0usize>("get_Handle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Handle",
                            0usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Uuid(&mut self) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::System::Guid, 0usize>("get_Uuid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Uuid",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Guid =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::GlobalNamespace::OVRAnchor,
        rhs: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRAnchor,
                        crate::GlobalNamespace::OVRAnchor,
                    ), bool, 2usize>("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Equality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (lhs, rhs))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::GlobalNamespace::OVRAnchor,
        rhs: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRAnchor,
                        crate::GlobalNamespace::OVRAnchor,
                    ), bool, 2usize>("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Inequality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (lhs, rhs))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRAnchor")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::OVRAnchor {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "OVRAnchor")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::OVRAnchor {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "OVRAnchor")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor>>
    for crate::GlobalNamespace::OVRAnchor
{
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor> {
        todo!()
    }
}
#[cfg(feature = "OVRAnchor")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor>>
    for crate::GlobalNamespace::OVRAnchor
{
    fn as_mut(&mut self) -> &mut crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor> {
        todo!()
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+ConfigureTrackerResult")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum OVRAnchor_ConfigureTrackerResult {
    #[default]
    Failure = -1000i32,
    Invalid = -1008i32,
    NotSupported = -1004i32,
    Success = 0i32,
}
#[cfg(feature = "cordl_class_OVRAnchor+ConfigureTrackerResult")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::OVRAnchor_ConfigureTrackerResult
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/ConfigureTrackerResult";
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
#[cfg(feature = "cordl_class_OVRAnchor+ConfigureTrackerResult")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::OVRAnchor_ConfigureTrackerResult
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+ConfigureTrackerResult")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::OVRAnchor_ConfigureTrackerResult
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
#[cfg(feature = "cordl_class_OVRAnchor+ConfigureTrackerResult")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::OVRAnchor_ConfigureTrackerResult
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
#[cfg(feature = "cordl_class_OVRAnchor+ConfigureTrackerResult")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::OVRAnchor_ConfigureTrackerResult
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
#[cfg(feature = "cordl_class_OVRAnchor+DeferredKey")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OVRAnchor_DeferredKey {
    pub Space: u64,
    pub ComponentType: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
}
#[cfg(feature = "cordl_class_OVRAnchor+DeferredKey")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRAnchor_DeferredKey {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/DeferredKey";
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
#[cfg(feature = "cordl_class_OVRAnchor+DeferredKey")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRAnchor_DeferredKey {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+DeferredKey")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRAnchor_DeferredKey {
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
#[cfg(feature = "cordl_class_OVRAnchor+DeferredKey")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRAnchor_DeferredKey {
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
#[cfg(feature = "cordl_class_OVRAnchor+DeferredKey")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRAnchor_DeferredKey {
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
#[cfg(feature = "cordl_class_OVRAnchor+DeferredKey")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::OVRAnchor_DeferredKey {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRAnchor+DeferredKey")]
impl crate::GlobalNamespace::OVRAnchor_DeferredKey {
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_OVRAnchor_DeferredKey0(
        &mut self,
        other: crate::GlobalNamespace::OVRAnchor_DeferredKey,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::GlobalNamespace::OVRAnchor_DeferredKey), bool, 1usize>(
                        "Equals",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromEvent(
        eventData: crate::GlobalNamespace::OVRDeserialize_SpaceSetComponentStatusCompleteData,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRAnchor_DeferredKey> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRDeserialize_SpaceSetComponentStatusCompleteData),
                        crate::GlobalNamespace::OVRAnchor_DeferredKey,
                        1usize,
                    >("FromEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRAnchor_DeferredKey =
            unsafe { cordl_method_info.invoke_unchecked((), (eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHashCode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRAnchor+DeferredKey")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor_DeferredKey>>
    for crate::GlobalNamespace::OVRAnchor_DeferredKey
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor_DeferredKey> {
        todo!()
    }
}
#[cfg(feature = "OVRAnchor+DeferredKey")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor_DeferredKey>>
    for crate::GlobalNamespace::OVRAnchor_DeferredKey
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor_DeferredKey> {
        todo!()
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+DeferredValue")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OVRAnchor_DeferredValue {
    pub Task: crate::GlobalNamespace::OVRTask_1<bool>,
    pub EnabledDesired: bool,
    pub RequestId: u64,
    pub Timeout: f64,
    pub StartTime: f32,
}
#[cfg(feature = "cordl_class_OVRAnchor+DeferredValue")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRAnchor_DeferredValue {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/DeferredValue";
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
#[cfg(feature = "cordl_class_OVRAnchor+DeferredValue")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRAnchor_DeferredValue {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+DeferredValue")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRAnchor_DeferredValue {
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
#[cfg(feature = "cordl_class_OVRAnchor+DeferredValue")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRAnchor_DeferredValue {
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
#[cfg(feature = "cordl_class_OVRAnchor+DeferredValue")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRAnchor_DeferredValue {
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
#[cfg(feature = "cordl_class_OVRAnchor+DeferredValue")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::OVRAnchor_DeferredValue
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRAnchor+DeferredValue")]
impl crate::GlobalNamespace::OVRAnchor_DeferredValue {}
#[cfg(feature = "cordl_class_OVRAnchor+EraseResult")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum OVRAnchor_EraseResult {
    #[default]
    Failure = -1000i32,
    FailureDataIsInvalid = -1008i32,
    FailureInsufficientResources = -9000i32,
    FailureInvalidAnchor = -1013i32,
    FailurePermissionInsufficient = -9003i32,
    FailurePersistenceNotEnabled = -2006i32,
    FailureRateLimited = -9004i32,
    FailureUnsupported = -1004i32,
    Success = 0i32,
}
#[cfg(feature = "cordl_class_OVRAnchor+EraseResult")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRAnchor_EraseResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/EraseResult";
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
#[cfg(feature = "cordl_class_OVRAnchor+EraseResult")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRAnchor_EraseResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+EraseResult")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRAnchor_EraseResult {
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
#[cfg(feature = "cordl_class_OVRAnchor+EraseResult")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRAnchor_EraseResult {
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
#[cfg(feature = "cordl_class_OVRAnchor+EraseResult")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRAnchor_EraseResult {
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
#[cfg(feature = "cordl_class_OVRAnchor+FetchOptions")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OVRAnchor_FetchOptions {
    pub SingleUuid: crate::System::Nullable_1<crate::System::Guid>,
    pub Uuids: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
    >,
    pub SingleComponentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub ComponentTypes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IEnumerable_1<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    >,
}
#[cfg(feature = "cordl_class_OVRAnchor+FetchOptions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRAnchor_FetchOptions {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/FetchOptions";
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
#[cfg(feature = "cordl_class_OVRAnchor+FetchOptions")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRAnchor_FetchOptions {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+FetchOptions")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRAnchor_FetchOptions {
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
#[cfg(feature = "cordl_class_OVRAnchor+FetchOptions")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRAnchor_FetchOptions {
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
#[cfg(feature = "cordl_class_OVRAnchor+FetchOptions")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRAnchor_FetchOptions {
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
#[cfg(feature = "cordl_class_OVRAnchor+FetchOptions")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::OVRAnchor_FetchOptions {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRAnchor+FetchOptions")]
impl crate::GlobalNamespace::OVRAnchor_FetchOptions {
    pub fn DiscoverSpaces(
        &mut self,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<u64>),
                        crate::GlobalNamespace::OVRPlugin_Result,
                        1usize,
                    >("DiscoverSpaces")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DiscoverSpaces", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result =
            unsafe { cordl_method_info.invoke_unchecked(self, (requestId))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceComponentType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_SpaceComponentType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                        1usize,
                    >("GetSpaceComponentType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSpaceComponentType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType =
            unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+FetchResult")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum OVRAnchor_FetchResult {
    #[default]
    Failure = -1000i32,
    FailureDataIsInvalid = -1008i32,
    FailureInsufficientResources = -9000i32,
    FailureInsufficientView = -9002i32,
    FailureInvalidOption = -1001i32,
    FailurePermissionInsufficient = -9003i32,
    FailureRateLimited = -9004i32,
    FailureTooBright = -9006i32,
    FailureTooDark = -9005i32,
    FailureUnsupported = -1004i32,
    Success = 0i32,
}
#[cfg(feature = "cordl_class_OVRAnchor+FetchResult")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRAnchor_FetchResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/FetchResult";
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
#[cfg(feature = "cordl_class_OVRAnchor+FetchResult")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRAnchor_FetchResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+FetchResult")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRAnchor_FetchResult {
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
#[cfg(feature = "cordl_class_OVRAnchor+FetchResult")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRAnchor_FetchResult {
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
#[cfg(feature = "cordl_class_OVRAnchor+FetchResult")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRAnchor_FetchResult {
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
#[cfg(feature = "cordl_class_OVRAnchor+FetchTaskData")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OVRAnchor_FetchTaskData {
    pub Anchors: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
    >,
    pub IncrementalResultsCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
            >,
            i32,
        >,
    >,
}
#[cfg(feature = "cordl_class_OVRAnchor+FetchTaskData")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRAnchor_FetchTaskData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/FetchTaskData";
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
#[cfg(feature = "cordl_class_OVRAnchor+FetchTaskData")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRAnchor_FetchTaskData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+FetchTaskData")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRAnchor_FetchTaskData {
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
#[cfg(feature = "cordl_class_OVRAnchor+FetchTaskData")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRAnchor_FetchTaskData {
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
#[cfg(feature = "cordl_class_OVRAnchor+FetchTaskData")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRAnchor_FetchTaskData {
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
#[cfg(feature = "cordl_class_OVRAnchor+FetchTaskData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::OVRAnchor_FetchTaskData
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRAnchor+FetchTaskData")]
impl crate::GlobalNamespace::OVRAnchor_FetchTaskData {}
#[cfg(feature = "cordl_class_OVRAnchor+FilterUnion")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OVRAnchor_FilterUnion {
    padding: quest_hook::libil2cpp::ValueTypePadding<16usize>,
}
#[cfg(feature = "cordl_class_OVRAnchor+FilterUnion")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRAnchor_FilterUnion {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/FilterUnion";
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
#[cfg(feature = "cordl_class_OVRAnchor+FilterUnion")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRAnchor_FilterUnion {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+FilterUnion")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRAnchor_FilterUnion {
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
#[cfg(feature = "cordl_class_OVRAnchor+FilterUnion")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRAnchor_FilterUnion {
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
#[cfg(feature = "cordl_class_OVRAnchor+FilterUnion")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRAnchor_FilterUnion {
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
#[cfg(feature = "cordl_class_OVRAnchor+FilterUnion")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::OVRAnchor_FilterUnion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRAnchor+FilterUnion")]
impl crate::GlobalNamespace::OVRAnchor_FilterUnion {}
#[cfg(feature = "cordl_class_OVRAnchor+SaveResult")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum OVRAnchor_SaveResult {
    #[default]
    Failure = -1000i32,
    FailureDataIsInvalid = -1008i32,
    FailureInsufficientResources = -9000i32,
    FailureInsufficientView = -9002i32,
    FailureInvalidAnchor = -1013i32,
    FailurePermissionInsufficient = -9003i32,
    FailurePersistenceNotEnabled = -2006i32,
    FailureRateLimited = -9004i32,
    FailureStorageAtCapacity = -9001i32,
    FailureTooBright = -9006i32,
    FailureTooDark = -9005i32,
    FailureUnsupported = -1004i32,
    Success = 0i32,
}
#[cfg(feature = "cordl_class_OVRAnchor+SaveResult")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRAnchor_SaveResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/SaveResult";
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
#[cfg(feature = "cordl_class_OVRAnchor+SaveResult")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRAnchor_SaveResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+SaveResult")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRAnchor_SaveResult {
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
#[cfg(feature = "cordl_class_OVRAnchor+SaveResult")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRAnchor_SaveResult {
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
#[cfg(feature = "cordl_class_OVRAnchor+SaveResult")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRAnchor_SaveResult {
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
#[cfg(feature = "cordl_class_OVRAnchor+ShareResult")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum OVRAnchor_ShareResult {
    #[default]
    Failure = -1000i32,
    FailureCloudStorageDisabled = -2000i32,
    FailureDataIsInvalid = -1008i32,
    FailureHandleInvalid = -1013i32,
    FailureInvalidParameter = -1001i32,
    FailureLocalizationFailed = -2002i32,
    FailureMappingInsufficient = -2001i32,
    FailureNetworkRequestFailed = -2004i32,
    FailureNetworkTimeout = -2003i32,
    FailureOperationFailed = -1006i32,
    FailurePermissionInsufficient = -9003i32,
    FailureSharableComponentNotEnabled = -2006i32,
    FailureUnsupported = -1004i32,
    Success = 0i32,
}
#[cfg(feature = "cordl_class_OVRAnchor+ShareResult")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRAnchor_ShareResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/ShareResult";
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
#[cfg(feature = "cordl_class_OVRAnchor+ShareResult")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRAnchor_ShareResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+ShareResult")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRAnchor_ShareResult {
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
#[cfg(feature = "cordl_class_OVRAnchor+ShareResult")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRAnchor_ShareResult {
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
#[cfg(feature = "cordl_class_OVRAnchor+ShareResult")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRAnchor_ShareResult {
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
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRAnchor_Telemetry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRAnchor_Telemetry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/Telemetry";
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
#[cfg(feature = "OVRAnchor+Telemetry")]
impl std::ops::Deref for crate::GlobalNamespace::OVRAnchor_Telemetry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRAnchor+Telemetry")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRAnchor_Telemetry {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRAnchor+Telemetry")]
impl crate::GlobalNamespace::OVRAnchor_Telemetry {
    #[cfg(feature = "OVRAnchor+Telemetry+Annotation")]
    pub type Annotation = crate::GlobalNamespace::Telemetry_OVRAnchor_Annotation;
    #[cfg(feature = "OVRAnchor+Telemetry+Key")]
    pub type Key = crate::GlobalNamespace::Telemetry_OVRAnchor_Key;
    #[cfg(feature = "OVRAnchor+Telemetry+MarkerId")]
    pub type MarkerId = crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId;
    pub fn AddMarker(
        requestId: u64,
        marker: crate::GlobalNamespace::OVRTelemetryMarker,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u64, crate::GlobalNamespace::OVRTelemetryMarker),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("AddMarker")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddMarker", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (requestId, marker))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMarker(
        markerId: crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId,
        requestId: u64,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::GlobalNamespace::OVRTelemetryMarker>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId, u64),
                        crate::System::Nullable_1<
                            crate::GlobalNamespace::OVRTelemetryMarker,
                        >,
                        2usize,
                    >("GetMarker")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMarker", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<crate::GlobalNamespace::OVRTelemetryMarker> =
            unsafe { cordl_method_info.invoke_unchecked((), (markerId, requestId))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRemove(
        markerId: crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId,
        requestId: u64,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::GlobalNamespace::OVRTelemetryMarker>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId, u64),
                        crate::System::Nullable_1<
                            crate::GlobalNamespace::OVRTelemetryMarker,
                        >,
                        2usize,
                    >("GetRemove")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRemove", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<crate::GlobalNamespace::OVRTelemetryMarker> =
            unsafe { cordl_method_info.invoke_unchecked((), (markerId, requestId))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnInit() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnInit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnInit",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Remove(
        markerId: crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId,
        requestId: u64,
        marker: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRTelemetryMarker>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId,
                        u64,
                        quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRTelemetryMarker>,
                    ), bool, 3usize>("Remove")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Remove",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (markerId, requestId, marker))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetAsyncResult(
        markerId: crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId,
        requestId: u64,
        result: i64,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::GlobalNamespace::OVRTelemetryMarker>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId, u64, i64),
                        crate::System::Nullable_1<
                            crate::GlobalNamespace::OVRTelemetryMarker,
                        >,
                        3usize,
                    >("SetAsyncResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetAsyncResult", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<crate::GlobalNamespace::OVRTelemetryMarker> =
            unsafe { cordl_method_info.invoke_unchecked((), (markerId, requestId, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetAsyncResultAndSend(
        markerId: crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId,
        requestId: u64,
        result: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId,
                        u64,
                        i64,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetAsyncResultAndSend"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetAsyncResultAndSend",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (markerId, requestId, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetSyncResult(
        marker: crate::GlobalNamespace::OVRTelemetryMarker,
        requestId: u64,
        result: crate::GlobalNamespace::OVRPlugin_Result,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRTelemetryMarker,
                        u64,
                        crate::GlobalNamespace::OVRPlugin_Result,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetSyncResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetSyncResult",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (marker, requestId, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        markerId: crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId,
        requestId: u64,
        result: crate::GlobalNamespace::OVRPlugin_Result,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTelemetryMarker> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId,
                        u64,
                        crate::GlobalNamespace::OVRPlugin_Result,
                    ), crate::GlobalNamespace::OVRTelemetryMarker, 3usize>(
                        "Start"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Start",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTelemetryMarker =
            unsafe { cordl_method_info.invoke_unchecked((), (markerId, requestId, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetMarker(
        markerId: crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId,
        requestId: u64,
        marker: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRTelemetryMarker>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId,
                        u64,
                        quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRTelemetryMarker>,
                    ), bool, 3usize>("TryGetMarker")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetMarker",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (markerId, requestId, marker))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRAnchor_Telemetry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+TrackableType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum OVRAnchor_TrackableType {
    #[default]
    Keyboard = 1i32,
    None = 0i32,
}
#[cfg(feature = "cordl_class_OVRAnchor+TrackableType")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRAnchor_TrackableType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/TrackableType";
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
#[cfg(feature = "cordl_class_OVRAnchor+TrackableType")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRAnchor_TrackableType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+TrackableType")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRAnchor_TrackableType {
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
#[cfg(feature = "cordl_class_OVRAnchor+TrackableType")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRAnchor_TrackableType {
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
#[cfg(feature = "cordl_class_OVRAnchor+TrackableType")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRAnchor_TrackableType {
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
#[cfg(feature = "cordl_class_OVRAnchor+Tracker")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRAnchor_Tracker {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _configuration: crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
    pub _asyncOperationCount: i32,
    pub _dynamicObjectTracker: u64,
}
#[cfg(feature = "cordl_class_OVRAnchor+Tracker")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRAnchor_Tracker {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/Tracker";
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
#[cfg(feature = "OVRAnchor+Tracker")]
impl std::ops::Deref for crate::GlobalNamespace::OVRAnchor_Tracker {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRAnchor+Tracker")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRAnchor_Tracker {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRAnchor+Tracker")]
impl crate::GlobalNamespace::OVRAnchor_Tracker {
    #[cfg(feature = "OVRAnchor+Tracker+AsyncLock")]
    pub type AsyncLock = crate::GlobalNamespace::Tracker_OVRAnchor_AsyncLock;
    #[cfg(
        feature = "OVRAnchor+Tracker+__SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d"
    )]
    pub type __SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d = crate::GlobalNamespace::Tracker_OVRAnchor___SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d;
    pub fn ConfigureAsync(
        &mut self,
        configuration: crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<
                crate::GlobalNamespace::OVRAnchor_ConfigureTrackerResult,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRAnchor_TrackerConfiguration),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::GlobalNamespace::OVRResult_1<
                                crate::GlobalNamespace::OVRAnchor_ConfigureTrackerResult,
                            >,
                        >,
                        1usize,
                    >("ConfigureAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConfigureAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<
                crate::GlobalNamespace::OVRAnchor_ConfigureTrackerResult,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (configuration))? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn FetchTrackablesAsync(
        &mut self,
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
        >,
        incrementalResultsCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
                >,
                i32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
                >,
                crate::GlobalNamespace::OVRAnchor_FetchResult,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::GlobalNamespace::OVRAnchor,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::List_1<
                                        crate::GlobalNamespace::OVRAnchor,
                                    >,
                                >,
                                i32,
                            >,
                        >,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::GlobalNamespace::OVRResult_2<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::GlobalNamespace::OVRAnchor,
                                >,
                            >,
                            crate::GlobalNamespace::OVRAnchor_FetchResult,
                        >,
                    >, 2usize>("FetchTrackablesAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FetchTrackablesAsync",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
                >,
                crate::GlobalNamespace::OVRAnchor_FetchResult,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (anchors, incrementalResultsCallback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Finalize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Finalize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetupDynamicObjectTracker(
        &mut self,
        config: crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::GlobalNamespace::OVRPlugin_Result>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRAnchor_TrackerConfiguration),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::GlobalNamespace::OVRPlugin_Result,
                        >,
                        1usize,
                    >("SetupDynamicObjectTracker")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetupDynamicObjectTracker", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRPlugin_Result,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (config))? };
        Ok(__cordl_ret.into())
    }
    pub fn _SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1(
        tracker: u64,
        config: crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_2<u64, crate::GlobalNamespace::OVRPlugin_Result>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u64, crate::GlobalNamespace::OVRAnchor_TrackerConfiguration),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::GlobalNamespace::OVRResult_2<
                                u64,
                                crate::GlobalNamespace::OVRPlugin_Result,
                            >,
                        >,
                        2usize,
                    >("<SetupDynamicObjectTracker>g__CreateAndConfigureTrackerAsync|5_1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<SetupDynamicObjectTracker>g__CreateAndConfigureTrackerAsync|5_1",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_2<u64, crate::GlobalNamespace::OVRPlugin_Result>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (tracker, config))? };
        Ok(__cordl_ret.into())
    }
    pub fn _SetupDynamicObjectTracker_g__SetClassesAsync_5_0(
        tracker: u64,
        config: crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRPlugin_Result>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u64, crate::GlobalNamespace::OVRAnchor_TrackerConfiguration),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::GlobalNamespace::OVRResult_1<
                                crate::GlobalNamespace::OVRPlugin_Result,
                            >,
                        >,
                        2usize,
                    >("<SetupDynamicObjectTracker>g__SetClassesAsync|5_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<SetupDynamicObjectTracker>g__SetClassesAsync|5_0", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRPlugin_Result>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (tracker, config))? };
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
    pub fn get_Configuration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRAnchor_TrackerConfiguration> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
                        0usize,
                    >("get_Configuration")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Configuration", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRAnchor_TrackerConfiguration =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+Tracker")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRAnchor_Tracker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRAnchor+Tracker")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::OVRAnchor_Tracker {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRAnchor+Tracker")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::OVRAnchor_Tracker {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+TrackerConfiguration")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OVRAnchor_TrackerConfiguration {
    pub _KeyboardTrackingEnabled_k__BackingField: bool,
}
#[cfg(feature = "cordl_class_OVRAnchor+TrackerConfiguration")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRAnchor_TrackerConfiguration {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/TrackerConfiguration";
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
#[cfg(feature = "cordl_class_OVRAnchor+TrackerConfiguration")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::OVRAnchor_TrackerConfiguration
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+TrackerConfiguration")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::OVRAnchor_TrackerConfiguration
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
#[cfg(feature = "cordl_class_OVRAnchor+TrackerConfiguration")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::OVRAnchor_TrackerConfiguration
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
#[cfg(feature = "cordl_class_OVRAnchor+TrackerConfiguration")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::OVRAnchor_TrackerConfiguration
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
#[cfg(feature = "cordl_class_OVRAnchor+TrackerConfiguration")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::OVRAnchor_TrackerConfiguration
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRAnchor+TrackerConfiguration")]
impl crate::GlobalNamespace::OVRAnchor_TrackerConfiguration {
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_OVRAnchor_TrackerConfiguration0(
        &mut self,
        other: crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRAnchor_TrackerConfiguration),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHashCode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackableTypes(
        &mut self,
        trackableTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::OVRAnchor_TrackableType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::GlobalNamespace::OVRAnchor_TrackableType,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>("GetTrackableTypes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTrackableTypes",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (trackableTypes))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResetDynamicObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ResetDynamicObjects")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ResetDynamicObjects",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SetDynamicObjectState(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "SetDynamicObjectState"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetDynamicObjectState",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToDynamicObjectClasses(
        &mut self,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRNativeList_1<
            crate::GlobalNamespace::OVRPlugin_DynamicObjectClass,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::Allocator),
                        crate::GlobalNamespace::OVRNativeList_1<
                            crate::GlobalNamespace::OVRPlugin_DynamicObjectClass,
                        >,
                        1usize,
                    >("ToDynamicObjectClasses")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToDynamicObjectClasses", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRNativeList_1<
            crate::GlobalNamespace::OVRPlugin_DynamicObjectClass,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_KeyboardTrackingEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_KeyboardTrackingEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_KeyboardTrackingEnabled",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_KeyboardTrackingSupported() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_KeyboardTrackingSupported")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_KeyboardTrackingSupported",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_RequiresDynamicObjectTracker(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_RequiresDynamicObjectTracker")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_RequiresDynamicObjectTracker",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
        rhs: crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
                        crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
                    ), bool, 2usize>("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Equality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (lhs, rhs))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
        rhs: crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
                        crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
                    ), bool, 2usize>("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Inequality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (lhs, rhs))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_KeyboardTrackingEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_KeyboardTrackingEnabled",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_KeyboardTrackingEnabled",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRAnchor+TrackerConfiguration")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor_TrackerConfiguration>>
    for crate::GlobalNamespace::OVRAnchor_TrackerConfiguration
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor_TrackerConfiguration> {
        todo!()
    }
}
#[cfg(feature = "OVRAnchor+TrackerConfiguration")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor_TrackerConfiguration>>
    for crate::GlobalNamespace::OVRAnchor_TrackerConfiguration
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor_TrackerConfiguration>
    {
        todo!()
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+__FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OVRAnchor___FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d {
    pub __1__state: i32,
    pub __t__builder:
        crate::GlobalNamespace::OVRTaskBuilder_1<crate::GlobalNamespace::OVRPlugin_Result>,
    pub componentType: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    pub anchors: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
    >,
    pub trackableTypes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            crate::GlobalNamespace::OVRAnchor_TrackableType,
        >,
    >,
    pub incrementalResultsCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
            >,
            i32,
        >,
    >,
    pub _anchorsWithComponent_5__2: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
    >,
    pub __7__wrap2:
        crate::GlobalNamespace::OVRObjectPool_ListScope_1<crate::GlobalNamespace::OVRAnchor>,
    pub __u__1: crate::GlobalNamespace::OVRTask_1_Awaiter<crate::GlobalNamespace::OVRPlugin_Result>,
}
#[cfg(feature = "cordl_class_OVRAnchor+__FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::OVRAnchor___FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str =
        "OVRAnchor/<<FetchTrackablesAsync>g__QuerySingleComponentAsync|66_0>d";
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
#[cfg(feature = "cordl_class_OVRAnchor+__FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::OVRAnchor___FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+__FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::OVRAnchor___FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d
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
#[cfg(feature = "cordl_class_OVRAnchor+__FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::OVRAnchor___FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d
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
#[cfg(feature = "cordl_class_OVRAnchor+__FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::OVRAnchor___FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d
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
#[cfg(feature = "cordl_class_OVRAnchor+__FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::OVRAnchor___FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRAnchor+__FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d")]
impl crate::GlobalNamespace::OVRAnchor___FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d {
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MoveNext",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::CompilerServices::IAsyncStateMachine,
                    >), quest_hook::libil2cpp::Void, 1usize>("SetStateMachine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetStateMachine",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (stateMachine))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRAnchor+__FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d")]
impl AsRef<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
    for crate::GlobalNamespace::OVRAnchor___FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d
{
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(feature = "OVRAnchor+__FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d")]
impl AsMut<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
    for crate::GlobalNamespace::OVRAnchor___FetchTrackablesAsync_g__QuerySingleComponentAsync_66_0_d
{
    fn as_mut(&mut self) -> &mut crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry+Annotation")]
#[repr(C)]
#[derive(Debug)]
pub struct Telemetry_OVRAnchor_Annotation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry+Annotation")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::Telemetry_OVRAnchor_Annotation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/Telemetry/Annotation";
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
#[cfg(feature = "OVRAnchor+Telemetry+Annotation")]
impl std::ops::Deref for crate::GlobalNamespace::Telemetry_OVRAnchor_Annotation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRAnchor+Telemetry+Annotation")]
impl std::ops::DerefMut for crate::GlobalNamespace::Telemetry_OVRAnchor_Annotation {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRAnchor+Telemetry+Annotation")]
impl crate::GlobalNamespace::Telemetry_OVRAnchor_Annotation {
    pub const AsynchronousResult: &'static str = "async_result";
    pub const ComponentTypes: &'static str = "component_types";
    pub const DynamicObjectClasses: &'static str = "dynamic_object_classes";
    pub const GroupCount: &'static str = "group_count";
    pub const MaxResults: &'static str = "max_results";
    pub const ResultsCount: &'static str = "results_count";
    pub const SpaceCount: &'static str = "space_count";
    pub const StorageLocation: &'static str = "storage_location";
    pub const SynchronousResult: &'static str = "sync_result";
    pub const Timeout: &'static str = "timeout";
    pub const TotalFilterCount: &'static str = "total_filter_count";
    pub const UuidCount: &'static str = "uuid_count";
}
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry+Annotation")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Telemetry_OVRAnchor_Annotation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry+Key")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Telemetry_OVRAnchor_Key {
    pub _markerId: i32,
    pub _requestId: u64,
}
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry+Key")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::Telemetry_OVRAnchor_Key {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/Telemetry/Key";
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
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry+Key")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::Telemetry_OVRAnchor_Key {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry+Key")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::Telemetry_OVRAnchor_Key {
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
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry+Key")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::Telemetry_OVRAnchor_Key {
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
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry+Key")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::Telemetry_OVRAnchor_Key {
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
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry+Key")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::Telemetry_OVRAnchor_Key
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRAnchor+Telemetry+Key")]
impl crate::GlobalNamespace::Telemetry_OVRAnchor_Key {
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Telemetry_OVRAnchor_Key0(
        &mut self,
        other: crate::GlobalNamespace::Telemetry_OVRAnchor_Key,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::GlobalNamespace::Telemetry_OVRAnchor_Key), bool, 1usize>(
                        "Equals",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHashCode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_OVRTelemetryMarker1(
        &mut self,
        marker: crate::GlobalNamespace::OVRTelemetryMarker,
        requestId: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRTelemetryMarker, u64),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (marker, requestId))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Telemetry_OVRAnchor_MarkerId0(
        &mut self,
        markerId: crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId,
        requestId: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId, u64),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (markerId, requestId))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRAnchor+Telemetry+Key")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::Telemetry_OVRAnchor_Key>>
    for crate::GlobalNamespace::Telemetry_OVRAnchor_Key
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::GlobalNamespace::Telemetry_OVRAnchor_Key> {
        todo!()
    }
}
#[cfg(feature = "OVRAnchor+Telemetry+Key")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::Telemetry_OVRAnchor_Key>>
    for crate::GlobalNamespace::Telemetry_OVRAnchor_Key
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::GlobalNamespace::Telemetry_OVRAnchor_Key> {
        todo!()
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry+MarkerId")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum Telemetry_OVRAnchor_MarkerId {
    #[default]
    ConfigureTracker = 163068237i32,
    DiscoverSpaces = 163054959i32,
    EraseSingleSpace = 163062284i32,
    EraseSpaces = 163061838i32,
    QuerySpaces = 163069062i32,
    SaveSpaceList = 163065048i32,
    SaveSpaces = 163056974i32,
}
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry+MarkerId")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/Telemetry/MarkerId";
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
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry+MarkerId")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry+MarkerId")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId
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
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry+MarkerId")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId
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
#[cfg(feature = "cordl_class_OVRAnchor+Telemetry+MarkerId")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::Telemetry_OVRAnchor_MarkerId {
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
#[cfg(feature = "cordl_class_OVRAnchor+Tracker+AsyncLock")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Tracker_OVRAnchor_AsyncLock {
    pub _tracker: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRAnchor_Tracker>,
}
#[cfg(feature = "cordl_class_OVRAnchor+Tracker+AsyncLock")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::Tracker_OVRAnchor_AsyncLock {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/Tracker/AsyncLock";
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
#[cfg(feature = "cordl_class_OVRAnchor+Tracker+AsyncLock")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::Tracker_OVRAnchor_AsyncLock
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRAnchor+Tracker+AsyncLock")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::Tracker_OVRAnchor_AsyncLock
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
#[cfg(feature = "cordl_class_OVRAnchor+Tracker+AsyncLock")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::Tracker_OVRAnchor_AsyncLock
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
#[cfg(feature = "cordl_class_OVRAnchor+Tracker+AsyncLock")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::Tracker_OVRAnchor_AsyncLock {
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
#[cfg(feature = "cordl_class_OVRAnchor+Tracker+AsyncLock")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::Tracker_OVRAnchor_AsyncLock
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRAnchor+Tracker+AsyncLock")]
impl crate::GlobalNamespace::Tracker_OVRAnchor_AsyncLock {
    pub fn AcquireAsync(
        tracker: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRAnchor_Tracker>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::GlobalNamespace::Tracker_OVRAnchor_AsyncLock>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::OVRAnchor_Tracker,
                        >),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::GlobalNamespace::Tracker_OVRAnchor_AsyncLock,
                        >,
                        1usize,
                    >("AcquireAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AcquireAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::Tracker_OVRAnchor_AsyncLock,
        > = unsafe { cordl_method_info.invoke_unchecked((), (tracker))? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        tracker: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRAnchor_Tracker>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::OVRAnchor_Tracker,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (tracker))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRAnchor+Tracker+AsyncLock")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::Tracker_OVRAnchor_AsyncLock {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "OVRAnchor+Tracker+AsyncLock")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::Tracker_OVRAnchor_AsyncLock {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_OVRAnchor+Tracker+__SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Tracker_OVRAnchor___SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d {
    pub __1__state: i32,
    pub __t__builder: crate::GlobalNamespace::OVRTaskBuilder_1<
        crate::GlobalNamespace::OVRResult_2<u64, crate::GlobalNamespace::OVRPlugin_Result>,
    >,
    pub tracker: u64,
    pub config: crate::GlobalNamespace::OVRAnchor_TrackerConfiguration,
    pub __7__wrap1: u64,
    pub __u__1: crate::GlobalNamespace::OVRTask_1_Awaiter<
        crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRPlugin_Result>,
    >,
    pub __u__2: crate::GlobalNamespace::OVRTask_1_Awaiter<
        crate::GlobalNamespace::OVRResult_2<u64, crate::GlobalNamespace::OVRPlugin_Result>,
    >,
}
#[cfg(
    feature = "cordl_class_OVRAnchor+Tracker+__SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::Tracker_OVRAnchor___SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRAnchor/Tracker/<<SetupDynamicObjectTracker>g__CreateAndConfigureTrackerAsync|5_1>d";
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
    feature = "cordl_class_OVRAnchor+Tracker+__SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::Tracker_OVRAnchor___SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_OVRAnchor+Tracker+__SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::Tracker_OVRAnchor___SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d {
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
    feature = "cordl_class_OVRAnchor+Tracker+__SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::Tracker_OVRAnchor___SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d {
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
    feature = "cordl_class_OVRAnchor+Tracker+__SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::Tracker_OVRAnchor___SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d {
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
    feature = "cordl_class_OVRAnchor+Tracker+__SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::Tracker_OVRAnchor___SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "OVRAnchor+Tracker+__SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d"
)]
impl crate::GlobalNamespace::Tracker_OVRAnchor___SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d {
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MoveNext", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetStateMachine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetStateMachine", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (stateMachine))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "OVRAnchor+Tracker+__SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d"
)]
impl AsRef<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::GlobalNamespace::Tracker_OVRAnchor___SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(
    feature = "OVRAnchor+Tracker+__SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d"
)]
impl AsMut<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::GlobalNamespace::Tracker_OVRAnchor___SetupDynamicObjectTracker_g__CreateAndConfigureTrackerAsync_5_1_d {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}

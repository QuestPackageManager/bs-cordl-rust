#[cfg(feature = "cordl_class_OVRLocatable")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRLocatable {
    pub _Handle_k__BackingField: u64,
}
#[cfg(feature = "cordl_class_OVRLocatable")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRLocatable {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRLocatable";
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
#[cfg(feature = "cordl_class_OVRLocatable")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRLocatable {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRLocatable")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRLocatable {
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
#[cfg(feature = "cordl_class_OVRLocatable")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRLocatable {
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
#[cfg(feature = "cordl_class_OVRLocatable")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRLocatable {
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
#[cfg(feature = "cordl_class_OVRLocatable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRLocatable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRLocatable")]
impl crate::GlobalNamespace::OVRLocatable {
    #[cfg(feature = "OVRLocatable+CopyPosesJob")]
    pub type CopyPosesJob = crate::GlobalNamespace::OVRLocatable_CopyPosesJob;
    #[cfg(feature = "OVRLocatable+GetSceneAnchorPosesJob")]
    pub type GetSceneAnchorPosesJob = crate::GlobalNamespace::OVRLocatable_GetSceneAnchorPosesJob;
    #[cfg(feature = "OVRLocatable+GetSpatialAnchorPosesJob")]
    pub type GetSpatialAnchorPosesJob = crate::GlobalNamespace::OVRLocatable_GetSpatialAnchorPosesJob;
    #[cfg(feature = "OVRLocatable+SetLocalSpaceTransformsJob")]
    pub type SetLocalSpaceTransformsJob = crate::GlobalNamespace::OVRLocatable_SetLocalSpaceTransformsJob;
    #[cfg(feature = "OVRLocatable+SetWorldSpaceTransformsJob")]
    pub type SetWorldSpaceTransformsJob = crate::GlobalNamespace::OVRLocatable_SetWorldSpaceTransformsJob;
    #[cfg(feature = "OVRLocatable+TrackingSpacePose")]
    pub type TrackingSpacePose = crate::GlobalNamespace::OVRLocatable_TrackingSpacePose;
    #[cfg(feature = "OVRLocatable+TransformPosesJob")]
    pub type TransformPosesJob = crate::GlobalNamespace::OVRLocatable_TransformPosesJob;
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
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
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_OVRLocatable0(
        &mut self,
        other: crate::GlobalNamespace::OVRLocatable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRLocatable),
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
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHashCode", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IOVRAnchorComponent_OVRLocatable__FromAnchor(
        &mut self,
        anchor: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRLocatable> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRAnchor),
                        crate::GlobalNamespace::OVRLocatable,
                        1usize,
                    >("IOVRAnchorComponent<OVRLocatable>.FromAnchor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IOVRAnchorComponent<OVRLocatable>.FromAnchor", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRLocatable = unsafe {
            cordl_method_info.invoke_unchecked(self, (anchor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IOVRAnchorComponent_OVRLocatable__get_Handle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        u64,
                        0usize,
                    >("IOVRAnchorComponent<OVRLocatable>.get_Handle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IOVRAnchorComponent<OVRLocatable>.get_Handle", 0usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IOVRAnchorComponent_OVRLocatable__get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                        0usize,
                    >("IOVRAnchorComponent<OVRLocatable>.get_Type")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IOVRAnchorComponent<OVRLocatable>.get_Type", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleUpdateTransforms(
        locatables: crate::Unity::Collections::NativeArray_1<
            crate::GlobalNamespace::OVRLocatable,
        >,
        transforms: crate::UnityEngine::Jobs::TransformAccessArray,
        trackingSpaceToWorldSpaceTransform: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Transform,
        >,
        posesOut: crate::Unity::Collections::NativeArray_1<
            crate::GlobalNamespace::OVRLocatable_TrackingSpacePose,
        >,
        inputDeps: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<
                                crate::GlobalNamespace::OVRLocatable,
                            >,
                            crate::UnityEngine::Jobs::TransformAccessArray,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            crate::Unity::Collections::NativeArray_1<
                                crate::GlobalNamespace::OVRLocatable_TrackingSpacePose,
                            >,
                            crate::Unity::Jobs::JobHandle,
                        ),
                        crate::Unity::Jobs::JobHandle,
                        5usize,
                    >("ScheduleUpdateTransforms")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ScheduleUpdateTransforms", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        locatables,
                        transforms,
                        trackingSpaceToWorldSpaceTransform,
                        posesOut,
                        inputDeps,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetEnabledAsync(
        &mut self,
        enabled: bool,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, f64),
                        crate::GlobalNamespace::OVRTask_1<bool>,
                        2usize,
                    >("SetEnabledAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetEnabledAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = unsafe {
            cordl_method_info.invoke_unchecked(self, (enabled, timeout))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetEnabledSafeAsync(
        &mut self,
        enabled: bool,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, f64),
                        crate::GlobalNamespace::OVRTask_1<bool>,
                        2usize,
                    >("SetEnabledSafeAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetEnabledSafeAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = unsafe {
            cordl_method_info.invoke_unchecked(self, (enabled, timeout))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSceneAnchorPose(
        &mut self,
        pose: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRLocatable_TrackingSpacePose,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::GlobalNamespace::OVRLocatable_TrackingSpacePose,
                        >),
                        bool,
                        1usize,
                    >("TryGetSceneAnchorPose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetSceneAnchorPose", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (pose))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSpatialAnchorPose(
        &mut self,
        pose: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRLocatable_TrackingSpacePose,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::GlobalNamespace::OVRLocatable_TrackingSpacePose,
                        >),
                        bool,
                        1usize,
                    >("TryGetSpatialAnchorPose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetSpatialAnchorPose", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (pose))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateSceneAnchorTransforms(
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::System::Collections::Generic::KeyValuePair_2<
                    crate::GlobalNamespace::OVRAnchor,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                >,
            >,
        >,
        trackingSpaceToWorldSpaceTransform: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Transform,
        >,
        trackingSpacePoses: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::OVRLocatable_TrackingSpacePose,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEnumerable_1<
                                    crate::System::Collections::Generic::KeyValuePair_2<
                                        crate::GlobalNamespace::OVRAnchor,
                                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::GlobalNamespace::OVRLocatable_TrackingSpacePose,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("UpdateSceneAnchorTransforms")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateSceneAnchorTransforms", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (anchors, trackingSpaceToWorldSpaceTransform, trackingSpacePoses),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _UpdateSceneAnchorTransforms_g__GetLocatableOrDefault_34_0(
        anchor: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRLocatable> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRAnchor),
                        crate::GlobalNamespace::OVRLocatable,
                        1usize,
                    >("<UpdateSceneAnchorTransforms>g__GetLocatableOrDefault|34_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<UpdateSceneAnchorTransforms>g__GetLocatableOrDefault|34_0",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRLocatable = unsafe {
            cordl_method_info.invoke_unchecked((), (anchor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        anchor: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRAnchor),
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
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (anchor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Handle(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u64, 0usize>("get_Handle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Handle", 0usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsEnabled", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsNull")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsNull", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                        0usize,
                    >("get_Type")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Type", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::GlobalNamespace::OVRLocatable,
        rhs: crate::GlobalNamespace::OVRLocatable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::OVRLocatable,
                            crate::GlobalNamespace::OVRLocatable,
                        ),
                        bool,
                        2usize,
                    >("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Equality", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::GlobalNamespace::OVRLocatable,
        rhs: crate::GlobalNamespace::OVRLocatable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::OVRLocatable,
                            crate::GlobalNamespace::OVRLocatable,
                        ),
                        bool,
                        2usize,
                    >("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Inequality", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRLocatable")]
impl AsRef<
    crate::GlobalNamespace::IOVRAnchorComponent_1<crate::GlobalNamespace::OVRLocatable>,
> for crate::GlobalNamespace::OVRLocatable {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IOVRAnchorComponent_1<
        crate::GlobalNamespace::OVRLocatable,
    > {
        todo!()
    }
}
#[cfg(feature = "OVRLocatable")]
impl AsMut<
    crate::GlobalNamespace::IOVRAnchorComponent_1<crate::GlobalNamespace::OVRLocatable>,
> for crate::GlobalNamespace::OVRLocatable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IOVRAnchorComponent_1<
        crate::GlobalNamespace::OVRLocatable,
    > {
        todo!()
    }
}
#[cfg(feature = "OVRLocatable")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::OVRLocatable>>
for crate::GlobalNamespace::OVRLocatable {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::GlobalNamespace::OVRLocatable> {
        todo!()
    }
}
#[cfg(feature = "OVRLocatable")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::OVRLocatable>>
for crate::GlobalNamespace::OVRLocatable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::GlobalNamespace::OVRLocatable> {
        todo!()
    }
}
#[cfg(feature = "cordl_class_OVRLocatable+CopyPosesJob")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRLocatable_CopyPosesJob {
    pub PosesIn: crate::Unity::Collections::NativeArray_1<
        crate::GlobalNamespace::OVRLocatable_TrackingSpacePose,
    >,
    pub PosesOut: crate::Unity::Collections::NativeArray_1<
        crate::GlobalNamespace::OVRLocatable_TrackingSpacePose,
    >,
}
#[cfg(feature = "cordl_class_OVRLocatable+CopyPosesJob")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRLocatable_CopyPosesJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRLocatable/CopyPosesJob";
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
#[cfg(feature = "cordl_class_OVRLocatable+CopyPosesJob")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRLocatable_CopyPosesJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRLocatable+CopyPosesJob")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRLocatable_CopyPosesJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+CopyPosesJob")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRLocatable_CopyPosesJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+CopyPosesJob")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRLocatable_CopyPosesJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+CopyPosesJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRLocatable_CopyPosesJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRLocatable+CopyPosesJob")]
impl crate::GlobalNamespace::OVRLocatable_CopyPosesJob {
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (index))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRLocatable+CopyPosesJob")]
impl AsRef<crate::Unity::Jobs::IJobFor>
for crate::GlobalNamespace::OVRLocatable_CopyPosesJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobFor {
        todo!()
    }
}
#[cfg(feature = "OVRLocatable+CopyPosesJob")]
impl AsMut<crate::Unity::Jobs::IJobFor>
for crate::GlobalNamespace::OVRLocatable_CopyPosesJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobFor {
        todo!()
    }
}
#[cfg(feature = "cordl_class_OVRLocatable+GetSceneAnchorPosesJob")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRLocatable_GetSceneAnchorPosesJob {
    pub Locatables: crate::Unity::Collections::NativeArray_1<
        crate::GlobalNamespace::OVRLocatable,
    >,
    pub Poses: crate::Unity::Collections::NativeArray_1<
        crate::GlobalNamespace::OVRLocatable_TrackingSpacePose,
    >,
}
#[cfg(feature = "cordl_class_OVRLocatable+GetSceneAnchorPosesJob")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRLocatable_GetSceneAnchorPosesJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRLocatable/GetSceneAnchorPosesJob";
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
#[cfg(feature = "cordl_class_OVRLocatable+GetSceneAnchorPosesJob")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRLocatable_GetSceneAnchorPosesJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRLocatable+GetSceneAnchorPosesJob")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRLocatable_GetSceneAnchorPosesJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+GetSceneAnchorPosesJob")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRLocatable_GetSceneAnchorPosesJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+GetSceneAnchorPosesJob")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRLocatable_GetSceneAnchorPosesJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+GetSceneAnchorPosesJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRLocatable_GetSceneAnchorPosesJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRLocatable+GetSceneAnchorPosesJob")]
impl crate::GlobalNamespace::OVRLocatable_GetSceneAnchorPosesJob {
    pub fn Unity_Jobs_IJobFor_Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Unity.Jobs.IJobFor.Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Jobs.IJobFor.Execute", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (index))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRLocatable+GetSceneAnchorPosesJob")]
impl AsRef<crate::Unity::Jobs::IJobFor>
for crate::GlobalNamespace::OVRLocatable_GetSceneAnchorPosesJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobFor {
        todo!()
    }
}
#[cfg(feature = "OVRLocatable+GetSceneAnchorPosesJob")]
impl AsMut<crate::Unity::Jobs::IJobFor>
for crate::GlobalNamespace::OVRLocatable_GetSceneAnchorPosesJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobFor {
        todo!()
    }
}
#[cfg(feature = "cordl_class_OVRLocatable+GetSpatialAnchorPosesJob")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRLocatable_GetSpatialAnchorPosesJob {
    pub Locatables: crate::Unity::Collections::NativeArray_1<
        crate::GlobalNamespace::OVRLocatable,
    >,
    pub Poses: crate::Unity::Collections::NativeArray_1<
        crate::GlobalNamespace::OVRLocatable_TrackingSpacePose,
    >,
}
#[cfg(feature = "cordl_class_OVRLocatable+GetSpatialAnchorPosesJob")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRLocatable_GetSpatialAnchorPosesJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRLocatable/GetSpatialAnchorPosesJob";
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
#[cfg(feature = "cordl_class_OVRLocatable+GetSpatialAnchorPosesJob")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRLocatable_GetSpatialAnchorPosesJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRLocatable+GetSpatialAnchorPosesJob")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRLocatable_GetSpatialAnchorPosesJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+GetSpatialAnchorPosesJob")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRLocatable_GetSpatialAnchorPosesJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+GetSpatialAnchorPosesJob")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRLocatable_GetSpatialAnchorPosesJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+GetSpatialAnchorPosesJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRLocatable_GetSpatialAnchorPosesJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRLocatable+GetSpatialAnchorPosesJob")]
impl crate::GlobalNamespace::OVRLocatable_GetSpatialAnchorPosesJob {
    pub fn Unity_Jobs_IJobFor_Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Unity.Jobs.IJobFor.Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Jobs.IJobFor.Execute", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (index))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRLocatable+GetSpatialAnchorPosesJob")]
impl AsRef<crate::Unity::Jobs::IJobFor>
for crate::GlobalNamespace::OVRLocatable_GetSpatialAnchorPosesJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobFor {
        todo!()
    }
}
#[cfg(feature = "OVRLocatable+GetSpatialAnchorPosesJob")]
impl AsMut<crate::Unity::Jobs::IJobFor>
for crate::GlobalNamespace::OVRLocatable_GetSpatialAnchorPosesJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobFor {
        todo!()
    }
}
#[cfg(feature = "cordl_class_OVRLocatable+SetLocalSpaceTransformsJob")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRLocatable_SetLocalSpaceTransformsJob {
    pub Poses: crate::Unity::Collections::NativeArray_1<
        crate::GlobalNamespace::OVRLocatable_TrackingSpacePose,
    >,
}
#[cfg(feature = "cordl_class_OVRLocatable+SetLocalSpaceTransformsJob")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRLocatable_SetLocalSpaceTransformsJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRLocatable/SetLocalSpaceTransformsJob";
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
#[cfg(feature = "cordl_class_OVRLocatable+SetLocalSpaceTransformsJob")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRLocatable_SetLocalSpaceTransformsJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRLocatable+SetLocalSpaceTransformsJob")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRLocatable_SetLocalSpaceTransformsJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+SetLocalSpaceTransformsJob")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRLocatable_SetLocalSpaceTransformsJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+SetLocalSpaceTransformsJob")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRLocatable_SetLocalSpaceTransformsJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+SetLocalSpaceTransformsJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRLocatable_SetLocalSpaceTransformsJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRLocatable+SetLocalSpaceTransformsJob")]
impl crate::GlobalNamespace::OVRLocatable_SetLocalSpaceTransformsJob {
    pub fn Execute(
        &mut self,
        index: i32,
        transform: crate::UnityEngine::Jobs::TransformAccess,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::UnityEngine::Jobs::TransformAccess),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (index, transform))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRLocatable+SetLocalSpaceTransformsJob")]
impl AsRef<crate::UnityEngine::Jobs::IJobParallelForTransform>
for crate::GlobalNamespace::OVRLocatable_SetLocalSpaceTransformsJob {
    fn as_ref(&self) -> &crate::UnityEngine::Jobs::IJobParallelForTransform {
        todo!()
    }
}
#[cfg(feature = "OVRLocatable+SetLocalSpaceTransformsJob")]
impl AsMut<crate::UnityEngine::Jobs::IJobParallelForTransform>
for crate::GlobalNamespace::OVRLocatable_SetLocalSpaceTransformsJob {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Jobs::IJobParallelForTransform {
        todo!()
    }
}
#[cfg(feature = "cordl_class_OVRLocatable+SetWorldSpaceTransformsJob")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRLocatable_SetWorldSpaceTransformsJob {
    pub Poses: crate::Unity::Collections::NativeArray_1<
        crate::GlobalNamespace::OVRLocatable_TrackingSpacePose,
    >,
}
#[cfg(feature = "cordl_class_OVRLocatable+SetWorldSpaceTransformsJob")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRLocatable_SetWorldSpaceTransformsJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRLocatable/SetWorldSpaceTransformsJob";
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
#[cfg(feature = "cordl_class_OVRLocatable+SetWorldSpaceTransformsJob")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRLocatable_SetWorldSpaceTransformsJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRLocatable+SetWorldSpaceTransformsJob")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRLocatable_SetWorldSpaceTransformsJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+SetWorldSpaceTransformsJob")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRLocatable_SetWorldSpaceTransformsJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+SetWorldSpaceTransformsJob")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRLocatable_SetWorldSpaceTransformsJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+SetWorldSpaceTransformsJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRLocatable_SetWorldSpaceTransformsJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRLocatable+SetWorldSpaceTransformsJob")]
impl crate::GlobalNamespace::OVRLocatable_SetWorldSpaceTransformsJob {
    pub fn UnityEngine_Jobs_IJobParallelForTransform_Execute(
        &mut self,
        index: i32,
        transform: crate::UnityEngine::Jobs::TransformAccess,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::UnityEngine::Jobs::TransformAccess),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("UnityEngine.Jobs.IJobParallelForTransform.Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.Jobs.IJobParallelForTransform.Execute", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (index, transform))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRLocatable+SetWorldSpaceTransformsJob")]
impl AsRef<crate::UnityEngine::Jobs::IJobParallelForTransform>
for crate::GlobalNamespace::OVRLocatable_SetWorldSpaceTransformsJob {
    fn as_ref(&self) -> &crate::UnityEngine::Jobs::IJobParallelForTransform {
        todo!()
    }
}
#[cfg(feature = "OVRLocatable+SetWorldSpaceTransformsJob")]
impl AsMut<crate::UnityEngine::Jobs::IJobParallelForTransform>
for crate::GlobalNamespace::OVRLocatable_SetWorldSpaceTransformsJob {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Jobs::IJobParallelForTransform {
        todo!()
    }
}
#[cfg(feature = "cordl_class_OVRLocatable+TrackingSpacePose")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRLocatable_TrackingSpacePose {
    pub _Position_k__BackingField: crate::System::Nullable_1<
        crate::UnityEngine::Vector3,
    >,
    pub _Rotation_k__BackingField: crate::System::Nullable_1<
        crate::UnityEngine::Quaternion,
    >,
    pub Flags: crate::GlobalNamespace::OVRPlugin_SpaceLocationFlags,
}
#[cfg(feature = "cordl_class_OVRLocatable+TrackingSpacePose")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRLocatable_TrackingSpacePose {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRLocatable/TrackingSpacePose";
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
#[cfg(feature = "cordl_class_OVRLocatable+TrackingSpacePose")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRLocatable_TrackingSpacePose {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRLocatable+TrackingSpacePose")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRLocatable_TrackingSpacePose {
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
#[cfg(feature = "cordl_class_OVRLocatable+TrackingSpacePose")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRLocatable_TrackingSpacePose {
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
#[cfg(feature = "cordl_class_OVRLocatable+TrackingSpacePose")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRLocatable_TrackingSpacePose {
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
#[cfg(feature = "cordl_class_OVRLocatable+TrackingSpacePose")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRLocatable_TrackingSpacePose {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRLocatable+TrackingSpacePose")]
impl crate::GlobalNamespace::OVRLocatable_TrackingSpacePose {
    pub const localToWorldPoseDeprecationMessage: &'static str = "Using this method after \\'await locatable.SetEnabledAsync(true);\\' is error-prone. OVRTask finishes the execution before OVRCameraRig.Update(), so camera will still use a pose from the previous frame. This results in descrepancy when localizing anchors against the stale camera pose.\\nUse an overload with the \\'trackingSpaceToWorldSpaceTransform\\' parameter instead.";
    pub fn ComputeWorldPosition_Camera0(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::Vector3>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        crate::System::Nullable_1<crate::UnityEngine::Vector3>,
                        1usize,
                    >("ComputeWorldPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeWorldPosition", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<crate::UnityEngine::Vector3> = unsafe {
            cordl_method_info.invoke_unchecked(self, (camera))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeWorldPosition_Transform1(
        &mut self,
        trackingSpaceToWorldSpaceTransform: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Transform,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::Vector3>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>),
                        crate::System::Nullable_1<crate::UnityEngine::Vector3>,
                        1usize,
                    >("ComputeWorldPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeWorldPosition", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<crate::UnityEngine::Vector3> = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (trackingSpaceToWorldSpaceTransform))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeWorldRotation_Camera0(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::Quaternion>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        crate::System::Nullable_1<crate::UnityEngine::Quaternion>,
                        1usize,
                    >("ComputeWorldRotation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeWorldRotation", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<crate::UnityEngine::Quaternion> = unsafe {
            cordl_method_info.invoke_unchecked(self, (camera))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeWorldRotation_Transform1(
        &mut self,
        trackingSpaceToWorldSpaceTransform: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Transform,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::Quaternion>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>),
                        crate::System::Nullable_1<crate::UnityEngine::Quaternion>,
                        1usize,
                    >("ComputeWorldRotation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeWorldRotation", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<crate::UnityEngine::Quaternion> = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (trackingSpaceToWorldSpaceTransform))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        flags: crate::GlobalNamespace::OVRPlugin_SpaceLocationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Quaternion,
                            crate::GlobalNamespace::OVRPlugin_SpaceLocationFlags,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (position, rotation, flags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsPositionTracked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsPositionTracked")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsPositionTracked", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsRotationTracked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsRotationTracked")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsRotationTracked", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::Vector3>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::Nullable_1<crate::UnityEngine::Vector3>,
                        0usize,
                    >("get_Position")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Position", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<crate::UnityEngine::Vector3> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Rotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::Quaternion>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::Nullable_1<crate::UnityEngine::Quaternion>,
                        0usize,
                    >("get_Rotation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Rotation", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<crate::UnityEngine::Quaternion> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRLocatable+TransformPosesJob")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRLocatable_TransformPosesJob {
    pub Poses: crate::Unity::Collections::NativeArray_1<
        crate::GlobalNamespace::OVRLocatable_TrackingSpacePose,
    >,
    pub Transform: crate::UnityEngine::Matrix4x4,
    pub Rotation: crate::UnityEngine::Quaternion,
}
#[cfg(feature = "cordl_class_OVRLocatable+TransformPosesJob")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRLocatable_TransformPosesJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRLocatable/TransformPosesJob";
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
#[cfg(feature = "cordl_class_OVRLocatable+TransformPosesJob")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRLocatable_TransformPosesJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRLocatable+TransformPosesJob")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRLocatable_TransformPosesJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+TransformPosesJob")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRLocatable_TransformPosesJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+TransformPosesJob")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRLocatable_TransformPosesJob {
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
#[cfg(feature = "cordl_class_OVRLocatable+TransformPosesJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRLocatable_TransformPosesJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRLocatable+TransformPosesJob")]
impl crate::GlobalNamespace::OVRLocatable_TransformPosesJob {
    pub fn Unity_Jobs_IJobFor_Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Unity.Jobs.IJobFor.Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Jobs.IJobFor.Execute", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (index))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRLocatable+TransformPosesJob")]
impl AsRef<crate::Unity::Jobs::IJobFor>
for crate::GlobalNamespace::OVRLocatable_TransformPosesJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobFor {
        todo!()
    }
}
#[cfg(feature = "OVRLocatable+TransformPosesJob")]
impl AsMut<crate::Unity::Jobs::IJobFor>
for crate::GlobalNamespace::OVRLocatable_TransformPosesJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobFor {
        todo!()
    }
}

#[cfg(feature = "cordl_class_BeatmapTypeConverters")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapTypeConverters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BeatmapTypeConverters")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapTypeConverters {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapTypeConverters";
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
#[cfg(feature = "cordl_class_BeatmapTypeConverters")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapTypeConverters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_BeatmapTypeConverters")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapTypeConverters {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapTypeConverters")]
impl crate::GlobalNamespace::BeatmapTypeConverters {
    pub fn ConvertBasicBeatmapEventType(
        beatmapEventType: crate::BeatmapSaveDataCommon::BeatmapEventType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BasicBeatmapEventType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BeatmapSaveDataCommon::BeatmapEventType),
                        crate::GlobalNamespace::BasicBeatmapEventType,
                        1usize,
                    >("ConvertBasicBeatmapEventType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertBasicBeatmapEventType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::BasicBeatmapEventType = unsafe {
            cordl_method_info.invoke_unchecked((), (beatmapEventType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertDistributionParamType(
        distributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BeatmapSaveDataCommon::DistributionParamType),
                        crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
                        1usize,
                    >("ConvertDistributionParamType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertDistributionParamType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType = unsafe {
            cordl_method_info.invoke_unchecked((), (distributionParamType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertEaseType(
        easeType: crate::BeatmapSaveDataCommon::EaseType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EaseType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BeatmapSaveDataCommon::EaseType),
                        crate::GlobalNamespace::EaseType,
                        1usize,
                    >("ConvertEaseType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertEaseType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::EaseType = unsafe {
            cordl_method_info.invoke_unchecked((), (easeType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertEnvironmentColorType(
        environmentColorType: crate::BeatmapSaveDataCommon::EnvironmentColorType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EnvironmentColorType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BeatmapSaveDataCommon::EnvironmentColorType),
                        crate::GlobalNamespace::EnvironmentColorType,
                        1usize,
                    >("ConvertEnvironmentColorType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertEnvironmentColorType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::EnvironmentColorType = unsafe {
            cordl_method_info.invoke_unchecked((), (environmentColorType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertLightAxis(
        axis: crate::BeatmapSaveDataCommon::Axis,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::LightAxis> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BeatmapSaveDataCommon::Axis),
                        crate::GlobalNamespace::LightAxis,
                        1usize,
                    >("ConvertLightAxis")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertLightAxis", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::LightAxis = unsafe {
            cordl_method_info.invoke_unchecked((), (axis))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertLightRotationDirection(
        rotationDirection: crate::BeatmapSaveDataCommon::RotationDirection,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::LightRotationDirection> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BeatmapSaveDataCommon::RotationDirection),
                        crate::GlobalNamespace::LightRotationDirection,
                        1usize,
                    >("ConvertLightRotationDirection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertLightRotationDirection", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::LightRotationDirection = unsafe {
            cordl_method_info.invoke_unchecked((), (rotationDirection))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertNoteColorType_ColorType0(
        noteType: crate::BeatmapSaveDataVersion2_6_0AndEarlier::ColorType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ColorType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BeatmapSaveDataVersion2_6_0AndEarlier::ColorType),
                        crate::GlobalNamespace::ColorType,
                        1usize,
                    >("ConvertNoteColorType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertNoteColorType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::ColorType = unsafe {
            cordl_method_info.invoke_unchecked((), (noteType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertNoteColorType_NoteColorType1(
        noteType: crate::BeatmapSaveDataCommon::NoteColorType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ColorType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BeatmapSaveDataCommon::NoteColorType),
                        crate::GlobalNamespace::ColorType,
                        1usize,
                    >("ConvertNoteColorType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertNoteColorType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::ColorType = unsafe {
            cordl_method_info.invoke_unchecked((), (noteType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertNoteCutDirection(
        noteCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteCutDirection> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BeatmapSaveDataCommon::NoteCutDirection),
                        crate::GlobalNamespace::NoteCutDirection,
                        1usize,
                    >("ConvertNoteCutDirection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertNoteCutDirection", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::NoteCutDirection = unsafe {
            cordl_method_info.invoke_unchecked((), (noteCutDirection))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertNoteLineLayer_NoteLineLayer1(
        layer: crate::BeatmapSaveDataCommon::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteLineLayer> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BeatmapSaveDataCommon::NoteLineLayer),
                        crate::GlobalNamespace::NoteLineLayer,
                        1usize,
                    >("ConvertNoteLineLayer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertNoteLineLayer", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::NoteLineLayer = unsafe {
            cordl_method_info.invoke_unchecked((), (layer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertNoteLineLayer_i32_0(
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteLineLayer> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        crate::GlobalNamespace::NoteLineLayer,
                        1usize,
                    >("ConvertNoteLineLayer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertNoteLineLayer", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::NoteLineLayer = unsafe {
            cordl_method_info.invoke_unchecked((), (layer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertOffsetDirection(
        offsetDirection: crate::BeatmapSaveDataCommon::OffsetDirection,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OffsetDirection> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BeatmapSaveDataCommon::OffsetDirection),
                        crate::GlobalNamespace::OffsetDirection,
                        1usize,
                    >("ConvertOffsetDirection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertOffsetDirection", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OffsetDirection = unsafe {
            cordl_method_info.invoke_unchecked((), (offsetDirection))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertSliderDataType(
        sliderType: crate::BeatmapSaveDataVersion3::SliderType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SliderData_Type> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BeatmapSaveDataVersion3::SliderType),
                        crate::GlobalNamespace::SliderData_Type,
                        1usize,
                    >("ConvertSliderDataType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertSliderDataType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::SliderData_Type = unsafe {
            cordl_method_info.invoke_unchecked((), (sliderType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertSliderMidAnchorMode(
        sliderMidAnchorMode: crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SliderMidAnchorMode> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BeatmapSaveDataCommon::SliderMidAnchorMode),
                        crate::GlobalNamespace::SliderMidAnchorMode,
                        1usize,
                    >("ConvertSliderMidAnchorMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertSliderMidAnchorMode", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::SliderMidAnchorMode = unsafe {
            cordl_method_info.invoke_unchecked((), (sliderMidAnchorMode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertTransitionTypeToEaseType(
        transitionType: crate::BeatmapSaveDataVersion3::TransitionType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EaseType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BeatmapSaveDataVersion3::TransitionType),
                        crate::GlobalNamespace::EaseType,
                        1usize,
                    >("ConvertTransitionTypeToEaseType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertTransitionTypeToEaseType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::EaseType = unsafe {
            cordl_method_info.invoke_unchecked((), (transitionType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertTransitionTypeToExtension(
        transitionType: crate::BeatmapSaveDataVersion3::TransitionType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BeatmapSaveDataVersion3::TransitionType),
                        bool,
                        1usize,
                    >("ConvertTransitionTypeToExtension")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertTransitionTypeToExtension", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (transitionType))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapTypeConverters")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapTypeConverters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

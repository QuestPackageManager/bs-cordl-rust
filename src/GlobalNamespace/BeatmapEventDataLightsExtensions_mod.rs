#[cfg(feature = "cordl_class_BeatmapEventDataLightsExtensions")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct BeatmapEventDataLightsExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BeatmapEventDataLightsExtensions")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::BeatmapEventDataLightsExtensions
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapEventDataLightsExtensions";
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
#[cfg(feature = "BeatmapEventDataLightsExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapEventDataLightsExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataLightsExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapEventDataLightsExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataLightsExtensions")]
impl crate::GlobalNamespace::BeatmapEventDataLightsExtensions {
    #[cfg(feature = "BeatmapEventDataLightsExtensions+LightSwitchEventEffectDataValues")]
    pub type LightSwitchEventEffectDataValues =
        crate::GlobalNamespace::BeatmapEventDataLightsExtensions_LightSwitchEventEffectDataValues;
    pub fn GetLightColorTypeFromEventDataValue(
        beatmapEventValue: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EnvironmentColorType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        crate::GlobalNamespace::EnvironmentColorType,
                        1usize,
                    >("GetLightColorTypeFromEventDataValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLightColorTypeFromEventDataValue", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::EnvironmentColorType =
            unsafe { cordl_method_info.invoke_unchecked((), (beatmapEventValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasFixedDurationLightSwitchEventDataValue_BasicBeatmapEventData0(
        basicBeatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BasicBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BasicBeatmapEventData,
                        >),
                        bool,
                        1usize,
                    >("HasFixedDurationLightSwitchEventDataValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HasFixedDurationLightSwitchEventDataValue", 1usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (basicBeatmapEventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasFixedDurationLightSwitchEventDataValue_i32_1(
        beatmapEventValue: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), bool, 1usize>(
                        "HasFixedDurationLightSwitchEventDataValue",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "HasFixedDurationLightSwitchEventDataValue",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (beatmapEventValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasLightFadeEventDataValue(
        basicBeatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BasicBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BasicBeatmapEventData,
                        >),
                        bool,
                        1usize,
                    >("HasLightFadeEventDataValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HasLightFadeEventDataValue", 1usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (basicBeatmapEventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn LightColorTypeFromEventDataValue(
        basicBeatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BasicBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EnvironmentColorType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BasicBeatmapEventData,
                        >),
                        crate::GlobalNamespace::EnvironmentColorType,
                        1usize,
                    >("LightColorTypeFromEventDataValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LightColorTypeFromEventDataValue", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::EnvironmentColorType =
            unsafe { cordl_method_info.invoke_unchecked((), (basicBeatmapEventData))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapEventDataLightsExtensions")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::BeatmapEventDataLightsExtensions
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_BeatmapEventDataLightsExtensions+LightSwitchEventEffectDataValues")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct BeatmapEventDataLightsExtensions_LightSwitchEventEffectDataValues {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BeatmapEventDataLightsExtensions+LightSwitchEventEffectDataValues")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::BeatmapEventDataLightsExtensions_LightSwitchEventEffectDataValues
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str =
        "BeatmapEventDataLightsExtensions/LightSwitchEventEffectDataValues";
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
#[cfg(feature = "BeatmapEventDataLightsExtensions+LightSwitchEventEffectDataValues")]
impl std::ops::Deref
    for crate::GlobalNamespace::BeatmapEventDataLightsExtensions_LightSwitchEventEffectDataValues
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataLightsExtensions+LightSwitchEventEffectDataValues")]
impl std::ops::DerefMut
    for crate::GlobalNamespace::BeatmapEventDataLightsExtensions_LightSwitchEventEffectDataValues
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataLightsExtensions+LightSwitchEventEffectDataValues")]
impl crate::GlobalNamespace::BeatmapEventDataLightsExtensions_LightSwitchEventEffectDataValues {
    pub const kFadeA: i32 = 4i32;
    pub const kFadeB: i32 = 8i32;
    pub const kFadeW: i32 = 12i32;
    pub const kFlashAndFadeToBlack: i32 = -1i32;
    pub const kFlashAndFadeToBlackA: i32 = 3i32;
    pub const kFlashAndFadeToBlackB: i32 = 7i32;
    pub const kFlashAndFadeToBlackW: i32 = 11i32;
    pub const kHighlightA: i32 = 2i32;
    pub const kHighlightB: i32 = 6i32;
    pub const kHighlightW: i32 = 10i32;
    pub const kOff: i32 = 0i32;
    pub const kOnA: i32 = 1i32;
    pub const kOnB: i32 = 5i32;
    pub const kOnW: i32 = 9i32;
}
#[cfg(feature = "cordl_class_BeatmapEventDataLightsExtensions+LightSwitchEventEffectDataValues")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::BeatmapEventDataLightsExtensions_LightSwitchEventEffectDataValues
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "cordl_class_ColorSchemeExtensions")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct ColorSchemeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_ColorSchemeExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::ColorSchemeExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ColorSchemeExtensions";
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
#[cfg(feature = "ColorSchemeExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::ColorSchemeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorSchemeExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeExtensions")]
impl crate::GlobalNamespace::ColorSchemeExtensions {
    pub fn GetColorSchemeForGameplayOverride(
        playerOverrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IGameplayColorScheme,
        >,
        beatmapOverrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IGameplayColorScheme,
        >,
        environmentColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IGameplayColorScheme,
        >,
        usingBeatmapDefaultEnvironment: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IGameplayColorScheme>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IGameplayColorScheme,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IGameplayColorScheme,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IGameplayColorScheme,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IGameplayColorScheme,
                        >,
                        4usize,
                    >("GetColorSchemeForGameplayOverride")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetColorSchemeForGameplayOverride", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IGameplayColorScheme> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    playerOverrideColorScheme,
                    beatmapOverrideColorScheme,
                    environmentColorScheme,
                    usingBeatmapDefaultEnvironment,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetColorSchemeForLightshowOverride(
        playerOverrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILightshowColorScheme,
        >,
        playerOverrideLightshowColors: bool,
        beatmapOverrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILightshowColorScheme,
        >,
        environmentColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILightshowColorScheme,
        >,
        usingBeatmapDefaultEnvironment: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILightshowColorScheme>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ILightshowColorScheme,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ILightshowColorScheme,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ILightshowColorScheme,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::ILightshowColorScheme,
                        >,
                        5usize,
                    >("GetColorSchemeForLightshowOverride")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetColorSchemeForLightshowOverride", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILightshowColorScheme> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    playerOverrideColorScheme,
                    playerOverrideLightshowColors,
                    beatmapOverrideColorScheme,
                    environmentColorScheme,
                    usingBeatmapDefaultEnvironment,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResolveColorScheme(
        playerOverrideColorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
        playerOverrideLightshowColors: bool,
        beatmapOverrideColorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
        environmentColorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
        usingBeatmapDefaultEnvironment: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
                        bool,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
                        bool,
                    ), quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>, 5usize>(
                        "ResolveColorScheme",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ResolveColorScheme",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    playerOverrideColorScheme,
                    playerOverrideLightshowColors,
                    beatmapOverrideColorScheme,
                    environmentColorScheme,
                    usingBeatmapDefaultEnvironment,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_ColorSchemeExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ColorSchemeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

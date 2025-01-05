#[cfg(feature = "ColorSchemeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorSchemeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "ColorSchemeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ColorSchemeExtensions => ""
    ."ColorSchemeExtensions"
);
#[cfg(feature = "ColorSchemeExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::ColorSchemeExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorSchemeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeExtensions")]
impl crate::GlobalNamespace::ColorSchemeExtensions {
    pub fn ResolveColor(
        playerOverrideColor: crate::System::Nullable_1<crate::UnityEngine::Color>,
        usePlayerOverride: bool,
        useBeatmapOverride: crate::System::Nullable_1<bool>,
        beatmapOverrideColor: crate::System::Nullable_1<crate::UnityEngine::Color>,
        environmentColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ResolveColor",
                (
                    playerOverrideColor,
                    usePlayerOverride,
                    useBeatmapOverride,
                    beatmapOverrideColor,
                    environmentColor,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveColorScheme(
        playerOverrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
        playerOverrideLightshowColors: bool,
        beatmapOverrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
        environmentColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ResolveColorScheme",
                (
                    playerOverrideColorScheme,
                    playerOverrideLightshowColors,
                    beatmapOverrideColorScheme,
                    environmentColorScheme,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ColorSchemeExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ColorSchemeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

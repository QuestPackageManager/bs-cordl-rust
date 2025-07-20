#[cfg(feature = "ColorSchemeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorSchemeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ColorSchemeExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ColorSchemeExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ColorSchemeExtensions";
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
#[cfg(feature = "ColorSchemeExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::ColorSchemeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ColorSchemeExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::Nullable_1<crate::UnityEngine::Color>,
                    bool,
                    crate::System::Nullable_1<bool>,
                    crate::System::Nullable_1<crate::UnityEngine::Color>,
                    crate::UnityEngine::Color,
                ),
                crate::UnityEngine::Color,
                5usize,
            >("ResolveColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ColorSchemeExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "ResolveColor", 5usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        playerOverrideColor,
                        usePlayerOverride,
                        useBeatmapOverride,
                        beatmapOverrideColor,
                        environmentColor,
                    ),
                )?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ColorSchemeExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
                ),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
                4usize,
            >("ResolveColorScheme")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ColorSchemeExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "ResolveColorScheme", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        playerOverrideColorScheme,
                        playerOverrideLightshowColors,
                        beatmapOverrideColorScheme,
                        environmentColorScheme,
                    ),
                )?
        };
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

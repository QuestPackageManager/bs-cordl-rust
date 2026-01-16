#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+FontAssetFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct FontAssetFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+FontAssetFactory")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::TextCore::Text::FontAssetFactory {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "FontAssetFactory";
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
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetFactory")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::FontAssetFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetFactory")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::FontAssetFactory {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetFactory")]
impl crate::UnityEngine::TextCore::Text::FontAssetFactory {
    pub fn CreateDefaultEditorFontAsset(
        font: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::FontAsset,
                        >,
                        2usize,
                    >("CreateDefaultEditorFontAsset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateDefaultEditorFontAsset", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset> =
            unsafe { cordl_method_info.invoke_unchecked((), (font, shader))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetHideFlags(
        fontAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::FontAsset,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetHideFlags")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetHideFlags", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (fontAsset))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupFontAssetSettings(
        fontAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetupFontAssetSettings"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupFontAssetSettings",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (fontAsset, shader))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+FontAssetFactory")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::TextCore::Text::FontAssetFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

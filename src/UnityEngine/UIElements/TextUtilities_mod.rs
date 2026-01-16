#[cfg(feature = "cordl_class_UnityEngine+UIElements+TextUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TextUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TextUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::TextUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "TextUtilities";
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
#[cfg(feature = "UnityEngine+UIElements+TextUtilities")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TextUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::TextUtilities {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextUtilities")]
impl crate::UnityEngine::UIElements::TextUtilities {
    pub fn GetFontAsset(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::FontAsset,
                        >,
                        1usize,
                    >("GetFontAsset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetFontAsset", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        > = unsafe { cordl_method_info.invoke_unchecked((), (ve))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextCoreSettingsForElement(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        ignoreColors: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::UIR::TextCoreSettings,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            bool,
                        ),
                        crate::UnityEngine::UIElements::UIR::TextCoreSettings,
                        2usize,
                    >("GetTextCoreSettingsForElement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTextCoreSettingsForElement", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::TextCoreSettings = unsafe {
            cordl_method_info.invoke_unchecked((), (ve, ignoreColors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextSettingsFrom(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextSettings>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::TextSettings,
                        >,
                        1usize,
                    >("GetTextSettingsFrom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTextSettingsFrom", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextSettings,
        > = unsafe { cordl_method_info.invoke_unchecked((), (ve))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsAdvancedTextEnabledForElement(
        te: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::TextElement,
                        >),
                        bool,
                        1usize,
                    >("IsAdvancedTextEnabledForElement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsAdvancedTextEnabledForElement", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (te))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsFontAssigned(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >),
                        bool,
                        1usize,
                    >("IsFontAssigned")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsFontAssigned", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (ve))? };
        Ok(__cordl_ret.into())
    }
    pub fn MeasureVisualElementTextSize(
        te: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
        textToMeasure: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::Text::RenderedText,
        >,
        width: f32,
        widthMode: crate::UnityEngine::UIElements::VisualElement_MeasureMode,
        height: f32,
        heightMode: crate::UnityEngine::UIElements::VisualElement_MeasureMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::TextElement,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::TextCore::Text::RenderedText,
                            >,
                            f32,
                            crate::UnityEngine::UIElements::VisualElement_MeasureMode,
                            f32,
                            crate::UnityEngine::UIElements::VisualElement_MeasureMode,
                        ),
                        crate::UnityEngine::Vector2,
                        6usize,
                    >("MeasureVisualElementTextSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MeasureVisualElementTextSize", 6usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (te, textToMeasure, width, widthMode, height, heightMode),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn toTextCore_TextOverflow_OverflowInternal1(
        textOverflow: crate::UnityEngine::UIElements::TextOverflow,
        overflow: crate::UnityEngine::UIElements::OverflowInternal,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::TextOverflow> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::UIElements::TextOverflow,
                            crate::UnityEngine::UIElements::OverflowInternal,
                        ),
                        crate::UnityEngine::TextCore::TextOverflow,
                        2usize,
                    >("toTextCore")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "toTextCore", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::TextOverflow = unsafe {
            cordl_method_info.invoke_unchecked((), (textOverflow, overflow))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn toTextCore_WhiteSpace__cordl_bool0(
        whiteSpace: crate::UnityEngine::UIElements::WhiteSpace,
        isInputField: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::WhiteSpace> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::UIElements::WhiteSpace, bool),
                        crate::UnityEngine::TextCore::WhiteSpace,
                        2usize,
                    >("toTextCore")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "toTextCore", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::WhiteSpace = unsafe {
            cordl_method_info.invoke_unchecked((), (whiteSpace, isInputField))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn toTextWrappingMode(
        whiteSpace: crate::UnityEngine::UIElements::WhiteSpace,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::Text::TextWrappingMode,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::UIElements::WhiteSpace),
                        crate::UnityEngine::TextCore::Text::TextWrappingMode,
                        1usize,
                    >("toTextWrappingMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "toTextWrappingMode", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::Text::TextWrappingMode = unsafe {
            cordl_method_info.invoke_unchecked((), (whiteSpace))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TextUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TextUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

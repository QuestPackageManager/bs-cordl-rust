#[cfg(feature = "UnityEngine+UIElements+TextUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TextUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+TextUtilities")]
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
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::TextUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextUtilities")]
impl crate::UnityEngine::UIElements::TextUtilities {
    pub fn ConvertPixelUnitsToTextCoreRelativeUnits(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        fontAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::FontAsset,
                    >,
                ),
                f32,
                2usize,
            >("ConvertPixelUnitsToTextCoreRelativeUnits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ConvertPixelUnitsToTextCoreRelativeUnits", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (ve, fontAsset)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetFontAsset(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
                1usize,
            >("GetFontAsset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetFontAsset", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        > = unsafe { method.invoke_unchecked((), (ve)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextCoreSettingsForElement(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::UIR::TextCoreSettings,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                crate::UnityEngine::UIElements::UIR::TextCoreSettings,
                1usize,
            >("GetTextCoreSettingsForElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTextCoreSettingsForElement", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::TextCoreSettings = unsafe {
            method.invoke_unchecked((), (ve))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextSettingsFrom(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PanelTextSettings>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::PanelTextSettings,
                >,
                1usize,
            >("GetTextSettingsFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTextSettingsFrom", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::PanelTextSettings,
        > = unsafe { method.invoke_unchecked((), (ve)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsFontAssigned(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self, "IsFontAssigned", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (ve)) };
        Ok(__cordl_ret.into())
    }
    pub fn MeasureVisualElementTextSize(
        te: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
        textToMeasure: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        width: f32,
        widthMode: crate::UnityEngine::UIElements::VisualElement_MeasureMode,
        height: f32,
        heightMode: crate::UnityEngine::UIElements::VisualElement_MeasureMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::TextElement,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
                    self, "MeasureVisualElementTextSize", 6usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (te, textToMeasure, width, widthMode, height, heightMode),
                )
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TextUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

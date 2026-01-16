#[cfg(feature = "cordl_class_UnityEngine+UIElements+InlineStyleAccess")]
#[repr(C)]
#[derive(Debug)]
pub struct InlineStyleAccess {
    __cordl_parent: crate::UnityEngine::UIElements::StyleValueCollection,
    pub m_ValuesManaged: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::StyleSheets::StyleValueManaged,
        >,
    >,
    pub _ve_k__BackingField:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    pub m_HasInlineCursor: bool,
    pub m_InlineCursor: crate::UnityEngine::UIElements::StyleCursor,
    pub m_HasInlineTextShadow: bool,
    pub m_InlineTextShadow: crate::UnityEngine::UIElements::StyleTextShadow,
    pub m_HasInlineTransformOrigin: bool,
    pub m_InlineTransformOrigin: crate::UnityEngine::UIElements::StyleTransformOrigin,
    pub m_HasInlineTranslate: bool,
    pub m_InlineTranslateOperation: crate::UnityEngine::UIElements::StyleTranslate,
    pub m_HasInlineRotate: bool,
    pub m_InlineRotateOperation: crate::UnityEngine::UIElements::StyleRotate,
    pub m_HasInlineScale: bool,
    pub m_InlineScale: crate::UnityEngine::UIElements::StyleScale,
    pub m_HasInlineBackgroundSize: bool,
    pub m_InlineBackgroundSize: crate::UnityEngine::UIElements::StyleBackgroundSize,
    pub m_InlineRule: crate::UnityEngine::UIElements::InlineStyleAccess_InlineRule,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+InlineStyleAccess")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::InlineStyleAccess {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "InlineStyleAccess";
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
#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess")]
impl std::ops::Deref for crate::UnityEngine::UIElements::InlineStyleAccess {
    type Target = crate::UnityEngine::UIElements::StyleValueCollection;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::InlineStyleAccess {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess")]
impl crate::UnityEngine::UIElements::InlineStyleAccess {
    #[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess+InlineRule")]
    pub type InlineRule = crate::UnityEngine::UIElements::InlineStyleAccess_InlineRule;
    pub fn ApplyFromComputedStyle(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        newStyle: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::ComputedStyle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::ComputedStyle,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "ApplyFromComputedStyle"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ApplyFromComputedStyle",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (id, newStyle))? };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyInlineStyles(
        &mut self,
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::ComputedStyle,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ApplyInlineStyles")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplyInlineStyles", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computedStyle))? };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleBackgroundSize(
        &mut self,
        backgroundSize: crate::UnityEngine::UIElements::StyleBackgroundSize,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleBackgroundSize),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ApplyStyleBackgroundSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplyStyleBackgroundSize", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (backgroundSize))? };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleCursor(
        &mut self,
        cursor: crate::UnityEngine::UIElements::StyleCursor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleCursor),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ApplyStyleCursor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplyStyleCursor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cursor))? };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleRotate(
        &mut self,
        rotate: crate::UnityEngine::UIElements::StyleRotate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleRotate),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ApplyStyleRotate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplyStyleRotate", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rotate))? };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleScale(
        &mut self,
        scale: crate::UnityEngine::UIElements::StyleScale,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleScale),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ApplyStyleScale")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplyStyleScale", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (scale))? };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleTextShadow(
        &mut self,
        textShadow: crate::UnityEngine::UIElements::StyleTextShadow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleTextShadow),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ApplyStyleTextShadow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplyStyleTextShadow", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (textShadow))? };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleTransformOrigin(
        &mut self,
        transformOrigin: crate::UnityEngine::UIElements::StyleTransformOrigin,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleTransformOrigin),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ApplyStyleTransformOrigin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplyStyleTransformOrigin", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (transformOrigin))? };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleTranslate(
        &mut self,
        translate: crate::UnityEngine::UIElements::StyleTranslate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleTranslate),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ApplyStyleTranslate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplyStyleTranslate", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (translate))? };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleValue_StyleValue0(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleSheets::StyleValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleSheets::StyleValue),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ApplyStyleValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplyStyleValue", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleValue_StyleValueManaged1(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleSheets::StyleValueManaged,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleSheets::StyleValueManaged),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ApplyStyleValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplyStyleValue", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
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
    pub fn GetStyleList<T>(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleList_1<T>>
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
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleSheets::StylePropertyId),
                        crate::UnityEngine::UIElements::StyleList_1<T>,
                        1usize,
                    >("GetStyleList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetStyleList", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleList_1<T> =
            unsafe { cordl_method_info.invoke_unchecked(self, (id))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsValueSet(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleSheets::StylePropertyId),
                        bool,
                        1usize,
                    >("IsValueSet")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsValueSet", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (id))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ve))?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveInlineStyle(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleSheets::StylePropertyId),
                        bool,
                        1usize,
                    >("RemoveInlineStyle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveInlineStyle", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (id))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetInlineBackgroundSize(
        &mut self,
        inlineValue: crate::UnityEngine::UIElements::StyleBackgroundSize,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleBackgroundSize),
                        bool,
                        1usize,
                    >("SetInlineBackgroundSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetInlineBackgroundSize", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetInlineCursor(
        &mut self,
        inlineValue: crate::UnityEngine::UIElements::StyleCursor,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleCursor), bool, 1usize>(
                        "SetInlineCursor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetInlineCursor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetInlineRotate(
        &mut self,
        inlineValue: crate::UnityEngine::UIElements::StyleRotate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleRotate), bool, 1usize>(
                        "SetInlineRotate",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetInlineRotate",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetInlineRule(
        &mut self,
        sheet: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
        rule: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleRule>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleRule>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetInlineRule")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetInlineRule",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (sheet, rule))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetInlineScale(
        &mut self,
        inlineValue: crate::UnityEngine::UIElements::StyleScale,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleScale), bool, 1usize>(
                        "SetInlineScale",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetInlineScale",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetInlineTextShadow(
        &mut self,
        inlineValue: crate::UnityEngine::UIElements::StyleTextShadow,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleTextShadow), bool, 1usize>(
                        "SetInlineTextShadow",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetInlineTextShadow",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetInlineTransformOrigin(
        &mut self,
        inlineValue: crate::UnityEngine::UIElements::StyleTransformOrigin,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleTransformOrigin),
                        bool,
                        1usize,
                    >("SetInlineTransformOrigin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetInlineTransformOrigin", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetInlineTranslate(
        &mut self,
        inlineValue: crate::UnityEngine::UIElements::StyleTranslate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleTranslate), bool, 1usize>(
                        "SetInlineTranslate",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetInlineTranslate",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValueManaged(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleSheets::StyleValueManaged,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleSheets::StyleValueManaged),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetStyleValueManaged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetStyleValueManaged", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValue_StyleBackground7(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        inlineValue: crate::UnityEngine::UIElements::StyleBackground,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                        crate::UnityEngine::UIElements::StyleBackground,
                    ), bool, 2usize>("SetStyleValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetStyleValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (id, inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValue_StyleBackgroundPosition0(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        inlineValue: crate::UnityEngine::UIElements::StyleBackgroundPosition,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                        crate::UnityEngine::UIElements::StyleBackgroundPosition,
                    ), bool, 2usize>("SetStyleValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetStyleValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (id, inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValue_StyleBackgroundRepeat1(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        inlineValue: crate::UnityEngine::UIElements::StyleBackgroundRepeat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                        crate::UnityEngine::UIElements::StyleBackgroundRepeat,
                    ), bool, 2usize>("SetStyleValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetStyleValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (id, inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValue_StyleColor5(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        inlineValue: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                        crate::UnityEngine::UIElements::StyleColor,
                    ), bool, 2usize>("SetStyleValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetStyleValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (id, inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValue_StyleEnum_1_6<T>(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        inlineValue: crate::UnityEngine::UIElements::StyleEnum_1<T>,
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
                    .find_method::<(
                        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                        crate::UnityEngine::UIElements::StyleEnum_1<T>,
                    ), bool, 2usize>("SetStyleValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetStyleValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (id, inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValue_StyleFloat3(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        inlineValue: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                        crate::UnityEngine::UIElements::StyleFloat,
                    ), bool, 2usize>("SetStyleValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetStyleValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (id, inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValue_StyleFont9(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        inlineValue: crate::UnityEngine::UIElements::StyleFont,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                        crate::UnityEngine::UIElements::StyleFont,
                    ), bool, 2usize>("SetStyleValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetStyleValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (id, inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValue_StyleFontDefinition8(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        inlineValue: crate::UnityEngine::UIElements::StyleFontDefinition,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                        crate::UnityEngine::UIElements::StyleFontDefinition,
                    ), bool, 2usize>("SetStyleValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetStyleValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (id, inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValue_StyleInt4(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        inlineValue: crate::UnityEngine::UIElements::StyleInt,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                        crate::UnityEngine::UIElements::StyleInt,
                    ), bool, 2usize>("SetStyleValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetStyleValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (id, inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValue_StyleLength2(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        inlineValue: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                        crate::UnityEngine::UIElements::StyleLength,
                    ), bool, 2usize>("SetStyleValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetStyleValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (id, inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValue_StyleList_1_10<T>(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        inlineValue: crate::UnityEngine::UIElements::StyleList_1<T>,
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
                    .find_method::<(
                        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                        crate::UnityEngine::UIElements::StyleList_1<T>,
                    ), bool, 2usize>("SetStyleValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetStyleValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (id, inlineValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInlineBackgroundSize(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::StyleBackgroundSize>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::StyleBackgroundSize,
                    >), bool, 1usize>("TryGetInlineBackgroundSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetInlineBackgroundSize",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInlineCursor(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::StyleCursor>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::StyleCursor,
                        >),
                        bool,
                        1usize,
                    >("TryGetInlineCursor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetInlineCursor", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInlineRotate(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::StyleRotate>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::StyleRotate,
                        >),
                        bool,
                        1usize,
                    >("TryGetInlineRotate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetInlineRotate", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInlineScale(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::StyleScale>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::StyleScale,
                        >),
                        bool,
                        1usize,
                    >("TryGetInlineScale")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetInlineScale", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInlineTextShadow(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::StyleTextShadow>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::StyleTextShadow,
                    >), bool, 1usize>("TryGetInlineTextShadow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetInlineTextShadow",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInlineTransformOrigin(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::StyleTransformOrigin,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::StyleTransformOrigin,
                    >), bool, 1usize>("TryGetInlineTransformOrigin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetInlineTransformOrigin",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInlineTranslate(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::StyleTranslate>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::StyleTranslate,
                    >), bool, 1usize>("TryGetInlineTranslate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetInlineTranslate",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetStyleValueManaged(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::StyleSheets::StyleValueManaged,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::StyleSheets::StyleValueManaged,
                        >,
                    ), bool, 2usize>("TryGetStyleValueManaged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetStyleValueManaged",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (id, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_alignContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::UIElements::Align>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::Align,
                    >, 0usize>("UnityEngine.UIElements.IStyle.get_alignContent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_alignContent",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::Align,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_alignItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::UIElements::Align>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::Align,
                    >, 0usize>("UnityEngine.UIElements.IStyle.get_alignItems")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_alignItems",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::Align,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_alignSelf(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::UIElements::Align>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::Align,
                    >, 0usize>("UnityEngine.UIElements.IStyle.get_alignSelf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_alignSelf",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::Align,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_backgroundColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleColor> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleColor, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_backgroundColor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_backgroundColor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleColor =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_backgroundImage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleBackground> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleBackground, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_backgroundImage",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_backgroundImage",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleBackground =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_backgroundPositionX(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleBackgroundPosition>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::StyleBackgroundPosition,
                        0usize,
                    >("UnityEngine.UIElements.IStyle.get_backgroundPositionX")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.get_backgroundPositionX",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleBackgroundPosition =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_backgroundPositionY(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleBackgroundPosition>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::StyleBackgroundPosition,
                        0usize,
                    >("UnityEngine.UIElements.IStyle.get_backgroundPositionY")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.get_backgroundPositionY",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleBackgroundPosition =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_backgroundRepeat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleBackgroundRepeat> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::StyleBackgroundRepeat,
                        0usize,
                    >("UnityEngine.UIElements.IStyle.get_backgroundRepeat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.get_backgroundRepeat", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleBackgroundRepeat =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_backgroundSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleBackgroundSize> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleBackgroundSize, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_backgroundSize",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_backgroundSize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleBackgroundSize =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_borderBottomColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleColor> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleColor, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_borderBottomColor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_borderBottomColor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleColor =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_borderBottomLeftRadius(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_borderBottomLeftRadius",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_borderBottomLeftRadius",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_borderBottomRightRadius(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_borderBottomRightRadius",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_borderBottomRightRadius",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_borderBottomWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleFloat> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleFloat, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_borderBottomWidth",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_borderBottomWidth",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleFloat =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_borderLeftColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleColor> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleColor, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_borderLeftColor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_borderLeftColor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleColor =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_borderLeftWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleFloat> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleFloat, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_borderLeftWidth",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_borderLeftWidth",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleFloat =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_borderRightColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleColor> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleColor, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_borderRightColor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_borderRightColor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleColor =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_borderRightWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleFloat> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleFloat, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_borderRightWidth",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_borderRightWidth",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleFloat =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_borderTopColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleColor> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleColor, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_borderTopColor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_borderTopColor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleColor =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_borderTopLeftRadius(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_borderTopLeftRadius",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_borderTopLeftRadius",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_borderTopRightRadius(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_borderTopRightRadius",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_borderTopRightRadius",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_borderTopWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleFloat> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleFloat, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_borderTopWidth",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_borderTopWidth",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleFloat =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_bottom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_bottom",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_bottom",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleColor> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleColor, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_color",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_color",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleColor =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_cursor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleCursor> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleCursor, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_cursor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_cursor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleCursor =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_display(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::UIElements::DisplayStyle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::DisplayStyle,
                    >, 0usize>("UnityEngine.UIElements.IStyle.get_display")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_display",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::DisplayStyle,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_flexBasis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_flexBasis",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_flexBasis",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_flexDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::UIElements::FlexDirection>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::FlexDirection,
                    >, 0usize>("UnityEngine.UIElements.IStyle.get_flexDirection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_flexDirection",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::FlexDirection,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_flexGrow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleFloat> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleFloat, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_flexGrow",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_flexGrow",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleFloat =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_flexShrink(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleFloat> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleFloat, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_flexShrink",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_flexShrink",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleFloat =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_flexWrap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::UIElements::Wrap>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::Wrap,
                    >, 0usize>("UnityEngine.UIElements.IStyle.get_flexWrap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_flexWrap",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::Wrap,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_fontSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_fontSize",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_fontSize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_height(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_height",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_height",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_justifyContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::UIElements::Justify>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::Justify,
                    >, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_justifyContent"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_justifyContent",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::Justify,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_left(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_left",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_left",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_letterSpacing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_letterSpacing",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_letterSpacing",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_marginBottom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_marginBottom",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_marginBottom",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_marginLeft(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_marginLeft",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_marginLeft",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_marginRight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_marginRight",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_marginRight",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_marginTop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_marginTop",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_marginTop",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_maxHeight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_maxHeight",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_maxHeight",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_maxWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_maxWidth",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_maxWidth",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_minHeight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_minHeight",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_minHeight",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_minWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_minWidth",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_minWidth",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_opacity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleFloat> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleFloat, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_opacity",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_opacity",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleFloat =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_overflow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::UIElements::Overflow>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::Overflow,
                    >, 0usize>("UnityEngine.UIElements.IStyle.get_overflow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_overflow",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::Overflow,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_paddingBottom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_paddingBottom",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_paddingBottom",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_paddingLeft(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_paddingLeft",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_paddingLeft",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_paddingRight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_paddingRight",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_paddingRight",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_paddingTop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_paddingTop",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_paddingTop",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::UIElements::Position>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::Position,
                    >, 0usize>("UnityEngine.UIElements.IStyle.get_position")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_position",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::Position,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_right(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_right",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_right",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_rotate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleRotate> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleRotate, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_rotate",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_rotate",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleRotate =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_scale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleScale> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleScale, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_scale",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_scale",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleScale =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_textOverflow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::UIElements::TextOverflow>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::TextOverflow,
                    >, 0usize>("UnityEngine.UIElements.IStyle.get_textOverflow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_textOverflow",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::TextOverflow,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_textShadow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleTextShadow> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleTextShadow, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_textShadow",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_textShadow",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleTextShadow =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_top(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_top",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_top",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_transformOrigin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleTransformOrigin> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::StyleTransformOrigin,
                        0usize,
                    >("UnityEngine.UIElements.IStyle.get_transformOrigin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.get_transformOrigin", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleTransformOrigin =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_transitionDelay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleList_1<crate::UnityEngine::UIElements::TimeValue>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleList_1<
                        crate::UnityEngine::UIElements::TimeValue,
                    >, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_transitionDelay"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_transitionDelay",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleList_1<
            crate::UnityEngine::UIElements::TimeValue,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_transitionDuration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleList_1<crate::UnityEngine::UIElements::TimeValue>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleList_1<
                        crate::UnityEngine::UIElements::TimeValue,
                    >, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_transitionDuration"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_transitionDuration",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleList_1<
            crate::UnityEngine::UIElements::TimeValue,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_transitionProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleList_1<
            crate::UnityEngine::UIElements::StylePropertyName,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleList_1<
                        crate::UnityEngine::UIElements::StylePropertyName,
                    >, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_transitionProperty"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_transitionProperty",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleList_1<
            crate::UnityEngine::UIElements::StylePropertyName,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_transitionTimingFunction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleList_1<crate::UnityEngine::UIElements::EasingFunction>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleList_1<
                        crate::UnityEngine::UIElements::EasingFunction,
                    >, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_transitionTimingFunction"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_transitionTimingFunction",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleList_1<
            crate::UnityEngine::UIElements::EasingFunction,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_translate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleTranslate> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleTranslate, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_translate",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_translate",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleTranslate =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_unityBackgroundImageTintColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleColor> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleColor, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_unityBackgroundImageTintColor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_unityBackgroundImageTintColor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleColor =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_unityEditorTextRenderingMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::EditorTextRenderingMode,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::EditorTextRenderingMode,
                    >, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_unityEditorTextRenderingMode"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_unityEditorTextRenderingMode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::EditorTextRenderingMode,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_unityFont(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleFont> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleFont, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_unityFont",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_unityFont",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleFont =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_unityFontDefinition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleFontDefinition> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleFontDefinition, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_unityFontDefinition",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_unityFontDefinition",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleFontDefinition =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_unityFontStyleAndWeight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::FontStyle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::StyleEnum_1<
                            crate::UnityEngine::FontStyle,
                        >,
                        0usize,
                    >("UnityEngine.UIElements.IStyle.get_unityFontStyleAndWeight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.get_unityFontStyleAndWeight",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::FontStyle,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_unityOverflowClipBox(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::OverflowClipBox,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::OverflowClipBox,
                    >, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_unityOverflowClipBox"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_unityOverflowClipBox",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::OverflowClipBox,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_unityParagraphSpacing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_unityParagraphSpacing",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_unityParagraphSpacing",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_unitySliceBottom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleInt> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleInt, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_unitySliceBottom",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_unitySliceBottom",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleInt =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_unitySliceLeft(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleInt> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleInt, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_unitySliceLeft",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_unitySliceLeft",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleInt =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_unitySliceRight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleInt> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleInt, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_unitySliceRight",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_unitySliceRight",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleInt =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_unitySliceScale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleFloat> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleFloat, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_unitySliceScale",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_unitySliceScale",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleFloat =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_unitySliceTop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleInt> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleInt, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_unitySliceTop",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_unitySliceTop",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleInt =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_unityTextAlign(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::TextAnchor>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::StyleEnum_1<
                            crate::UnityEngine::TextAnchor,
                        >,
                        0usize,
                    >("UnityEngine.UIElements.IStyle.get_unityTextAlign")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.get_unityTextAlign", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::TextAnchor,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_unityTextGenerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::TextGeneratorType>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::TextGeneratorType,
                    >, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_unityTextGenerator"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_unityTextGenerator",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::TextGeneratorType,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_unityTextOutlineColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleColor> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleColor, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_unityTextOutlineColor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_unityTextOutlineColor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleColor =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_unityTextOutlineWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleFloat> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleFloat, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_unityTextOutlineWidth",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_unityTextOutlineWidth",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleFloat =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_unityTextOverflowPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::TextOverflowPosition,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::TextOverflowPosition,
                    >, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_unityTextOverflowPosition"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_unityTextOverflowPosition",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::TextOverflowPosition,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_visibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::UIElements::Visibility>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::Visibility,
                    >, 0usize>("UnityEngine.UIElements.IStyle.get_visibility")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_visibility",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::Visibility,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_whiteSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::UIElements::WhiteSpace>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::WhiteSpace,
                    >, 0usize>("UnityEngine.UIElements.IStyle.get_whiteSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_whiteSpace",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::WhiteSpace,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_width(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_width",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_width",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_wordSpacing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::StyleLength, 0usize>(
                        "UnityEngine.UIElements.IStyle.get_wordSpacing",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.get_wordSpacing",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_alignContent(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::UIElements::Align>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::Align,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_alignContent",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_alignContent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_alignItems(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::UIElements::Align>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::Align,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_alignItems",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_alignItems",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_alignSelf(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::UIElements::Align>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::Align,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_alignSelf",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_alignSelf",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_backgroundColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleColor),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_backgroundColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_backgroundColor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_backgroundImage(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleBackground,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleBackground),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_backgroundImage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_backgroundImage", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_backgroundPositionX(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleBackgroundPosition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleBackgroundPosition),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_backgroundPositionX")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_backgroundPositionX",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_backgroundPositionY(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleBackgroundPosition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleBackgroundPosition),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_backgroundPositionY")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_backgroundPositionY",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_backgroundRepeat(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleBackgroundRepeat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleBackgroundRepeat),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_backgroundRepeat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_backgroundRepeat", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_backgroundSize(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleBackgroundSize,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleBackgroundSize),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_backgroundSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_backgroundSize", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderBottomColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleColor),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_borderBottomColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_borderBottomColor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderBottomLeftRadius(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_borderBottomLeftRadius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_borderBottomLeftRadius",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderBottomRightRadius(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_borderBottomRightRadius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_borderBottomRightRadius",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderBottomWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleFloat),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_borderBottomWidth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_borderBottomWidth", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderLeftColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleColor),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_borderLeftColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_borderLeftColor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderLeftWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleFloat),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_borderLeftWidth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_borderLeftWidth", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderRightColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleColor),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_borderRightColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_borderRightColor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderRightWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleFloat),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_borderRightWidth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_borderRightWidth", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderTopColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleColor),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_borderTopColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_borderTopColor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderTopLeftRadius(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_borderTopLeftRadius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_borderTopLeftRadius",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderTopRightRadius(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_borderTopRightRadius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_borderTopRightRadius",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderTopWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleFloat),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_borderTopWidth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_borderTopWidth", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_bottom(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_bottom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_bottom", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_color(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleColor),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_color")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_color", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_cursor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleCursor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleCursor),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_cursor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_cursor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_display(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::DisplayStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::DisplayStyle,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_display",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_display",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_flexBasis(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_flexBasis")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_flexBasis", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_flexDirection(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::FlexDirection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::FlexDirection,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_flexDirection",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_flexDirection",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_flexGrow(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleFloat),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_flexGrow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_flexGrow", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_flexShrink(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleFloat),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_flexShrink")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_flexShrink", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_flexWrap(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::UIElements::Wrap>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::Wrap,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_flexWrap",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_flexWrap",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_fontSize(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_fontSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_fontSize", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_height(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_height")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_height", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_justifyContent(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::UIElements::Justify>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::Justify,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_justifyContent",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_justifyContent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_left(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_left")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_left", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_letterSpacing(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_letterSpacing")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_letterSpacing", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_marginBottom(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_marginBottom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_marginBottom", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_marginLeft(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_marginLeft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_marginLeft", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_marginRight(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_marginRight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_marginRight", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_marginTop(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_marginTop")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_marginTop", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_maxHeight(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_maxHeight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_maxHeight", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_maxWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_maxWidth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_maxWidth", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_minHeight(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_minHeight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_minHeight", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_minWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_minWidth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_minWidth", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_opacity(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleFloat),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_opacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_opacity", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_overflow(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::Overflow,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::Overflow,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_overflow",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_overflow",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_paddingBottom(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_paddingBottom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_paddingBottom", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_paddingLeft(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_paddingLeft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_paddingLeft", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_paddingRight(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_paddingRight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_paddingRight", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_paddingTop(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_paddingTop")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_paddingTop", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_position(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::Position,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::Position,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_position",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_position",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_right(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_right")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_right", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_rotate(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleRotate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleRotate),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_rotate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_rotate", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_scale(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleScale,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleScale),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_scale")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_scale", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_textOverflow(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::TextOverflow,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::TextOverflow,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_textOverflow",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_textOverflow",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_textShadow(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleTextShadow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleTextShadow),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_textShadow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_textShadow", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_top(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_top")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_top", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_transformOrigin(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleTransformOrigin,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleTransformOrigin),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_transformOrigin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_transformOrigin", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_transitionDelay(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleList_1<
            crate::UnityEngine::UIElements::TimeValue,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleList_1<
                        crate::UnityEngine::UIElements::TimeValue,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_transitionDelay",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_transitionDelay",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_transitionDuration(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleList_1<
            crate::UnityEngine::UIElements::TimeValue,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleList_1<
                        crate::UnityEngine::UIElements::TimeValue,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_transitionDuration",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_transitionDuration",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_transitionProperty(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleList_1<
            crate::UnityEngine::UIElements::StylePropertyName,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleList_1<
                        crate::UnityEngine::UIElements::StylePropertyName,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_transitionProperty",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_transitionProperty",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_transitionTimingFunction(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleList_1<
            crate::UnityEngine::UIElements::EasingFunction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleList_1<
                        crate::UnityEngine::UIElements::EasingFunction,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_transitionTimingFunction",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_transitionTimingFunction",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_translate(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleTranslate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleTranslate),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_translate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_translate", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unityBackgroundImageTintColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleColor),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_unityBackgroundImageTintColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_unityBackgroundImageTintColor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unityEditorTextRenderingMode(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::EditorTextRenderingMode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::EditorTextRenderingMode,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_unityEditorTextRenderingMode",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_unityEditorTextRenderingMode",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unityFont(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFont,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleFont),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_unityFont")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_unityFont", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unityFontDefinition(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFontDefinition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleFontDefinition),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_unityFontDefinition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_unityFontDefinition",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unityFontStyleAndWeight(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::FontStyle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleEnum_1<
                            crate::UnityEngine::FontStyle,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_unityFontStyleAndWeight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_unityFontStyleAndWeight",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unityOverflowClipBox(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::OverflowClipBox,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::OverflowClipBox,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_unityOverflowClipBox",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_unityOverflowClipBox",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unityParagraphSpacing(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_unityParagraphSpacing")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_unityParagraphSpacing",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unitySliceBottom(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleInt,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleInt),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_unitySliceBottom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_unitySliceBottom", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unitySliceLeft(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleInt,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleInt),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_unitySliceLeft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_unitySliceLeft", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unitySliceRight(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleInt,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleInt),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_unitySliceRight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_unitySliceRight", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unitySliceScale(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleFloat),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_unitySliceScale")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_unitySliceScale", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unitySliceTop(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleInt,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleInt),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_unitySliceTop")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_unitySliceTop", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unityTextAlign(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::TextAnchor>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleEnum_1<
                            crate::UnityEngine::TextAnchor,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_unityTextAlign")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_unityTextAlign", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unityTextGenerator(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<crate::UnityEngine::TextGeneratorType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::TextGeneratorType,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_unityTextGenerator",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_unityTextGenerator",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unityTextOutlineColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleColor),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_unityTextOutlineColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_unityTextOutlineColor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unityTextOutlineWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleFloat),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_unityTextOutlineWidth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_unityTextOutlineWidth",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unityTextOverflowPosition(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::TextOverflowPosition,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::TextOverflowPosition,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_unityTextOverflowPosition",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_unityTextOverflowPosition",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_visibility(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::Visibility,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::Visibility,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_visibility",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_visibility",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_whiteSpace(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::WhiteSpace,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::UIElements::StyleEnum_1<
                        crate::UnityEngine::UIElements::WhiteSpace,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UnityEngine.UIElements.IStyle.set_whiteSpace",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.IStyle.set_whiteSpace",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_width(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_width")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_width", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_wordSpacing(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleLength),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.UIElements.IStyle.set_wordSpacing")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IStyle.set_wordSpacing", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
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
            unsafe { cordl_method_info.invoke_unchecked(self, (ve))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ve(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >,
                        0usize,
                    >("get_ve")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "get_ve",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_ve(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_ve")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "set_ve",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+InlineStyleAccess")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::InlineStyleAccess {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess")]
impl AsRef<crate::UnityEngine::UIElements::IStyle>
    for crate::UnityEngine::UIElements::InlineStyleAccess
{
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IStyle {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess")]
impl AsMut<crate::UnityEngine::UIElements::IStyle>
    for crate::UnityEngine::UIElements::InlineStyleAccess
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IStyle {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+InlineStyleAccess+InlineRule")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InlineStyleAccess_InlineRule {
    pub sheet: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
    pub rule: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleRule>,
    pub propertyIds: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+InlineStyleAccess+InlineRule")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::InlineStyleAccess_InlineRule
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "InlineStyleAccess/InlineRule";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+InlineStyleAccess+InlineRule")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::InlineStyleAccess_InlineRule
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+InlineStyleAccess+InlineRule")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::InlineStyleAccess_InlineRule
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+InlineStyleAccess+InlineRule")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::InlineStyleAccess_InlineRule
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+InlineStyleAccess+InlineRule")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::InlineStyleAccess_InlineRule
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+InlineStyleAccess+InlineRule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::InlineStyleAccess_InlineRule
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess+InlineRule")]
impl crate::UnityEngine::UIElements::InlineStyleAccess_InlineRule {}

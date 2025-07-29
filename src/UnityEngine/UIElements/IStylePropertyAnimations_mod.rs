#[cfg(feature = "cordl_class_UnityEngine+UIElements+IStylePropertyAnimations")]
#[repr(C)]
#[derive(Debug)]
pub struct IStylePropertyAnimations {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+IStylePropertyAnimations")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::IStylePropertyAnimations {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "IStylePropertyAnimations";
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
#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimations")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IStylePropertyAnimations {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimations")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IStylePropertyAnimations {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimations")]
impl crate::UnityEngine::UIElements::IStylePropertyAnimations {
    pub fn CancelAllAnimations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("CancelAllAnimations")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CancelAllAnimations", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CancelAnimation(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleSheets::StylePropertyId),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CancelAnimation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CancelAnimation", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllAnimations(
        &mut self,
        outPropertyIds: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("GetAllAnimations")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAllAnimations", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (outPropertyIds))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartEnum(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: i32,
        to: i32,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            i32,
                            i32,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<f32, f32>,
                        ),
                        bool,
                        6usize,
                    >("StartEnum")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartEnum", 6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (id, from, to, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start_BackgroundPosition_BackgroundPosition12(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::BackgroundPosition,
        to: crate::UnityEngine::UIElements::BackgroundPosition,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::BackgroundPosition,
                            crate::UnityEngine::UIElements::BackgroundPosition,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<f32, f32>,
                        ),
                        bool,
                        6usize,
                    >("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (id, from, to, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start_BackgroundRepeat_BackgroundRepeat13(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::BackgroundRepeat,
        to: crate::UnityEngine::UIElements::BackgroundRepeat,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::BackgroundRepeat,
                            crate::UnityEngine::UIElements::BackgroundRepeat,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<f32, f32>,
                        ),
                        bool,
                        6usize,
                    >("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (id, from, to, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start_BackgroundSize_BackgroundSize14(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::BackgroundSize,
        to: crate::UnityEngine::UIElements::BackgroundSize,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::BackgroundSize,
                            crate::UnityEngine::UIElements::BackgroundSize,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<f32, f32>,
                        ),
                        bool,
                        6usize,
                    >("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (id, from, to, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start_Background_Background4(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::Background,
        to: crate::UnityEngine::UIElements::Background,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::Background,
                            crate::UnityEngine::UIElements::Background,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<f32, f32>,
                        ),
                        bool,
                        6usize,
                    >("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (id, from, to, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start_Color_Color3(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::Color,
        to: crate::UnityEngine::Color,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::Color,
                            crate::UnityEngine::Color,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<f32, f32>,
                        ),
                        bool,
                        6usize,
                    >("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (id, from, to, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start_FontDefinition_FontDefinition5(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::FontDefinition,
        to: crate::UnityEngine::UIElements::FontDefinition,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::FontDefinition,
                            crate::UnityEngine::UIElements::FontDefinition,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<f32, f32>,
                        ),
                        bool,
                        6usize,
                    >("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (id, from, to, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start_Font_Font6(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        to: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<f32, f32>,
                        ),
                        bool,
                        6usize,
                    >("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (id, from, to, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start_Length_Length2(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::Length,
        to: crate::UnityEngine::UIElements::Length,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::Length,
                            crate::UnityEngine::UIElements::Length,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<f32, f32>,
                        ),
                        bool,
                        6usize,
                    >("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (id, from, to, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start_Rotate_Rotate10(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::Rotate,
        to: crate::UnityEngine::UIElements::Rotate,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::Rotate,
                            crate::UnityEngine::UIElements::Rotate,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<f32, f32>,
                        ),
                        bool,
                        6usize,
                    >("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (id, from, to, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start_Scale_Scale8(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::Scale,
        to: crate::UnityEngine::UIElements::Scale,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::Scale,
                            crate::UnityEngine::UIElements::Scale,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<f32, f32>,
                        ),
                        bool,
                        6usize,
                    >("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (id, from, to, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start_TextShadow_TextShadow7(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::TextShadow,
        to: crate::UnityEngine::UIElements::TextShadow,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::TextShadow,
                            crate::UnityEngine::UIElements::TextShadow,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<f32, f32>,
                        ),
                        bool,
                        6usize,
                    >("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (id, from, to, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start_TransformOrigin_TransformOrigin11(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::TransformOrigin,
        to: crate::UnityEngine::UIElements::TransformOrigin,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::TransformOrigin,
                            crate::UnityEngine::UIElements::TransformOrigin,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<f32, f32>,
                        ),
                        bool,
                        6usize,
                    >("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (id, from, to, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start_Translate_Translate9(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::Translate,
        to: crate::UnityEngine::UIElements::Translate,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::Translate,
                            crate::UnityEngine::UIElements::Translate,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<f32, f32>,
                        ),
                        bool,
                        6usize,
                    >("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (id, from, to, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start_f32_f32_0(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: f32,
        to: f32,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            f32,
                            f32,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<f32, f32>,
                        ),
                        bool,
                        6usize,
                    >("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (id, from, to, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start_i32_i32_1(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: i32,
        to: i32,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            i32,
                            i32,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<f32, f32>,
                        ),
                        bool,
                        6usize,
                    >("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (id, from, to, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAnimation(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::StyleSheets::StylePropertyId),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UpdateAnimation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateAnimation", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_completedAnimationCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_completedAnimationCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_completedAnimationCount", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_runningAnimationCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_runningAnimationCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_runningAnimationCount", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_completedAnimationCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_completedAnimationCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_completedAnimationCount", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_runningAnimationCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_runningAnimationCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_runningAnimationCount", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+IStylePropertyAnimations")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IStylePropertyAnimations {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

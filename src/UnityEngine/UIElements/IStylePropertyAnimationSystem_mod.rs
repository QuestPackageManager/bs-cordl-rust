#[cfg(feature = "cordl_class_UnityEngine+UIElements+IStylePropertyAnimationSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct IStylePropertyAnimationSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+IStylePropertyAnimationSystem")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::IStylePropertyAnimationSystem {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "IStylePropertyAnimationSystem";
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
#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimationSystem")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IStylePropertyAnimationSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimationSystem")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::IStylePropertyAnimationSystem {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimationSystem")]
impl crate::UnityEngine::UIElements::IStylePropertyAnimationSystem {
    pub fn CancelAllAnimations_0(
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
    pub fn CancelAllAnimations_VisualElement1(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CancelAllAnimations")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CancelAllAnimations", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (owner))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CancelAnimation(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CancelAnimation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CancelAnimation", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (owner, id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllAnimations(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        propertyIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("GetAllAnimations")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAllAnimations", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (owner, propertyIds))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_BackgroundPosition_BackgroundPosition12(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::BackgroundPosition,
        endValue: crate::UnityEngine::UIElements::BackgroundPosition,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::BackgroundPosition,
                            crate::UnityEngine::UIElements::BackgroundPosition,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
                        ),
                        bool,
                        7usize,
                    >("StartTransition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartTransition", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_BackgroundRepeat_BackgroundRepeat13(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::BackgroundRepeat,
        endValue: crate::UnityEngine::UIElements::BackgroundRepeat,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::BackgroundRepeat,
                            crate::UnityEngine::UIElements::BackgroundRepeat,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
                        ),
                        bool,
                        7usize,
                    >("StartTransition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartTransition", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_BackgroundSize_BackgroundSize14(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::BackgroundSize,
        endValue: crate::UnityEngine::UIElements::BackgroundSize,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::BackgroundSize,
                            crate::UnityEngine::UIElements::BackgroundSize,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
                        ),
                        bool,
                        7usize,
                    >("StartTransition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartTransition", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_Background_Background4(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::Background,
        endValue: crate::UnityEngine::UIElements::Background,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::Background,
                            crate::UnityEngine::UIElements::Background,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
                        ),
                        bool,
                        7usize,
                    >("StartTransition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartTransition", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_Color_Color3(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::Color,
        endValue: crate::UnityEngine::Color,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::Color,
                            crate::UnityEngine::Color,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
                        ),
                        bool,
                        7usize,
                    >("StartTransition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartTransition", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_FontDefinition_FontDefinition5(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::FontDefinition,
        endValue: crate::UnityEngine::UIElements::FontDefinition,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::FontDefinition,
                            crate::UnityEngine::UIElements::FontDefinition,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
                        ),
                        bool,
                        7usize,
                    >("StartTransition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartTransition", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_Font_Font6(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        endValue: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
                        ),
                        bool,
                        7usize,
                    >("StartTransition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartTransition", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_Length_Length2(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::Length,
        endValue: crate::UnityEngine::UIElements::Length,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::Length,
                            crate::UnityEngine::UIElements::Length,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
                        ),
                        bool,
                        7usize,
                    >("StartTransition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartTransition", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_Rotate_Rotate11(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::Rotate,
        endValue: crate::UnityEngine::UIElements::Rotate,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::Rotate,
                            crate::UnityEngine::UIElements::Rotate,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
                        ),
                        bool,
                        7usize,
                    >("StartTransition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartTransition", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_Scale_Scale8(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::Scale,
        endValue: crate::UnityEngine::UIElements::Scale,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::Scale,
                            crate::UnityEngine::UIElements::Scale,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
                        ),
                        bool,
                        7usize,
                    >("StartTransition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartTransition", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_TextShadow_TextShadow7(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::TextShadow,
        endValue: crate::UnityEngine::UIElements::TextShadow,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::TextShadow,
                            crate::UnityEngine::UIElements::TextShadow,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
                        ),
                        bool,
                        7usize,
                    >("StartTransition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartTransition", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_TransformOrigin_TransformOrigin9(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::TransformOrigin,
        endValue: crate::UnityEngine::UIElements::TransformOrigin,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::TransformOrigin,
                            crate::UnityEngine::UIElements::TransformOrigin,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
                        ),
                        bool,
                        7usize,
                    >("StartTransition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartTransition", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_Translate_Translate10(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::Translate,
        endValue: crate::UnityEngine::UIElements::Translate,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            crate::UnityEngine::UIElements::Translate,
                            crate::UnityEngine::UIElements::Translate,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
                        ),
                        bool,
                        7usize,
                    >("StartTransition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartTransition", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_f32_f32_0(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: f32,
        endValue: f32,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            f32,
                            f32,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
                        ),
                        bool,
                        7usize,
                    >("StartTransition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartTransition", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_i32_i32_1(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: i32,
        endValue: i32,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                            i32,
                            i32,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
                        ),
                        bool,
                        7usize,
                    >("StartTransition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartTransition", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Update")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Update",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAnimation(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("UpdateAnimation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateAnimation", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (owner, id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+IStylePropertyAnimationSystem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IStylePropertyAnimationSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

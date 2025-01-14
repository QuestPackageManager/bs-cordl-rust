#[cfg(feature = "UnityEngine+UIElements+StyleSheets+InitialStyle")]
#[repr(C)]
#[derive(Debug)]
pub struct InitialStyle {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+InitialStyle")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::StyleSheets::InitialStyle {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.StyleSheets";
    const CLASS_NAME: &'static str = "InitialStyle";
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
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+InitialStyle")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleSheets::InitialStyle {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+InitialStyle")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::StyleSheets::InitialStyle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+InitialStyle")]
impl crate::UnityEngine::UIElements::StyleSheets::InitialStyle {
    pub fn Acquire() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::ComputedStyle,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::ComputedStyle,
                0usize,
            >("Acquire")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Acquire", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::ComputedStyle = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Get() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::ComputedStyle>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::UIElements::ComputedStyle,
                >,
                0usize,
            >("Get")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Get", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_alignContent() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Align,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Align,
                0usize,
            >("get_alignContent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_alignContent", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Align = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_alignItems() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Align,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Align,
                0usize,
            >("get_alignItems")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_alignItems", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Align = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_alignSelf() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Align,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Align,
                0usize,
            >("get_alignSelf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_alignSelf", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Align = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::Color,
                0usize,
            >("get_backgroundColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_backgroundColor", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundImage() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Background,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Background,
                0usize,
            >("get_backgroundImage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_backgroundImage", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Background = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundPositionX() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundPosition,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::BackgroundPosition,
                0usize,
            >("get_backgroundPositionX")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_backgroundPositionX", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundPosition = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundPositionY() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundPosition,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::BackgroundPosition,
                0usize,
            >("get_backgroundPositionY")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_backgroundPositionY", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundPosition = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundRepeat() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundRepeat,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::BackgroundRepeat,
                0usize,
            >("get_backgroundRepeat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_backgroundRepeat", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundRepeat = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundSize() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundSize,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::BackgroundSize,
                0usize,
            >("get_backgroundSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_backgroundSize", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundSize = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_borderBottomColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::Color,
                0usize,
            >("get_borderBottomColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_borderBottomColor", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_borderBottomLeftRadius() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_borderBottomLeftRadius")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_borderBottomLeftRadius", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_borderBottomRightRadius() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_borderBottomRightRadius")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_borderBottomRightRadius", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_borderBottomWidth() -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), f32, 0usize>("get_borderBottomWidth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_borderBottomWidth", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_borderLeftColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::Color,
                0usize,
            >("get_borderLeftColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_borderLeftColor", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_borderLeftWidth() -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), f32, 0usize>("get_borderLeftWidth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_borderLeftWidth", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_borderRightColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::Color,
                0usize,
            >("get_borderRightColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_borderRightColor", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_borderRightWidth() -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), f32, 0usize>("get_borderRightWidth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_borderRightWidth", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_borderTopColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::Color,
                0usize,
            >("get_borderTopColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_borderTopColor", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_borderTopLeftRadius() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_borderTopLeftRadius")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_borderTopLeftRadius", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_borderTopRightRadius() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_borderTopRightRadius")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_borderTopRightRadius", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_borderTopWidth() -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), f32, 0usize>("get_borderTopWidth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_borderTopWidth", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_bottom() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_bottom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_bottom", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_color() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), crate::UnityEngine::Color, 0usize>("get_color")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_color", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_cursor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Cursor,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Cursor,
                0usize,
            >("get_cursor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_cursor", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Cursor = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_display() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::DisplayStyle,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::DisplayStyle,
                0usize,
            >("get_display")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_display", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::DisplayStyle = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_flexBasis() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_flexBasis")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_flexBasis", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_flexDirection() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::FlexDirection,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::FlexDirection,
                0usize,
            >("get_flexDirection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_flexDirection", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::FlexDirection = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_flexGrow() -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), f32, 0usize>("get_flexGrow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_flexGrow", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_flexShrink() -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), f32, 0usize>("get_flexShrink")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_flexShrink", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_flexWrap() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Wrap,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Wrap,
                0usize,
            >("get_flexWrap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_flexWrap", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Wrap = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_fontSize() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_fontSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_fontSize", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_height() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_height")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_height", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_justifyContent() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Justify,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Justify,
                0usize,
            >("get_justifyContent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_justifyContent", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Justify = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_left() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_left")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_left", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_letterSpacing() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_letterSpacing")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_letterSpacing", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_marginBottom() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_marginBottom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_marginBottom", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_marginLeft() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_marginLeft")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_marginLeft", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_marginRight() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_marginRight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_marginRight", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_marginTop() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_marginTop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_marginTop", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_maxHeight() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_maxHeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_maxHeight", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_maxWidth() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_maxWidth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_maxWidth", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_minHeight() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_minHeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_minHeight", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_minWidth() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_minWidth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_minWidth", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_opacity() -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), f32, 0usize>("get_opacity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_opacity", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_overflow() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::OverflowInternal,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::OverflowInternal,
                0usize,
            >("get_overflow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_overflow", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::OverflowInternal = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_paddingBottom() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_paddingBottom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_paddingBottom", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_paddingLeft() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_paddingLeft")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_paddingLeft", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_paddingRight() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_paddingRight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_paddingRight", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_paddingTop() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_paddingTop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_paddingTop", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_position() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Position,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Position,
                0usize,
            >("get_position")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_position", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Position = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_right() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_right")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_right", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_rotate() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Rotate,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Rotate,
                0usize,
            >("get_rotate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_rotate", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Rotate = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_scale() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Scale,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Scale,
                0usize,
            >("get_scale")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_scale", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Scale = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_textOverflow() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::TextOverflow,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::TextOverflow,
                0usize,
            >("get_textOverflow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_textOverflow", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::TextOverflow = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_textShadow() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::TextShadow,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::TextShadow,
                0usize,
            >("get_textShadow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_textShadow", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::TextShadow = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_top() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_top")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_top", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_transformOrigin() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::TransformOrigin,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::TransformOrigin,
                0usize,
            >("get_transformOrigin")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_transformOrigin", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::TransformOrigin = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_transitionDelay() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::TimeValue,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::UnityEngine::UIElements::TimeValue,
                    >,
                >,
                0usize,
            >("get_transitionDelay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_transitionDelay", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::TimeValue,
            >,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_transitionDuration() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::TimeValue,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::UnityEngine::UIElements::TimeValue,
                    >,
                >,
                0usize,
            >("get_transitionDuration")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_transitionDuration", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::TimeValue,
            >,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_transitionProperty() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::StylePropertyName,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::UnityEngine::UIElements::StylePropertyName,
                    >,
                >,
                0usize,
            >("get_transitionProperty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_transitionProperty", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::StylePropertyName,
            >,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_transitionTimingFunction() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::EasingFunction,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::UnityEngine::UIElements::EasingFunction,
                    >,
                >,
                0usize,
            >("get_transitionTimingFunction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_transitionTimingFunction", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::EasingFunction,
            >,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_translate() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Translate,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Translate,
                0usize,
            >("get_translate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_translate", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Translate = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_unityBackgroundImageTintColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::Color,
                0usize,
            >("get_unityBackgroundImageTintColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_unityBackgroundImageTintColor", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_unityFont() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
                0usize,
            >("get_unityFont")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_unityFont", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font> = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_unityFontDefinition() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::FontDefinition,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::FontDefinition,
                0usize,
            >("get_unityFontDefinition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_unityFontDefinition", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::FontDefinition = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_unityFontStyleAndWeight() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::FontStyle,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::FontStyle,
                0usize,
            >("get_unityFontStyleAndWeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_unityFontStyleAndWeight", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::FontStyle = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_unityOverflowClipBox() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::OverflowClipBox,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::OverflowClipBox,
                0usize,
            >("get_unityOverflowClipBox")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_unityOverflowClipBox", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::OverflowClipBox = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_unityParagraphSpacing() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_unityParagraphSpacing")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_unityParagraphSpacing", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_unitySliceBottom() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("get_unitySliceBottom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_unitySliceBottom", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_unitySliceLeft() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("get_unitySliceLeft")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_unitySliceLeft", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_unitySliceRight() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("get_unitySliceRight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_unitySliceRight", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_unitySliceScale() -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), f32, 0usize>("get_unitySliceScale")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_unitySliceScale", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_unitySliceTop() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("get_unitySliceTop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_unitySliceTop", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_unityTextAlign() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextAnchor,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::TextAnchor,
                0usize,
            >("get_unityTextAlign")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_unityTextAlign", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::TextAnchor = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_unityTextOutlineColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::Color,
                0usize,
            >("get_unityTextOutlineColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_unityTextOutlineColor", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_unityTextOutlineWidth() -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), f32, 0usize>("get_unityTextOutlineWidth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_unityTextOutlineWidth", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_unityTextOverflowPosition() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::TextOverflowPosition,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::TextOverflowPosition,
                0usize,
            >("get_unityTextOverflowPosition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_unityTextOverflowPosition", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::TextOverflowPosition = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_visibility() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Visibility,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Visibility,
                0usize,
            >("get_visibility")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_visibility", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Visibility = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_whiteSpace() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::WhiteSpace,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::WhiteSpace,
                0usize,
            >("get_whiteSpace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_whiteSpace", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::WhiteSpace = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_width() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_width")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_width", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_wordSpacing() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::UIElements::Length,
                0usize,
            >("get_wordSpacing")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_wordSpacing", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+InitialStyle")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleSheets::InitialStyle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

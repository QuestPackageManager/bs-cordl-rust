#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle")]
#[repr(C)]
#[derive(Debug)]
pub struct LayoutStyle {
    __cordl_parent: crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Style,
    pub flexDirection:
        crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle_Direction,
    pub layout: crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle_Layout,
    pub anchor: crate::UnityEngine::TextAnchor,
    pub pivot: crate::UnityEngine::TextAnchor,
    pub _cordl_size: crate::UnityEngine::Vector2,
    pub margin: crate::UnityEngine::Vector2,
    pub useBottomRightMargin: bool,
    pub bottomRightMargin: crate::UnityEngine::Vector2,
    pub spacing: f32,
    pub masks: bool,
    pub adaptHeight: bool,
    pub autoFitChildren: bool,
    pub isOverlayCanvas: bool,
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger.UserInterface.Generic";
    const CLASS_NAME: &'static str = "LayoutStyle";
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
#[cfg(feature = "Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle")]
impl std::ops::Deref for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle {
    type Target = crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Style;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle")]
impl std::ops::DerefMut
    for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle")]
impl crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle {
    #[cfg(feature = "Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle+Direction")]
    pub type Direction =
        crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle_Direction;
    #[cfg(feature = "Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle+Layout")]
    pub type Layout =
        crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle_Layout;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetHeight(&mut self, height: f32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f32), bool, 1usize>("SetHeight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetHeight",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (height))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetIndent(&mut self, value: f32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f32), bool, 1usize>("SetIndent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetIndent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetWidth(&mut self, width: f32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f32), bool, 1usize>("SetWidth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetWidth",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (width))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_BottomMargin(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_BottomMargin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_BottomMargin",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_BottomRightMargin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::Vector2, 0usize>("get_BottomRightMargin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_BottomRightMargin",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_LeftMargin(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_LeftMargin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_LeftMargin",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_RightMargin(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_RightMargin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_RightMargin",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_TopLeftMargin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::Vector2, 0usize>("get_TopLeftMargin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_TopLeftMargin",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_TopMargin(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_TopMargin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_TopMargin",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle+Direction"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum LayoutStyle_Direction {
    #[default]
    Down = 2i32,
    Left = 0i32,
    Right = 1i32,
    Up = 3i32,
}
#[cfg(
    feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle+Direction"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle_Direction
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger.UserInterface.Generic";
    const CLASS_NAME: &'static str = "LayoutStyle/Direction";
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
#[cfg(
    feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle+Direction"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle_Direction
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle+Direction"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle_Direction
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
#[cfg(
    feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle+Direction"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle_Direction
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
#[cfg(
    feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle+Direction"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle_Direction
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
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle+Layout")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum LayoutStyle_Layout {
    #[default]
    Fill = 1i32,
    FillHorizontal = 2i32,
    FillVertical = 3i32,
    Fixed = 0i32,
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle+Layout")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle_Layout
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger.UserInterface.Generic";
    const CLASS_NAME: &'static str = "LayoutStyle/Layout";
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
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle+Layout")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle_Layout
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle+Layout")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle_Layout
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
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle+Layout")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle_Layout
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
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+LayoutStyle+Layout")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::LayoutStyle_Layout
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

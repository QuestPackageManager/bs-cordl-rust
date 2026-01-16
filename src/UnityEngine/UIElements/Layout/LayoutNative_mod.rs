#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutNative")]
#[repr(C)]
#[derive(Debug)]
pub struct LayoutNative {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutNative")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::Layout::LayoutNative {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Layout";
    const CLASS_NAME: &'static str = "LayoutNative";
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
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutNative")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Layout::LayoutNative {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutNative")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Layout::LayoutNative {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutNative")]
impl crate::UnityEngine::UIElements::Layout::LayoutNative {
    #[cfg(feature = "UnityEngine+UIElements+Layout+LayoutNative+LayoutLogData")]
    pub type LayoutLogData = crate::UnityEngine::UIElements::Layout::LayoutNative_LayoutLogData;
    #[cfg(feature = "UnityEngine+UIElements+Layout+LayoutNative+LayoutLogEventType")]
    pub type LayoutLogEventType =
        crate::UnityEngine::UIElements::Layout::LayoutNative_LayoutLogEventType;
    pub fn CalculateLayout(
        node: crate::System::IntPtr,
        parentWidth: f32,
        parentHeight: f32,
        parentDirection: i32,
        state: crate::System::IntPtr,
        exceptionGCHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        f32,
                        f32,
                        i32,
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 6usize>("CalculateLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CalculateLayout",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    node,
                    parentWidth,
                    parentHeight,
                    parentDirection,
                    state,
                    exceptionGCHandle,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LayoutLog_Internal(
        nodePtr: crate::System::IntPtr,
        _cordl_type: crate::UnityEngine::UIElements::Layout::LayoutNative_LayoutLogEventType,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::UnityEngine::UIElements::Layout::LayoutNative_LayoutLogEventType,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 3usize>("LayoutLog_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LayoutLog_Internal",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (nodePtr, _cordl_type, message))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutNative")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::Layout::LayoutNative {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutNative+LayoutLogData")]
#[repr(C)]
#[derive(Debug)]
pub struct LayoutNative_LayoutLogData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub node: crate::UnityEngine::UIElements::Layout::LayoutNode,
    pub eventType: crate::UnityEngine::UIElements::Layout::LayoutNative_LayoutLogEventType,
    pub message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutNative+LayoutLogData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::Layout::LayoutNative_LayoutLogData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Layout";
    const CLASS_NAME: &'static str = "LayoutNative/LayoutLogData";
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
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutNative+LayoutLogData")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Layout::LayoutNative_LayoutLogData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutNative+LayoutLogData")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Layout::LayoutNative_LayoutLogData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutNative+LayoutLogData")]
impl crate::UnityEngine::UIElements::Layout::LayoutNative_LayoutLogData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutNative+LayoutLogData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::UIElements::Layout::LayoutNative_LayoutLogData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutNative+LayoutLogEventType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum LayoutNative_LayoutLogEventType {
    #[default]
    BeginLayout = 5i32,
    CacheUsage = 4i32,
    EndLayout = 6i32,
    Error = 1i32,
    Layout = 3i32,
    Measure = 2i32,
    None = 0i32,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutNative+LayoutLogEventType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::Layout::LayoutNative_LayoutLogEventType
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Layout";
    const CLASS_NAME: &'static str = "LayoutNative/LayoutLogEventType";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutNative+LayoutLogEventType")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::Layout::LayoutNative_LayoutLogEventType
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutNative+LayoutLogEventType")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::Layout::LayoutNative_LayoutLogEventType
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutNative+LayoutLogEventType")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::Layout::LayoutNative_LayoutLogEventType
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutNative+LayoutLogEventType")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::Layout::LayoutNative_LayoutLogEventType
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

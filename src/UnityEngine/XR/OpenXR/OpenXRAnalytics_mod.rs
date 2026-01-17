#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+OpenXRAnalytics")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct OpenXRAnalytics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+OpenXRAnalytics")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::XR::OpenXR::OpenXRAnalytics {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.OpenXR";
    const CLASS_NAME: &'static str = "OpenXRAnalytics";
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
#[cfg(feature = "UnityEngine+XR+OpenXR+OpenXRAnalytics")]
impl std::ops::Deref for crate::UnityEngine::XR::OpenXR::OpenXRAnalytics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+OpenXR+OpenXRAnalytics")]
impl std::ops::DerefMut for crate::UnityEngine::XR::OpenXR::OpenXRAnalytics {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+OpenXR+OpenXRAnalytics")]
impl crate::UnityEngine::XR::OpenXR::OpenXRAnalytics {
    pub const kEventInitialize: &'static str = "openxr_initialize";
    pub const kMaxEventsPerHour: i32 = 1000i32;
    pub const kMaxNumberOfElements: i32 = 1000i32;
    pub const kVendorKey: &'static str = "unity.openxr";
    #[cfg(feature = "UnityEngine+XR+OpenXR+OpenXRAnalytics+InitializeEvent")]
    pub type InitializeEvent = crate::UnityEngine::XR::OpenXR::OpenXRAnalytics_InitializeEvent;
    pub fn CreateInitializeEvent(
        success: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::XR::OpenXR::OpenXRAnalytics_InitializeEvent,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (bool),
                        crate::UnityEngine::XR::OpenXR::OpenXRAnalytics_InitializeEvent,
                        1usize,
                    >("CreateInitializeEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateInitializeEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::XR::OpenXR::OpenXRAnalytics_InitializeEvent =
            unsafe { cordl_method_info.invoke_unchecked((), (success))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Initialize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SendInitializeEvent(
        success: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "SendInitializeEvent",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SendInitializeEvent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (success))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+OpenXRAnalytics")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::XR::OpenXR::OpenXRAnalytics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+OpenXRAnalytics+InitializeEvent")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct OpenXRAnalytics_InitializeEvent {
    pub success: bool,
    pub runtime: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub runtime_version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub plugin_version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub api_version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub available_extensions: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub enabled_extensions: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub enabled_features: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub failed_features: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+OpenXRAnalytics+InitializeEvent")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::XR::OpenXR::OpenXRAnalytics_InitializeEvent
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.OpenXR";
    const CLASS_NAME: &'static str = "OpenXRAnalytics/InitializeEvent";
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+OpenXRAnalytics+InitializeEvent")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::XR::OpenXR::OpenXRAnalytics_InitializeEvent
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+OpenXRAnalytics+InitializeEvent")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::XR::OpenXR::OpenXRAnalytics_InitializeEvent
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+OpenXRAnalytics+InitializeEvent")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::XR::OpenXR::OpenXRAnalytics_InitializeEvent
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+OpenXRAnalytics+InitializeEvent")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::XR::OpenXR::OpenXRAnalytics_InitializeEvent
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+OpenXRAnalytics+InitializeEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::XR::OpenXR::OpenXRAnalytics_InitializeEvent
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+OpenXR+OpenXRAnalytics+InitializeEvent")]
impl crate::UnityEngine::XR::OpenXR::OpenXRAnalytics_InitializeEvent {}

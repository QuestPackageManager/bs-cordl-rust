#[cfg(
    feature = "cordl_class_UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature+NativeApi+XrPerformanceNotificationDelegate"
)]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct NativeApi_XrPerformanceSettingsFeature_XrPerformanceNotificationDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "cordl_class_UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature+NativeApi+XrPerformanceNotificationDelegate"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::NativeApi_XrPerformanceSettingsFeature_XrPerformanceNotificationDelegate {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.OpenXR.Features.Extensions.PerformanceSettings";
    const CLASS_NAME: &'static str = "XrPerformanceSettingsFeature/NativeApi/XrPerformanceNotificationDelegate";
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
#[cfg(
    feature = "UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature+NativeApi+XrPerformanceNotificationDelegate"
)]
impl std::ops::Deref
for crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::NativeApi_XrPerformanceSettingsFeature_XrPerformanceNotificationDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature+NativeApi+XrPerformanceNotificationDelegate"
)]
impl std::ops::DerefMut
for crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::NativeApi_XrPerformanceSettingsFeature_XrPerformanceNotificationDelegate {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature+NativeApi+XrPerformanceNotificationDelegate"
)]
impl crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::NativeApi_XrPerformanceSettingsFeature_XrPerformanceNotificationDelegate {
    pub fn BeginInvoke(
        &mut self,
        notification: crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceChangeNotification,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceChangeNotification,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        3usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            cordl_method_info.invoke_unchecked(self, (notification, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        notification: crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceChangeNotification,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceChangeNotification),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (notification))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature+NativeApi+XrPerformanceNotificationDelegate"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::NativeApi_XrPerformanceSettingsFeature_XrPerformanceNotificationDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature"
)]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct XrPerformanceSettingsFeature {
    __cordl_parent: crate::UnityEngine::XR::OpenXR::Features::OpenXRFeature,
}
#[cfg(
    feature = "cordl_class_UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::XrPerformanceSettingsFeature {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.OpenXR.Features.Extensions.PerformanceSettings";
    const CLASS_NAME: &'static str = "XrPerformanceSettingsFeature";
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
#[cfg(
    feature = "UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature"
)]
impl std::ops::Deref
for crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::XrPerformanceSettingsFeature {
    type Target = crate::UnityEngine::XR::OpenXR::Features::OpenXRFeature;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature"
)]
impl std::ops::DerefMut
for crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::XrPerformanceSettingsFeature {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature"
)]
impl crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::XrPerformanceSettingsFeature {
    pub const extensionString: &'static str = "XR_EXT_performance_settings";
    pub const featureId: &'static str = "com.unity.openxr.feature.extension.performance_settings";
    #[cfg(
        feature = "UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature+NativeApi"
    )]
    pub type NativeApi = crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::XrPerformanceSettingsFeature_NativeApi;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnInstanceCreate(
        &mut self,
        xrInstance: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u64), bool, 1usize>("OnInstanceCreate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnInstanceCreate", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (xrInstance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnXrPerformanceNotificationCallback(
        notification: crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceChangeNotification,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceChangeNotification),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnXrPerformanceNotificationCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnXrPerformanceNotificationCallback", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (notification))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPerformanceLevelHint(
        domain: crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceDomain,
        level: crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceLevelHint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceDomain,
                            crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceLevelHint,
                        ),
                        bool,
                        2usize,
                    >("SetPerformanceLevelHint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetPerformanceLevelHint", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (domain, level))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_OnXrPerformanceChangeNotification(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<
                crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceChangeNotification,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Events::UnityAction_1<
                                crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceChangeNotification,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_OnXrPerformanceChangeNotification")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_OnXrPerformanceChangeNotification", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_OnXrPerformanceChangeNotification(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<
                crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceChangeNotification,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Events::UnityAction_1<
                                crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceChangeNotification,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_OnXrPerformanceChangeNotification")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_OnXrPerformanceChangeNotification", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::XrPerformanceSettingsFeature {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature+NativeApi"
)]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct XrPerformanceSettingsFeature_NativeApi {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature+NativeApi"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::XrPerformanceSettingsFeature_NativeApi {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.OpenXR.Features.Extensions.PerformanceSettings";
    const CLASS_NAME: &'static str = "XrPerformanceSettingsFeature/NativeApi";
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
#[cfg(
    feature = "UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature+NativeApi"
)]
impl std::ops::Deref
for crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::XrPerformanceSettingsFeature_NativeApi {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature+NativeApi"
)]
impl std::ops::DerefMut
for crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::XrPerformanceSettingsFeature_NativeApi {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature+NativeApi"
)]
impl crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::XrPerformanceSettingsFeature_NativeApi {
    #[cfg(
        feature = "UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature+NativeApi+XrPerformanceNotificationDelegate"
    )]
    pub type XrPerformanceNotificationDelegate = crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::NativeApi_XrPerformanceSettingsFeature_XrPerformanceNotificationDelegate;
    pub fn xr_performance_settings_setEventCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::NativeApi_XrPerformanceSettingsFeature_XrPerformanceNotificationDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::NativeApi_XrPerformanceSettingsFeature_XrPerformanceNotificationDelegate,
                        >),
                        bool,
                        1usize,
                    >("xr_performance_settings_setEventCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "xr_performance_settings_setEventCallback", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn xr_performance_settings_setPerformanceLevel(
        domain: crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceDomain,
        level: crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceLevelHint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceDomain,
                            crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::PerformanceLevelHint,
                        ),
                        bool,
                        2usize,
                    >("xr_performance_settings_setPerformanceLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "xr_performance_settings_setPerformanceLevel", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (domain, level))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+XR+OpenXR+Features+Extensions+PerformanceSettings+XrPerformanceSettingsFeature+NativeApi"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::XR::OpenXR::Features::Extensions::PerformanceSettings::XrPerformanceSettingsFeature_NativeApi {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

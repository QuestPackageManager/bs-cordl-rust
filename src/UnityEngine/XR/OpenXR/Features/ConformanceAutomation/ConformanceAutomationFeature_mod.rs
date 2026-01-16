#[cfg(
    feature = "cordl_class_UnityEngine+XR+OpenXR+Features+ConformanceAutomation+ConformanceAutomationFeature"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ConformanceAutomationFeature {
    __cordl_parent: crate::UnityEngine::XR::OpenXR::Features::OpenXRFeature,
}
#[cfg(
    feature = "cordl_class_UnityEngine+XR+OpenXR+Features+ConformanceAutomation+ConformanceAutomationFeature"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::OpenXR::Features::ConformanceAutomation::ConformanceAutomationFeature {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.OpenXR.Features.ConformanceAutomation";
    const CLASS_NAME: &'static str = "ConformanceAutomationFeature";
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
    feature = "UnityEngine+XR+OpenXR+Features+ConformanceAutomation+ConformanceAutomationFeature"
)]
impl std::ops::Deref
for crate::UnityEngine::XR::OpenXR::Features::ConformanceAutomation::ConformanceAutomationFeature {
    type Target = crate::UnityEngine::XR::OpenXR::Features::OpenXRFeature;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+XR+OpenXR+Features+ConformanceAutomation+ConformanceAutomationFeature"
)]
impl std::ops::DerefMut
for crate::UnityEngine::XR::OpenXR::Features::ConformanceAutomation::ConformanceAutomationFeature {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+XR+OpenXR+Features+ConformanceAutomation+ConformanceAutomationFeature"
)]
impl crate::UnityEngine::XR::OpenXR::Features::ConformanceAutomation::ConformanceAutomationFeature {
    pub const ExtLib: &'static str = "ConformanceAutomationExt";
    pub const featureId: &'static str = "com.unity.openxr.feature.conformance";
    pub fn ConformanceAutomationSetActive(
        interactionProfile: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        topLevelPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isActive: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        bool,
                    ), bool, 3usize>("ConformanceAutomationSetActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConformanceAutomationSetActive",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (interactionProfile, topLevelPath, isActive))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConformanceAutomationSetBool(
        topLevelPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        inputSourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        bool,
                    ), bool, 3usize>("ConformanceAutomationSetBool")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConformanceAutomationSetBool",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (topLevelPath, inputSourcePath, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConformanceAutomationSetFloat(
        topLevelPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        inputSourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        state: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), bool, 3usize>("ConformanceAutomationSetFloat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConformanceAutomationSetFloat",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (topLevelPath, inputSourcePath, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConformanceAutomationSetPose(
        topLevelPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        inputSourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        position: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                    ), bool, 4usize>("ConformanceAutomationSetPose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConformanceAutomationSetPose",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (topLevelPath, inputSourcePath, position, orientation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConformanceAutomationSetVec2(
        topLevelPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        inputSourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        state: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Vector2,
                    ), bool, 3usize>("ConformanceAutomationSetVec2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConformanceAutomationSetVec2",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (topLevelPath, inputSourcePath, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConformanceAutomationSetVelocity(
        topLevelPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        inputSourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        linearValid: bool,
        linear: crate::UnityEngine::Vector3,
        angularValid: bool,
        angular: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        bool,
                        crate::UnityEngine::Vector3,
                        bool,
                        crate::UnityEngine::Vector3,
                    ), bool, 6usize>("ConformanceAutomationSetVelocity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConformanceAutomationSetVelocity",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    topLevelPath,
                    inputSourcePath,
                    linearValid,
                    linear,
                    angularValid,
                    angular,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnInstanceCreate(&mut self, instance: u64) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u64), bool, 1usize>("OnInstanceCreate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnInstanceCreate",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnInstanceDestroy(
        &mut self,
        xrInstance: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u64), quest_hook::libil2cpp::Void, 1usize>("OnInstanceDestroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnInstanceDestroy",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (xrInstance))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnSessionCreate(
        &mut self,
        xrSessionId: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u64), quest_hook::libil2cpp::Void, 1usize>("OnSessionCreate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnSessionCreate",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (xrSessionId))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnSessionDestroy(
        &mut self,
        xrSessionId: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u64), quest_hook::libil2cpp::Void, 1usize>("OnSessionDestroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnSessionDestroy",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (xrSessionId))? };
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
    pub fn initialize(
        xrGetInstanceProcAddr: crate::System::IntPtr,
        xrInstance: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, u64),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "initialize", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (xrGetInstanceProcAddr, xrInstance))? };
        Ok(__cordl_ret.into())
    }
    pub fn xrSetInputDeviceActiveEXT(
        xrSession: u64,
        interactionProfile: u64,
        topLevelPath: u64,
        isActive: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64, u64, u64, bool), bool, 4usize>(
                        "xrSetInputDeviceActiveEXT",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "xrSetInputDeviceActiveEXT",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (xrSession, interactionProfile, topLevelPath, isActive))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn xrSetInputDeviceLocationEXT(
        xrSession: u64,
        topLevelPath: u64,
        inputSourcePath: u64,
        space: u64,
        pose: crate::UnityEngine::XR::OpenXR::NativeTypes::XrPosef,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u64,
                        u64,
                        u64,
                        u64,
                        crate::UnityEngine::XR::OpenXR::NativeTypes::XrPosef,
                    ), bool, 5usize>("xrSetInputDeviceLocationEXT")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "xrSetInputDeviceLocationEXT",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (xrSession, topLevelPath, inputSourcePath, space, pose))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn xrSetInputDeviceStateBoolEXT(
        xrSession: u64,
        topLevelPath: u64,
        inputSourcePath: u64,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64, u64, u64, bool), bool, 4usize>(
                        "xrSetInputDeviceStateBoolEXT",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "xrSetInputDeviceStateBoolEXT",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (xrSession, topLevelPath, inputSourcePath, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn xrSetInputDeviceStateFloatEXT(
        xrSession: u64,
        topLevelPath: u64,
        inputSourcePath: u64,
        state: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64, u64, u64, f32), bool, 4usize>(
                        "xrSetInputDeviceStateFloatEXT",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "xrSetInputDeviceStateFloatEXT",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (xrSession, topLevelPath, inputSourcePath, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn xrSetInputDeviceStateVector2fEXT(
        xrSession: u64,
        topLevelPath: u64,
        inputSourcePath: u64,
        state: crate::UnityEngine::XR::OpenXR::NativeTypes::XrVector2f,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u64,
                        u64,
                        u64,
                        crate::UnityEngine::XR::OpenXR::NativeTypes::XrVector2f,
                    ), bool, 4usize>("xrSetInputDeviceStateVector2fEXT")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "xrSetInputDeviceStateVector2fEXT",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (xrSession, topLevelPath, inputSourcePath, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn xrSetInputDeviceVelocityUNITY(
        xrSession: u64,
        topLevelPath: u64,
        inputSourcePath: u64,
        linearValid: bool,
        linear: crate::UnityEngine::XR::OpenXR::NativeTypes::XrVector3f,
        angularValid: bool,
        angular: crate::UnityEngine::XR::OpenXR::NativeTypes::XrVector3f,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u64,
                        u64,
                        u64,
                        bool,
                        crate::UnityEngine::XR::OpenXR::NativeTypes::XrVector3f,
                        bool,
                        crate::UnityEngine::XR::OpenXR::NativeTypes::XrVector3f,
                    ), bool, 7usize>("xrSetInputDeviceVelocityUNITY")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "xrSetInputDeviceVelocityUNITY",
                            7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    xrSession,
                    topLevelPath,
                    inputSourcePath,
                    linearValid,
                    linear,
                    angularValid,
                    angular,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+XR+OpenXR+Features+ConformanceAutomation+ConformanceAutomationFeature"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::XR::OpenXR::Features::ConformanceAutomation::ConformanceAutomationFeature {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

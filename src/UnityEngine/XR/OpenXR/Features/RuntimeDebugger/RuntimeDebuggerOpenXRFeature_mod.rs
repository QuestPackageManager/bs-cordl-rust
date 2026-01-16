#[cfg(
    feature = "cordl_class_UnityEngine+XR+OpenXR+Features+RuntimeDebugger+RuntimeDebuggerOpenXRFeature"
)]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeDebuggerOpenXRFeature {
    __cordl_parent: crate::UnityEngine::XR::OpenXR::Features::OpenXRFeature,
    pub cacheSize: u32,
    pub perThreadCacheSize: u32,
    pub lutOffset: u32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+XR+OpenXR+Features+RuntimeDebugger+RuntimeDebuggerOpenXRFeature"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::OpenXR::Features::RuntimeDebugger::RuntimeDebuggerOpenXRFeature {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.OpenXR.Features.RuntimeDebugger";
    const CLASS_NAME: &'static str = "RuntimeDebuggerOpenXRFeature";
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
    feature = "UnityEngine+XR+OpenXR+Features+RuntimeDebugger+RuntimeDebuggerOpenXRFeature"
)]
impl std::ops::Deref
for crate::UnityEngine::XR::OpenXR::Features::RuntimeDebugger::RuntimeDebuggerOpenXRFeature {
    type Target = crate::UnityEngine::XR::OpenXR::Features::OpenXRFeature;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+XR+OpenXR+Features+RuntimeDebugger+RuntimeDebuggerOpenXRFeature"
)]
impl std::ops::DerefMut
for crate::UnityEngine::XR::OpenXR::Features::RuntimeDebugger::RuntimeDebuggerOpenXRFeature {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+XR+OpenXR+Features+RuntimeDebugger+RuntimeDebuggerOpenXRFeature"
)]
impl crate::UnityEngine::XR::OpenXR::Features::RuntimeDebugger::RuntimeDebuggerOpenXRFeature {
    pub const Library: &'static str = "openxr_runtime_debugger";
    pub fn HookGetInstanceProcAddr(
        &mut self,
        func: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::IntPtr),
                        crate::System::IntPtr,
                        1usize,
                    >("HookGetInstanceProcAddr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HookGetInstanceProcAddr", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            cordl_method_info.invoke_unchecked(self, (func))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Native_EndDataAccess() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("Native_EndDataAccess")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Native_EndDataAccess", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Native_GetDataForRead(
        ptr: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        _cordl_size: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                        ),
                        bool,
                        2usize,
                    >("Native_GetDataForRead")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Native_GetDataForRead", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (ptr, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Native_GetLUTData(
        ptr: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        _cordl_size: quest_hook::libil2cpp::ByRefMut<u32>,
        offset: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            u32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Native_GetLUTData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Native_GetLUTData", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (ptr, _cordl_size, offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Native_HookGetInstanceProcAddr(
        func: crate::System::IntPtr,
        cacheSize: u32,
        perThreadCacheSize: u32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, u32, u32),
                        crate::System::IntPtr,
                        3usize,
                    >("Native_HookGetInstanceProcAddr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Native_HookGetInstanceProcAddr", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            cordl_method_info
                .invoke_unchecked((), (func, cacheSize, perThreadCacheSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Native_StartDataAccess() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("Native_StartDataAccess")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Native_StartDataAccess", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RecvMsg(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RecvMsg")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "RecvMsg",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (args))?
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
}
#[cfg(
    feature = "cordl_class_UnityEngine+XR+OpenXR+Features+RuntimeDebugger+RuntimeDebuggerOpenXRFeature"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::XR::OpenXR::Features::RuntimeDebugger::RuntimeDebuggerOpenXRFeature {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

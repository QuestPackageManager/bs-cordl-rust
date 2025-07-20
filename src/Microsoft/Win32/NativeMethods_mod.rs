#[cfg(feature = "Microsoft+Win32+NativeMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Microsoft+Win32+NativeMethods")]
unsafe impl quest_hook::libil2cpp::Type for crate::Microsoft::Win32::NativeMethods {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Microsoft.Win32";
    const CLASS_NAME: &'static str = "NativeMethods";
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
#[cfg(feature = "Microsoft+Win32+NativeMethods")]
impl std::ops::Deref for crate::Microsoft::Win32::NativeMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+NativeMethods")]
impl std::ops::DerefMut for crate::Microsoft::Win32::NativeMethods {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+NativeMethods")]
impl crate::Microsoft::Win32::NativeMethods {
    pub fn CloseProcess(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr),
                        bool,
                        1usize,
                    >("CloseProcess")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CloseProcess", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (handle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DuplicateHandle(
        hSourceProcessHandle: crate::System::Runtime::InteropServices::HandleRef,
        hSourceHandle: crate::System::Runtime::InteropServices::HandleRef,
        hTargetProcess: crate::System::Runtime::InteropServices::HandleRef,
        targetHandle: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
            >,
        >,
        dwDesiredAccess: i32,
        bInheritHandle: bool,
        dwOptions: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::Runtime::InteropServices::HandleRef,
                            crate::System::Runtime::InteropServices::HandleRef,
                            crate::System::Runtime::InteropServices::HandleRef,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
                                >,
                            >,
                            i32,
                            bool,
                            i32,
                        ),
                        bool,
                        7usize,
                    >("DuplicateHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DuplicateHandle", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        hSourceProcessHandle,
                        hSourceHandle,
                        hTargetProcess,
                        targetHandle,
                        dwDesiredAccess,
                        bInheritHandle,
                        dwOptions,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentProcess() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::System::IntPtr,
                        0usize,
                    >("GetCurrentProcess")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCurrentProcess", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentProcessId() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("GetCurrentProcessId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCurrentProcessId", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExitCodeProcess_IntPtr0(
        processHandle: crate::System::IntPtr,
        exitCode: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, quest_hook::libil2cpp::ByRefMut<i32>),
                        bool,
                        2usize,
                    >("GetExitCodeProcess")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetExitCodeProcess", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (processHandle, exitCode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetExitCodeProcess_SafeProcessHandle1(
        processHandle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        >,
        exitCode: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        bool,
                        2usize,
                    >("GetExitCodeProcess")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetExitCodeProcess", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (processHandle, exitCode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetProcessTimes_IntPtr0(
        handle: crate::System::IntPtr,
        creation: quest_hook::libil2cpp::ByRefMut<i64>,
        _cordl_exit: quest_hook::libil2cpp::ByRefMut<i64>,
        kernel: quest_hook::libil2cpp::ByRefMut<i64>,
        user: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::ByRefMut<i64>,
                            quest_hook::libil2cpp::ByRefMut<i64>,
                            quest_hook::libil2cpp::ByRefMut<i64>,
                            quest_hook::libil2cpp::ByRefMut<i64>,
                        ),
                        bool,
                        5usize,
                    >("GetProcessTimes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetProcessTimes", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (handle, creation, _cordl_exit, kernel, user))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetProcessTimes_SafeProcessHandle1(
        handle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        >,
        creation: quest_hook::libil2cpp::ByRefMut<i64>,
        _cordl_exit: quest_hook::libil2cpp::ByRefMut<i64>,
        kernel: quest_hook::libil2cpp::ByRefMut<i64>,
        user: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i64>,
                            quest_hook::libil2cpp::ByRefMut<i64>,
                            quest_hook::libil2cpp::ByRefMut<i64>,
                            quest_hook::libil2cpp::ByRefMut<i64>,
                        ),
                        bool,
                        5usize,
                    >("GetProcessTimes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetProcessTimes", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (handle, creation, _cordl_exit, kernel, user))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Microsoft+Win32+NativeMethods")]
impl quest_hook::libil2cpp::ObjectType for crate::Microsoft::Win32::NativeMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

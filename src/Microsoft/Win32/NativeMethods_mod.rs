#[cfg(feature = "Microsoft+Win32+NativeMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Microsoft+Win32+NativeMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Microsoft::Win32::NativeMethods =>
    "Microsoft.Win32"."NativeMethods"
);
#[cfg(feature = "Microsoft+Win32+NativeMethods")]
impl std::ops::Deref for crate::Microsoft::Win32::NativeMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+NativeMethods")]
impl std::ops::DerefMut for crate::Microsoft::Win32::NativeMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+NativeMethods")]
impl crate::Microsoft::Win32::NativeMethods {
    pub fn CloseProcess(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CloseProcess", (handle))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DuplicateHandle",
                (
                    hSourceProcessHandle,
                    hSourceHandle,
                    hTargetProcess,
                    targetHandle,
                    dwDesiredAccess,
                    bInheritHandle,
                    dwOptions,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentProcess() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentProcess", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentProcessId() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentProcessId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExitCodeProcess_IntPtr0(
        processHandle: crate::System::IntPtr,
        exitCode: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetExitCodeProcess", (processHandle, exitCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExitCodeProcess_SafeProcessHandle1(
        processHandle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        >,
        exitCode: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetExitCodeProcess", (processHandle, exitCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProcessTimes_IntPtr0(
        handle: crate::System::IntPtr,
        creation: quest_hook::libil2cpp::ByRefMut<i64>,
        _cordl_exit: quest_hook::libil2cpp::ByRefMut<i64>,
        kernel: quest_hook::libil2cpp::ByRefMut<i64>,
        user: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProcessTimes", (handle, creation, _cordl_exit, kernel, user))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProcessTimes", (handle, creation, _cordl_exit, kernel, user))?;
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

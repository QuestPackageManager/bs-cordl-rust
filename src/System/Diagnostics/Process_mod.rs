#[cfg(feature = "System+Diagnostics+Process")]
#[repr(C)]
#[derive(Debug)]
pub struct Process {
    __cordl_parent: crate::System::ComponentModel::Component,
    pub haveProcessId: bool,
    pub processId: i32,
    pub haveProcessHandle: bool,
    pub m_processHandle: quest_hook::libil2cpp::Gc<
        crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
    >,
    pub isRemoteMachine: bool,
    pub machineName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_processAccess: i32,
    pub threads: quest_hook::libil2cpp::Gc<
        crate::System::Diagnostics::ProcessThreadCollection,
    >,
    pub modules: quest_hook::libil2cpp::Gc<
        crate::System::Diagnostics::ProcessModuleCollection,
    >,
    pub haveWorkingSetLimits: bool,
    pub havePriorityClass: bool,
    pub startInfo: quest_hook::libil2cpp::Gc<
        crate::System::Diagnostics::ProcessStartInfo,
    >,
    pub watchForExit: bool,
    pub watchingForExit: bool,
    pub onExited: quest_hook::libil2cpp::Gc<crate::System::EventHandler>,
    pub exited: bool,
    pub exitCode: i32,
    pub signaled: bool,
    pub haveExitTime: bool,
    pub raisedOnExited: bool,
    pub registeredWaitHandle: quest_hook::libil2cpp::Gc<
        crate::System::Threading::RegisteredWaitHandle,
    >,
    pub waitHandle: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
    pub synchronizingObject: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::ISynchronizeInvoke,
    >,
    pub standardOutput: quest_hook::libil2cpp::Gc<crate::System::IO::StreamReader>,
    pub standardInput: quest_hook::libil2cpp::Gc<crate::System::IO::StreamWriter>,
    pub standardError: quest_hook::libil2cpp::Gc<crate::System::IO::StreamReader>,
    pub disposed: bool,
    pub outputStreamReadMode: crate::System::Diagnostics::Process_StreamReadMode,
    pub errorStreamReadMode: crate::System::Diagnostics::Process_StreamReadMode,
    pub inputStreamReadMode: crate::System::Diagnostics::Process_StreamReadMode,
    pub output: quest_hook::libil2cpp::Gc<crate::System::Diagnostics::AsyncStreamReader>,
    pub error: quest_hook::libil2cpp::Gc<crate::System::Diagnostics::AsyncStreamReader>,
    pub process_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Diagnostics+Process")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::Process =>
    "System.Diagnostics"."Process"
);
#[cfg(feature = "System+Diagnostics+Process")]
impl std::ops::Deref for crate::System::Diagnostics::Process {
    type Target = crate::System::ComponentModel::Component;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+Process")]
impl std::ops::DerefMut for crate::System::Diagnostics::Process {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+Process")]
impl crate::System::Diagnostics::Process {
    #[cfg(feature = "System+Diagnostics+Process+ProcInfo")]
    pub type ProcInfo = crate::System::Diagnostics::Process_ProcInfo;
    #[cfg(feature = "System+Diagnostics+Process+State")]
    pub type State = crate::System::Diagnostics::Process_State;
    #[cfg(feature = "System+Diagnostics+Process+StreamReadMode")]
    pub type StreamReadMode = crate::System::Diagnostics::Process_StreamReadMode;
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CompletionCallback(
        &mut self,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        wasSignaled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompletionCallback", (context, wasSignaled))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePipe(
        read: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        write: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        writeDirection: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePipe", (read, write, writeDirection))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateProcess_internal(
        startInfo: quest_hook::libil2cpp::Gc<
            crate::System::Diagnostics::ProcessStartInfo,
        >,
        stdin: crate::System::IntPtr,
        stdout: crate::System::IntPtr,
        stderr: crate::System::IntPtr,
        procInfo: quest_hook::libil2cpp::ByRefMut<
            crate::System::Diagnostics::Process_ProcInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateProcess_internal",
                (startInfo, stdin, stdout, stderr, procInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureState(
        &mut self,
        state: crate::System::Diagnostics::Process_State,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureState", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureWatchingForExit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureWatchingForExit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FillUserInfo(
        startInfo: quest_hook::libil2cpp::Gc<
            crate::System::Diagnostics::ProcessStartInfo,
        >,
        procInfo: quest_hook::libil2cpp::ByRefMut<
            crate::System::Diagnostics::Process_ProcInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FillUserInfo", (startInfo, procInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentProcess() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Diagnostics::Process>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Diagnostics::Process,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentProcess", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProcessById_Il2CppString1(
        processId: i32,
        machineName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Diagnostics::Process>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Diagnostics::Process,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProcessById", (processId, machineName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProcessById_i32_0(
        processId: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Diagnostics::Process>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Diagnostics::Process,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProcessById", (processId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProcessHandle__cordl_bool0(
        &mut self,
        access: i32,
        throwIfExited: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        > = __cordl_object.invoke("GetProcessHandle", (access, throwIfExited))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProcessHandle_i32_1(
        &mut self,
        access: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        > = __cordl_object.invoke("GetProcessHandle", (access))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProcessTimes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Diagnostics::ProcessThreadTimes>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Diagnostics::ProcessThreadTimes,
        > = __cordl_object.invoke("GetProcessTimes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProcess_internal(
        pid: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProcess_internal", (pid))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLocalMachine(
        machineName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLocalMachine", (machineName))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString__cordl_bool_i32_ProcessInfo1(
        machineName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isRemoteMachine: bool,
        processId: i32,
        processInfo: quest_hook::libil2cpp::Gc<crate::System::Diagnostics::ProcessInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (machineName, isRemoteMachine, processId, processInfo),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_SafeProcessHandle_i32_2(
        handle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        >,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handle, id))?;
        Ok(__cordl_object.into())
    }
    pub fn OnExited(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnExited", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenProcessHandle(
        &mut self,
        access: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        > = __cordl_object.invoke("OpenProcessHandle", (access))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessName_icall(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessName_icall", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessName_internal(
        handle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessName_internal", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn RaiseOnExited(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseOnExited", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseProcessHandle(
        &mut self,
        handle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleaseProcessHandle", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetProcessHandle(
        &mut self,
        processHandle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetProcessHandle", (processHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetProcessId(
        &mut self,
        processId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetProcessId", (processId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShellExecuteEx_internal(
        startInfo: quest_hook::libil2cpp::Gc<
            crate::System::Diagnostics::ProcessStartInfo,
        >,
        procInfo: quest_hook::libil2cpp::ByRefMut<
            crate::System::Diagnostics::Process_ProcInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShellExecuteEx_internal", (startInfo, procInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartWithCreateProcess(
        &mut self,
        startInfo: quest_hook::libil2cpp::Gc<
            crate::System::Diagnostics::ProcessStartInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("StartWithCreateProcess", (startInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartWithShellExecuteEx(
        &mut self,
        startInfo: quest_hook::libil2cpp::Gc<
            crate::System::Diagnostics::ProcessStartInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("StartWithShellExecuteEx", (startInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn StopWatchingForExit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopWatchingForExit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString__cordl_bool_i32_ProcessInfo1(
        &mut self,
        machineName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isRemoteMachine: bool,
        processId: i32,
        processInfo: quest_hook::libil2cpp::Gc<crate::System::Diagnostics::ProcessInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (machineName, isRemoteMachine, processId, processInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SafeProcessHandle_i32_2(
        &mut self,
        handle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        >,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (handle, id))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Associated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Associated", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExitCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ExitCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Handle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("get_Handle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasExited(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasExited", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Id(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Id", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsWindows() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsWindows", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProcessName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ProcessName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StandardError(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::StreamReader>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::StreamReader> = __cordl_object
            .invoke("get_StandardError", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StandardOutput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::StreamReader>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::StreamReader> = __cordl_object
            .invoke("get_StandardOutput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StartInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Diagnostics::ProcessStartInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Diagnostics::ProcessStartInfo,
        > = __cordl_object.invoke("get_StartInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SynchronizingObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ISynchronizeInvoke>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ISynchronizeInvoke,
        > = __cordl_object.invoke("get_SynchronizingObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TotalProcessorTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("get_TotalProcessorTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_StartInfo(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Diagnostics::ProcessStartInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_StartInfo", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Diagnostics+Process")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Diagnostics::Process {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Diagnostics+Process+ProcInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Process_ProcInfo {
    pub process_handle: crate::System::IntPtr,
    pub pid: i32,
    pub envVariables: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub UserName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Domain: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Password: crate::System::IntPtr,
    pub LoadUserProfile: bool,
}
#[cfg(feature = "System+Diagnostics+Process+ProcInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::Process_ProcInfo =>
    "System.Diagnostics"."Process/ProcInfo"
);
#[cfg(feature = "System+Diagnostics+Process+ProcInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Diagnostics::Process_ProcInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Diagnostics+Process+ProcInfo")]
impl crate::System::Diagnostics::Process_ProcInfo {}
#[cfg(feature = "System+Diagnostics+Process+State")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Process_State {
    #[default]
    Associated = 32i32,
    Exited = 16i32,
    HaveId = 1i32,
    HaveNtProcessInfo = 12i32,
    HaveProcessInfo = 8i32,
    IsLocal = 2i32,
    IsNt = 4i32,
    IsWin2k = 64i32,
}
#[cfg(feature = "System+Diagnostics+Process+State")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::Process_State =>
    "System.Diagnostics"."Process/State"
);
#[cfg(feature = "System+Diagnostics+Process+StreamReadMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Process_StreamReadMode {
    #[default]
    asyncMode = 2i32,
    syncMode = 1i32,
    undefined = 0i32,
}
#[cfg(feature = "System+Diagnostics+Process+StreamReadMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::Process_StreamReadMode =>
    "System.Diagnostics"."Process/StreamReadMode"
);

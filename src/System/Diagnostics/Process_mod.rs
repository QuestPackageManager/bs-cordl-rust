#[cfg(feature = "System+Diagnostics+Process")]
#[repr(C)]
#[derive(Debug)]
pub struct Process {
    __cordl_parent: crate::System::ComponentModel::Component,
    pub haveProcessId: bool,
    pub processId: i32,
    pub haveProcessHandle: bool,
    pub m_processHandle: *mut crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
    pub isRemoteMachine: bool,
    pub machineName: *mut crate::System::String,
    pub m_processAccess: i32,
    pub threads: *mut crate::System::Diagnostics::ProcessThreadCollection,
    pub modules: *mut crate::System::Diagnostics::ProcessModuleCollection,
    pub haveWorkingSetLimits: bool,
    pub havePriorityClass: bool,
    pub startInfo: *mut crate::System::Diagnostics::ProcessStartInfo,
    pub watchForExit: bool,
    pub watchingForExit: bool,
    pub onExited: *mut crate::System::EventHandler,
    pub exited: bool,
    pub exitCode: i32,
    pub signaled: bool,
    pub haveExitTime: bool,
    pub raisedOnExited: bool,
    pub registeredWaitHandle: *mut crate::System::Threading::RegisteredWaitHandle,
    pub waitHandle: *mut crate::System::Threading::WaitHandle,
    pub synchronizingObject: *mut crate::System::ComponentModel::ISynchronizeInvoke,
    pub standardOutput: *mut crate::System::IO::StreamReader,
    pub standardInput: *mut crate::System::IO::StreamWriter,
    pub standardError: *mut crate::System::IO::StreamReader,
    pub disposed: bool,
    pub outputStreamReadMode: crate::System::Diagnostics::Process_StreamReadMode,
    pub errorStreamReadMode: crate::System::Diagnostics::Process_StreamReadMode,
    pub inputStreamReadMode: crate::System::Diagnostics::Process_StreamReadMode,
    pub output: *mut crate::System::Diagnostics::AsyncStreamReader,
    pub error: *mut crate::System::Diagnostics::AsyncStreamReader,
    pub process_name: *mut crate::System::String,
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
        Ok(__cordl_ret)
    }
    pub fn CompletionCallback(
        &mut self,
        context: *mut crate::System::Object,
        wasSignaled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompletionCallback", (context, wasSignaled))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn EnsureWatchingForExit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureWatchingForExit", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetProcessHandle__cordl_bool0(
        &mut self,
        access: i32,
        throwIfExited: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Microsoft::Win32::SafeHandles::SafeProcessHandle = __cordl_object
            .invoke("GetProcessHandle", (access, throwIfExited))?;
        Ok(__cordl_ret)
    }
    pub fn GetProcessHandle_i32_1(
        &mut self,
        access: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Microsoft::Win32::SafeHandles::SafeProcessHandle = __cordl_object
            .invoke("GetProcessHandle", (access))?;
        Ok(__cordl_ret)
    }
    pub fn GetProcessTimes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Diagnostics::ProcessThreadTimes,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Diagnostics::ProcessThreadTimes = __cordl_object
            .invoke("GetProcessTimes", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_SafeProcessHandle_i32_2(
        handle: *mut crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handle, id))?;
        Ok(__cordl_object)
    }
    pub fn New_String__cordl_bool_i32_ProcessInfo1(
        machineName: *mut crate::System::String,
        isRemoteMachine: bool,
        processId: i32,
        processInfo: *mut crate::System::Diagnostics::ProcessInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (machineName, isRemoteMachine, processId, processInfo),
            )?;
        Ok(__cordl_object)
    }
    pub fn OnExited(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnExited", ())?;
        Ok(__cordl_ret)
    }
    pub fn OpenProcessHandle(
        &mut self,
        access: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Microsoft::Win32::SafeHandles::SafeProcessHandle = __cordl_object
            .invoke("OpenProcessHandle", (access))?;
        Ok(__cordl_ret)
    }
    pub fn RaiseOnExited(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseOnExited", ())?;
        Ok(__cordl_ret)
    }
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseProcessHandle(
        &mut self,
        handle: *mut crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleaseProcessHandle", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn SetProcessHandle(
        &mut self,
        processHandle: *mut crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetProcessHandle", (processHandle))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn Start(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartWithCreateProcess(
        &mut self,
        startInfo: *mut crate::System::Diagnostics::ProcessStartInfo,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("StartWithCreateProcess", (startInfo))?;
        Ok(__cordl_ret)
    }
    pub fn StartWithShellExecuteEx(
        &mut self,
        startInfo: *mut crate::System::Diagnostics::ProcessStartInfo,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("StartWithShellExecuteEx", (startInfo))?;
        Ok(__cordl_ret)
    }
    pub fn StopWatchingForExit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopWatchingForExit", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SafeProcessHandle_i32_2(
        &mut self,
        handle: *mut crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (handle, id))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String__cordl_bool_i32_ProcessInfo1(
        &mut self,
        machineName: *mut crate::System::String,
        isRemoteMachine: bool,
        processId: i32,
        processInfo: *mut crate::System::Diagnostics::ProcessInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (machineName, isRemoteMachine, processId, processInfo))?;
        Ok(__cordl_ret)
    }
    pub fn get_Associated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Associated", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ExitCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ExitCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Handle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("get_Handle", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasExited(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasExited", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Id(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Id", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ProcessName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ProcessName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_StandardError(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::StreamReader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::StreamReader = __cordl_object
            .invoke("get_StandardError", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_StandardOutput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::StreamReader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::StreamReader = __cordl_object
            .invoke("get_StandardOutput", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_StartInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Diagnostics::ProcessStartInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Diagnostics::ProcessStartInfo = __cordl_object
            .invoke("get_StartInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SynchronizingObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::ISynchronizeInvoke,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::ISynchronizeInvoke = __cordl_object
            .invoke("get_SynchronizingObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TotalProcessorTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("get_TotalProcessorTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_StartInfo(
        &mut self,
        value: *mut crate::System::Diagnostics::ProcessStartInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_StartInfo", (value))?;
        Ok(__cordl_ret)
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
#[derive(Debug, Clone)]
pub struct Process_ProcInfo {
    pub process_handle: crate::System::IntPtr,
    pub pid: i32,
    pub envVariables: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub UserName: *mut crate::System::String,
    pub Domain: *mut crate::System::String,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Process_State {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Process_StreamReadMode {
    asyncMode = 2i32,
    syncMode = 1i32,
    undefined = 0i32,
}
#[cfg(feature = "System+Diagnostics+Process+StreamReadMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::Process_StreamReadMode =>
    "System.Diagnostics"."Process/StreamReadMode"
);

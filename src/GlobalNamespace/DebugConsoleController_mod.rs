#[cfg(feature = "DebugConsoleController+ConsoleMessage")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DebugConsoleController_ConsoleMessage {
    pub Message: *mut crate::System::String,
    pub Type: crate::UnityEngine::LogType,
}
#[cfg(feature = "DebugConsoleController+ConsoleMessage")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DebugConsoleController_ConsoleMessage => ""
    ."DebugConsoleController/ConsoleMessage"
);
#[cfg(feature = "DebugConsoleController+ConsoleMessage")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::DebugConsoleController_ConsoleMessage {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "DebugConsoleController+ConsoleMessage")]
impl crate::GlobalNamespace::DebugConsoleController_ConsoleMessage {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        message: *mut crate::System::String,
        _cordl_type: crate::UnityEngine::LogType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (message, _cordl_type),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "DebugConsoleController")]
#[repr(C)]
#[derive(Debug)]
pub struct DebugConsoleController {
    __cordl_parent: crate::System::Object,
    pub onNewMessageToOutput: *mut crate::System::Action_2<
        *mut crate::System::String,
        crate::UnityEngine::LogType,
    >,
    pub _stringsFromSTDIN: *mut crate::System::Collections::Generic::Queue_1<
        *mut crate::System::String,
    >,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _commands: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::GlobalNamespace::ConsoleCommandBase,
    >,
    pub _commandsExecutionTask: *mut crate::System::Threading::Tasks::Task,
}
#[cfg(feature = "DebugConsoleController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DebugConsoleController => ""
    ."DebugConsoleController"
);
#[cfg(feature = "DebugConsoleController")]
impl std::ops::Deref for crate::GlobalNamespace::DebugConsoleController {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DebugConsoleController")]
impl std::ops::DerefMut for crate::GlobalNamespace::DebugConsoleController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DebugConsoleController")]
impl crate::GlobalNamespace::DebugConsoleController {
    #[cfg(feature = "DebugConsoleController+ConsoleMessage")]
    pub type ConsoleMessage = crate::GlobalNamespace::DebugConsoleController_ConsoleMessage;
    #[cfg(feature = "DebugConsoleController+_ExecuteCommandsAsync_d__9")]
    pub type _ExecuteCommandsAsync_d__9 = crate::GlobalNamespace::DebugConsoleController__ExecuteCommandsAsync_d__9;
    #[cfg(feature = "DebugConsoleController+_ExecuteCommands_d__8")]
    pub type _ExecuteCommands_d__8 = crate::GlobalNamespace::DebugConsoleController__ExecuteCommands_d__8;
    #[cfg(feature = "DebugConsoleController+_TryExecuteCommand_d__11")]
    pub type _TryExecuteCommand_d__11 = crate::GlobalNamespace::DebugConsoleController__TryExecuteCommand_d__11;
    pub fn CheckCommand(
        &mut self,
        command: *mut crate::GlobalNamespace::ConsoleCommandBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckCommand", (command))?;
        Ok(__cordl_ret)
    }
    pub fn CommandNotFoundMessage(
        &mut self,
        command: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::DebugConsoleController_ConsoleMessage = __cordl_object
            .invoke("CommandNotFoundMessage", (command))?;
        Ok(__cordl_ret)
    }
    pub fn DisplayMessage(
        &mut self,
        message: crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisplayMessage", (message))?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteCommands(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteCommands", ())?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteCommandsAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("ExecuteCommandsAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn FillDictOfCommands(
        &mut self,
        commandInstances: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::ConsoleCommandBase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillDictOfCommands", (commandInstances))?;
        Ok(__cordl_ret)
    }
    pub fn GetAllConsoleCommandInstances(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::ConsoleCommandBase,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::ConsoleCommandBase,
        > = __cordl_object.invoke("GetAllConsoleCommandInstances", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCommand(
        &mut self,
        commandName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::ConsoleCommandBase> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConsoleCommandBase = __cordl_object
            .invoke("GetCommand", (commandName))?;
        Ok(__cordl_ret)
    }
    pub fn GetCommands(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::ConsoleCommandBase,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::ConsoleCommandBase,
        > = __cordl_object.invoke("GetCommands", ())?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsCommandClass(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsCommandClass", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn QueueNewInput(
        &mut self,
        input: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueNewInput", (input))?;
        Ok(__cordl_ret)
    }
    pub fn StripExtraWhitespace(
        &mut self,
        commandText: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("StripExtraWhitespace", (commandText))?;
        Ok(__cordl_ret)
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryExecuteCommand(
        &mut self,
        commandText: *mut crate::System::String,
        messages: *mut crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("TryExecuteCommand", (commandText, messages))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "DebugConsoleController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DebugConsoleController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

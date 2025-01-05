#[cfg(feature = "DebugConsoleController")]
#[repr(C)]
#[derive(Debug)]
pub struct DebugConsoleController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub onNewMessageToOutput: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            crate::UnityEngine::LogType,
        >,
    >,
    pub _stringsFromSTDIN: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Queue_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub _commandLineParserResult: crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult,
    pub _coroutineStarter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ICoroutineStarter,
    >,
    pub _commands: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ConsoleCommandBase>,
        >,
    >,
    pub _commandsExecutionTask: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::Task,
    >,
}
#[cfg(feature = "DebugConsoleController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DebugConsoleController => ""
    ."DebugConsoleController"
);
#[cfg(feature = "DebugConsoleController")]
impl std::ops::Deref for crate::GlobalNamespace::DebugConsoleController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn CheckCommand(
        &mut self,
        command: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ConsoleCommandBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckCommand", (command))?;
        Ok(__cordl_ret.into())
    }
    pub fn CommandNotFoundMessage(
        &mut self,
        command: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::DebugConsoleController_ConsoleMessage = __cordl_object
            .invoke("CommandNotFoundMessage", (command))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteCommands(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteCommands", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteCommandsAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ExecuteCommandsAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FillDictOfCommands(
        &mut self,
        commandInstances: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ConsoleCommandBase>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillDictOfCommands", (commandInstances))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllConsoleCommandInstances(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ConsoleCommandBase>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ConsoleCommandBase>,
            >,
        > = __cordl_object.invoke("GetAllConsoleCommandInstances", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCommand(
        &mut self,
        commandName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ConsoleCommandBase>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConsoleCommandBase,
        > = __cordl_object.invoke("GetCommand", (commandName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCommands(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ConsoleCommandBase>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ConsoleCommandBase>,
            >,
        > = __cordl_object.invoke("GetCommands", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCommandClass(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsCommandClass", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn QueueNewInput(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueNewInput", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn StripExtraWhitespace(
        &mut self,
        commandText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("StripExtraWhitespace", (commandText))?;
        Ok(__cordl_ret.into())
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryExecuteCommand(
        &mut self,
        commandText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        messages: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("TryExecuteCommand", (commandText, messages))?;
        Ok(__cordl_ret.into())
    }
    pub fn _Initialize_b__8_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Initialize>b__8_0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "DebugConsoleController")]
impl AsRef<crate::Zenject::IInitializable>
for crate::GlobalNamespace::DebugConsoleController {
    fn as_ref(&self) -> &crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DebugConsoleController")]
impl AsMut<crate::Zenject::IInitializable>
for crate::GlobalNamespace::DebugConsoleController {
    fn as_mut(&mut self) -> &mut crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DebugConsoleController")]
impl AsRef<crate::Zenject::ITickable>
for crate::GlobalNamespace::DebugConsoleController {
    fn as_ref(&self) -> &crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DebugConsoleController")]
impl AsMut<crate::Zenject::ITickable>
for crate::GlobalNamespace::DebugConsoleController {
    fn as_mut(&mut self) -> &mut crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DebugConsoleController+ConsoleMessage")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DebugConsoleController_ConsoleMessage {
    pub Message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: crate::UnityEngine::LogType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (message, _cordl_type),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        a: crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
        b: crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
    > {
        let __cordl_ret: crate::GlobalNamespace::DebugConsoleController_ConsoleMessage = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (a, b))?;
        Ok(__cordl_ret.into())
    }
}

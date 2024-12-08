#[cfg(feature = "ConsoleCommandBase")]
#[repr(C)]
#[derive(Debug)]
pub struct ConsoleCommandBase {
    __cordl_parent: crate::System::Object,
    pub _console: *mut DebugConsoleController,
    pub _arguments: *mut quest_hook::libil2cpp::Il2CppArray<*mut ArgumentBase>,
}
#[cfg(feature = "ConsoleCommandBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ConsoleCommandBase => ""."ConsoleCommandBase"
);
#[cfg(feature = "ConsoleCommandBase")]
impl std::ops::Deref for ConsoleCommandBase {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConsoleCommandBase")]
impl std::ops::DerefMut for ConsoleCommandBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConsoleCommandBase")]
impl ConsoleCommandBase {
    #[cfg(feature = "ConsoleCommandBase+__c__DisplayClass11_0")]
    pub type __c__DisplayClass11_0 = crate::GlobalNamespace::ConsoleCommandBase___c__DisplayClass11_0;
    #[cfg(feature = "ConsoleCommandBase+__c")]
    pub type __c = crate::GlobalNamespace::ConsoleCommandBase___c;
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
    pub fn GetLongUsageHelp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::DebugConsoleController_ConsoleMessage = __cordl_object
            .invoke("GetLongUsageHelp", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetArguments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut ArgumentBase>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<*mut ArgumentBase> = __cordl_object
            .invoke("GetArguments", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetShortHelpText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::DebugConsoleController_ConsoleMessage = __cordl_object
            .invoke("GetShortHelpText", ())?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteAsync_Il2CppArray_List_1_0(
        &mut self,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        messages: *mut crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("ExecuteAsync", (args, messages))?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteAsync_List_1_1(
        &mut self,
        messages: *mut crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("ExecuteAsync", (messages))?;
        Ok(__cordl_ret)
    }
    pub fn AreArgumentsValid(
        &mut self,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        messages: *mut crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AreArgumentsValid", (args, messages))?;
        Ok(__cordl_ret)
    }
    pub fn GetArgumentsText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::DebugConsoleController_ConsoleMessage = __cordl_object
            .invoke("GetArgumentsText", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseUnmatchedArguments(
        &mut self,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        matchedArguments: *mut crate::System::Collections::Generic::List_1<
            *mut ArgumentBase,
        >,
        messages: *mut crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut ArgumentBase>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut ArgumentBase,
        > = __cordl_object
            .invoke("ParseUnmatchedArguments", (args, matchedArguments, messages))?;
        Ok(__cordl_ret)
    }
    pub fn ParseMatchedArguments(
        &mut self,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        messages: *mut crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut ArgumentBase>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut ArgumentBase,
        > = __cordl_object.invoke("ParseMatchedArguments", (args, messages))?;
        Ok(__cordl_ret)
    }
    pub fn GetInvalidArgumentMessage(
        &mut self,
        missingArguments: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut ArgumentBase,
        >,
        messages: *mut crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetInvalidArgumentMessage", (missingArguments, messages))?;
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
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompareTo(
        &mut self,
        other: *mut ConsoleCommandBase,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (other))?;
        Ok(__cordl_ret)
    }
    pub fn GetCommandName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetCommandName", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ConsoleCommandBase")]
impl quest_hook::libil2cpp::ObjectType for ConsoleCommandBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

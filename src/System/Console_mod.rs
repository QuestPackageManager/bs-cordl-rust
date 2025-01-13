#[cfg(feature = "System+Console")]
#[repr(C)]
#[derive(Debug)]
pub struct Console {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Console")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Console {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "Console";
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
#[cfg(feature = "System+Console")]
impl std::ops::Deref for crate::System::Console {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Console")]
impl std::ops::DerefMut for crate::System::Console {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Console")]
impl crate::System::Console {
    #[cfg(feature = "System+Console+WindowsConsole")]
    pub type WindowsConsole = crate::System::Console_WindowsConsole;
    pub fn DoConsoleCancelEvent() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoConsoleCancelEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Open(
        handle: crate::System::IntPtr,
        access: crate::System::IO::FileAccess,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Open", (handle, access, bufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenStandardError(
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OpenStandardError", (bufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenStandardInput(
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OpenStandardInput", (bufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenStandardOutput(
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OpenStandardOutput", (bufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadKey_0() -> quest_hook::libil2cpp::Result<crate::System::ConsoleKeyInfo> {
        let __cordl_ret: crate::System::ConsoleKeyInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadKey__cordl_bool1(
        intercept: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::ConsoleKeyInfo> {
        let __cordl_ret: crate::System::ConsoleKeyInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadKey", (intercept))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetError(
        newError: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetError", (newError))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetOut(
        newOut: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetOut", (newOut))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupStreams(
        inputEncoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
        outputEncoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetupStreams", (inputEncoding, outputEncoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteLine(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteLine", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Error() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Error", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InputEncoding() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InputEncoding", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Out() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Out", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OutputEncoding() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_OutputEncoding", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Console")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Console {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Console+WindowsConsole")]
#[repr(C)]
#[derive(Debug)]
pub struct Console_WindowsConsole {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Console+WindowsConsole")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Console_WindowsConsole {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "Console/WindowsConsole";
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
#[cfg(feature = "System+Console+WindowsConsole")]
impl std::ops::Deref for crate::System::Console_WindowsConsole {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Console+WindowsConsole")]
impl std::ops::DerefMut for crate::System::Console_WindowsConsole {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Console+WindowsConsole")]
impl crate::System::Console_WindowsConsole {
    #[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
    pub type WindowsCancelHandler = crate::System::WindowsConsole_Console_WindowsCancelHandler;
    pub fn DoWindowsConsoleCancelEvent(
        keyCode: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoWindowsConsoleCancelEvent", (keyCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetConsoleCP() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetConsoleCP", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetConsoleOutputCP() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetConsoleOutputCP", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInputCodePage() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInputCodePage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOutputCodePage() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOutputCodePage", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Console+WindowsConsole")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Console_WindowsConsole {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct WindowsConsole_Console_WindowsCancelHandler {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::WindowsConsole_Console_WindowsCancelHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "Console/WindowsConsole/WindowsCancelHandler";
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
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
impl std::ops::Deref for crate::System::WindowsConsole_Console_WindowsCancelHandler {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
impl std::ops::DerefMut for crate::System::WindowsConsole_Console_WindowsCancelHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
impl crate::System::WindowsConsole_Console_WindowsCancelHandler {
    pub fn Invoke(&mut self, keyCode: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (keyCode))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::WindowsConsole_Console_WindowsCancelHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "System+ConsoleDriver")]
#[repr(C)]
#[derive(Debug)]
pub struct ConsoleDriver {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ConsoleDriver")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::ConsoleDriver {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "ConsoleDriver";
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
#[cfg(feature = "System+ConsoleDriver")]
impl std::ops::Deref for crate::System::ConsoleDriver {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ConsoleDriver")]
impl std::ops::DerefMut for crate::System::ConsoleDriver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ConsoleDriver")]
impl crate::System::ConsoleDriver {
    pub fn CreateNullConsoleDriver() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IConsoleDriver>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IConsoleDriver> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateNullConsoleDriver", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTermInfoDriver(
        term: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IConsoleDriver>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IConsoleDriver> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateTermInfoDriver", (term))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateWindowsConsoleDriver() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IConsoleDriver>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IConsoleDriver> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateWindowsConsoleDriver", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalKeyAvailable(ms_timeout: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalKeyAvailable", (ms_timeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn Isatty(handle: crate::System::IntPtr) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Isatty", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadKey(
        intercept: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::ConsoleKeyInfo> {
        let __cordl_ret: crate::System::ConsoleKeyInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadKey", (intercept))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetEcho(wantEcho: bool) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetEcho", (wantEcho))?;
        Ok(__cordl_ret.into())
    }
    pub fn TtySetup(
        keypadXmit: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        teardown: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        control_characters: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        address: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TtySetup", (keypadXmit, teardown, control_characters, address))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsConsole() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsConsole", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ConsoleDriver")]
impl quest_hook::libil2cpp::ObjectType for crate::System::ConsoleDriver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

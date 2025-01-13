#[cfg(feature = "System+Environment")]
#[repr(C)]
#[derive(Debug)]
pub struct Environment {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Environment")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Environment {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "Environment";
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
#[cfg(feature = "System+Environment")]
impl std::ops::Deref for crate::System::Environment {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Environment")]
impl std::ops::DerefMut for crate::System::Environment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Environment")]
impl crate::System::Environment {
    pub const mono_corlib_version: &'static str = "1A5E0066-58DC-428A-B21C-0AD6CDAE2789";
    #[cfg(feature = "System+Environment+SpecialFolder")]
    pub type SpecialFolder = crate::System::Environment_SpecialFolder;
    #[cfg(feature = "System+Environment+SpecialFolderOption")]
    pub type SpecialFolderOption = crate::System::Environment_SpecialFolderOption;
    pub fn CreateVersionFromString(
        info: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Version>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Version> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateVersionFromString", (info))?;
        Ok(__cordl_ret.into())
    }
    pub fn Exit(
        exitCode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Exit", (exitCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn FailFast_Il2CppString1(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        errorSource: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FailFast", (message, exception, errorSource))?;
        Ok(__cordl_ret.into())
    }
    pub fn FailFast_Il2CppString_Exception0(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FailFast", (message, exception))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCommandLineArgs() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCommandLineArgs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnvironmentVariableNames() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEnvironmentVariableNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnvironmentVariable_EnvironmentVariableTarget1(
        variable: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: crate::System::EnvironmentVariableTarget,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEnvironmentVariable", (variable, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnvironmentVariable_Il2CppString0(
        variable: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEnvironmentVariable", (variable))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnvironmentVariables() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IDictionary,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEnvironmentVariables", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFolderPath_Environment_SpecialFolder0(
        folder: crate::System::Environment_SpecialFolder,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFolderPath", (folder))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFolderPath_Environment_SpecialFolderOption1(
        folder: crate::System::Environment_SpecialFolder,
        option: crate::System::Environment_SpecialFolderOption,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFolderPath", (folder, option))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIs64BitOperatingSystem() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIs64BitOperatingSystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLogicalDrivesInternal() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLogicalDrivesInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMachineConfigPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMachineConfigPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNewLine() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetNewLine", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOSVersionString() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOSVersionString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPageSize() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPageSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResourceStringEncodingName(
        codePage: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetResourceStringEncodingName", (codePage))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResourceString_Il2CppArray1(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetResourceString", (key, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResourceString_Il2CppString0(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetResourceString", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStackTrace(
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        needFileInfo: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStackTrace", (e, needFileInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWindowsFolderPath(
        folder: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWindowsFolderPath", (folder))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetEnvironmentVariable_Il2CppObject_i32_Il2CppObject_i32_0(
        variable: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        variable_length: i32,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value_length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InternalSetEnvironmentVariable",
                (variable, variable_length, value, value_length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetEnvironmentVariable_Il2CppString_Il2CppString1(
        variable: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalSetEnvironmentVariable", (variable, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadXdgUserDir(
        config_dir: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        home_dir: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fallback: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadXdgUserDir", (config_dir, home_dir, key, fallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetEnvironmentVariable(
        variable: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetEnvironmentVariable", (variable, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnixGetFolderPath(
        folder: crate::System::Environment_SpecialFolder,
        option: crate::System::Environment_SpecialFolderOption,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnixGetFolderPath", (folder, option))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentDirectory() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CurrentDirectory", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentManagedThreadId() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CurrentManagedThreadId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasShutdownStarted() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_HasShutdownStarted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Is64BitOperatingSystem() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Is64BitOperatingSystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Is64BitProcess() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Is64BitProcess", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsRunningOnWindows() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsRunningOnWindows", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsUnix() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsUnix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MachineName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_MachineName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NewLine() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_NewLine", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OSVersion() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::OperatingSystem>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::OperatingSystem> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_OSVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Platform() -> quest_hook::libil2cpp::Result<crate::System::PlatformID> {
        let __cordl_ret: crate::System::PlatformID = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Platform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProcessorCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ProcessorCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StackTrace() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_StackTrace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TickCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_TickCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UserDomainName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UserDomainName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UserName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_UserName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn internalGetEnvironmentVariable(
        variable: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("internalGetEnvironmentVariable", (variable))?;
        Ok(__cordl_ret.into())
    }
    pub fn internalGetEnvironmentVariable_native(
        variable: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("internalGetEnvironmentVariable_native", (variable))?;
        Ok(__cordl_ret.into())
    }
    pub fn internalGetHome() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("internalGetHome", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Environment")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Environment {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Environment+SpecialFolder")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Environment_SpecialFolder {
    #[default]
    AdminTools = 48i32,
    ApplicationData = 26i32,
    CDBurning = 59i32,
    CommonAdminTools = 47i32,
    CommonApplicationData = 35i32,
    CommonDesktopDirectory = 25i32,
    CommonDocuments = 46i32,
    CommonMusic = 53i32,
    CommonOemLinks = 58i32,
    CommonPictures = 54i32,
    CommonProgramFiles = 43i32,
    CommonProgramFilesX86 = 44i32,
    CommonPrograms = 23i32,
    CommonStartMenu = 22i32,
    CommonStartup = 24i32,
    CommonTemplates = 45i32,
    CommonVideos = 55i32,
    Cookies = 33i32,
    Desktop = 0i32,
    DesktopDirectory = 16i32,
    Favorites = 6i32,
    Fonts = 20i32,
    History = 34i32,
    InternetCache = 32i32,
    LocalApplicationData = 28i32,
    LocalizedResources = 57i32,
    MyComputer = 17i32,
    MyDocuments = 5i32,
    MyMusic = 13i32,
    MyPictures = 39i32,
    MyVideos = 14i32,
    NetworkShortcuts = 19i32,
    PrinterShortcuts = 27i32,
    ProgramFiles = 38i32,
    ProgramFilesX86 = 42i32,
    Programs = 2i32,
    Recent = 8i32,
    Resources = 56i32,
    SendTo = 9i32,
    StartMenu = 11i32,
    Startup = 7i32,
    System = 37i32,
    SystemX86 = 41i32,
    Templates = 21i32,
    UserProfile = 40i32,
    Windows = 36i32,
}
#[cfg(feature = "System+Environment+SpecialFolder")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Environment_SpecialFolder {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "Environment/SpecialFolder";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Environment+SpecialFolder")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Environment_SpecialFolder {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Environment+SpecialFolder")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Environment_SpecialFolder {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Environment+SpecialFolder")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Environment_SpecialFolder {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Environment+SpecialFolder")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Environment_SpecialFolder {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+Environment+SpecialFolderOption")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Environment_SpecialFolderOption {
    #[default]
    Create = 32768i32,
    DoNotVerify = 16384i32,
    None = 0i32,
}
#[cfg(feature = "System+Environment+SpecialFolderOption")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Environment_SpecialFolderOption {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "Environment/SpecialFolderOption";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Environment+SpecialFolderOption")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Environment_SpecialFolderOption {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Environment+SpecialFolderOption")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Environment_SpecialFolderOption {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Environment+SpecialFolderOption")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Environment_SpecialFolderOption {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Environment+SpecialFolderOption")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Environment_SpecialFolderOption {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}

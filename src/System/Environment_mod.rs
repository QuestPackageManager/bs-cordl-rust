#[cfg(feature = "System+Environment")]
#[repr(C)]
#[derive(Debug)]
pub struct Environment {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Environment")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Environment => "System"."Environment"
);
#[cfg(feature = "System+Environment")]
impl std::ops::Deref for crate::System::Environment {
    type Target = crate::System::Object;
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
    #[cfg(feature = "System+Environment+SpecialFolderOption")]
    pub type SpecialFolderOption = crate::System::Environment_SpecialFolderOption;
    #[cfg(feature = "System+Environment+SpecialFolder")]
    pub type SpecialFolder = crate::System::Environment_SpecialFolder;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment_SpecialFolder {
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Environment_SpecialFolder => "System"
    ."Environment/SpecialFolder"
);
#[cfg(feature = "System+Environment+SpecialFolderOption")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment_SpecialFolderOption {
    Create = 32768i32,
    DoNotVerify = 16384i32,
    None = 0i32,
}
#[cfg(feature = "System+Environment+SpecialFolderOption")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Environment_SpecialFolderOption =>
    "System"."Environment/SpecialFolderOption"
);

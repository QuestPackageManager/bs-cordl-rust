#[cfg(feature = "AppInitSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct AppInitSetupData {
    __cordl_parent: crate::System::Object,
    pub runMode: crate::GlobalNamespace::AppInitSetupData_RunMode,
}
#[cfg(feature = "AppInitSetupData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AppInitSetupData => ""
    ."AppInitSetupData"
);
#[cfg(feature = "AppInitSetupData")]
impl std::ops::Deref for crate::GlobalNamespace::AppInitSetupData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AppInitSetupData")]
impl std::ops::DerefMut for crate::GlobalNamespace::AppInitSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AppInitSetupData")]
impl crate::GlobalNamespace::AppInitSetupData {
    #[cfg(feature = "AppInitSetupData+RunMode")]
    pub type RunMode = crate::GlobalNamespace::AppInitSetupData_RunMode;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "AppInitSetupData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AppInitSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AppInitSetupData+RunMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppInitSetupData_RunMode {
    Game = 0i32,
    PlayTest = 1i32,
}
#[cfg(feature = "AppInitSetupData+RunMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AppInitSetupData_RunMode => ""
    ."AppInitSetupData/RunMode"
);

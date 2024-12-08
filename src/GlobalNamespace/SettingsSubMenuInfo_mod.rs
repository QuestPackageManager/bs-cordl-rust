#[cfg(feature = "SettingsSubMenuInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingsSubMenuInfo {
    __cordl_parent: crate::System::Object,
    pub _viewController: *mut crate::HMUI::ViewController,
    pub _menuName: *mut crate::System::String,
}
#[cfg(feature = "SettingsSubMenuInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SettingsSubMenuInfo => ""."SettingsSubMenuInfo"
);
#[cfg(feature = "SettingsSubMenuInfo")]
impl std::ops::Deref for SettingsSubMenuInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SettingsSubMenuInfo")]
impl std::ops::DerefMut for SettingsSubMenuInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SettingsSubMenuInfo")]
impl SettingsSubMenuInfo {
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
    pub fn get_viewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::ViewController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::ViewController = __cordl_object
            .invoke("get_viewController", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localizedMenuName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_localizedMenuName", ())?;
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
#[cfg(feature = "SettingsSubMenuInfo")]
impl quest_hook::libil2cpp::ObjectType for SettingsSubMenuInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

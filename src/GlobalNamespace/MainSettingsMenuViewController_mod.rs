#[cfg(feature = "MainSettingsMenuViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct MainSettingsMenuViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub didSelectSettingsSubMenuEvent: *mut crate::System::Action_2<
        *mut SettingsSubMenuInfo,
        i32,
    >,
    pub _settingsSubMenuInfos: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut SettingsSubMenuInfo,
    >,
    pub _settingsMenuSegmentedControl: *mut crate::HMUI::TextSegmentedControl,
    pub _selectedSubMenuInfo: *mut SettingsSubMenuInfo,
    pub _selectedSubMenuInfoIdx: i32,
}
#[cfg(feature = "MainSettingsMenuViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MainSettingsMenuViewController => ""
    ."MainSettingsMenuViewController"
);
#[cfg(feature = "MainSettingsMenuViewController")]
impl std::ops::Deref for MainSettingsMenuViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainSettingsMenuViewController")]
impl std::ops::DerefMut for MainSettingsMenuViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainSettingsMenuViewController")]
impl MainSettingsMenuViewController {
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
    pub fn get_numberOfSubMenus(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_numberOfSubMenus", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedSubMenuInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut SettingsSubMenuInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut SettingsSubMenuInfo = __cordl_object
            .invoke("get_selectedSubMenuInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        selectedSubMenuInfoIdx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (selectedSubMenuInfoIdx))?;
        Ok(__cordl_ret)
    }
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DidActivate",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret)
    }
    pub fn add_didSelectSettingsSubMenuEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut SettingsSubMenuInfo, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectSettingsSubMenuEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectSettingsSubMenuEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut SettingsSubMenuInfo, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectSettingsSubMenuEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSettingsMenuSegmentedControlDidSelectCell(
        &mut self,
        segmentedControl: *mut crate::HMUI::SegmentedControl,
        cellIdx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSettingsMenuSegmentedControlDidSelectCell",
                (segmentedControl, cellIdx),
            )?;
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
#[cfg(feature = "MainSettingsMenuViewController")]
impl quest_hook::libil2cpp::ObjectType for MainSettingsMenuViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

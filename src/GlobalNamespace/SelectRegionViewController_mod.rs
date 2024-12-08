#[cfg(feature = "SelectRegionViewController+Region")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectRegionViewController_Region {
    Europe = 2i32,
    Japan = 4i32,
    None = 0i32,
    NorthAndSouthAmerica = 1i32,
    Other = 5i32,
    SouthKorea = 3i32,
}
#[cfg(feature = "SelectRegionViewController+Region")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SelectRegionViewController_Region => ""
    ."SelectRegionViewController/Region"
);
#[cfg(feature = "SelectRegionViewController+RegionToLocalizationKeyPair")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SelectRegionViewController_RegionToLocalizationKeyPair {
    pub region: crate::GlobalNamespace::SelectRegionViewController_Region,
    pub localizationKey: *mut crate::System::String,
}
#[cfg(feature = "SelectRegionViewController+RegionToLocalizationKeyPair")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SelectRegionViewController_RegionToLocalizationKeyPair => ""
    ."SelectRegionViewController/RegionToLocalizationKeyPair"
);
#[cfg(feature = "SelectRegionViewController+RegionToLocalizationKeyPair")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::SelectRegionViewController_RegionToLocalizationKeyPair {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "SelectRegionViewController+RegionToLocalizationKeyPair")]
impl crate::GlobalNamespace::SelectRegionViewController_RegionToLocalizationKeyPair {
    pub fn _ctor(
        &mut self,
        region: crate::GlobalNamespace::SelectRegionViewController_Region,
        localizationKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (region, localizationKey),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SelectRegionViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectRegionViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _continueButton: *mut crate::UnityEngine::UI::Button,
    pub _regionSelectionDropdown: *mut crate::HMUI::SimpleTextDropdown,
    pub _regionLocalizationKeys: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::SelectRegionViewController_RegionToLocalizationKeyPair,
    >,
    pub didPressContinueButtonEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::SelectRegionViewController_Region,
    >,
}
#[cfg(feature = "SelectRegionViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SelectRegionViewController => ""
    ."SelectRegionViewController"
);
#[cfg(feature = "SelectRegionViewController")]
impl std::ops::Deref for SelectRegionViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SelectRegionViewController")]
impl std::ops::DerefMut for SelectRegionViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SelectRegionViewController")]
impl SelectRegionViewController {
    #[cfg(feature = "SelectRegionViewController+RegionToLocalizationKeyPair")]
    pub type RegionToLocalizationKeyPair = crate::GlobalNamespace::SelectRegionViewController_RegionToLocalizationKeyPair;
    #[cfg(feature = "SelectRegionViewController+__c")]
    pub type __c = crate::GlobalNamespace::SelectRegionViewController___c;
    #[cfg(feature = "SelectRegionViewController+Region")]
    pub type Region = crate::GlobalNamespace::SelectRegionViewController_Region;
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
    pub fn _DidActivate_b__8_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__8_0", ())?;
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
    pub fn remove_didPressContinueButtonEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::GlobalNamespace::SelectRegionViewController_Region,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressContinueButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didPressContinueButtonEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::GlobalNamespace::SelectRegionViewController_Region,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressContinueButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleRegionSelectionDropdownDidSelectCell(
        &mut self,
        dropdown: *mut crate::HMUI::DropdownWithTableView,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRegionSelectionDropdownDidSelectCell", (dropdown, idx))?;
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
#[cfg(feature = "SelectRegionViewController")]
impl quest_hook::libil2cpp::ObjectType for SelectRegionViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "SelectLevelCategoryViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectLevelCategoryViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _allLevelCategoryInfos: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategoryInfo,
    >,
    pub _levelFilterCategoryIconSegmentedControl: *mut crate::HMUI::IconSegmentedControl,
    pub _analyticsModel: *mut crate::GlobalNamespace::IAnalyticsModel,
    pub didSelectLevelCategoryEvent: *mut crate::System::Action_2<
        *mut crate::GlobalNamespace::SelectLevelCategoryViewController,
        crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
    >,
    pub _prevSelectedLevelCategory: crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
    pub _levelCategoryInfos: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategoryInfo,
    >,
}
#[cfg(feature = "SelectLevelCategoryViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SelectLevelCategoryViewController => ""
    ."SelectLevelCategoryViewController"
);
#[cfg(feature = "SelectLevelCategoryViewController")]
impl std::ops::Deref for crate::GlobalNamespace::SelectLevelCategoryViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SelectLevelCategoryViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::SelectLevelCategoryViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SelectLevelCategoryViewController")]
impl crate::GlobalNamespace::SelectLevelCategoryViewController {
    #[cfg(feature = "SelectLevelCategoryViewController+LevelCategory")]
    pub type LevelCategory = crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory;
    #[cfg(feature = "SelectLevelCategoryViewController+LevelCategoryInfo")]
    pub type LevelCategoryInfo = crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategoryInfo;
    #[cfg(feature = "SelectLevelCategoryViewController+__c")]
    pub type __c = crate::GlobalNamespace::SelectLevelCategoryViewController___c;
    #[cfg(feature = "SelectLevelCategoryViewController+__c__DisplayClass12_0")]
    pub type __c__DisplayClass12_0 = crate::GlobalNamespace::SelectLevelCategoryViewController___c__DisplayClass12_0;
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
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidDeactivate", (removedFromHierarchy, screenSystemDisabling))?;
        Ok(__cordl_ret)
    }
    pub fn LevelFilterCategoryIconSegmentedControlDidSelectCell(
        &mut self,
        segmentedControl: *mut crate::HMUI::SegmentedControl,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LevelFilterCategoryIconSegmentedControlDidSelectCell",
                (segmentedControl, index),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Setup(
        &mut self,
        selectedCategory: crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
        enabledLevelCategories: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (selectedCategory, enabledLevelCategories))?;
        Ok(__cordl_ret)
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
    pub fn add_didSelectLevelCategoryEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::SelectLevelCategoryViewController,
            crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectLevelCategoryEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedLevelCategory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory = __cordl_object
            .invoke("get_selectedLevelCategory", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectLevelCategoryEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::SelectLevelCategoryViewController,
            crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectLevelCategoryEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SelectLevelCategoryViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SelectLevelCategoryViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SelectLevelCategoryViewController+LevelCategory")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectLevelCategoryViewController_LevelCategory {
    All = 4i32,
    CustomSongs = 2i32,
    Favorites = 3i32,
    MusicPacks = 1i32,
    None = 0i32,
}
#[cfg(feature = "SelectLevelCategoryViewController+LevelCategory")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory => ""
    ."SelectLevelCategoryViewController/LevelCategory"
);
#[cfg(feature = "SelectLevelCategoryViewController+LevelCategoryInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectLevelCategoryViewController_LevelCategoryInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub levelCategory: crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
    pub localizedKey: *mut quest_hook::libil2cpp::Il2CppString,
    pub categoryIcon: *mut crate::UnityEngine::Sprite,
}
#[cfg(feature = "SelectLevelCategoryViewController+LevelCategoryInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SelectLevelCategoryViewController_LevelCategoryInfo => ""
    ."SelectLevelCategoryViewController/LevelCategoryInfo"
);
#[cfg(feature = "SelectLevelCategoryViewController+LevelCategoryInfo")]
impl std::ops::Deref
for crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategoryInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SelectLevelCategoryViewController+LevelCategoryInfo")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategoryInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SelectLevelCategoryViewController+LevelCategoryInfo")]
impl crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategoryInfo {
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
#[cfg(feature = "SelectLevelCategoryViewController+LevelCategoryInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategoryInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

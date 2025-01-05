#[cfg(feature = "SelectLevelCategoryViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectLevelCategoryViewController {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    pub _allLevelCategoryInfos: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategoryInfo,
            >,
        >,
    >,
    pub _levelFilterCategoryIconSegmentedControl: quest_hook::libil2cpp::Gc<
        crate::HMUI::IconSegmentedControl,
    >,
    pub _analyticsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAnalyticsModel,
    >,
    pub didSelectLevelCategoryEvent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SelectLevelCategoryViewController,
        >,
        crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
    >,
    pub _prevSelectedLevelCategory: crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
    pub _levelCategoryInfos: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategoryInfo,
            >,
        >,
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
    type Target = quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>;
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn LevelFilterCategoryIconSegmentedControlDidSelectCell(
        &mut self,
        segmentedControl: quest_hook::libil2cpp::Gc<crate::HMUI::SegmentedControl>,
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Setup(
        &mut self,
        selectedCategory: crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
        enabledLevelCategories: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (selectedCategory, enabledLevelCategories))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didSelectLevelCategoryEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::SelectLevelCategoryViewController,
            >,
            crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectLevelCategoryEvent", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSelectLevelCategoryEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::SelectLevelCategoryViewController,
            >,
            crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectLevelCategoryEvent", (value))?;
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectLevelCategoryViewController_LevelCategory {
    #[default]
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
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub levelCategory: crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
    pub localizedKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub categoryIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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

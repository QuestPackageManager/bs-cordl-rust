#[cfg(feature = "LevelListTableCell")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelListTableCell {
    __cordl_parent: crate::HMUI::TableCell,
    pub _backgroundImage: *mut crate::UnityEngine::UI::Image,
    pub _canvasGroup: *mut crate::UnityEngine::CanvasGroup,
    pub _coverImage: *mut crate::UnityEngine::UI::Image,
    pub _songNameText: *mut crate::TMPro::TextMeshProUGUI,
    pub _songAuthorText: *mut crate::TMPro::TextMeshProUGUI,
    pub _favoritesBadgeImage: *mut crate::UnityEngine::UI::Image,
    pub _songDurationText: *mut crate::TMPro::TextMeshProUGUI,
    pub _songBpmText: *mut crate::TMPro::TextMeshProUGUI,
    pub _highlightBackgroundColor: crate::UnityEngine::Color,
    pub _selectedBackgroundColor: crate::UnityEngine::Color,
    pub _selectedAndHighlightedBackgroundColor: crate::UnityEngine::Color,
    pub _notOwnedAlpha: f32,
    pub _promoBadgeGo: *mut crate::UnityEngine::GameObject,
    pub _updatedBadgeGo: *mut crate::UnityEngine::GameObject,
    pub _defaultCoverImage: *mut crate::UnityEngine::Sprite,
    pub _refreshingAvailabilityCancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _settingDataCancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _notOwned: bool,
    pub _refreshingAvailabilityLevelID: *mut quest_hook::libil2cpp::Il2CppString,
    pub _settingDataFromLevelId: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "LevelListTableCell")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LevelListTableCell => ""
    ."LevelListTableCell"
);
#[cfg(feature = "LevelListTableCell")]
impl std::ops::Deref for crate::GlobalNamespace::LevelListTableCell {
    type Target = crate::HMUI::TableCell;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelListTableCell")]
impl std::ops::DerefMut for crate::GlobalNamespace::LevelListTableCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelListTableCell")]
impl crate::GlobalNamespace::LevelListTableCell {
    #[cfg(feature = "LevelListTableCell+_RefreshAvailabilityAsync_d__25")]
    pub type _RefreshAvailabilityAsync_d__25 = crate::GlobalNamespace::LevelListTableCell__RefreshAvailabilityAsync_d__25;
    #[cfg(feature = "LevelListTableCell+_SetDataFromLevelAsync_d__20")]
    pub type _SetDataFromLevelAsync_d__20 = crate::GlobalNamespace::LevelListTableCell__SetDataFromLevelAsync_d__20;
    pub fn CancelAsyncOperations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAsyncOperations", ())?;
        Ok(__cordl_ret)
    }
    pub fn HighlightDidChange(
        &mut self,
        transitionType: crate::HMUI::SelectableCell_TransitionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HighlightDidChange", (transitionType))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RefreshAvailabilityAsync(
        &mut self,
        entitlementModel: *mut crate::GlobalNamespace::IEntitlementModel,
        levelID: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshAvailabilityAsync", (entitlementModel, levelID))?;
        Ok(__cordl_ret)
    }
    pub fn RefreshVisuals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshVisuals", ())?;
        Ok(__cordl_ret)
    }
    pub fn SelectionDidChange(
        &mut self,
        transitionType: crate::HMUI::SelectableCell_TransitionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectionDidChange", (transitionType))?;
        Ok(__cordl_ret)
    }
    pub fn SetDataFromLevelAsync(
        &mut self,
        level: *mut crate::GlobalNamespace::BeatmapLevel,
        isFavorite: bool,
        isPromoted: bool,
        isUpdated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetDataFromLevelAsync",
                (level, isFavorite, isPromoted, isUpdated),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WasPreparedForReuse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WasPreparedForReuse", ())?;
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
}
#[cfg(feature = "LevelListTableCell")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LevelListTableCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

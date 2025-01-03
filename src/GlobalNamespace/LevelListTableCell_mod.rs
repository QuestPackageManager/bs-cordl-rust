#[cfg(feature = "LevelListTableCell")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelListTableCell {
    __cordl_parent: crate::HMUI::TableCell,
    pub _backgroundImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub _canvasGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
    pub _coverImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub _songNameText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _songAuthorText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _favoritesBadgeImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub _songDurationText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _songBpmText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _highlightBackgroundColor: crate::UnityEngine::Color,
    pub _selectedBackgroundColor: crate::UnityEngine::Color,
    pub _selectedAndHighlightedBackgroundColor: crate::UnityEngine::Color,
    pub _notOwnedAlpha: f32,
    pub _promoBadgeGo: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _updatedBadgeGo: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _defaultCoverImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _refreshingAvailabilityCancellationTokenSource: quest_hook::libil2cpp::Gc<
        crate::System::Threading::CancellationTokenSource,
    >,
    pub _settingDataCancellationTokenSource: quest_hook::libil2cpp::Gc<
        crate::System::Threading::CancellationTokenSource,
    >,
    pub _beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    pub _notOwned: bool,
    pub _refreshingAvailabilityLevelID: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _settingDataFromLevelId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
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
    pub fn CancelAsyncOperations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAsyncOperations", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshAvailabilityAsync(
        &mut self,
        entitlementModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEntitlementModel,
        >,
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshAvailabilityAsync", (entitlementModel, levelID))?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshVisuals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshVisuals", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetDataFromLevelAsync(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        isFavorite: bool,
        isPromoted: bool,
        isUpdated: bool,
        interactable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetDataFromLevelAsync",
                (beatmapLevel, isFavorite, isPromoted, isUpdated, interactable),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WasPreparedForReuse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WasPreparedForReuse", ())?;
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

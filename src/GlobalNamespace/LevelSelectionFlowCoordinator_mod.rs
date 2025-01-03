#[cfg(feature = "LevelSelectionFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelSelectionFlowCoordinator {
    __cordl_parent: crate::HMUI::FlowCoordinator,
    pub playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub levelSelectionNavigationController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LevelSelectionNavigationController,
    >,
    pub _searchFilterParamsViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SearchFilterParamsViewController,
    >,
    pub _levelSearchViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LevelSearchViewController,
    >,
    pub _startState: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LevelSelectionFlowCoordinator_State,
    >,
}
#[cfg(feature = "LevelSelectionFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LevelSelectionFlowCoordinator
    => ""."LevelSelectionFlowCoordinator"
);
#[cfg(feature = "LevelSelectionFlowCoordinator")]
impl std::ops::Deref for crate::GlobalNamespace::LevelSelectionFlowCoordinator {
    type Target = crate::HMUI::FlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelSelectionFlowCoordinator")]
impl std::ops::DerefMut for crate::GlobalNamespace::LevelSelectionFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelSelectionFlowCoordinator")]
impl crate::GlobalNamespace::LevelSelectionFlowCoordinator {
    #[cfg(feature = "LevelSelectionFlowCoordinator+State")]
    pub type State = crate::GlobalNamespace::LevelSelectionFlowCoordinator_State;
    pub fn ActionButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ActionButtonWasPressed", ())?;
        Ok(__cordl_ret.into())
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
    pub fn HandleLevelSearchViewControllerDidPressSearchButton(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelSearchViewController,
        >,
        filter: crate::GlobalNamespace::LevelFilter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelSearchViewControllerDidPressSearchButton",
                (viewController, filter),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelSelectionNavigationControllerDidChangeDifficultyBeatmap(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelSelectionNavigationController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelSelectionNavigationControllerDidChangeDifficultyBeatmap",
                (viewController),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelSelectionNavigationControllerDidChangeLevelDetailContent(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelSelectionNavigationController,
        >,
        contentType: crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelSelectionNavigationControllerDidChangeLevelDetailContent",
                (viewController, contentType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelSelectionNavigationControllerDidPressActionButton(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelSelectionNavigationController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelSelectionNavigationControllerDidPressActionButton",
                (viewController),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelSelectionNavigationControllerDidPressPracticeButton(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelSelectionNavigationController,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelSelectionNavigationControllerDidPressPracticeButton",
                (viewController, beatmapLevel),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelSelectionNavigationControllerDidSelectPack(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelSelectionNavigationController,
        >,
        pack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelSelectionNavigationControllerDidSelectPack",
                (viewController, pack),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSearchFilterParamsViewControllerDidFinish(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SearchFilterParamsViewController,
        >,
        filter: crate::GlobalNamespace::LevelFilter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSearchFilterParamsViewControllerDidFinish",
                (viewController, filter),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMainViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsMainViewController", (viewController))?;
        Ok(__cordl_ret.into())
    }
    pub fn LevelSelectionFlowCoordinatorDidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LevelSelectionFlowCoordinatorDidActivate",
                (firstActivation, addedToHierarchy),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LevelSelectionFlowCoordinatorDidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LevelSelectionFlowCoordinatorDidDeactivate",
                (removedFromHierarchy),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LevelSelectionFlowCoordinatorTopViewControllerWillChange(
        &mut self,
        oldViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        newViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LevelSelectionFlowCoordinatorTopViewControllerWillChange",
                (oldViewController, newViewController, animationType),
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
    pub fn PracticeButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PracticeButtonWasPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PresentMainViewController(
        &mut self,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PresentMainViewController", (finishedCallback, animationType))?;
        Ok(__cordl_ret.into())
    }
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectionDidChange(
        &mut self,
        pack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectionDidChange", (pack, beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelSelectionFlowCoordinator_State,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn TopViewControllerWillChange(
        &mut self,
        oldViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        newViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TopViewControllerWillChange",
                (oldViewController, newViewController, animationType),
            )?;
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
    pub fn get_actionButtonText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_actionButtonText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_allowedBeatmapDifficultyMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapDifficultyMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapDifficultyMask = __cordl_object
            .invoke("get_allowedBeatmapDifficultyMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enableCustomLevels(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableCustomLevels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hidePacksIfOneOrNone(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hidePacksIfOneOrNone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hidePracticeButton(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hidePracticeButton", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_initialLeftScreenViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController> = __cordl_object
            .invoke("get_initialLeftScreenViewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_initialRightScreenViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController> = __cordl_object
            .invoke("get_initialRightScreenViewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_initialTopScreenViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController> = __cordl_object
            .invoke("get_initialTopScreenViewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isInRootViewController(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInRootViewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mainTitle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_mainTitle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_notAllowedCharacteristics(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
            >,
        > = __cordl_object.invoke("get_notAllowedCharacteristics", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedBeatmapKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapKey = __cordl_object
            .invoke("get_selectedBeatmapKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedBeatmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevel,
        > = __cordl_object.invoke("get_selectedBeatmapLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedBeatmapLevelPack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        > = __cordl_object.invoke("get_selectedBeatmapLevelPack", ())?;
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
    pub fn get_showBackButtonForMainViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_showBackButtonForMainViewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songPackMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SongPackMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::SongPackMask = __cordl_object
            .invoke("get_songPackMask", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LevelSelectionFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LevelSelectionFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LevelSelectionFlowCoordinator+State")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelSelectionFlowCoordinator_State {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub levelCategory: crate::System::Nullable_1<
        crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
    >,
    pub beatmapLevelPack: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelPack,
    >,
    pub beatmapKey: crate::GlobalNamespace::BeatmapKey,
    pub beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
}
#[cfg(feature = "LevelSelectionFlowCoordinator+State")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LevelSelectionFlowCoordinator_State => ""
    ."LevelSelectionFlowCoordinator/State"
);
#[cfg(feature = "LevelSelectionFlowCoordinator+State")]
impl std::ops::Deref for crate::GlobalNamespace::LevelSelectionFlowCoordinator_State {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelSelectionFlowCoordinator+State")]
impl std::ops::DerefMut for crate::GlobalNamespace::LevelSelectionFlowCoordinator_State {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelSelectionFlowCoordinator+State")]
impl crate::GlobalNamespace::LevelSelectionFlowCoordinator_State {
    pub fn New_BeatmapLevelPack1(
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapLevelPack))?;
        Ok(__cordl_object.into())
    }
    pub fn New_BeatmapLevelPack_BeatmapLevel2(
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapLevelPack, beatmapLevel))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Nullable_1_BeatmapLevelPack_ByRefMut_BeatmapLevel0(
        levelCategory: crate::System::Nullable_1<
            crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
        >,
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (levelCategory, beatmapLevelPack, beatmapKey, beatmapLevel),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_BeatmapLevelPack1(
        &mut self,
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapLevelPack))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_BeatmapLevelPack_BeatmapLevel2(
        &mut self,
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapLevelPack, beatmapLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Nullable_1_BeatmapLevelPack_ByRefMut_BeatmapLevel0(
        &mut self,
        levelCategory: crate::System::Nullable_1<
            crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
        >,
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (levelCategory, beatmapLevelPack, beatmapKey, beatmapLevel),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LevelSelectionFlowCoordinator+State")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LevelSelectionFlowCoordinator_State {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

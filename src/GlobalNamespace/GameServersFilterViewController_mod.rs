#[cfg(feature = "GameServersFilterViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServersFilterViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _filterByDifficultyToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _filterByDifficultyButton: *mut crate::UnityEngine::UI::Button,
    pub _beatmapDifficultyDropdown: *mut crate::GlobalNamespace::BeatmapDifficultyDropdown,
    pub _filterByModifiersToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _filterByModifiersButton: *mut crate::UnityEngine::UI::Button,
    pub _gameplayModifiersDropdown: *mut crate::GlobalNamespace::GameplayModifiersDropdown,
    pub _filterBySongsToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _filterBySongsButton: *mut crate::UnityEngine::UI::Button,
    pub _songPacksDropdown: *mut crate::GlobalNamespace::SongPacksDropdown,
    pub _showFullToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _showPasswordProtectedToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _toggleBinder: *mut crate::HMUI::ToggleBinder,
    pub _showInternetGames: bool,
}
#[cfg(feature = "GameServersFilterViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameServersFilterViewController
    => ""."GameServersFilterViewController"
);
#[cfg(feature = "GameServersFilterViewController")]
impl std::ops::Deref for crate::GlobalNamespace::GameServersFilterViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameServersFilterViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameServersFilterViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameServersFilterViewController")]
impl crate::GlobalNamespace::GameServersFilterViewController {
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn Refresh(
        &mut self,
        currentFilter: *mut crate::GlobalNamespace::GameServersFilter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", (currentFilter))?;
        Ok(__cordl_ret)
    }
    pub fn SetupGameServersFilter(
        &mut self,
        gameServersFilter: *mut crate::GlobalNamespace::GameServersFilter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupGameServersFilter", (gameServersFilter))?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__16_0(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__16_0", (isOn))?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__16_1(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__16_1", (isOn))?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__16_2(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__16_2", (isOn))?;
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
    pub fn get_gameServersFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::GameServersFilter> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::GameServersFilter = __cordl_object
            .invoke("get_gameServersFilter", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "GameServersFilterViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameServersFilterViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

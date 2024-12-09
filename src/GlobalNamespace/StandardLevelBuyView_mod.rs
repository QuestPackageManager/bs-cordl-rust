#[cfg(feature = "StandardLevelBuyView")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelBuyView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _levelBar: *mut crate::GlobalNamespace::LevelBar,
    pub _buyButton: *mut crate::UnityEngine::UI::Button,
    pub _favoriteToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _steamMessageGameObject: *mut crate::UnityEngine::GameObject,
    pub didFavoriteToggleChangeEvent: *mut crate::System::Action_1<
        *mut crate::UnityEngine::UI::Toggle,
    >,
    pub _toggleBinder: *mut crate::HMUI::ToggleBinder,
    pub _beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
    pub _playerData: *mut crate::GlobalNamespace::PlayerData,
}
#[cfg(feature = "StandardLevelBuyView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StandardLevelBuyView => ""
    ."StandardLevelBuyView"
);
#[cfg(feature = "StandardLevelBuyView")]
impl std::ops::Deref for crate::GlobalNamespace::StandardLevelBuyView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelBuyView")]
impl std::ops::DerefMut for crate::GlobalNamespace::StandardLevelBuyView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelBuyView")]
impl crate::GlobalNamespace::StandardLevelBuyView {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
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
    pub fn RefreshFavouriteToggle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshFavouriteToggle", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetContent(
        &mut self,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        playerData: *mut crate::GlobalNamespace::PlayerData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetContent", (beatmapLevel, playerData))?;
        Ok(__cordl_ret)
    }
    pub fn _Awake_b__12_0(
        &mut self,
        _cordl__: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Awake>b__12_0", (_cordl__))?;
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
    pub fn add_didFavoriteToggleChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::UnityEngine::UI::Toggle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFavoriteToggleChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_buyButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Button> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Button = __cordl_object
            .invoke("get_buyButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFavoriteToggleChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::UnityEngine::UI::Toggle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFavoriteToggleChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "StandardLevelBuyView")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::StandardLevelBuyView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

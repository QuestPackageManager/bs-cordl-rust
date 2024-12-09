#[cfg(feature = "LevelPackDetailViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelPackDetailViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _detailWrapper: *mut crate::UnityEngine::GameObject,
    pub _packImage: *mut crate::HMUI::ImageView,
    pub _buyButton: *mut crate::UnityEngine::UI::Button,
    pub _buyContainer: *mut crate::UnityEngine::GameObject,
    pub _loadingControl: *mut crate::GlobalNamespace::LoadingControl,
    pub _requireInternetContainer: *mut crate::UnityEngine::GameObject,
    pub _steamMessageGameObject: *mut crate::UnityEngine::GameObject,
    pub _kawaseBlurRenderer: *mut crate::GlobalNamespace::KawaseBlurRendererSO,
    pub _defaultCoverSprite: *mut crate::UnityEngine::Sprite,
    pub _additionalContentModel: *mut crate::GlobalNamespace::IAdditionalContentModel,
    pub _entitlementModel: *mut crate::GlobalNamespace::IEntitlementModel,
    pub _dlcPromoPanelModel: *mut crate::GlobalNamespace::DlcPromoPanelModel,
    pub _analyticsModel: *mut crate::GlobalNamespace::IAnalyticsModel,
    pub _eventBinder: *mut crate::GlobalNamespace::EventBinder,
    pub _cancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _pack: *mut crate::GlobalNamespace::BeatmapLevelPack,
    pub _blurredPackArtwork: *mut crate::UnityEngine::Sprite,
}
#[cfg(feature = "LevelPackDetailViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LevelPackDetailViewController
    => ""."LevelPackDetailViewController"
);
#[cfg(feature = "LevelPackDetailViewController")]
impl std::ops::Deref for crate::GlobalNamespace::LevelPackDetailViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelPackDetailViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::LevelPackDetailViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelPackDetailViewController")]
impl crate::GlobalNamespace::LevelPackDetailViewController {
    #[cfg(feature = "LevelPackDetailViewController+ContentType")]
    pub type ContentType = crate::GlobalNamespace::LevelPackDetailViewController_ContentType;
    #[cfg(
        feature = "LevelPackDetailViewController+_OpenLevelPackProductStoreAsync_d__24"
    )]
    pub type _OpenLevelPackProductStoreAsync_d__24 = crate::GlobalNamespace::LevelPackDetailViewController__OpenLevelPackProductStoreAsync_d__24;
    #[cfg(feature = "LevelPackDetailViewController+_RefreshAvailabilityAsync_d__22")]
    pub type _RefreshAvailabilityAsync_d__22 = crate::GlobalNamespace::LevelPackDetailViewController__RefreshAvailabilityAsync_d__22;
    pub fn BuyPackButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuyPackButtonWasPressed", ())?;
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
    pub fn HandleAdditionalContentModelDidInvalidateData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAdditionalContentModelDidInvalidateData", ())?;
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
    pub fn OpenLevelPackProductStoreAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenLevelPackProductStoreAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn RefreshAvailabilityAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshAvailabilityAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetData(
        &mut self,
        pack: *mut crate::GlobalNamespace::BeatmapLevelPack,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (pack))?;
        Ok(__cordl_ret)
    }
    pub fn ShowContent(
        &mut self,
        contentType: crate::GlobalNamespace::LevelPackDetailViewController_ContentType,
        errorText: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowContent", (contentType, errorText))?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__19_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__19_1", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__19_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__19_2", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_g__HandleDidPressRefreshButton_19_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>g__HandleDidPressRefreshButton|19_0", ())?;
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
#[cfg(feature = "LevelPackDetailViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LevelPackDetailViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LevelPackDetailViewController+ContentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelPackDetailViewController_ContentType {
    Buy = 2i32,
    Error = 3i32,
    Loading = 0i32,
    Owned = 1i32,
}
#[cfg(feature = "LevelPackDetailViewController+ContentType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LevelPackDetailViewController_ContentType => ""
    ."LevelPackDetailViewController/ContentType"
);

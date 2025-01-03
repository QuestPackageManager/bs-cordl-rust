#[cfg(feature = "MusicPackPromoBanner")]
#[repr(C)]
#[derive(Debug)]
pub struct MusicPackPromoBanner {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _promoBannerGo: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _loadingIndicator: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _promoBannerTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::RectTransform,
    >,
    pub _promoText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _backgroundImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub _goButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _promoInfo_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
    >,
    pub _text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "MusicPackPromoBanner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MusicPackPromoBanner => ""
    ."MusicPackPromoBanner"
);
#[cfg(feature = "MusicPackPromoBanner")]
impl std::ops::Deref for crate::GlobalNamespace::MusicPackPromoBanner {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MusicPackPromoBanner")]
impl std::ops::DerefMut for crate::GlobalNamespace::MusicPackPromoBanner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MusicPackPromoBanner")]
impl crate::GlobalNamespace::MusicPackPromoBanner {
    pub const kGetNow: &'static str = "PROMO_GET_NOW_LABEL";
    pub const kPlayNow: &'static str = "PROMO_BANNER_PLAY_NOW_LABEL";
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Setup(
        &mut self,
        newPromoInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
        >,
        probablyOwned: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (newPromoInfo, probablyOwned))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartLoading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartLoading", ())?;
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
    pub fn get_promoButtonText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_promoButtonText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_promoInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
        > = __cordl_object.invoke("get_promoInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_promoInfo(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_promoInfo", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MusicPackPromoBanner")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MusicPackPromoBanner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "PlayingCampaignRichPresenceData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayingCampaignRichPresenceData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _localizedDescription: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "PlayingCampaignRichPresenceData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayingCampaignRichPresenceData
    => ""."PlayingCampaignRichPresenceData"
);
#[cfg(feature = "PlayingCampaignRichPresenceData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayingCampaignRichPresenceData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayingCampaignRichPresenceData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayingCampaignRichPresenceData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayingCampaignRichPresenceData")]
impl crate::GlobalNamespace::PlayingCampaignRichPresenceData {
    pub const kPlayingCampaignRichPresenceLocalizationKey: &'static str = "PLAYING_CAMPAIGN_PRESENCE";
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
    pub fn get_apiName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_apiName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_localizedDescription(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_localizedDescription", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlayingCampaignRichPresenceData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayingCampaignRichPresenceData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayingCampaignRichPresenceData")]
impl AsRef<crate::GlobalNamespace::IRichPresenceData>
for crate::GlobalNamespace::PlayingCampaignRichPresenceData {
    fn as_ref(&self) -> &crate::GlobalNamespace::IRichPresenceData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PlayingCampaignRichPresenceData")]
impl AsMut<crate::GlobalNamespace::IRichPresenceData>
for crate::GlobalNamespace::PlayingCampaignRichPresenceData {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IRichPresenceData {
        unsafe { std::mem::transmute(self) }
    }
}

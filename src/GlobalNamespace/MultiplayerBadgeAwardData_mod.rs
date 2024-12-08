#[cfg(feature = "MultiplayerBadgeAwardData")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerBadgeAwardData {
    __cordl_parent: crate::System::Object,
    pub _awardedPlayer: *mut crate::GlobalNamespace::IConnectedPlayer,
    pub _weight: f32,
    pub _title: *mut crate::System::String,
    pub _subtitle: *mut crate::System::String,
    pub _icon: *mut crate::UnityEngine::Sprite,
    pub _badgeData: *mut crate::GlobalNamespace::MultiplayerBadgeDataSO,
}
#[cfg(feature = "MultiplayerBadgeAwardData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerBadgeAwardData => ""
    ."MultiplayerBadgeAwardData"
);
#[cfg(feature = "MultiplayerBadgeAwardData")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerBadgeAwardData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeAwardData")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerBadgeAwardData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeAwardData")]
impl crate::GlobalNamespace::MultiplayerBadgeAwardData {
    pub fn CompareTo(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        awardedPlayer: *mut crate::GlobalNamespace::IConnectedPlayer,
        weight: f32,
        title: *mut crate::System::String,
        subtitle: *mut crate::System::String,
        badgeData: *mut crate::GlobalNamespace::MultiplayerBadgeDataSO,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (awardedPlayer, weight, title, subtitle, badgeData))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        awardedPlayer: *mut crate::GlobalNamespace::IConnectedPlayer,
        weight: f32,
        title: *mut crate::System::String,
        subtitle: *mut crate::System::String,
        badgeData: *mut crate::GlobalNamespace::MultiplayerBadgeDataSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (awardedPlayer, weight, title, subtitle, badgeData))?;
        Ok(__cordl_ret)
    }
    pub fn get_awardedPlayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::IConnectedPlayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::IConnectedPlayer = __cordl_object
            .invoke("get_awardedPlayer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_icon(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("get_icon", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_subtitle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_subtitle", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_title(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_title", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_titleLocalizationKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_titleLocalizationKey", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerBadgeAwardData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerBadgeAwardData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

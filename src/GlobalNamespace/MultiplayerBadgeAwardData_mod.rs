#[cfg(feature = "MultiplayerBadgeAwardData")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerBadgeAwardData {
    __cordl_parent: crate::System::Object,
    pub _awardedPlayer: *mut IConnectedPlayer,
    pub _weight: f32,
    pub _title: *mut crate::System::String,
    pub _subtitle: *mut crate::System::String,
    pub _icon: *mut crate::UnityEngine::Sprite,
    pub _badgeData: *mut MultiplayerBadgeDataSO,
}
#[cfg(feature = "MultiplayerBadgeAwardData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerBadgeAwardData => ""
    ."MultiplayerBadgeAwardData"
);
#[cfg(feature = "MultiplayerBadgeAwardData")]
impl std::ops::Deref for MultiplayerBadgeAwardData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeAwardData")]
impl std::ops::DerefMut for MultiplayerBadgeAwardData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeAwardData")]
impl MultiplayerBadgeAwardData {
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
    pub fn get_awardedPlayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut IConnectedPlayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IConnectedPlayer = __cordl_object
            .invoke("get_awardedPlayer", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        awardedPlayer: *mut IConnectedPlayer,
        weight: f32,
        title: *mut crate::System::String,
        subtitle: *mut crate::System::String,
        badgeData: *mut MultiplayerBadgeDataSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (awardedPlayer, weight, title, subtitle, badgeData))?;
        Ok(__cordl_ret)
    }
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
    pub fn New(
        awardedPlayer: *mut IConnectedPlayer,
        weight: f32,
        title: *mut crate::System::String,
        subtitle: *mut crate::System::String,
        badgeData: *mut MultiplayerBadgeDataSO,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (awardedPlayer, weight, title, subtitle, badgeData))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MultiplayerBadgeAwardData")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerBadgeAwardData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

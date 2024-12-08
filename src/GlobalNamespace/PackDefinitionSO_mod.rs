#[cfg(feature = "PackDefinitionSO+Credits")]
#[repr(C)]
#[derive(Debug)]
pub struct PackDefinitionSO_Credits {
    __cordl_parent: crate::System::Object,
    pub order: i32,
    pub creditsPrefab: *mut crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
}
#[cfg(feature = "PackDefinitionSO+Credits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PackDefinitionSO_Credits => ""
    ."PackDefinitionSO/Credits"
);
#[cfg(feature = "PackDefinitionSO+Credits")]
impl std::ops::Deref for crate::GlobalNamespace::PackDefinitionSO_Credits {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PackDefinitionSO+Credits")]
impl std::ops::DerefMut for crate::GlobalNamespace::PackDefinitionSO_Credits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PackDefinitionSO+Credits")]
impl crate::GlobalNamespace::PackDefinitionSO_Credits {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PackDefinitionSO+Credits")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PackDefinitionSO_Credits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PackDefinitionSO+LeaderboardIds")]
#[repr(C)]
#[derive(Debug)]
pub struct PackDefinitionSO_LeaderboardIds {
    __cordl_parent: crate::System::Object,
    pub _riftLeaderboardIds: *mut LeaderboardIdsSO,
    pub _questLeaderboardIds: *mut LeaderboardIdsSO,
    pub _steamLeaderboardIds: *mut LeaderboardIdsSO,
    pub _ps4LeaderboardIds: *mut SonyLeaderboardIdsSO,
    pub _ps5LeaderboardIds: *mut SonyLeaderboardIdsSO,
}
#[cfg(feature = "PackDefinitionSO+LeaderboardIds")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PackDefinitionSO_LeaderboardIds
    => ""."PackDefinitionSO/LeaderboardIds"
);
#[cfg(feature = "PackDefinitionSO+LeaderboardIds")]
impl std::ops::Deref for crate::GlobalNamespace::PackDefinitionSO_LeaderboardIds {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PackDefinitionSO+LeaderboardIds")]
impl std::ops::DerefMut for crate::GlobalNamespace::PackDefinitionSO_LeaderboardIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PackDefinitionSO+LeaderboardIds")]
impl crate::GlobalNamespace::PackDefinitionSO_LeaderboardIds {
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
    pub fn get_riftLeaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut LeaderboardIdsSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut LeaderboardIdsSO = __cordl_object
            .invoke("get_riftLeaderboardIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_steamLeaderboardIds(
        &mut self,
        value: *mut LeaderboardIdsSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_steamLeaderboardIds", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_ps4LeaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut SonyLeaderboardIdsSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut SonyLeaderboardIdsSO = __cordl_object
            .invoke("get_ps4LeaderboardIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_questLeaderboardIds(
        &mut self,
        value: *mut LeaderboardIdsSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_questLeaderboardIds", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_steamLeaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut LeaderboardIdsSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut LeaderboardIdsSO = __cordl_object
            .invoke("get_steamLeaderboardIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_questLeaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut LeaderboardIdsSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut LeaderboardIdsSO = __cordl_object
            .invoke("get_questLeaderboardIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ps4LeaderboardIds(
        &mut self,
        value: *mut SonyLeaderboardIdsSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ps4LeaderboardIds", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_riftLeaderboardIds(
        &mut self,
        value: *mut LeaderboardIdsSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_riftLeaderboardIds", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_ps5LeaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut SonyLeaderboardIdsSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut SonyLeaderboardIdsSO = __cordl_object
            .invoke("get_ps5LeaderboardIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ps5LeaderboardIds(
        &mut self,
        value: *mut SonyLeaderboardIdsSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ps5LeaderboardIds", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PackDefinitionSO+LeaderboardIds")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PackDefinitionSO_LeaderboardIds {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PackDefinitionSO+LevelProductPacks")]
#[repr(C)]
#[derive(Debug)]
pub struct PackDefinitionSO_LevelProductPacks {
    __cordl_parent: crate::System::Object,
    pub _steamLevelProductPacks: *mut SteamLevelProductPacksSO,
    pub _oculusLevelProductPacks: *mut OculusLevelProductPacksSO,
    pub _sonyLevelProductPackSource: *mut SonyLevelProductPackSourceSO,
    pub _ps4LevelProductPacks: *mut PS4LevelProductPacksSO,
    pub _ps5LevelProductPacks: *mut PS5LevelProductPacksSO,
}
#[cfg(feature = "PackDefinitionSO+LevelProductPacks")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PackDefinitionSO_LevelProductPacks => ""
    ."PackDefinitionSO/LevelProductPacks"
);
#[cfg(feature = "PackDefinitionSO+LevelProductPacks")]
impl std::ops::Deref for crate::GlobalNamespace::PackDefinitionSO_LevelProductPacks {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PackDefinitionSO+LevelProductPacks")]
impl std::ops::DerefMut for crate::GlobalNamespace::PackDefinitionSO_LevelProductPacks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PackDefinitionSO+LevelProductPacks")]
impl crate::GlobalNamespace::PackDefinitionSO_LevelProductPacks {
    pub fn get_ps4LevelProductPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PS4LevelProductPacksSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PS4LevelProductPacksSO = __cordl_object
            .invoke("get_ps4LevelProductPacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sonyLevelProductPackSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut SonyLevelProductPackSourceSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut SonyLevelProductPackSourceSO = __cordl_object
            .invoke("get_sonyLevelProductPackSource", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ps5LevelProductPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PS5LevelProductPacksSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PS5LevelProductPacksSO = __cordl_object
            .invoke("get_ps5LevelProductPacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_sonyLevelProductPackSource(
        &mut self,
        value: *mut SonyLevelProductPackSourceSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sonyLevelProductPackSource", (value))?;
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
    pub fn get_steamLevelProductPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut SteamLevelProductPacksSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut SteamLevelProductPacksSO = __cordl_object
            .invoke("get_steamLevelProductPacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ps5LevelProductPacks(
        &mut self,
        value: *mut PS5LevelProductPacksSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ps5LevelProductPacks", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_steamLevelProductPacks(
        &mut self,
        value: *mut SteamLevelProductPacksSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_steamLevelProductPacks", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_oculusLevelProductPacks(
        &mut self,
        value: *mut OculusLevelProductPacksSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_oculusLevelProductPacks", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ps4LevelProductPacks(
        &mut self,
        value: *mut PS4LevelProductPacksSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ps4LevelProductPacks", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_oculusLevelProductPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut OculusLevelProductPacksSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut OculusLevelProductPacksSO = __cordl_object
            .invoke("get_oculusLevelProductPacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PackDefinitionSO+LevelProductPacks")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PackDefinitionSO_LevelProductPacks {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PackDefinitionSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PackDefinitionSO {
    __cordl_parent: PersistentScriptableObject,
    pub _beatmapLevelPack: *mut BeatmapLevelPackSO,
    pub _order: i32,
    pub _leaderboardIds: *mut crate::GlobalNamespace::PackDefinitionSO_LeaderboardIds,
    pub _levelProductPacks: *mut crate::GlobalNamespace::PackDefinitionSO_LevelProductPacks,
    pub _tags: crate::GlobalNamespace::PackDefinitionSO_Tags,
    pub _packPromoInfoReference: *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
        *mut PackPromoInfoSO,
    >,
    pub _perceivedLoudnessPerLevel: *mut PerceivedLoudnessSO,
    pub _credits: *mut crate::GlobalNamespace::PackDefinitionSO_Credits,
}
#[cfg(feature = "PackDefinitionSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PackDefinitionSO => ""."PackDefinitionSO"
);
#[cfg(feature = "PackDefinitionSO")]
impl std::ops::Deref for PackDefinitionSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PackDefinitionSO")]
impl std::ops::DerefMut for PackDefinitionSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PackDefinitionSO")]
impl PackDefinitionSO {
    #[cfg(feature = "PackDefinitionSO+LeaderboardIds")]
    pub type LeaderboardIds = crate::GlobalNamespace::PackDefinitionSO_LeaderboardIds;
    #[cfg(feature = "PackDefinitionSO+LevelProductPacks")]
    pub type LevelProductPacks = crate::GlobalNamespace::PackDefinitionSO_LevelProductPacks;
    #[cfg(feature = "PackDefinitionSO+Credits")]
    pub type Credits = crate::GlobalNamespace::PackDefinitionSO_Credits;
    #[cfg(feature = "PackDefinitionSO+Tags")]
    pub type Tags = crate::GlobalNamespace::PackDefinitionSO_Tags;
    pub fn get_isBuiltInContent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isBuiltInContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_perceivedLoudness(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PerceivedLoudnessSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PerceivedLoudnessSO = __cordl_object
            .invoke("get_perceivedLoudness", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_packPromoInfoReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
            *mut PackPromoInfoSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
            *mut PackPromoInfoSO,
        > = __cordl_object.invoke("get_packPromoInfoReference", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isBuiltIn(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isBuiltIn", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_content(
        &mut self,
        value: *mut BeatmapLevelPackSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_content", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_levelProductPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PackDefinitionSO_LevelProductPacks,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PackDefinitionSO_LevelProductPacks = __cordl_object
            .invoke("get_levelProductPacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_perceivedLoudness(
        &mut self,
        value: *mut PerceivedLoudnessSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_perceivedLoudness", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_credits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PackDefinitionSO_Credits,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PackDefinitionSO_Credits = __cordl_object
            .invoke("get_credits", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_tags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PackDefinitionSO_Tags> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::PackDefinitionSO_Tags = __cordl_object
            .invoke("get_tags", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_order(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_order", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_tags(
        &mut self,
        value: crate::GlobalNamespace::PackDefinitionSO_Tags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_tags", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_hasCredits(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasCredits", ())?;
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
    pub fn get_isEntitlementCheckSkipped(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isEntitlementCheckSkipped", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isOST(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isOST", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_levelProductPacks(
        &mut self,
        value: *mut crate::GlobalNamespace::PackDefinitionSO_LevelProductPacks,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_levelProductPacks", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_content(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapLevelPackSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapLevelPackSO = __cordl_object
            .invoke("get_content", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isHidden(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isHidden", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PackDefinitionSO_LeaderboardIds,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PackDefinitionSO_LeaderboardIds = __cordl_object
            .invoke("get_leaderboardIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_order(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_order", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isInDevelopment(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInDevelopment", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isDLC(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDLC", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PackDefinitionSO")]
impl quest_hook::libil2cpp::ObjectType for PackDefinitionSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PackDefinitionSO+Tags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackDefinitionSO_Tags {
    BuiltIn = 1i32,
    DLC = 2i32,
    Hidden = 0i32,
    InDevelopment = 4i32,
}
#[cfg(feature = "PackDefinitionSO+Tags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PackDefinitionSO_Tags => ""
    ."PackDefinitionSO/Tags"
);

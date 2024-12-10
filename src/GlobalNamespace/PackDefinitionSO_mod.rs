#[cfg(feature = "PackDefinitionSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PackDefinitionSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _beatmapLevelPack: *mut crate::GlobalNamespace::BeatmapLevelPackSO,
    pub _order: i32,
    pub _leaderboardIds: *mut crate::GlobalNamespace::PackDefinitionSO_LeaderboardIds,
    pub _levelProductPacks: *mut crate::GlobalNamespace::PackDefinitionSO_LevelProductPacks,
    pub _tags: crate::GlobalNamespace::PackDefinitionSO_Tags,
    pub _packPromoInfoReference: *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
        *mut crate::GlobalNamespace::PackPromoInfoSO,
    >,
    pub _perceivedLoudnessPerLevel: *mut crate::GlobalNamespace::PerceivedLoudnessSO,
    pub _credits: *mut crate::GlobalNamespace::PackDefinitionSO_Credits,
}
#[cfg(feature = "PackDefinitionSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PackDefinitionSO => ""
    ."PackDefinitionSO"
);
#[cfg(feature = "PackDefinitionSO")]
impl std::ops::Deref for crate::GlobalNamespace::PackDefinitionSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PackDefinitionSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::PackDefinitionSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PackDefinitionSO")]
impl crate::GlobalNamespace::PackDefinitionSO {
    #[cfg(feature = "PackDefinitionSO+Credits")]
    pub type Credits = crate::GlobalNamespace::PackDefinitionSO_Credits;
    #[cfg(feature = "PackDefinitionSO+LeaderboardIds")]
    pub type LeaderboardIds = crate::GlobalNamespace::PackDefinitionSO_LeaderboardIds;
    #[cfg(feature = "PackDefinitionSO+LevelProductPacks")]
    pub type LevelProductPacks = crate::GlobalNamespace::PackDefinitionSO_LevelProductPacks;
    #[cfg(feature = "PackDefinitionSO+Tags")]
    pub type Tags = crate::GlobalNamespace::PackDefinitionSO_Tags;
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
    pub fn get_content(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPackSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPackSO,
        > = __cordl_object.invoke("get_content", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_credits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PackDefinitionSO_Credits>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PackDefinitionSO_Credits,
        > = __cordl_object.invoke("get_credits", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasCredits(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasCredits", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isBuiltIn(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isBuiltIn", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isBuiltInContent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isBuiltInContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isDLC(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDLC", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isEntitlementCheckSkipped(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isEntitlementCheckSkipped", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isHidden(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isHidden", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isInDevelopment(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInDevelopment", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isOST(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isOST", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PackDefinitionSO_LeaderboardIds,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PackDefinitionSO_LeaderboardIds,
        > = __cordl_object.invoke("get_leaderboardIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelProductPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PackDefinitionSO_LevelProductPacks,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PackDefinitionSO_LevelProductPacks,
        > = __cordl_object.invoke("get_levelProductPacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_order(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_order", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_packPromoInfoReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
                *mut crate::GlobalNamespace::PackPromoInfoSO,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
                *mut crate::GlobalNamespace::PackPromoInfoSO,
            >,
        > = __cordl_object.invoke("get_packPromoInfoReference", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_perceivedLoudness(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PerceivedLoudnessSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PerceivedLoudnessSO,
        > = __cordl_object.invoke("get_perceivedLoudness", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PackDefinitionSO_Tags> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::PackDefinitionSO_Tags = __cordl_object
            .invoke("get_tags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_content(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPackSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_content", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_levelProductPacks(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PackDefinitionSO_LevelProductPacks,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_levelProductPacks", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_perceivedLoudness(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PerceivedLoudnessSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_perceivedLoudness", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PackDefinitionSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PackDefinitionSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PackDefinitionSO+Credits")]
#[repr(C)]
#[derive(Debug)]
pub struct PackDefinitionSO_Credits {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _riftLeaderboardIds: *mut crate::GlobalNamespace::LeaderboardIdsSO,
    pub _questLeaderboardIds: *mut crate::GlobalNamespace::LeaderboardIdsSO,
    pub _steamLeaderboardIds: *mut crate::GlobalNamespace::LeaderboardIdsSO,
    pub _ps4LeaderboardIds: *mut crate::GlobalNamespace::SonyLeaderboardIdsSO,
    pub _ps5LeaderboardIds: *mut crate::GlobalNamespace::SonyLeaderboardIdsSO,
}
#[cfg(feature = "PackDefinitionSO+LeaderboardIds")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PackDefinitionSO_LeaderboardIds
    => ""."PackDefinitionSO/LeaderboardIds"
);
#[cfg(feature = "PackDefinitionSO+LeaderboardIds")]
impl std::ops::Deref for crate::GlobalNamespace::PackDefinitionSO_LeaderboardIds {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn get_ps4LeaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SonyLeaderboardIdsSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyLeaderboardIdsSO,
        > = __cordl_object.invoke("get_ps4LeaderboardIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ps5LeaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SonyLeaderboardIdsSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyLeaderboardIdsSO,
        > = __cordl_object.invoke("get_ps5LeaderboardIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_questLeaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LeaderboardIdsSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LeaderboardIdsSO,
        > = __cordl_object.invoke("get_questLeaderboardIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_riftLeaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LeaderboardIdsSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LeaderboardIdsSO,
        > = __cordl_object.invoke("get_riftLeaderboardIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_steamLeaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LeaderboardIdsSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LeaderboardIdsSO,
        > = __cordl_object.invoke("get_steamLeaderboardIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ps4LeaderboardIds(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SonyLeaderboardIdsSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ps4LeaderboardIds", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ps5LeaderboardIds(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SonyLeaderboardIdsSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ps5LeaderboardIds", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_questLeaderboardIds(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LeaderboardIdsSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_questLeaderboardIds", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_riftLeaderboardIds(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LeaderboardIdsSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_riftLeaderboardIds", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_steamLeaderboardIds(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LeaderboardIdsSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_steamLeaderboardIds", (value))?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _steamLevelProductPacks: *mut crate::GlobalNamespace::SteamLevelProductPacksSO,
    pub _oculusLevelProductPacks: *mut crate::GlobalNamespace::OculusLevelProductPacksSO,
    pub _sonyLevelProductPackSource: *mut crate::GlobalNamespace::SonyLevelProductPackSourceSO,
    pub _ps4LevelProductPacks: *mut crate::GlobalNamespace::PS4LevelProductPacksSO,
    pub _ps5LevelProductPacks: *mut crate::GlobalNamespace::PS5LevelProductPacksSO,
}
#[cfg(feature = "PackDefinitionSO+LevelProductPacks")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PackDefinitionSO_LevelProductPacks => ""
    ."PackDefinitionSO/LevelProductPacks"
);
#[cfg(feature = "PackDefinitionSO+LevelProductPacks")]
impl std::ops::Deref for crate::GlobalNamespace::PackDefinitionSO_LevelProductPacks {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn get_oculusLevelProductPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OculusLevelProductPacksSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductPacksSO,
        > = __cordl_object.invoke("get_oculusLevelProductPacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ps4LevelProductPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PS4LevelProductPacksSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PS4LevelProductPacksSO,
        > = __cordl_object.invoke("get_ps4LevelProductPacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ps5LevelProductPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PS5LevelProductPacksSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PS5LevelProductPacksSO,
        > = __cordl_object.invoke("get_ps5LevelProductPacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sonyLevelProductPackSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SonyLevelProductPackSourceSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyLevelProductPackSourceSO,
        > = __cordl_object.invoke("get_sonyLevelProductPackSource", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_steamLevelProductPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SteamLevelProductPacksSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductPacksSO,
        > = __cordl_object.invoke("get_steamLevelProductPacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_oculusLevelProductPacks(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductPacksSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_oculusLevelProductPacks", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ps4LevelProductPacks(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PS4LevelProductPacksSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ps4LevelProductPacks", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ps5LevelProductPacks(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PS5LevelProductPacksSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ps5LevelProductPacks", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sonyLevelProductPackSource(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyLevelProductPackSourceSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sonyLevelProductPackSource", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_steamLevelProductPacks(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductPacksSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_steamLevelProductPacks", (value))?;
        Ok(__cordl_ret.into())
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

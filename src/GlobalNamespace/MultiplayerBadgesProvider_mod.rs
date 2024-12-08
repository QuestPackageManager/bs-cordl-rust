#[cfg(feature = "MultiplayerBadgesProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerBadgesProvider {
    __cordl_parent: crate::System::Object,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _beatmapKey: BeatmapKey,
    pub _multiplayerBadgesModel: *mut MultiplayerBadgesModelSO,
}
#[cfg(feature = "MultiplayerBadgesProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerBadgesProvider => ""
    ."MultiplayerBadgesProvider"
);
#[cfg(feature = "MultiplayerBadgesProvider")]
impl std::ops::Deref for MultiplayerBadgesProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgesProvider")]
impl std::ops::DerefMut for MultiplayerBadgesProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgesProvider")]
impl MultiplayerBadgesProvider {
    pub const kMaxRandomMultiplierAmount: f32 = 1.2f32;
    pub const kMinRandomMultiplierAmount: f32 = 0.8f32;
    pub const kTargetNegativeBadgesCount: i32 = 1i32;
    pub const kTargetPositiveBadgesCount: i32 = 2i32;
    #[cfg(feature = "MultiplayerBadgesProvider+__c")]
    pub type __c = crate::GlobalNamespace::MultiplayerBadgesProvider___c;
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
    pub fn SelectBadgesAndPutThemIntoResults(
        &mut self,
        playerResults: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut MultiplayerPlayerResultsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectBadgesAndPutThemIntoResults", (playerResults))?;
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
#[cfg(feature = "MultiplayerBadgesProvider")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerBadgesProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

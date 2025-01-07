#[cfg(feature = "MultiplayerResultsPyramidView")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerResultsPyramidView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _multiplayerOffsetByLocalPlayerPosition: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerOffsetPositionByLocalPlayerPosition,
    >,
    pub _spawnPoints: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        >,
    >,
    pub _spawnPointsParent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _evenCountOffset: f32,
    pub _avatarsFactory: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar_Factory,
    >,
    pub _avatarsDictionary: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar,
            >,
        >,
    >,
    pub _resultAvatarDirectors: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    >,
    pub _badgeTimelines: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    >,
    pub _anyResultsAvatar: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar,
    >,
}
#[cfg(feature = "MultiplayerResultsPyramidView")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerResultsPyramidView {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerResultsPyramidView";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "MultiplayerResultsPyramidView")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerResultsPyramidView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerResultsPyramidView")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerResultsPyramidView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerResultsPyramidView")]
impl crate::GlobalNamespace::MultiplayerResultsPyramidView {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PrespawnAvatars(
        &mut self,
        activePlayers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrespawnAvatars", (activePlayers))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupResults(
        &mut self,
        resultsData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerPlayerResultsData,
                >,
            >,
        >,
        badgeStartTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        badgeMidTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetupResults",
                (resultsData, badgeStartTransform, badgeMidTransform),
            )?;
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
    pub fn get_badgeTimelines(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        > = __cordl_object.invoke("get_badgeTimelines", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_resultAvatarDirectors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        > = __cordl_object.invoke("get_resultAvatarDirectors", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerResultsPyramidView")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerResultsPyramidView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

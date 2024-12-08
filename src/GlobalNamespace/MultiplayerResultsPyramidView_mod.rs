#[cfg(feature = "MultiplayerResultsPyramidView")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerResultsPyramidView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _multiplayerOffsetByLocalPlayerPosition: *mut MultiplayerOffsetPositionByLocalPlayerPosition,
    pub _spawnPoints: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Transform,
    >,
    pub _spawnPointsParent: *mut crate::UnityEngine::Transform,
    pub _evenCountOffset: f32,
    pub _avatarsFactory: *mut crate::GlobalNamespace::MultiplayerResultsPyramidViewAvatar_Factory,
    pub _avatarsDictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut MultiplayerResultsPyramidViewAvatar,
    >,
    pub _resultAvatarDirectors: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::GameObject,
    >,
    pub _badgeTimelines: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::GameObject,
    >,
    pub _anyResultsAvatar: *mut MultiplayerResultsPyramidViewAvatar,
}
#[cfg(feature = "MultiplayerResultsPyramidView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerResultsPyramidView => ""
    ."MultiplayerResultsPyramidView"
);
#[cfg(feature = "MultiplayerResultsPyramidView")]
impl std::ops::Deref for MultiplayerResultsPyramidView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerResultsPyramidView")]
impl std::ops::DerefMut for MultiplayerResultsPyramidView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerResultsPyramidView")]
impl MultiplayerResultsPyramidView {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PrespawnAvatars(
        &mut self,
        activePlayers: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrespawnAvatars", (activePlayers))?;
        Ok(__cordl_ret)
    }
    pub fn SetupResults(
        &mut self,
        resultsData: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut MultiplayerPlayerResultsData,
        >,
        badgeStartTransform: *mut crate::UnityEngine::Transform,
        badgeMidTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetupResults",
                (resultsData, badgeStartTransform, badgeMidTransform),
            )?;
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
    pub fn get_badgeTimelines(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::GameObject,
        > = __cordl_object.invoke("get_badgeTimelines", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_resultAvatarDirectors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::GameObject,
        > = __cordl_object.invoke("get_resultAvatarDirectors", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerResultsPyramidView")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerResultsPyramidView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

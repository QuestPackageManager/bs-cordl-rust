#[cfg(feature = "LeaderboardIdsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardIdsSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _leaderboardIds: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData,
    >,
}
#[cfg(feature = "LeaderboardIdsSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LeaderboardIdsSO => ""
    ."LeaderboardIdsSO"
);
#[cfg(feature = "LeaderboardIdsSO")]
impl std::ops::Deref for crate::GlobalNamespace::LeaderboardIdsSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardIdsSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::LeaderboardIdsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardIdsSO")]
impl crate::GlobalNamespace::LeaderboardIdsSO {
    #[cfg(feature = "LeaderboardIdsSO+LeaderboardIdData")]
    pub type LeaderboardIdData = crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData;
    pub fn Add(
        &mut self,
        difficultyBeatmapId: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        platformLeaderboardId: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (difficultyBeatmapId, platformLeaderboardId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ILeaderboardIdContainer_get_leaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::ILeaderboardIdData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::ILeaderboardIdData,
            >,
        > = __cordl_object.invoke("ILeaderboardIdContainer.get_leaderboardIds", ())?;
        Ok(__cordl_ret.into())
    }
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
    pub fn get_leaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData,
            >,
        > = __cordl_object.invoke("get_leaderboardIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_leaderboardIds(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leaderboardIds", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LeaderboardIdsSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LeaderboardIdsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LeaderboardIdsSO")]
impl AsRef<crate::GlobalNamespace::ILeaderboardIdContainer>
for crate::GlobalNamespace::LeaderboardIdsSO {
    fn as_ref(&self) -> &crate::GlobalNamespace::ILeaderboardIdContainer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LeaderboardIdsSO")]
impl AsMut<crate::GlobalNamespace::ILeaderboardIdContainer>
for crate::GlobalNamespace::LeaderboardIdsSO {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ILeaderboardIdContainer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LeaderboardIdsSO+LeaderboardIdData")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardIdsSO_LeaderboardIdData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _difficultyBeatmapId: *mut quest_hook::libil2cpp::Il2CppString,
    pub _platformLeaderboardId: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "LeaderboardIdsSO+LeaderboardIdData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData => ""
    ."LeaderboardIdsSO/LeaderboardIdData"
);
#[cfg(feature = "LeaderboardIdsSO+LeaderboardIdData")]
impl std::ops::Deref for crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardIdsSO+LeaderboardIdData")]
impl std::ops::DerefMut for crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardIdsSO+LeaderboardIdData")]
impl crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData {
    pub fn New(
        difficultyBeatmapId: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        platformLeaderboardId: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (difficultyBeatmapId, platformLeaderboardId))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        difficultyBeatmapId: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        platformLeaderboardId: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (difficultyBeatmapId, platformLeaderboardId))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_difficultyBeatmapId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_difficultyBeatmapId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_platformLeaderboardId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_platformLeaderboardId", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LeaderboardIdsSO+LeaderboardIdData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LeaderboardIdsSO+LeaderboardIdData")]
impl AsRef<crate::GlobalNamespace::ILeaderboardIdData>
for crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData {
    fn as_ref(&self) -> &crate::GlobalNamespace::ILeaderboardIdData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LeaderboardIdsSO+LeaderboardIdData")]
impl AsMut<crate::GlobalNamespace::ILeaderboardIdData>
for crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ILeaderboardIdData {
        unsafe { std::mem::transmute(self) }
    }
}

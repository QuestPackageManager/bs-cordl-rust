#[cfg(feature = "SonyLeaderboardIdsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLeaderboardIdsSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _leaderboardIds: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData,
            >,
        >,
    >,
}
#[cfg(feature = "SonyLeaderboardIdsSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SonyLeaderboardIdsSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SonyLeaderboardIdsSO";
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
#[cfg(feature = "SonyLeaderboardIdsSO")]
impl std::ops::Deref for crate::GlobalNamespace::SonyLeaderboardIdsSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLeaderboardIdsSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::SonyLeaderboardIdsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLeaderboardIdsSO")]
impl crate::GlobalNamespace::SonyLeaderboardIdsSO {
    #[cfg(feature = "SonyLeaderboardIdsSO+LeaderboardIdData")]
    pub type LeaderboardIdData = crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData;
    pub fn ILeaderboardIdContainer_get_leaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILeaderboardIdData>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILeaderboardIdData>,
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
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData,
                >,
            >,
        > = __cordl_object.invoke("get_leaderboardIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_leaderboardIds(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData,
                >,
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
#[cfg(feature = "SonyLeaderboardIdsSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SonyLeaderboardIdsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SonyLeaderboardIdsSO")]
impl AsRef<crate::GlobalNamespace::ILeaderboardIdContainer>
for crate::GlobalNamespace::SonyLeaderboardIdsSO {
    fn as_ref(&self) -> &crate::GlobalNamespace::ILeaderboardIdContainer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "SonyLeaderboardIdsSO")]
impl AsMut<crate::GlobalNamespace::ILeaderboardIdContainer>
for crate::GlobalNamespace::SonyLeaderboardIdsSO {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ILeaderboardIdContainer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "SonyLeaderboardIdsSO+LeaderboardIdData")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLeaderboardIdsSO_LeaderboardIdData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _difficultyBeatmapId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _sonyLeaderboardId: u32,
}
#[cfg(feature = "SonyLeaderboardIdsSO+LeaderboardIdData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LeaderboardIdData";
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
#[cfg(feature = "SonyLeaderboardIdsSO+LeaderboardIdData")]
impl std::ops::Deref for crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLeaderboardIdsSO+LeaderboardIdData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLeaderboardIdsSO+LeaderboardIdData")]
impl crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData {
    pub fn New(
        sonyLeaderboardId: u32,
        difficultyBeatmapId: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sonyLeaderboardId, difficultyBeatmapId))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        sonyLeaderboardId: u32,
        difficultyBeatmapId: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sonyLeaderboardId, difficultyBeatmapId))?;
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
    pub fn get_sonyLeaderboardId(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_sonyLeaderboardId", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SonyLeaderboardIdsSO+LeaderboardIdData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SonyLeaderboardIdsSO+LeaderboardIdData")]
impl AsRef<crate::GlobalNamespace::ILeaderboardIdData>
for crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData {
    fn as_ref(&self) -> &crate::GlobalNamespace::ILeaderboardIdData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "SonyLeaderboardIdsSO+LeaderboardIdData")]
impl AsMut<crate::GlobalNamespace::ILeaderboardIdData>
for crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ILeaderboardIdData {
        unsafe { std::mem::transmute(self) }
    }
}

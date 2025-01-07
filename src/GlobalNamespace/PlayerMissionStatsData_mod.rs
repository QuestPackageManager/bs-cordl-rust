#[cfg(feature = "PlayerMissionStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerMissionStatsData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _missionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _cleared: bool,
}
#[cfg(feature = "PlayerMissionStatsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerMissionStatsData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerMissionStatsData";
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
#[cfg(feature = "PlayerMissionStatsData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerMissionStatsData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerMissionStatsData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerMissionStatsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerMissionStatsData")]
impl crate::GlobalNamespace::PlayerMissionStatsData {
    pub fn New(
        missionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cleared: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (missionId, cleared))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        missionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cleared: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (missionId, cleared))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cleared(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_cleared", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_missionId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_missionId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cleared(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cleared", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlayerMissionStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerMissionStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

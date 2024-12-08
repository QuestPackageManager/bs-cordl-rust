#[cfg(feature = "MultiplayerActivePlayersTimeOffsetAverage")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerActivePlayersTimeOffsetAverage {
    __cordl_parent: crate::System::Object,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub _lastReturnedOffsetSyncTime: i64,
    pub _timeOfLastValidReturnedTime: i64,
}
#[cfg(feature = "MultiplayerActivePlayersTimeOffsetAverage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerActivePlayersTimeOffsetAverage => ""
    ."MultiplayerActivePlayersTimeOffsetAverage"
);
#[cfg(feature = "MultiplayerActivePlayersTimeOffsetAverage")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerActivePlayersTimeOffsetAverage {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerActivePlayersTimeOffsetAverage")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerActivePlayersTimeOffsetAverage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerActivePlayersTimeOffsetAverage")]
impl crate::GlobalNamespace::MultiplayerActivePlayersTimeOffsetAverage {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_isFailed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isFailed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_offsetSyncTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_offsetSyncTime", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerActivePlayersTimeOffsetAverage")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerActivePlayersTimeOffsetAverage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

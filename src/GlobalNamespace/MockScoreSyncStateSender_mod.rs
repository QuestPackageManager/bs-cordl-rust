#[cfg(feature = "MockScoreSyncStateSender")]
#[repr(C)]
#[derive(Debug)]
pub struct MockScoreSyncStateSender {
    __cordl_parent: crate::System::Object,
    pub _multiplayerSessionManager: *mut IMultiplayerSessionManager,
}
#[cfg(feature = "MockScoreSyncStateSender")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MockScoreSyncStateSender => ""
    ."MockScoreSyncStateSender"
);
#[cfg(feature = "MockScoreSyncStateSender")]
impl std::ops::Deref for MockScoreSyncStateSender {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockScoreSyncStateSender")]
impl std::ops::DerefMut for MockScoreSyncStateSender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockScoreSyncStateSender")]
impl MockScoreSyncStateSender {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleScoreSyncStateUpdate(
        &mut self,
        nodePose: *mut StandardScoreSyncStateNetSerializable,
        connectedPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleScoreSyncStateUpdate", (nodePose, connectedPlayer))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        msm: *mut IMultiplayerSessionManager,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (msm))?;
        Ok(__cordl_object)
    }
    pub fn SendScore(
        &mut self,
        modifiedScore: i32,
        multipliedScore: i32,
        immediateMaxPossibleMultipliedScore: i32,
        combo: i32,
        multiplier: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendScore",
                (
                    modifiedScore,
                    multipliedScore,
                    immediateMaxPossibleMultipliedScore,
                    combo,
                    multiplier,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        msm: *mut IMultiplayerSessionManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (msm))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MockScoreSyncStateSender")]
impl quest_hook::libil2cpp::ObjectType for MockScoreSyncStateSender {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

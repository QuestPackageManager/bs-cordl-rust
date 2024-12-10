#[cfg(feature = "MockScoreSyncStateSender")]
#[repr(C)]
#[derive(Debug)]
pub struct MockScoreSyncStateSender {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
}
#[cfg(feature = "MockScoreSyncStateSender")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockScoreSyncStateSender => ""
    ."MockScoreSyncStateSender"
);
#[cfg(feature = "MockScoreSyncStateSender")]
impl std::ops::Deref for crate::GlobalNamespace::MockScoreSyncStateSender {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockScoreSyncStateSender")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockScoreSyncStateSender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockScoreSyncStateSender")]
impl crate::GlobalNamespace::MockScoreSyncStateSender {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleScoreSyncStateUpdate(
        &mut self,
        nodePose: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
        >,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleScoreSyncStateUpdate", (nodePose, connectedPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        msm: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (msm))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        msm: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (msm))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockScoreSyncStateSender")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockScoreSyncStateSender {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MockScoreSyncStateSender")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::MockScoreSyncStateSender {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MockScoreSyncStateSender")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::MockScoreSyncStateSender {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}

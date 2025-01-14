#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlayerGamePoseGeneratorMirror {
    __cordl_parent: crate::GlobalNamespace::MockPlayerGamePoseGenerator,
    pub _nodePoseSyncStateManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NodePoseSyncStateManager,
    >,
    pub _mirroredPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectedPlayer,
    >,
    pub _onSongFinished: quest_hook::libil2cpp::Gc<crate::System::Action>,
}
#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MockPlayerGamePoseGeneratorMirror {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MockPlayerGamePoseGeneratorMirror";
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
#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
impl std::ops::Deref for crate::GlobalNamespace::MockPlayerGamePoseGeneratorMirror {
    type Target = crate::GlobalNamespace::MockPlayerGamePoseGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockPlayerGamePoseGeneratorMirror {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
impl crate::GlobalNamespace::MockPlayerGamePoseGeneratorMirror {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindPlayerToMirror(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("FindPlayerToMirror")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FindPlayerToMirror", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelFinished(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        results: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::MultiplayerLevelCompletionResults,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleLevelFinished")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLevelFinished", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (userId, results))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteWasCut(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        syncTime: i64,
        songTime: f32,
        noteCutInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteCutInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i64,
                    f32,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::NoteCutInfoNetSerializable,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("HandleNoteWasCut")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleNoteWasCut", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (userId, syncTime, songTime, noteCutInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteWasMissed(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        syncTime: i64,
        songTime: f32,
        noteMissInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteMissInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i64,
                    f32,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::NoteMissInfoNetSerializable,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("HandleNoteWasMissed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleNoteWasMissed", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (userId, syncTime, songTime, noteMissInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteWasSpawned(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        syncTime: i64,
        songTime: f32,
        noteSpawnInfoNetSerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i64,
                    f32,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("HandleNoteWasSpawned")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleNoteWasSpawned", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (userId, syncTime, songTime, noteSpawnInfoNetSerializable),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleObstacleWasSpawned(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        syncTime: i64,
        songTime: f32,
        obstacleSpawnInfoNetSerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i64,
                    f32,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("HandleObstacleWasSpawned")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleObstacleWasSpawned", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (userId, syncTime, songTime, obstacleSpawnInfoNetSerializable),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleOptionalAvatarDataReceived(
        &mut self,
        optionalAvatarDataPacket: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket,
        >,
        player: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleOptionalAvatarDataReceived")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleOptionalAvatarDataReceived", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (optionalAvatarDataPacket, player))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleScoreSyncStateUpdate(
        &mut self,
        nodePose: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
        >,
        player: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleScoreSyncStateUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleScoreSyncStateUpdate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (nodePose, player))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleSliderWasSpawned(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        syncTime: i64,
        songTime: f32,
        sliderSpawnInfoNetSerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i64,
                    f32,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("HandleSliderWasSpawned")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleSliderWasSpawned", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (userId, syncTime, songTime, sliderSpawnInfoNetSerializable),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        introStartTime: i64,
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockBeatmapData>,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        onSongFinished: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i64,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockBeatmapData>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Init", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (introStartTime, beatmapData, gameplayModifiers, onSongFinished),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
        gameplayRpcManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IGameplayRpcManager,
        >,
        leftHanded: bool,
        nodePoseSyncStateManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NodePoseSyncStateManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    multiplayerSessionManager,
                    gameplayRpcManager,
                    leftHanded,
                    nodePoseSyncStateManager,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Tick")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Tick", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
        gameplayRpcManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IGameplayRpcManager,
        >,
        leftHanded: bool,
        nodePoseSyncStateManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NodePoseSyncStateManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IMultiplayerSessionManager,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IGameplayRpcManager,
                    >,
                    bool,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::NodePoseSyncStateManager,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        multiplayerSessionManager,
                        gameplayRpcManager,
                        leftHanded,
                        nodePoseSyncStateManager,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockPlayerGamePoseGeneratorMirror {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

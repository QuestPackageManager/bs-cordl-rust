#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectEventManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerBeatmapObjectEventManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _connectedPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectedPlayer,
    >,
    pub _gameplayRpcManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IGameplayRpcManager,
    >,
    pub _songTimeController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerConnectedPlayerSongTimeSyncController,
    >,
    pub connectedPlayerNoteWasSpawnedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
            >,
        >,
    >,
    pub connectedPlayerObstacleWasSpawnedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
            >,
        >,
    >,
    pub connectedPlayerSliderWasSpawnedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
            >,
        >,
    >,
    pub connectedPlayerNoteWasCutEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteCutInfoNetSerializable>,
        >,
    >,
    pub connectedPlayerNoteWasMissedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::NoteMissInfoNetSerializable,
            >,
        >,
    >,
    pub _beatmapObjectEventQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Queue_1<
            crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager_TimestampedBeatmapObjectEventData,
        >,
    >,
    pub _paused: bool,
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectEventManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerConnectedPlayerBeatmapObjectEventManager";
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
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectEventManager")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectEventManager")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectEventManager")]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager {
    #[cfg(
        feature = "MultiplayerConnectedPlayerBeatmapObjectEventManager+TimestampedBeatmapObjectEventData"
    )]
    pub type TimestampedBeatmapObjectEventData = crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager_TimestampedBeatmapObjectEventData;
    pub fn HandleBeatmapObjectEventData<T>(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        syncTime: i64,
        songTime: f32,
        beatmapObjectEventData: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeatmapObjectEventData",
                (userId, syncTime, songTime, beatmapObjectEventData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeCallback(
        &mut self,
        noteEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPoolableSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeCallback", (noteEventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Pause(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Pause", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Resume(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Resume", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
    pub fn add_connectedPlayerNoteWasCutEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::NoteCutInfoNetSerializable,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectedPlayerNoteWasCutEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_connectedPlayerNoteWasMissedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::NoteMissInfoNetSerializable,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectedPlayerNoteWasMissedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_connectedPlayerNoteWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectedPlayerNoteWasSpawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_connectedPlayerObstacleWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectedPlayerObstacleWasSpawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_connectedPlayerSliderWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectedPlayerSliderWasSpawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_connectedPlayerNoteWasCutEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::NoteCutInfoNetSerializable,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectedPlayerNoteWasCutEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_connectedPlayerNoteWasMissedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::NoteMissInfoNetSerializable,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectedPlayerNoteWasMissedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_connectedPlayerNoteWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectedPlayerNoteWasSpawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_connectedPlayerObstacleWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectedPlayerObstacleWasSpawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_connectedPlayerSliderWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectedPlayerSliderWasSpawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectEventManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectEventManager")]
impl AsRef<crate::GlobalNamespace::IConnectedPlayerBeatmapObjectEventManager>
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IConnectedPlayerBeatmapObjectEventManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectEventManager")]
impl AsMut<crate::GlobalNamespace::IConnectedPlayerBeatmapObjectEventManager>
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IConnectedPlayerBeatmapObjectEventManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "MultiplayerConnectedPlayerBeatmapObjectEventManager+TimestampedBeatmapObjectEventData"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MultiplayerConnectedPlayerBeatmapObjectEventManager_TimestampedBeatmapObjectEventData {
    pub _cordl_time: f32,
    pub beatmapObjectEventData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IPoolableSerializable,
    >,
}
#[cfg(
    feature = "MultiplayerConnectedPlayerBeatmapObjectEventManager+TimestampedBeatmapObjectEventData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager_TimestampedBeatmapObjectEventData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerConnectedPlayerBeatmapObjectEventManager/TimestampedBeatmapObjectEventData";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "MultiplayerConnectedPlayerBeatmapObjectEventManager+TimestampedBeatmapObjectEventData"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager_TimestampedBeatmapObjectEventData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "MultiplayerConnectedPlayerBeatmapObjectEventManager+TimestampedBeatmapObjectEventData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager_TimestampedBeatmapObjectEventData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(
    feature = "MultiplayerConnectedPlayerBeatmapObjectEventManager+TimestampedBeatmapObjectEventData"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager_TimestampedBeatmapObjectEventData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "MultiplayerConnectedPlayerBeatmapObjectEventManager+TimestampedBeatmapObjectEventData"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager_TimestampedBeatmapObjectEventData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "MultiplayerConnectedPlayerBeatmapObjectEventManager+TimestampedBeatmapObjectEventData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager_TimestampedBeatmapObjectEventData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "MultiplayerConnectedPlayerBeatmapObjectEventManager+TimestampedBeatmapObjectEventData"
)]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager_TimestampedBeatmapObjectEventData {
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        beatmapObjectEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPoolableSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_time, beatmapObjectEventData),
        )?;
        Ok(__cordl_ret.into())
    }
}

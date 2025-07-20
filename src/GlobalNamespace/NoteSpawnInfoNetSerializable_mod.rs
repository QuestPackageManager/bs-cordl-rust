#[cfg(feature = "NoteSpawnInfoNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteSpawnInfoNetSerializable {
    __cordl_parent: crate::GlobalNamespace::PoolableSerializable,
    pub _cordl_time: f32,
    pub beat: f32,
    pub lineIndex: i32,
    pub noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
    pub beforeJumpNoteLineLayer: crate::GlobalNamespace::NoteLineLayer,
    pub gameplayType: crate::GlobalNamespace::NoteData_GameplayType,
    pub scoringType: crate::GlobalNamespace::NoteData_ScoringType,
    pub colorType: crate::GlobalNamespace::ColorType,
    pub cutDirection: crate::GlobalNamespace::NoteCutDirection,
    pub timeToNextColorNote: f32,
    pub timeToPrevColorNote: f32,
    pub flipLineIndex: i32,
    pub flipYSide: f32,
    pub moveStartOffset: crate::GlobalNamespace::Vector3Serializable,
    pub moveEndOffset: crate::GlobalNamespace::Vector3Serializable,
    pub jumpEndOffset: crate::GlobalNamespace::Vector3Serializable,
    pub gravityBase: f32,
    pub rotation: f32,
    pub cutDirectionAngleOffset: f32,
    pub cutSfxVolumeMultiplier: f32,
}
#[cfg(feature = "NoteSpawnInfoNetSerializable")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoteSpawnInfoNetSerializable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteSpawnInfoNetSerializable";
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
#[cfg(feature = "NoteSpawnInfoNetSerializable")]
impl std::ops::Deref for crate::GlobalNamespace::NoteSpawnInfoNetSerializable {
    type Target = crate::GlobalNamespace::PoolableSerializable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteSpawnInfoNetSerializable")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteSpawnInfoNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteSpawnInfoNetSerializable")]
impl crate::GlobalNamespace::NoteSpawnInfoNetSerializable {
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::NoteSpawnInfoNetSerializable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Deserialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::NoteSpawnInfoNetSerializable as
                    quest_hook::libil2cpp::Type > ::class(), "Deserialize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (reader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        _cordl_time: f32,
        beat: f32,
        lineIndex: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
        beforeJumpNoteLineLayer: crate::GlobalNamespace::NoteLineLayer,
        gameplayType: crate::GlobalNamespace::NoteData_GameplayType,
        scoringType: crate::GlobalNamespace::NoteData_ScoringType,
        colorType: crate::GlobalNamespace::ColorType,
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
        timeToNextColorNote: f32,
        timeToPrevColorNote: f32,
        flipLineIndex: i32,
        flipYSide: f32,
        moveStartOffset: crate::UnityEngine::Vector3,
        moveEndOffset: crate::UnityEngine::Vector3,
        jumpEndOffset: crate::UnityEngine::Vector3,
        gravityBase: f32,
        rotation: f32,
        cutDirectionAngleOffset: f32,
        cutSfxVolumeMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteSpawnInfoNetSerializable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::NoteSpawnInfoNetSerializable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    f32,
                    i32,
                    crate::GlobalNamespace::NoteLineLayer,
                    crate::GlobalNamespace::NoteLineLayer,
                    crate::GlobalNamespace::NoteData_GameplayType,
                    crate::GlobalNamespace::NoteData_ScoringType,
                    crate::GlobalNamespace::ColorType,
                    crate::GlobalNamespace::NoteCutDirection,
                    f32,
                    f32,
                    i32,
                    f32,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    f32,
                    f32,
                    f32,
                    f32,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
                >,
                20usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::NoteSpawnInfoNetSerializable as
                    quest_hook::libil2cpp::Type > ::class(), "Init", 20usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        _cordl_time,
                        beat,
                        lineIndex,
                        noteLineLayer,
                        beforeJumpNoteLineLayer,
                        gameplayType,
                        scoringType,
                        colorType,
                        cutDirection,
                        timeToNextColorNote,
                        timeToPrevColorNote,
                        flipLineIndex,
                        flipYSide,
                        moveStartOffset,
                        moveEndOffset,
                        jumpEndOffset,
                        gravityBase,
                        rotation,
                        cutDirectionAngleOffset,
                        cutSfxVolumeMultiplier,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Obtain() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteSpawnInfoNetSerializable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::NoteSpawnInfoNetSerializable as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
                >,
                0usize,
            >("Obtain")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::NoteSpawnInfoNetSerializable as
                    quest_hook::libil2cpp::Type > ::class(), "Obtain", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::NoteSpawnInfoNetSerializable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Serialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::NoteSpawnInfoNetSerializable as
                    quest_hook::libil2cpp::Type > ::class(), "Serialize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::NoteSpawnInfoNetSerializable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::NoteSpawnInfoNetSerializable as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteSpawnInfoNetSerializable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteSpawnInfoNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "NoteDebrisSpawner")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteDebrisSpawner {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _rotation: f32,
    pub _cutDirMultiplier: f32,
    pub _fromCenterSpeed: f32,
    pub _moveSpeedMultiplier: f32,
    pub _normalNotesDebrisPool: *mut crate::GlobalNamespace::NoteDebris_Pool,
    pub _burstSliderHeadNotesDebrisPool: *mut crate::GlobalNamespace::NoteDebris_Pool,
    pub _burstSliderElementNotesDebrisPool: *mut crate::GlobalNamespace::NoteDebris_Pool,
    pub _poolForNoteGameplayType: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::GlobalNamespace::NoteData_GameplayType,
        *mut crate::GlobalNamespace::NoteDebris_Pool,
    >,
    pub _poolForNoteDebris: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::GlobalNamespace::NoteDebris,
        *mut crate::GlobalNamespace::NoteDebris_Pool,
    >,
}
#[cfg(feature = "NoteDebrisSpawner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteDebrisSpawner => ""
    ."NoteDebrisSpawner"
);
#[cfg(feature = "NoteDebrisSpawner")]
impl std::ops::Deref for crate::GlobalNamespace::NoteDebrisSpawner {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDebrisSpawner")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteDebrisSpawner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDebrisSpawner")]
impl crate::GlobalNamespace::NoteDebrisSpawner {
    pub const kLifeTimeOffset: f32 = 0.05f32;
    pub const kMaxLifeTime: f32 = 2f32;
    pub const kMinLifeTime: f32 = 0.2f32;
    pub fn DespawnNoteDebris(
        &mut self,
        noteDebris: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteDebris>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DespawnNoteDebris", (noteDebris))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteDebrisDidFinish(
        &mut self,
        noteDebris: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteDebris>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteDebrisDidFinish", (noteDebris))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SpawnDebris(
        &mut self,
        noteGameplayType: crate::GlobalNamespace::NoteData_GameplayType,
        cutPoint: crate::UnityEngine::Vector3,
        cutNormal: crate::UnityEngine::Vector3,
        saberSpeed: f32,
        saberDir: crate::UnityEngine::Vector3,
        notePos: crate::UnityEngine::Vector3,
        noteRotation: crate::UnityEngine::Quaternion,
        noteScale: crate::UnityEngine::Vector3,
        colorType: crate::GlobalNamespace::ColorType,
        timeToNextColorNote: f32,
        moveVec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SpawnDebris",
                (
                    noteGameplayType,
                    cutPoint,
                    cutNormal,
                    saberSpeed,
                    saberDir,
                    notePos,
                    noteRotation,
                    noteScale,
                    colorType,
                    timeToNextColorNote,
                    moveVec,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnNoteDebris(
        &mut self,
        noteGameplayType: crate::GlobalNamespace::NoteData_GameplayType,
        debris0: quest_hook::libil2cpp::ByRefMut<
            *mut crate::GlobalNamespace::NoteDebris,
        >,
        debris1: quest_hook::libil2cpp::ByRefMut<*mut crate::GlobalNamespace::NoteDebris>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SpawnNoteDebris", (noteGameplayType, debris0, debris1))?;
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
}
#[cfg(feature = "NoteDebrisSpawner")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NoteDebrisSpawner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

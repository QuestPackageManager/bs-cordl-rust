#[cfg(feature = "TutorialNoteController")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialNoteController {
    __cordl_parent: crate::GlobalNamespace::NoteController,
    pub _cuttableBySaberCore: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BoxCuttableBySaber,
    >,
    pub _cuttableBySaberBeforeNote: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BoxCuttableBySaber,
    >,
    pub _wrapperGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _beforeNoteCutWasOk: bool,
    pub _cutAngleTolerance: f32,
}
#[cfg(feature = "TutorialNoteController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TutorialNoteController => ""
    ."TutorialNoteController"
);
#[cfg(feature = "TutorialNoteController")]
impl std::ops::Deref for crate::GlobalNamespace::TutorialNoteController {
    type Target = crate::GlobalNamespace::NoteController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialNoteController")]
impl std::ops::DerefMut for crate::GlobalNamespace::TutorialNoteController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialNoteController")]
impl crate::GlobalNamespace::TutorialNoteController {
    #[cfg(feature = "TutorialNoteController+Pool")]
    pub type Pool = crate::GlobalNamespace::TutorialNoteController_Pool;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleBeforeNoteWasCutBySaber(
        &mut self,
        saber: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>,
        cutPoint: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        cutDirVec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeforeNoteWasCutBySaber",
                (saber, cutPoint, orientation, cutDirVec),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleCoreWasCutBySaber(
        &mut self,
        saber: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>,
        cutPoint: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        cutDirVec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleCoreWasCutBySaber",
                (saber, cutPoint, orientation, cutDirVec),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HiddenStateDidChange(
        &mut self,
        hide: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HiddenStateDidChange", (hide))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
        noteSpawnData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NoteSpawnData,
        >,
        cutAngleTolerance: f32,
        uniformScale: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (noteData, noteSpawnData, cutAngleTolerance, uniformScale))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NoteDidPassMissedMarker(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NoteDidPassMissedMarker", ())?;
        Ok(__cordl_ret.into())
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
        pause: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Pause", (pause))?;
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
    pub fn get_gameplayType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteData_GameplayType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NoteData_GameplayType = __cordl_object
            .invoke("get_gameplayType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteMovement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteMovement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteMovement,
        > = __cordl_object.invoke("get_noteMovement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteVisualModifierType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteVisualModifierType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NoteVisualModifierType = __cordl_object
            .invoke("get_noteVisualModifierType", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TutorialNoteController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TutorialNoteController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TutorialNoteController")]
impl AsRef<crate::GlobalNamespace::IGameNoteMirrorable>
for crate::GlobalNamespace::TutorialNoteController {
    fn as_ref(&self) -> &crate::GlobalNamespace::IGameNoteMirrorable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TutorialNoteController")]
impl AsMut<crate::GlobalNamespace::IGameNoteMirrorable>
for crate::GlobalNamespace::TutorialNoteController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IGameNoteMirrorable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TutorialNoteController")]
impl AsRef<crate::GlobalNamespace::INoteMirrorable>
for crate::GlobalNamespace::TutorialNoteController {
    fn as_ref(&self) -> &crate::GlobalNamespace::INoteMirrorable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TutorialNoteController")]
impl AsMut<crate::GlobalNamespace::INoteMirrorable>
for crate::GlobalNamespace::TutorialNoteController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::INoteMirrorable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TutorialNoteController+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialNoteController_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::TutorialNoteController>,
    >,
}
#[cfg(feature = "TutorialNoteController+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TutorialNoteController_Pool =>
    ""."TutorialNoteController/Pool"
);
#[cfg(feature = "TutorialNoteController+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::TutorialNoteController_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::TutorialNoteController>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialNoteController+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::TutorialNoteController_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialNoteController+Pool")]
impl crate::GlobalNamespace::TutorialNoteController_Pool {
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
}
#[cfg(feature = "TutorialNoteController+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TutorialNoteController_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

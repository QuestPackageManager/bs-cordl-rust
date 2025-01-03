#[cfg(feature = "BombNoteController")]
#[repr(C)]
#[derive(Debug)]
pub struct BombNoteController {
    __cordl_parent: crate::GlobalNamespace::NoteController,
    pub _cuttableBySaber: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CuttableBySaber,
    >,
    pub _wrapperGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
}
#[cfg(feature = "BombNoteController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BombNoteController => ""
    ."BombNoteController"
);
#[cfg(feature = "BombNoteController")]
impl std::ops::Deref for crate::GlobalNamespace::BombNoteController {
    type Target = crate::GlobalNamespace::NoteController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BombNoteController")]
impl std::ops::DerefMut for crate::GlobalNamespace::BombNoteController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BombNoteController")]
impl crate::GlobalNamespace::BombNoteController {
    #[cfg(feature = "BombNoteController+Pool")]
    pub type Pool = crate::GlobalNamespace::BombNoteController_Pool;
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
    pub fn HandleDidPassHalfJump(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidPassHalfJump", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleWasCutBySaber(
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
            .invoke("HandleWasCutBySaber", (saber, cutPoint, orientation, cutDirVec))?;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (noteData, noteSpawnData))?;
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
    pub fn NoteDidStartDissolving(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NoteDidStartDissolving", ())?;
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
}
#[cfg(feature = "BombNoteController")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BombNoteController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BombNoteController+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct BombNoteController_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::BombNoteController,
    >,
}
#[cfg(feature = "BombNoteController+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BombNoteController_Pool => ""
    ."BombNoteController/Pool"
);
#[cfg(feature = "BombNoteController+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::BombNoteController_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::BombNoteController,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BombNoteController+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::BombNoteController_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BombNoteController+Pool")]
impl crate::GlobalNamespace::BombNoteController_Pool {
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
#[cfg(feature = "BombNoteController+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BombNoteController_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

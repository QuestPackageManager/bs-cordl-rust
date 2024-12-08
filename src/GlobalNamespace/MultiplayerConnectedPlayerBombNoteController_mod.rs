#[cfg(feature = "MultiplayerConnectedPlayerBombNoteController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerBombNoteController {
    __cordl_parent: crate::GlobalNamespace::MultiplayerConnectedPlayerNoteController,
}
#[cfg(feature = "MultiplayerConnectedPlayerBombNoteController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController => ""
    ."MultiplayerConnectedPlayerBombNoteController"
);
#[cfg(feature = "MultiplayerConnectedPlayerBombNoteController")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController {
    type Target = crate::GlobalNamespace::MultiplayerConnectedPlayerNoteController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBombNoteController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBombNoteController")]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController {
    #[cfg(feature = "MultiplayerConnectedPlayerBombNoteController+Pool")]
    pub type Pool = crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController_Pool;
    pub fn Init(
        &mut self,
        noteData: *mut crate::GlobalNamespace::NoteData,
        worldRotation: f32,
        moveStartPos: crate::UnityEngine::Vector3,
        moveEndPos: crate::UnityEngine::Vector3,
        jumpEndPos: crate::UnityEngine::Vector3,
        moveDuration: f32,
        jumpDuration: f32,
        jumpGravity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    noteData,
                    worldRotation,
                    moveStartPos,
                    moveEndPos,
                    jumpEndPos,
                    moveDuration,
                    jumpDuration,
                    jumpGravity,
                ),
            )?;
        Ok(__cordl_ret)
    }
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
}
#[cfg(feature = "MultiplayerConnectedPlayerBombNoteController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBombNoteController+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerBombNoteController_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController,
    >,
}
#[cfg(feature = "MultiplayerConnectedPlayerBombNoteController+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController_Pool => ""
    ."MultiplayerConnectedPlayerBombNoteController/Pool"
);
#[cfg(feature = "MultiplayerConnectedPlayerBombNoteController+Pool")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBombNoteController+Pool")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBombNoteController+Pool")]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController_Pool {
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
}
#[cfg(feature = "MultiplayerConnectedPlayerBombNoteController+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

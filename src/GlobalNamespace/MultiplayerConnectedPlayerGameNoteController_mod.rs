#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerGameNoteController {
    __cordl_parent: crate::GlobalNamespace::MultiplayerConnectedPlayerNoteController,
    pub cubeNoteControllerDidInitEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
            >,
        >,
    >,
    pub _noteVisualModifierType: crate::GlobalNamespace::NoteVisualModifierType,
    pub _gameplayType: crate::GlobalNamespace::NoteData_GameplayType,
}
#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController => ""
    ."MultiplayerConnectedPlayerGameNoteController"
);
#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController {
    type Target = crate::GlobalNamespace::MultiplayerConnectedPlayerNoteController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController")]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController {
    #[cfg(feature = "MultiplayerConnectedPlayerGameNoteController+Pool")]
    pub type Pool = crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool;
    pub fn Init(
        &mut self,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
        noteSpawnData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NoteSpawnData,
        >,
        noteVisualModifierType: crate::GlobalNamespace::NoteVisualModifierType,
        uniformScale: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (noteData, noteSpawnData, noteVisualModifierType, uniformScale),
            )?;
        Ok(__cordl_ret.into())
    }
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
    pub fn add_cubeNoteControllerDidInitEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_cubeNoteControllerDidInitEvent", (value))?;
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
    pub fn remove_cubeNoteControllerDidInitEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_cubeNoteControllerDidInitEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController")]
impl AsRef<
    crate::GlobalNamespace::ICubeNoteControllerInitializable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
        >,
    >,
> for crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::ICubeNoteControllerInitializable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController")]
impl AsMut<
    crate::GlobalNamespace::ICubeNoteControllerInitializable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
        >,
    >,
> for crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ICubeNoteControllerInitializable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController")]
impl AsRef<crate::GlobalNamespace::INoteMovementProvider>
for crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController {
    fn as_ref(&self) -> &crate::GlobalNamespace::INoteMovementProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController")]
impl AsMut<crate::GlobalNamespace::INoteMovementProvider>
for crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::INoteMovementProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController")]
impl AsRef<crate::GlobalNamespace::INoteVisualModifierTypeProvider>
for crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController {
    fn as_ref(&self) -> &crate::GlobalNamespace::INoteVisualModifierTypeProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController")]
impl AsMut<crate::GlobalNamespace::INoteVisualModifierTypeProvider>
for crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::INoteVisualModifierTypeProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerGameNoteController_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
        >,
    >,
}
#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool => ""
    ."MultiplayerConnectedPlayerGameNoteController/Pool"
);
#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController+Pool")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
        >,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController+Pool")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController+Pool")]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool {
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
#[cfg(feature = "MultiplayerConnectedPlayerGameNoteController+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

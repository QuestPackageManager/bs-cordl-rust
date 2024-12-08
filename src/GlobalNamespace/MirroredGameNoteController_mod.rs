#[cfg(feature = "MirroredGameNoteController")]
#[repr(C)]
#[derive(Debug)]
pub struct MirroredGameNoteController {
    __cordl_parent: MirroredNoteController_1<*mut IGameNoteMirrorable>,
    pub _materialPropertyBlockController: *mut MaterialPropertyBlockController,
    pub _colorManager: *mut ColorManager,
    pub cubeNoteControllerDidInitEvent: *mut crate::System::Action_1<
        *mut MirroredGameNoteController,
    >,
}
#[cfg(feature = "MirroredGameNoteController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MirroredGameNoteController => ""
    ."MirroredGameNoteController"
);
#[cfg(feature = "MirroredGameNoteController")]
impl std::ops::Deref for MirroredGameNoteController {
    type Target = MirroredNoteController_1<*mut IGameNoteMirrorable>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MirroredGameNoteController")]
impl std::ops::DerefMut for MirroredGameNoteController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MirroredGameNoteController")]
impl MirroredGameNoteController {
    #[cfg(feature = "MirroredGameNoteController+Pool")]
    pub type Pool = crate::GlobalNamespace::MirroredGameNoteController_Pool;
    pub fn Mirror(
        &mut self,
        noteController: *mut IGameNoteMirrorable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Mirror", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn get_noteVisualModifierType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<NoteVisualModifierType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: NoteVisualModifierType = __cordl_object
            .invoke("get_noteVisualModifierType", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_cubeNoteControllerDidInitEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MirroredGameNoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_cubeNoteControllerDidInitEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_noteMovement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut NoteMovement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut NoteMovement = __cordl_object
            .invoke("get_noteMovement", ())?;
        Ok(__cordl_ret)
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
    pub fn remove_cubeNoteControllerDidInitEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MirroredGameNoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_cubeNoteControllerDidInitEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MirroredGameNoteController")]
impl quest_hook::libil2cpp::ObjectType for MirroredGameNoteController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MirroredGameNoteController+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct MirroredGameNoteController_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<*mut MirroredGameNoteController>,
}
#[cfg(feature = "MirroredGameNoteController+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MirroredGameNoteController_Pool
    => ""."MirroredGameNoteController/Pool"
);
#[cfg(feature = "MirroredGameNoteController+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::MirroredGameNoteController_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<*mut MirroredGameNoteController>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MirroredGameNoteController+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::MirroredGameNoteController_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MirroredGameNoteController+Pool")]
impl crate::GlobalNamespace::MirroredGameNoteController_Pool {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MirroredGameNoteController+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MirroredGameNoteController_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

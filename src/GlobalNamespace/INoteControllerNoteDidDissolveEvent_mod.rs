#[cfg(feature = "INoteControllerNoteDidDissolveEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct INoteControllerNoteDidDissolveEvent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "INoteControllerNoteDidDissolveEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for INoteControllerNoteDidDissolveEvent => ""
    ."INoteControllerNoteDidDissolveEvent"
);
#[cfg(feature = "INoteControllerNoteDidDissolveEvent")]
impl std::ops::Deref for INoteControllerNoteDidDissolveEvent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "INoteControllerNoteDidDissolveEvent")]
impl std::ops::DerefMut for INoteControllerNoteDidDissolveEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "INoteControllerNoteDidDissolveEvent")]
impl INoteControllerNoteDidDissolveEvent {
    pub fn HandleNoteControllerNoteDidDissolve(
        &mut self,
        noteController: *mut NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteControllerNoteDidDissolve", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "INoteControllerNoteDidDissolveEvent")]
impl quest_hook::libil2cpp::ObjectType for INoteControllerNoteDidDissolveEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
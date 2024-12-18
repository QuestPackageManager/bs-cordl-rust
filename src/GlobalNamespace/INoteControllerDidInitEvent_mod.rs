#[cfg(feature = "INoteControllerDidInitEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct INoteControllerDidInitEvent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "INoteControllerDidInitEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::INoteControllerDidInitEvent =>
    ""."INoteControllerDidInitEvent"
);
#[cfg(feature = "INoteControllerDidInitEvent")]
impl std::ops::Deref for crate::GlobalNamespace::INoteControllerDidInitEvent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "INoteControllerDidInitEvent")]
impl std::ops::DerefMut for crate::GlobalNamespace::INoteControllerDidInitEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "INoteControllerDidInitEvent")]
impl crate::GlobalNamespace::INoteControllerDidInitEvent {
    pub fn HandleNoteControllerDidInit(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteControllerBase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteControllerDidInit", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "INoteControllerDidInitEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::INoteControllerDidInitEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

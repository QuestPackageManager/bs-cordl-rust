#[cfg(feature = "INoteDebrisDidFinishEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct INoteDebrisDidFinishEvent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "INoteDebrisDidFinishEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::INoteDebrisDidFinishEvent => ""
    ."INoteDebrisDidFinishEvent"
);
#[cfg(feature = "INoteDebrisDidFinishEvent")]
impl std::ops::Deref for crate::GlobalNamespace::INoteDebrisDidFinishEvent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "INoteDebrisDidFinishEvent")]
impl std::ops::DerefMut for crate::GlobalNamespace::INoteDebrisDidFinishEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "INoteDebrisDidFinishEvent")]
impl crate::GlobalNamespace::INoteDebrisDidFinishEvent {
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "INoteDebrisDidFinishEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::INoteDebrisDidFinishEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

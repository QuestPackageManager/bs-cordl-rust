#[cfg(feature = "IMockPlayerScoreCalculator")]
#[repr(C)]
#[derive(Debug)]
pub struct IMockPlayerScoreCalculator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IMockPlayerScoreCalculator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IMockPlayerScoreCalculator => ""
    ."IMockPlayerScoreCalculator"
);
#[cfg(feature = "IMockPlayerScoreCalculator")]
impl std::ops::Deref for IMockPlayerScoreCalculator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IMockPlayerScoreCalculator")]
impl std::ops::DerefMut for IMockPlayerScoreCalculator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IMockPlayerScoreCalculator")]
impl IMockPlayerScoreCalculator {
    pub fn GetScoreForNote(
        &mut self,
        noteData: *mut MockNoteData,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetScoreForNote", (noteData))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IMockPlayerScoreCalculator")]
impl quest_hook::libil2cpp::ObjectType for IMockPlayerScoreCalculator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

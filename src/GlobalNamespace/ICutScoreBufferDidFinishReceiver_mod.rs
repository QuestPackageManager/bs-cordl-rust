#[cfg(feature = "ICutScoreBufferDidFinishReceiver")]
#[repr(C)]
#[derive(Debug)]
pub struct ICutScoreBufferDidFinishReceiver {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ICutScoreBufferDidFinishReceiver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ICutScoreBufferDidFinishReceiver => ""
    ."ICutScoreBufferDidFinishReceiver"
);
#[cfg(feature = "ICutScoreBufferDidFinishReceiver")]
impl std::ops::Deref for crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ICutScoreBufferDidFinishReceiver")]
impl std::ops::DerefMut for crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ICutScoreBufferDidFinishReceiver")]
impl crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver {
    pub fn HandleCutScoreBufferDidFinish(
        &mut self,
        cutScoreBuffer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::CutScoreBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCutScoreBufferDidFinish", (cutScoreBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ICutScoreBufferDidFinishReceiver")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "ICutScoreBufferDidChangeReceiver")]
#[repr(C)]
#[derive(Debug)]
pub struct ICutScoreBufferDidChangeReceiver {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ICutScoreBufferDidChangeReceiver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ICutScoreBufferDidChangeReceiver => ""
    ."ICutScoreBufferDidChangeReceiver"
);
#[cfg(feature = "ICutScoreBufferDidChangeReceiver")]
impl std::ops::Deref for crate::GlobalNamespace::ICutScoreBufferDidChangeReceiver {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ICutScoreBufferDidChangeReceiver")]
impl std::ops::DerefMut for crate::GlobalNamespace::ICutScoreBufferDidChangeReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ICutScoreBufferDidChangeReceiver")]
impl crate::GlobalNamespace::ICutScoreBufferDidChangeReceiver {
    pub fn HandleCutScoreBufferDidChange(
        &mut self,
        cutScoreBuffer: *mut crate::GlobalNamespace::CutScoreBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCutScoreBufferDidChange", (cutScoreBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ICutScoreBufferDidChangeReceiver")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ICutScoreBufferDidChangeReceiver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

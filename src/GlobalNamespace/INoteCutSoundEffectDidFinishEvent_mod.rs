#[cfg(feature = "INoteCutSoundEffectDidFinishEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct INoteCutSoundEffectDidFinishEvent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "INoteCutSoundEffectDidFinishEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::INoteCutSoundEffectDidFinishEvent => ""
    ."INoteCutSoundEffectDidFinishEvent"
);
#[cfg(feature = "INoteCutSoundEffectDidFinishEvent")]
impl std::ops::Deref for crate::GlobalNamespace::INoteCutSoundEffectDidFinishEvent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "INoteCutSoundEffectDidFinishEvent")]
impl std::ops::DerefMut for crate::GlobalNamespace::INoteCutSoundEffectDidFinishEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "INoteCutSoundEffectDidFinishEvent")]
impl crate::GlobalNamespace::INoteCutSoundEffectDidFinishEvent {
    pub fn HandleNoteCutSoundEffectDidFinish(
        &mut self,
        noteCutSoundEffect: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteCutSoundEffect,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteCutSoundEffectDidFinish", (noteCutSoundEffect))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "INoteCutSoundEffectDidFinishEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::INoteCutSoundEffectDidFinishEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

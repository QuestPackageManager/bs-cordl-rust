#[cfg(feature = "ISaberSwingRatingCounterDidFinishReceiver")]
#[repr(C)]
#[derive(Debug)]
pub struct ISaberSwingRatingCounterDidFinishReceiver {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ISaberSwingRatingCounterDidFinishReceiver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver => ""
    ."ISaberSwingRatingCounterDidFinishReceiver"
);
#[cfg(feature = "ISaberSwingRatingCounterDidFinishReceiver")]
impl std::ops::Deref
for crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ISaberSwingRatingCounterDidFinishReceiver")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ISaberSwingRatingCounterDidFinishReceiver")]
impl crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver {
    pub fn HandleSaberSwingRatingCounterDidFinish(
        &mut self,
        saberSwingRatingCounter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberSwingRatingCounter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSaberSwingRatingCounterDidFinish",
                (saberSwingRatingCounter),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ISaberSwingRatingCounterDidFinishReceiver")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

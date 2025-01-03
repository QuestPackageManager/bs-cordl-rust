#[cfg(feature = "ISaberSwingRatingCounter")]
#[repr(C)]
#[derive(Debug)]
pub struct ISaberSwingRatingCounter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ISaberSwingRatingCounter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ISaberSwingRatingCounter => ""
    ."ISaberSwingRatingCounter"
);
#[cfg(feature = "ISaberSwingRatingCounter")]
impl std::ops::Deref for crate::GlobalNamespace::ISaberSwingRatingCounter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ISaberSwingRatingCounter")]
impl std::ops::DerefMut for crate::GlobalNamespace::ISaberSwingRatingCounter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ISaberSwingRatingCounter")]
impl crate::GlobalNamespace::ISaberSwingRatingCounter {
    pub fn RegisterDidChangeReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberSwingRatingCounterDidChangeReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterDidChangeReceiver", (receiver))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDidFinishReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterDidFinishReceiver", (receiver))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterDidChangeReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberSwingRatingCounterDidChangeReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterDidChangeReceiver", (receiver))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterDidFinishReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterDidFinishReceiver", (receiver))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_afterCutRating(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_afterCutRating", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beforeCutRating(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beforeCutRating", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ISaberSwingRatingCounter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ISaberSwingRatingCounter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

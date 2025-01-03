#[cfg(feature = "AudioHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "AudioHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AudioHelpers => ""
    ."AudioHelpers"
);
#[cfg(feature = "AudioHelpers")]
impl std::ops::Deref for crate::GlobalNamespace::AudioHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioHelpers")]
impl std::ops::DerefMut for crate::GlobalNamespace::AudioHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioHelpers")]
impl crate::GlobalNamespace::AudioHelpers {
    pub fn DBToNormalizedVolume(db: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DBToNormalizedVolume", (db))?;
        Ok(__cordl_ret.into())
    }
    pub fn NormalizedVolumeToDB(
        normalizedVolume: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NormalizedVolumeToDB", (normalizedVolume))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AudioHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AudioHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

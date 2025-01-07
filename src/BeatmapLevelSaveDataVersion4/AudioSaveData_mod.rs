#[cfg(feature = "BeatmapLevelSaveDataVersion4+AudioSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub songChecksum: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub songSampleCount: i32,
    pub songFrequency: i32,
    pub bpmData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatmapLevelSaveDataVersion4::BpmData>,
        >,
    >,
    pub lufsData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapLevelSaveDataVersion4::LufsData>,
    >,
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+AudioSaveData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapLevelSaveDataVersion4::AudioSaveData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapLevelSaveDataVersion4";
    const CLASS_NAME: &'static str = "AudioSaveData";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+AudioSaveData")]
impl std::ops::Deref for crate::BeatmapLevelSaveDataVersion4::AudioSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+AudioSaveData")]
impl std::ops::DerefMut for crate::BeatmapLevelSaveDataVersion4::AudioSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+AudioSaveData")]
impl crate::BeatmapLevelSaveDataVersion4::AudioSaveData {
    pub const kCurrentVersion: &'static str = "4.0.0";
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+AudioSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapLevelSaveDataVersion4::AudioSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

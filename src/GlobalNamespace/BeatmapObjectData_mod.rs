#[cfg(feature = "BeatmapObjectData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectData {
    __cordl_parent: crate::GlobalNamespace::BeatmapDataItem,
    pub _beat_k__BackingField: f32,
    pub _rotation_k__BackingField: i32,
}
#[cfg(feature = "BeatmapObjectData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapObjectData => ""
    ."BeatmapObjectData"
);
#[cfg(feature = "BeatmapObjectData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapObjectData {
    type Target = crate::GlobalNamespace::BeatmapDataItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectData")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapObjectData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectData")]
impl crate::GlobalNamespace::BeatmapObjectData {
    pub fn Mirror(
        &mut self,
        lineCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Mirror", (lineCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_time: f32,
        beat: f32,
        rotation: i32,
        subtypeIdentifier: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_time, beat, rotation, subtypeIdentifier))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        beat: f32,
        rotation: i32,
        subtypeIdentifier: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_time, beat, rotation, subtypeIdentifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beat(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beat", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotation(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_rotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rotation(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rotation", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapObjectData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapObjectData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

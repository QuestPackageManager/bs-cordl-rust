#[cfg(feature = "NoteJumpSpeedEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteJumpSpeedEventData {
    __cordl_parent: crate::GlobalNamespace::BeatmapEventData,
    pub _relativeNoteJumpSpeed_k__BackingField: f32,
    pub easeType: crate::GlobalNamespace::EaseType,
    pub usePreviousValue: bool,
}
#[cfg(feature = "NoteJumpSpeedEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteJumpSpeedEventData => ""
    ."NoteJumpSpeedEventData"
);
#[cfg(feature = "NoteJumpSpeedEventData")]
impl std::ops::Deref for crate::GlobalNamespace::NoteJumpSpeedEventData {
    type Target = crate::GlobalNamespace::BeatmapEventData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteJumpSpeedEventData")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteJumpSpeedEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteJumpSpeedEventData")]
impl crate::GlobalNamespace::NoteJumpSpeedEventData {
    pub fn ChangeRelativeNoteJumpSpeed(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeRelativeNoteJumpSpeed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataItem,
        > = __cordl_object.invoke("GetCopy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefault(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        > = __cordl_object.invoke("GetDefault", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_time: f32,
        relativeNoteJumpSpeed: f32,
        easeType: crate::GlobalNamespace::EaseType,
        usePreviousValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_time, relativeNoteJumpSpeed, easeType, usePreviousValue),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        relativeNoteJumpSpeed: f32,
        easeType: crate::GlobalNamespace::EaseType,
        usePreviousValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (_cordl_time, relativeNoteJumpSpeed, easeType, usePreviousValue),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_relativeNoteJumpSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_relativeNoteJumpSpeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_relativeNoteJumpSpeed(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_relativeNoteJumpSpeed", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteJumpSpeedEventData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteJumpSpeedEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

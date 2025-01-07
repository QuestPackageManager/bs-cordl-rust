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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoteJumpSpeedEventData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteJumpSpeedEventData";
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

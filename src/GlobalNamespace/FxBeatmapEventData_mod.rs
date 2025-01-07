#[cfg(feature = "FxBeatmapEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct FxBeatmapEventData {
    __cordl_parent: crate::GlobalNamespace::BeatmapEventData,
    pub groupId: i32,
    pub elementId: i32,
    pub usePreviousEventValue: bool,
}
#[cfg(feature = "FxBeatmapEventData")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::FxBeatmapEventData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FxBeatmapEventData";
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
#[cfg(feature = "FxBeatmapEventData")]
impl std::ops::Deref for crate::GlobalNamespace::FxBeatmapEventData {
    type Target = crate::GlobalNamespace::BeatmapEventData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FxBeatmapEventData")]
impl std::ops::DerefMut for crate::GlobalNamespace::FxBeatmapEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FxBeatmapEventData")]
impl crate::GlobalNamespace::FxBeatmapEventData {
    pub fn New(
        _cordl_time: f32,
        groupId: i32,
        elementId: i32,
        usePreviousEventValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_time, groupId, elementId, usePreviousEventValue),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn SubtypeIdentifier(
        groupId: i32,
        elementId: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubtypeIdentifier", (groupId, elementId))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        groupId: i32,
        elementId: i32,
        usePreviousEventValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_time, groupId, elementId, usePreviousEventValue))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FxBeatmapEventData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FxBeatmapEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

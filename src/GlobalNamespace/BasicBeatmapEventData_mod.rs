#[cfg(feature = "BasicBeatmapEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicBeatmapEventData {
    __cordl_parent: crate::GlobalNamespace::BeatmapEventData,
    pub _sameTypeIndex_k__BackingField: i32,
    pub basicBeatmapEventType: crate::GlobalNamespace::BasicBeatmapEventType,
    pub value: i32,
    pub floatValue: f32,
}
#[cfg(feature = "BasicBeatmapEventData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BasicBeatmapEventData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BasicBeatmapEventData";
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
#[cfg(feature = "BasicBeatmapEventData")]
impl std::ops::Deref for crate::GlobalNamespace::BasicBeatmapEventData {
    type Target = crate::GlobalNamespace::BeatmapEventData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BasicBeatmapEventData")]
impl std::ops::DerefMut for crate::GlobalNamespace::BasicBeatmapEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BasicBeatmapEventData")]
impl crate::GlobalNamespace::BasicBeatmapEventData {
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
        basicBeatmapEventType: crate::GlobalNamespace::BasicBeatmapEventType,
        value: i32,
        floatValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_time, basicBeatmapEventType, value, floatValue),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn RecalculateSameTypeIndexFromPreviousEvent(
        &mut self,
        basicBeatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BasicBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RecalculateSameTypeIndexFromPreviousEvent",
                (basicBeatmapEventData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFirstSameTypeIndex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFirstSameTypeIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SubtypeIdentifier(
        _cordl_type: crate::GlobalNamespace::BasicBeatmapEventType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubtypeIdentifier", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        basicBeatmapEventType: crate::GlobalNamespace::BasicBeatmapEventType,
        value: i32,
        floatValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_time, basicBeatmapEventType, value, floatValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_nextSameTypeEventData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BasicBeatmapEventData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BasicBeatmapEventData,
        > = __cordl_object.invoke("get_nextSameTypeEventData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_previousSameTypeEventData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BasicBeatmapEventData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BasicBeatmapEventData,
        > = __cordl_object.invoke("get_previousSameTypeEventData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sameTypeIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_sameTypeIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sameTypeIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sameTypeIndex", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BasicBeatmapEventData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BasicBeatmapEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

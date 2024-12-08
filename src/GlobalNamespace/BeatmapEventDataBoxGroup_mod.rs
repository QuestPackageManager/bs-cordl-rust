#[cfg(feature = "BeatmapEventDataBoxGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEventDataBoxGroup {
    __cordl_parent: crate::System::Object,
    pub _beat: f32,
    pub _elementDataDict: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::System::ValueTuple_3<i32, *mut crate::System::Type, i32>,
        *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
    >,
    pub _unpackedBeatmapEventData: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::BeatmapEventData,
    >,
    pub _beatmapEventDataBoxList: *mut crate::System::Collections::Generic::IReadOnlyCollection_1<
        *mut crate::GlobalNamespace::BeatmapEventDataBox,
    >,
}
#[cfg(feature = "BeatmapEventDataBoxGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapEventDataBoxGroup => ""
    ."BeatmapEventDataBoxGroup"
);
#[cfg(feature = "BeatmapEventDataBoxGroup")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapEventDataBoxGroup {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroup")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapEventDataBoxGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroup")]
impl crate::GlobalNamespace::BeatmapEventDataBoxGroup {
    #[cfg(feature = "BeatmapEventDataBoxGroup+ElementData")]
    pub type ElementData = crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData;
    pub fn CompareTo(
        &mut self,
        b: *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (b))?;
        Ok(__cordl_ret)
    }
    pub fn GetCopyWithNewBeat(
        &mut self,
        newBeat: f32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup = __cordl_object
            .invoke("GetCopyWithNewBeat", (newBeat))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beat: f32,
        beatmapEventDataBoxList: *mut crate::System::Collections::Generic::IReadOnlyCollection_1<
            *mut crate::GlobalNamespace::BeatmapEventDataBox,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beat, beatmapEventDataBoxList))?;
        Ok(__cordl_object)
    }
    pub fn RemoveBeatmapEventDataFromBeatmapData(
        &mut self,
        beatmapData: *mut crate::GlobalNamespace::BeatmapData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveBeatmapEventDataFromBeatmapData", (beatmapData))?;
        Ok(__cordl_ret)
    }
    pub fn SyncWithBeatmapData(
        &mut self,
        groupId: i32,
        beatmapData: *mut crate::GlobalNamespace::BeatmapData,
        beatToTimeConverter: *mut crate::GlobalNamespace::IBeatToTimeConverter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncWithBeatmapData", (groupId, beatmapData, beatToTimeConverter))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        beatmapEventDataBoxList: *mut crate::System::Collections::Generic::IReadOnlyCollection_1<
            *mut crate::GlobalNamespace::BeatmapEventDataBox,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beat, beatmapEventDataBoxList))?;
        Ok(__cordl_ret)
    }
    pub fn get_elementDataDict(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyDictionary_2<
            crate::System::ValueTuple_3<i32, *mut crate::System::Type, i32>,
            *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyDictionary_2<
            crate::System::ValueTuple_3<i32, *mut crate::System::Type, i32>,
            *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
        > = __cordl_object.invoke("get_elementDataDict", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapEventDataBoxGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroup+ElementData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEventDataBoxGroup_ElementData {
    __cordl_parent: crate::System::Object,
    pub startBeat: f32,
    pub elementId: i32,
    pub durationOrderIndex: i32,
    pub distributionOrderIndex: i32,
    pub eventBoxType: *mut crate::System::Type,
    pub eventBoxSubtypeIdentifier: i32,
    pub eventBox: *mut crate::GlobalNamespace::BeatmapEventDataBox,
    pub boxGroup: *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup,
    pub _next: *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
    pub _previous: *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
}
#[cfg(feature = "BeatmapEventDataBoxGroup+ElementData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData => ""
    ."BeatmapEventDataBoxGroup/ElementData"
);
#[cfg(feature = "BeatmapEventDataBoxGroup+ElementData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroup+ElementData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroup+ElementData")]
impl crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData {
    pub fn ConnectWithNext(
        &mut self,
        nextElementData: *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConnectWithNext", (nextElementData))?;
        Ok(__cordl_ret)
    }
    pub fn ConnectWithPrevious(
        &mut self,
        prevElementData: *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConnectWithPrevious", (prevElementData))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        boxGroup: *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup,
        eventBox: *mut crate::GlobalNamespace::BeatmapEventDataBox,
        elementId: i32,
        durationOrderIndex: i32,
        distributionOrderIndex: i32,
        startBeat: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    boxGroup,
                    eventBox,
                    elementId,
                    durationOrderIndex,
                    distributionOrderIndex,
                    startBeat,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn ResetConnections(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetConnections", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        boxGroup: *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup,
        eventBox: *mut crate::GlobalNamespace::BeatmapEventDataBox,
        elementId: i32,
        durationOrderIndex: i32,
        distributionOrderIndex: i32,
        startBeat: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    boxGroup,
                    eventBox,
                    elementId,
                    durationOrderIndex,
                    distributionOrderIndex,
                    startBeat,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_next(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData = __cordl_object
            .invoke("get_next", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_previous(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData = __cordl_object
            .invoke("get_previous", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroup+ElementData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

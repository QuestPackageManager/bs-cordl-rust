#[cfg(feature = "BeatmapEventDataBoxGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEventDataBoxGroup {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _beat: f32,
    pub _elementDataDict: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::System::ValueTuple_3<
                i32,
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                i32,
            >,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
            >,
        >,
    >,
    pub _unpackedBeatmapEventData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
        >,
    >,
    pub _beatmapEventDataBoxList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyCollection_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBox>,
        >,
    >,
}
#[cfg(feature = "BeatmapEventDataBoxGroup")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapEventDataBoxGroup {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapEventDataBoxGroup";
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
#[cfg(feature = "BeatmapEventDataBoxGroup")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapEventDataBoxGroup {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        b: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBoxGroup>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (b))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCopyWithNewBeat(
        &mut self,
        newBeat: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBoxGroup>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBoxGroup,
        > = __cordl_object.invoke("GetCopyWithNewBeat", (newBeat))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        beat: f32,
        beatmapEventDataBoxList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBox>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beat, beatmapEventDataBoxList))?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveBeatmapEventDataFromBeatmapData(
        &mut self,
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveBeatmapEventDataFromBeatmapData", (beatmapData))?;
        Ok(__cordl_ret.into())
    }
    pub fn SyncWithBeatmapData(
        &mut self,
        groupId: i32,
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
        beatToTimeConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SyncWithBeatmapData",
                (groupId, beatmapData, beatToTimeConverter, lightEventConverter),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        beatmapEventDataBoxList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBox>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beat, beatmapEventDataBoxList))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapEventDataBoxList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBox>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBox>,
            >,
        > = __cordl_object.invoke("get_beatmapEventDataBoxList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_elementDataDict(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyDictionary_2<
                crate::System::ValueTuple_3<
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    i32,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyDictionary_2<
                crate::System::ValueTuple_3<
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    i32,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
                >,
            >,
        > = __cordl_object.invoke("get_elementDataDict", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "BeatmapEventDataBoxGroup")]
impl AsRef<
    crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBoxGroup>,
    >,
> for crate::GlobalNamespace::BeatmapEventDataBoxGroup {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBoxGroup>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroup")]
impl AsMut<
    crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBoxGroup>,
    >,
> for crate::GlobalNamespace::BeatmapEventDataBoxGroup {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBoxGroup>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroup+ElementData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEventDataBoxGroup_ElementData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub startBeat: f32,
    pub elementId: i32,
    pub durationOrderIndex: i32,
    pub distributionOrderIndex: i32,
    pub eventBoxType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub eventBoxSubtypeIdentifier: i32,
    pub eventBox: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBox>,
    pub boxGroup: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapEventDataBoxGroup,
    >,
    pub _next: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
    >,
    pub _previous: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
    >,
}
#[cfg(feature = "BeatmapEventDataBoxGroup+ElementData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapEventDataBoxGroup/ElementData";
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
#[cfg(feature = "BeatmapEventDataBoxGroup+ElementData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        nextElementData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConnectWithNext", (nextElementData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConnectWithPrevious(
        &mut self,
        prevElementData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConnectWithPrevious", (prevElementData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        boxGroup: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBoxGroup,
        >,
        eventBox: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBox>,
        elementId: i32,
        durationOrderIndex: i32,
        distributionOrderIndex: i32,
        startBeat: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn ResetConnections(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetConnections", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        boxGroup: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBoxGroup,
        >,
        eventBox: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBox>,
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
        Ok(__cordl_ret.into())
    }
    pub fn get_next(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
        > = __cordl_object.invoke("get_next", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_previous(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBoxGroup_ElementData,
        > = __cordl_object.invoke("get_previous", ())?;
        Ok(__cordl_ret.into())
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

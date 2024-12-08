#[cfg(feature = "BeatmapObjectsInTimeRowProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectsInTimeRowProcessor {
    __cordl_parent: crate::System::Object,
    pub _currentTimeSliceColorNotes: *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
        *mut NoteData,
    >,
    pub _currentTimeSliceAllNotesAndSliders: *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
        *mut BeatmapDataItem,
    >,
    pub _currentTimeSliceNotesByColorType: *mut crate::System::Collections::Generic::Dictionary_2<
        ColorType,
        *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
            *mut NoteData,
        >,
    >,
    pub _unprocessedSliderTails: *mut crate::System::Collections::Generic::List_1<
        *mut SliderData,
    >,
    pub _notesInColumnsReusableProcessingListOfLists: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Collections::Generic::List_1<*mut NoteData>,
    >,
    pub _numberOfLines: i32,
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapObjectsInTimeRowProcessor => ""
    ."BeatmapObjectsInTimeRowProcessor"
);
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor")]
impl std::ops::Deref for BeatmapObjectsInTimeRowProcessor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor")]
impl std::ops::DerefMut for BeatmapObjectsInTimeRowProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor")]
impl BeatmapObjectsInTimeRowProcessor {
    pub const kMaxNotesAlignmentAngle: f32 = 40f32;
    pub const kTimeRowEpsilon: f32 = 0.001f32;
    #[cfg(feature = "BeatmapObjectsInTimeRowProcessor+TimeSliceContainer_1")]
    pub type TimeSliceContainer_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
        T,
    >;
    #[cfg(feature = "BeatmapObjectsInTimeRowProcessor+__c")]
    pub type __c = crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor___c;
    #[cfg(feature = "BeatmapObjectsInTimeRowProcessor+SliderTailData")]
    pub type SliderTailData = crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_SliderTailData;
    #[cfg(feature = "BeatmapObjectsInTimeRowProcessor+__c__DisplayClass17_0")]
    pub type __c__DisplayClass17_0 = crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor___c__DisplayClass17_0;
    pub fn HandleCurrentNewTimeSliceAllNotesAndSlidersDidStartNewTimeSlice(
        &mut self,
        allObjectsTimeSlice: *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
            *mut BeatmapDataItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleCurrentNewTimeSliceAllNotesAndSlidersDidStartNewTimeSlice",
                (allObjectsTimeSlice),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleCurrentTimeSliceAllNotesAndSlidersDidFinishTimeSlice(
        &mut self,
        allObjectsTimeSlice: *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
            *mut BeatmapDataItem,
        >,
        nextTimeSliceTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleCurrentTimeSliceAllNotesAndSlidersDidFinishTimeSlice",
                (allObjectsTimeSlice, nextTimeSliceTime),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleCurrentTimeSliceColorNotesDidAddItem(
        &mut self,
        timeSliceContainer: *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
            *mut NoteData,
        >,
        noteData: *mut NoteData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleCurrentTimeSliceColorNotesDidAddItem",
                (timeSliceContainer, noteData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleCurrentTimeSliceColorNotesDidFinishTimeSlice(
        &mut self,
        currentTimeSlice: *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
            *mut NoteData,
        >,
        nextTimeSliceTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleCurrentTimeSliceColorNotesDidFinishTimeSlice",
                (currentTimeSlice, nextTimeSliceTime),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandlePerColorTypeTimeSliceContainerDidFinishTimeSlice(
        &mut self,
        timeSliceContainer: *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
            *mut NoteData,
        >,
        nextTimeSliceTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandlePerColorTypeTimeSliceContainerDidFinishTimeSlice",
                (timeSliceContainer, nextTimeSliceTime),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(numberOfLines: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (numberOfLines))?;
        Ok(__cordl_object)
    }
    pub fn ProcessAllRemainingData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAllRemainingData", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessNote(
        &mut self,
        noteData: *mut NoteData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessNote", (noteData))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessSlider(
        &mut self,
        sliderData: *mut SliderData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessSlider", (sliderData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        numberOfLines: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (numberOfLines))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor")]
impl quest_hook::libil2cpp::ObjectType for BeatmapObjectsInTimeRowProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor+SliderTailData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectsInTimeRowProcessor_SliderTailData {
    __cordl_parent: BeatmapDataItem,
    pub slider: *mut SliderData,
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor+SliderTailData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_SliderTailData => ""
    ."BeatmapObjectsInTimeRowProcessor/SliderTailData"
);
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor+SliderTailData")]
impl std::ops::Deref
for crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_SliderTailData {
    type Target = BeatmapDataItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor+SliderTailData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_SliderTailData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor+SliderTailData")]
impl crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_SliderTailData {
    pub fn GetCopy(&mut self) -> quest_hook::libil2cpp::Result<*mut BeatmapDataItem> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapDataItem = __cordl_object.invoke("GetCopy", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(slider: *mut SliderData) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (slider))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        slider: *mut SliderData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (slider))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor+SliderTailData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_SliderTailData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor+TimeSliceContainer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
    T: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Object,
    pub _time_k__BackingField: f32,
    pub _previousTimeSliceTime_k__BackingField: f32,
    pub didFinishTimeSliceEvent: *mut crate::System::Action_2<
        *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
            T,
        >,
        f32,
    >,
    pub didStartNewTimeSliceEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
            T,
        >,
    >,
    pub didAddItemEvent: *mut crate::System::Action_2<
        *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
            T,
        >,
        T,
    >,
    pub _items: *mut crate::System::Collections::Generic::List_1<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor+TimeSliceContainer_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1 < T > => ""
    ."BeatmapObjectsInTimeRowProcessor/TimeSliceContainer`1" < T >
);
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor+TimeSliceContainer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<T> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor+TimeSliceContainer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor+TimeSliceContainer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<T> {
    pub fn Add(
        &mut self,
        item: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (item))?;
        Ok(__cordl_ret)
    }
    pub fn AddWithoutNotifications(
        &mut self,
        item: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddWithoutNotifications", (item))?;
        Ok(__cordl_ret)
    }
    pub fn FinishTimeSlice(
        &mut self,
        nextTimeSliceTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishTimeSlice", (nextTimeSliceTime))?;
        Ok(__cordl_ret)
    }
    pub fn New(capacity: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (capacity))?;
        Ok(__cordl_object)
    }
    pub fn StartNewTimeSlice(
        &mut self,
        newSliceTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartNewTimeSlice", (newSliceTime))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (capacity))?;
        Ok(__cordl_ret)
    }
    pub fn add_didAddItemEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                T,
            >,
            T,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didAddItemEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didFinishTimeSliceEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                T,
            >,
            f32,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishTimeSliceEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didStartNewTimeSliceEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                T,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didStartNewTimeSliceEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_items(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<T> = __cordl_object
            .invoke("get_items", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_previousTimeSliceTime(&mut self) -> quest_hook::libil2cpp::Result<f32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_previousTimeSliceTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_time", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didAddItemEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                T,
            >,
            T,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didAddItemEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishTimeSliceEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                T,
            >,
            f32,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishTimeSliceEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didStartNewTimeSliceEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                T,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didStartNewTimeSliceEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_previousTimeSliceTime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_previousTimeSliceTime", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_time(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_time", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor+TimeSliceContainer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

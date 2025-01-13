#[cfg(feature = "BeatmapObjectsInTimeRowProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectsInTimeRowProcessor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _currentTimeSliceColorNotes: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
        >,
    >,
    pub _currentTimeSliceAllNotesAndSliders: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
        >,
    >,
    pub _currentTimeSliceNotesByColorType: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::GlobalNamespace::ColorType,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
                >,
            >,
        >,
    >,
    pub _unprocessedSliderTails: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
        >,
    >,
    pub _notesInColumnsReusableProcessingListOfLists: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
                >,
            >,
        >,
    >,
    pub _numberOfLines: i32,
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapObjectsInTimeRowProcessor";
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
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor")]
impl crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor {
    pub const kMaxNotesAlignmentAngle: f32 = 40f32;
    pub const kTimeRowEpsilon: f32 = 0.001f32;
    #[cfg(feature = "BeatmapObjectsInTimeRowProcessor+SliderTailData")]
    pub type SliderTailData = crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_SliderTailData;
    #[cfg(feature = "BeatmapObjectsInTimeRowProcessor+TimeSliceContainer_1")]
    pub type TimeSliceContainer_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
        T,
    >;
    pub fn HandleCurrentNewTimeSliceAllNotesAndSlidersDidStartNewTimeSlice(
        &mut self,
        allObjectsTimeSlice: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleCurrentTimeSliceAllNotesAndSlidersDidFinishTimeSlice(
        &mut self,
        allObjectsTimeSlice: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleCurrentTimeSliceColorNotesDidAddItem(
        &mut self,
        timeSliceContainer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
            >,
        >,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleCurrentTimeSliceColorNotesDidAddItem",
                (timeSliceContainer, noteData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleCurrentTimeSliceColorNotesDidFinishTimeSlice(
        &mut self,
        currentTimeSlice: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandlePerColorTypeTimeSliceContainerDidFinishTimeSlice(
        &mut self,
        timeSliceContainer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        numberOfLines: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (numberOfLines))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessAllRemainingData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAllRemainingData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessNote(
        &mut self,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessNote", (noteData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessSlider(
        &mut self,
        sliderData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessSlider", (sliderData))?;
        Ok(__cordl_ret.into())
    }
    pub fn SliderHeadPositionOverlapsWithBurstTail(
        slider: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
        sliderTail: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SliderHeadPositionOverlapsWithBurstTail", (slider, sliderTail))?;
        Ok(__cordl_ret.into())
    }
    pub fn SliderHeadPositionOverlapsWithNote(
        slider: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
        note: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SliderHeadPositionOverlapsWithNote", (slider, note))?;
        Ok(__cordl_ret.into())
    }
    pub fn SliderTailPositionOverlapsWithNote(
        slider: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
        note: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SliderTailPositionOverlapsWithNote", (slider, note))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor {
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
    __cordl_parent: crate::GlobalNamespace::BeatmapDataItem,
    pub slider: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor+SliderTailData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_SliderTailData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapObjectsInTimeRowProcessor/SliderTailData";
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
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor+SliderTailData")]
impl std::ops::Deref
for crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_SliderTailData {
    type Target = crate::GlobalNamespace::BeatmapDataItem;
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
    pub fn New(
        slider: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (slider))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        slider: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (slider))?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _time_k__BackingField: f32,
    pub _previousTimeSliceTime_k__BackingField: f32,
    pub didFinishTimeSliceEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                    T,
                >,
            >,
            f32,
        >,
    >,
    pub didStartNewTimeSliceEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                    T,
                >,
            >,
        >,
    >,
    pub didAddItemEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                    T,
                >,
            >,
            T,
        >,
    >,
    pub _items: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<T>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor+TimeSliceContainer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapObjectsInTimeRowProcessor/TimeSliceContainer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "",
                        "BeatmapObjectsInTimeRowProcessor/TimeSliceContainer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "BeatmapObjectsInTimeRowProcessor+TimeSliceContainer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (capacity))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn add_didAddItemEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                        T,
                    >,
                >,
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
            .invoke("add_didAddItemEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didFinishTimeSliceEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                        T,
                    >,
                >,
                f32,
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
            .invoke("add_didFinishTimeSliceEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didStartNewTimeSliceEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                        T,
                    >,
                >,
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
        Ok(__cordl_ret.into())
    }
    pub fn get_items(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<T>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<T>,
        > = __cordl_object.invoke("get_items", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn remove_didAddItemEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                        T,
                    >,
                >,
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
            .invoke("remove_didAddItemEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFinishTimeSliceEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                        T,
                    >,
                >,
                f32,
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
            .invoke("remove_didFinishTimeSliceEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didStartNewTimeSliceEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor_TimeSliceContainer_1<
                        T,
                    >,
                >,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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

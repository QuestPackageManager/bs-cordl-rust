#[cfg(feature = "BeatmapSaveDataVersion3+EventBoxGroup_1")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBoxGroup_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::BeatmapSaveDataVersion3::EventBoxGroup,
    pub e: *mut crate::System::Collections::Generic::List_1<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "BeatmapSaveDataVersion3+EventBoxGroup_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion3::EventBoxGroup_1 < T >
    => "BeatmapSaveDataVersion3"."EventBoxGroup`1" < T >
);
#[cfg(feature = "BeatmapSaveDataVersion3+EventBoxGroup_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::BeatmapSaveDataVersion3::EventBoxGroup_1<T> {
    type Target = crate::BeatmapSaveDataVersion3::EventBoxGroup;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+EventBoxGroup_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::BeatmapSaveDataVersion3::EventBoxGroup_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+EventBoxGroup_1")]
impl<T: quest_hook::libil2cpp::Type> crate::BeatmapSaveDataVersion3::EventBoxGroup_1<T> {
    pub fn New(
        beat: f32,
        groupId: i32,
        eventBoxes: *mut crate::System::Collections::Generic::List_1<T>,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beat, groupId, eventBoxes))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        groupId: i32,
        eventBoxes: *mut crate::System::Collections::Generic::List_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beat, groupId, eventBoxes))?;
        Ok(__cordl_ret)
    }
    pub fn get_baseEventBoxes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatmapSaveDataVersion3::EventBox,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatmapSaveDataVersion3::EventBox,
        > = __cordl_object.invoke("get_baseEventBoxes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_eventBoxes(
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
            .invoke("get_eventBoxes", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+EventBoxGroup_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::EventBoxGroup_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

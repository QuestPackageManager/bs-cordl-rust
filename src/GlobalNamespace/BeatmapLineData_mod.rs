#[cfg(feature = "BeatmapLineData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLineData {
    __cordl_parent: crate::System::Object,
    pub _beatmapObjectsData: *mut crate::System::Collections::Generic::List_1<
        *mut BeatmapObjectData,
    >,
}
#[cfg(feature = "BeatmapLineData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapLineData => ""."BeatmapLineData"
);
#[cfg(feature = "BeatmapLineData")]
impl std::ops::Deref for BeatmapLineData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLineData")]
impl std::ops::DerefMut for BeatmapLineData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLineData")]
impl BeatmapLineData {
    pub fn AddBeatmapObjectData(
        &mut self,
        beatmapObjectData: *mut BeatmapObjectData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddBeatmapObjectData", (beatmapObjectData))?;
        Ok(__cordl_ret)
    }
    pub fn New_List_1_1(
        beatmapObjectData: *mut crate::System::Collections::Generic::List_1<
            *mut BeatmapObjectData,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapObjectData))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_0(initialCapacity: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialCapacity))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_List_1_1(
        &mut self,
        beatmapObjectData: *mut crate::System::Collections::Generic::List_1<
            *mut BeatmapObjectData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapObjectData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_0(
        &mut self,
        initialCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialCapacity))?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapObjectsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<*mut BeatmapObjectData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut BeatmapObjectData,
        > = __cordl_object.invoke("get_beatmapObjectsData", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapLineData")]
impl quest_hook::libil2cpp::ObjectType for BeatmapLineData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
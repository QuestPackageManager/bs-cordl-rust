#[cfg(feature = "BeatmapLineData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLineData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _beatmapObjectsData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::BeatmapObjectData,
        >,
    >,
}
#[cfg(feature = "BeatmapLineData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLineData => ""
    ."BeatmapLineData"
);
#[cfg(feature = "BeatmapLineData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLineData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLineData")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLineData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLineData")]
impl crate::GlobalNamespace::BeatmapLineData {
    pub fn AddBeatmapObjectData(
        &mut self,
        beatmapObjectData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddBeatmapObjectData", (beatmapObjectData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_List_1_1(
        beatmapObjectData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::BeatmapObjectData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapObjectData))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_0(
        initialCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialCapacity))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_List_1_1(
        &mut self,
        beatmapObjectData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::BeatmapObjectData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapObjectData))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapObjectsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::BeatmapObjectData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::BeatmapObjectData,
            >,
        > = __cordl_object.invoke("get_beatmapObjectsData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLineData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapLineData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapLineData")]
impl AsRef<crate::GlobalNamespace::IReadonlyBeatmapLineData>
for crate::GlobalNamespace::BeatmapLineData {
    fn as_ref(&self) -> &crate::GlobalNamespace::IReadonlyBeatmapLineData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapLineData")]
impl AsMut<crate::GlobalNamespace::IReadonlyBeatmapLineData>
for crate::GlobalNamespace::BeatmapLineData {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IReadonlyBeatmapLineData {
        unsafe { std::mem::transmute(self) }
    }
}

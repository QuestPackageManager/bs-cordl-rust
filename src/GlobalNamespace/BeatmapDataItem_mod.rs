#[cfg(feature = "BeatmapDataItem")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataItem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _time_k__BackingField: f32,
    pub _executionOrder_k__BackingField: i32,
    pub _subtypeIdentifier_k__BackingField: i32,
    pub _type_k__BackingField: crate::GlobalNamespace::BeatmapDataItem_BeatmapDataItemType,
}
#[cfg(feature = "BeatmapDataItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapDataItem => ""
    ."BeatmapDataItem"
);
#[cfg(feature = "BeatmapDataItem")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataItem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataItem")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDataItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataItem")]
impl crate::GlobalNamespace::BeatmapDataItem {
    #[cfg(feature = "BeatmapDataItem+BeatmapDataItemType")]
    pub type BeatmapDataItemType = crate::GlobalNamespace::BeatmapDataItem_BeatmapDataItemType;
    pub fn CompareTo(
        &mut self,
        b: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (b))?;
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
    pub fn New(
        _cordl_time: f32,
        executionOrder: i32,
        subtypeIdentifier: i32,
        _cordl_type: crate::GlobalNamespace::BeatmapDataItem_BeatmapDataItemType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_time, executionOrder, subtypeIdentifier, _cordl_type),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        executionOrder: i32,
        subtypeIdentifier: i32,
        _cordl_type: crate::GlobalNamespace::BeatmapDataItem_BeatmapDataItemType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (_cordl_time, executionOrder, subtypeIdentifier, _cordl_type),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_executionOrder(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_executionOrder", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subtypeGroupIdentifier(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_subtypeGroupIdentifier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subtypeIdentifier(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_subtypeIdentifier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_time", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BeatmapDataItem_BeatmapDataItemType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapDataItem_BeatmapDataItemType = __cordl_object
            .invoke("get_type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_subtypeIdentifier(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_subtypeIdentifier", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_time(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_time", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_type(
        &mut self,
        value: crate::GlobalNamespace::BeatmapDataItem_BeatmapDataItemType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_type", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataItem")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapDataItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataItem")]
impl AsRef<crate::System::IComparable_1<*mut crate::GlobalNamespace::BeatmapDataItem>>
for crate::GlobalNamespace::BeatmapDataItem {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<*mut crate::GlobalNamespace::BeatmapDataItem> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapDataItem")]
impl AsMut<crate::System::IComparable_1<*mut crate::GlobalNamespace::BeatmapDataItem>>
for crate::GlobalNamespace::BeatmapDataItem {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        *mut crate::GlobalNamespace::BeatmapDataItem,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapDataItem+BeatmapDataItemType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeatmapDataItem_BeatmapDataItemType {
    BeatmapEvent = 1i32,
    BeatmapObject = 0i32,
}
#[cfg(feature = "BeatmapDataItem+BeatmapDataItemType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapDataItem_BeatmapDataItemType => ""
    ."BeatmapDataItem/BeatmapDataItemType"
);

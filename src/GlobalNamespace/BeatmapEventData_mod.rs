#[cfg(feature = "BeatmapEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEventData {
    __cordl_parent: crate::GlobalNamespace::BeatmapDataItem,
    pub _previousSameTypeEventData_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapEventData,
    >,
    pub _nextSameTypeEventData_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapEventData,
    >,
}
#[cfg(feature = "BeatmapEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapEventData => ""
    ."BeatmapEventData"
);
#[cfg(feature = "BeatmapEventData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapEventData {
    type Target = crate::GlobalNamespace::BeatmapDataItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventData")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventData")]
impl crate::GlobalNamespace::BeatmapEventData {
    pub const kGroupIdMultiplier: i32 = 10000i32;
    pub const kSecondaryGroupIdMultiplier: i32 = 10000000i32;
    pub fn GetDefault_0(
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
    pub fn GetDefault_BeatmapEventData1(
        &mut self,
        nextData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        > = __cordl_object.invoke("GetDefault", (nextData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_time: f32,
        executionOrder: i32,
        subtypeIdentifier: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_time, executionOrder, subtypeIdentifier))?;
        Ok(__cordl_object.into())
    }
    pub fn __ConnectWithNextSameTypeEventData(
        &mut self,
        newNextSameTypeEvent: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__ConnectWithNextSameTypeEventData", (newNextSameTypeEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn __ConnectWithPreviousSameTypeEventData(
        &mut self,
        newPreviousSameTypeEvent: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "__ConnectWithPreviousSameTypeEventData",
                (newPreviousSameTypeEvent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn __ResetConnections(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__ResetConnections", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        executionOrder: i32,
        subtypeIdentifier: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_time, executionOrder, subtypeIdentifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_nextSameTypeEventData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        > = __cordl_object.invoke("get_nextSameTypeEventData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_previousSameTypeEventData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        > = __cordl_object.invoke("get_previousSameTypeEventData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_nextSameTypeEventData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_nextSameTypeEventData", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_previousSameTypeEventData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_previousSameTypeEventData", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapEventData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

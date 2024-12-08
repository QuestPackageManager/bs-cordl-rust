#[cfg(feature = "CallbacksInTime")]
#[repr(C)]
#[derive(Debug)]
pub struct CallbacksInTime {
    __cordl_parent: crate::System::Object,
    pub lastProcessedNode: *mut crate::System::Collections::Generic::LinkedListNode_1<
        *mut BeatmapDataItem,
    >,
    pub aheadTime: f32,
    pub beatmapEventDataForCallbacksAfterNodeRemoval: *mut BeatmapEventData,
    pub _callbacksWithSubtypeIdentifier: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::System::ValueTuple_2<*mut crate::System::Type, i32>,
        *mut crate::System::Collections::Generic::List_1<*mut BeatmapDataCallbackWrapper>,
    >,
    pub _callbacks: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Type,
        *mut crate::System::Collections::Generic::List_1<*mut BeatmapDataCallbackWrapper>,
    >,
}
#[cfg(feature = "CallbacksInTime")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for CallbacksInTime => ""."CallbacksInTime"
);
#[cfg(feature = "CallbacksInTime")]
impl std::ops::Deref for CallbacksInTime {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CallbacksInTime")]
impl std::ops::DerefMut for CallbacksInTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CallbacksInTime")]
impl CallbacksInTime {
    pub fn RemoveCallback(
        &mut self,
        callbackWrapper: *mut BeatmapDataCallbackWrapper,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveCallback", (callbackWrapper))?;
        Ok(__cordl_ret)
    }
    pub fn get_isEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isEmpty", ())?;
        Ok(__cordl_ret)
    }
    pub fn CallCallbacks_BeatmapDataItem0(
        &mut self,
        beatmapDataItem: *mut BeatmapDataItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CallCallbacks", (beatmapDataItem))?;
        Ok(__cordl_ret)
    }
    pub fn CallCallbacks_Type_BeatmapDataItem1(
        &mut self,
        beatmapEventDataType: *mut crate::System::Type,
        beatmapDataItem: *mut BeatmapDataItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CallCallbacks", (beatmapEventDataType, beatmapDataItem))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        aheadTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (aheadTime))?;
        Ok(__cordl_ret)
    }
    pub fn AddCallback(
        &mut self,
        callbackWrapper: *mut BeatmapDataCallbackWrapper,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCallback", (callbackWrapper))?;
        Ok(__cordl_ret)
    }
    pub fn New(aheadTime: f32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (aheadTime))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "CallbacksInTime")]
impl quest_hook::libil2cpp::ObjectType for CallbacksInTime {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "CallbacksInTime")]
#[repr(C)]
#[derive(Debug)]
pub struct CallbacksInTime {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub lastProcessedNode: *mut crate::System::Collections::Generic::LinkedListNode_1<
        *mut crate::GlobalNamespace::BeatmapDataItem,
    >,
    pub aheadTime: f32,
    pub beatmapEventDataForCallbacksAfterNodeRemoval: *mut crate::GlobalNamespace::BeatmapEventData,
    pub _callbacksWithSubtypeIdentifier: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::System::ValueTuple_2<*mut crate::System::Type, i32>,
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::BeatmapDataCallbackWrapper,
        >,
    >,
    pub _callbacks: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Type,
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::BeatmapDataCallbackWrapper,
        >,
    >,
}
#[cfg(feature = "CallbacksInTime")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CallbacksInTime => ""
    ."CallbacksInTime"
);
#[cfg(feature = "CallbacksInTime")]
impl std::ops::Deref for crate::GlobalNamespace::CallbacksInTime {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CallbacksInTime")]
impl std::ops::DerefMut for crate::GlobalNamespace::CallbacksInTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CallbacksInTime")]
impl crate::GlobalNamespace::CallbacksInTime {
    pub fn AddCallback(
        &mut self,
        callbackWrapper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataCallbackWrapper,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCallback", (callbackWrapper))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallCallbacks_BeatmapDataItem0(
        &mut self,
        beatmapDataItem: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CallCallbacks", (beatmapDataItem))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallCallbacks_Type_BeatmapDataItem1(
        &mut self,
        beatmapEventDataType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        beatmapDataItem: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CallCallbacks", (beatmapEventDataType, beatmapDataItem))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        aheadTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (aheadTime))?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveCallback(
        &mut self,
        callbackWrapper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataCallbackWrapper,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveCallback", (callbackWrapper))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_isEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isEmpty", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "CallbacksInTime")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CallbacksInTime {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

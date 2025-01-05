#[cfg(feature = "BeatmapCallbacksController")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCallbacksController {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub willStartProcessingCallbacksThisFrameEvent: quest_hook::libil2cpp::Gc<f32>,
    pub didProcessAllCallbacksThisFrameEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub _callbacksInTimes: quest_hook::libil2cpp::Gc<
        f32,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::CallbacksInTime>,
    >,
    pub _beatmapData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IReadonlyBeatmapData,
    >,
    pub _callCallbacksBehavior: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController_ICallCallbacksBehavior,
    >,
    pub _startFilterTime: f32,
    pub _prevSongTime: f32,
    pub _songTime: f32,
    pub _sendCallbacksOnBeatmapDataChangeChange: bool,
    pub _processingCallbacks: bool,
}
#[cfg(feature = "BeatmapCallbacksController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapCallbacksController =>
    ""."BeatmapCallbacksController"
);
#[cfg(feature = "BeatmapCallbacksController")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapCallbacksController {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCallbacksController")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapCallbacksController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCallbacksController")]
impl crate::GlobalNamespace::BeatmapCallbacksController {
    #[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehavior")]
    pub type CallCallbacksBehavior = crate::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehavior;
    #[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehaviorWithLastState")]
    pub type CallCallbacksBehaviorWithLastState = crate::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehaviorWithLastState;
    #[cfg(feature = "BeatmapCallbacksController+ICallCallbacksBehavior")]
    type ICallCallbacksBehavior = crate::GlobalNamespace::BeatmapCallbacksController_ICallCallbacksBehavior;
    #[cfg(feature = "BeatmapCallbacksController+InitData")]
    pub type InitData = crate::GlobalNamespace::BeatmapCallbacksController_InitData;
    pub fn AddBeatmapCallback_Gc1<T>(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataCallbackWrapper>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataCallbackWrapper,
        > = __cordl_object.invoke("AddBeatmapCallback", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddBeatmapCallback_Gc_Gc2<T>(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<T>,
        beatmapDataSubtypeIdentifiers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataCallbackWrapper>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataCallbackWrapper,
        > = __cordl_object
            .invoke("AddBeatmapCallback", (callback, beatmapDataSubtypeIdentifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddBeatmapCallback_f32_Gc0<T>(
        &mut self,
        aheadTime: f32,
        callback: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataCallbackWrapper>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataCallbackWrapper,
        > = __cordl_object.invoke("AddBeatmapCallback", (aheadTime, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddBeatmapCallback_f32_Gc_Gc3<T>(
        &mut self,
        aheadTime: f32,
        callback: quest_hook::libil2cpp::Gc<T>,
        beatmapDataSubtypeIdentifiers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataCallbackWrapper>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataCallbackWrapper,
        > = __cordl_object
            .invoke(
                "AddBeatmapCallback",
                (aheadTime, callback, beatmapDataSubtypeIdentifiers),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleBeatmapEventDataWasInserted(
        &mut self,
        beatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        >,
        node: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapEventDataWasInserted", (beatmapEventData, node))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleBeatmapEventDataWasRemoved(
        &mut self,
        beatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapEventDataWasRemoved", (beatmapEventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleBeatmapEventDataWillBeRemoved(
        &mut self,
        beatmapEventDataToRemove: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        >,
        nodeToRemove: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeatmapEventDataWillBeRemoved",
                (beatmapEventDataToRemove, nodeToRemove),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ManualUpdate(
        &mut self,
        songTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualUpdate", (songTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController_InitData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initData))?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveBeatmapCallback(
        &mut self,
        callbackWrapper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataCallbackWrapper,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveBeatmapCallback", (callbackWrapper))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReplayState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReplayState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerBeatmapEvent(
        &mut self,
        beatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerBeatmapEvent", (beatmapEventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController_InitData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initData))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didProcessAllCallbacksThisFrameEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didProcessAllCallbacksThisFrameEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_willStartProcessingCallbacksThisFrameEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_willStartProcessingCallbacksThisFrameEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sendCallbacksOnBeatmapDataChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_sendCallbacksOnBeatmapDataChange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didProcessAllCallbacksThisFrameEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didProcessAllCallbacksThisFrameEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_willStartProcessingCallbacksThisFrameEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_willStartProcessingCallbacksThisFrameEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sendCallbacksOnBeatmapDataChange(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sendCallbacksOnBeatmapDataChange", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapCallbacksController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapCallbacksController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapCallbacksController")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::BeatmapCallbacksController {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapCallbacksController")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::BeatmapCallbacksController {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehavior")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCallbacksController_CallCallbacksBehavior {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehavior")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehavior => ""
    ."BeatmapCallbacksController/CallCallbacksBehavior"
);
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehavior")]
impl std::ops::Deref
for crate::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehavior {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehavior")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehavior")]
impl crate::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehavior {
    pub fn CallCallbacks(
        &mut self,
        callbacksInTime: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::CallbacksInTime,
        >,
        beatmapDataItem: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CallCallbacks", (callbacksInTime, beatmapDataItem))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Replay(
        &mut self,
        callbacksInTimes: quest_hook::libil2cpp::Gc<
            f32,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::CallbacksInTime>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Replay", (callbacksInTimes))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehavior")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehavior {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehavior")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController_ICallCallbacksBehavior,
    >,
> for crate::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehavior {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController_ICallCallbacksBehavior,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehavior")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController_ICallCallbacksBehavior,
    >,
> for crate::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehavior {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController_ICallCallbacksBehavior,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehaviorWithLastState")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCallbacksController_CallCallbacksBehaviorWithLastState {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _replayState: quest_hook::libil2cpp::Gc<
        crate::System::ValueTuple_2<quest_hook::libil2cpp::Gc<crate::System::Type>, i32>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
    >,
}
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehaviorWithLastState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehaviorWithLastState =>
    ""."BeatmapCallbacksController/CallCallbacksBehaviorWithLastState"
);
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehaviorWithLastState")]
impl std::ops::Deref
for crate::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehaviorWithLastState {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehaviorWithLastState")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehaviorWithLastState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehaviorWithLastState")]
impl crate::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehaviorWithLastState {
    pub fn CallCallbacks(
        &mut self,
        callbacksInTime: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::CallbacksInTime,
        >,
        beatmapDataItem: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CallCallbacks", (callbacksInTime, beatmapDataItem))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Replay(
        &mut self,
        callbacksInTimes: quest_hook::libil2cpp::Gc<
            f32,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::CallbacksInTime>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Replay", (callbacksInTimes))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehaviorWithLastState")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehaviorWithLastState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehaviorWithLastState")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController_ICallCallbacksBehavior,
    >,
>
for crate::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehaviorWithLastState {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController_ICallCallbacksBehavior,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehaviorWithLastState")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController_ICallCallbacksBehavior,
    >,
>
for crate::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehaviorWithLastState {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController_ICallCallbacksBehavior,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapCallbacksController+ICallCallbacksBehavior")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCallbacksController_ICallCallbacksBehavior {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapCallbacksController+ICallCallbacksBehavior")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapCallbacksController_ICallCallbacksBehavior => ""
    ."BeatmapCallbacksController/ICallCallbacksBehavior"
);
#[cfg(feature = "BeatmapCallbacksController+ICallCallbacksBehavior")]
impl std::ops::Deref
for crate::GlobalNamespace::BeatmapCallbacksController_ICallCallbacksBehavior {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCallbacksController+ICallCallbacksBehavior")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapCallbacksController_ICallCallbacksBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCallbacksController+ICallCallbacksBehavior")]
impl crate::GlobalNamespace::BeatmapCallbacksController_ICallCallbacksBehavior {
    pub fn CallCallbacks(
        &mut self,
        callbacksInTime: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::CallbacksInTime,
        >,
        beatmapDataItem: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CallCallbacks", (callbacksInTime, beatmapDataItem))?;
        Ok(__cordl_ret.into())
    }
    pub fn Replay(
        &mut self,
        callbacksInTimes: quest_hook::libil2cpp::Gc<
            f32,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::CallbacksInTime>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Replay", (callbacksInTimes))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "BeatmapCallbacksController+ICallCallbacksBehavior")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapCallbacksController_ICallCallbacksBehavior {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapCallbacksController+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCallbacksController_InitData {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub beatmapData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IReadonlyBeatmapData,
    >,
    pub startFilterTime: f32,
    pub shouldKeepReplayState: bool,
}
#[cfg(feature = "BeatmapCallbacksController+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapCallbacksController_InitData => ""
    ."BeatmapCallbacksController/InitData"
);
#[cfg(feature = "BeatmapCallbacksController+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapCallbacksController_InitData {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCallbacksController+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapCallbacksController_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCallbacksController+InitData")]
impl crate::GlobalNamespace::BeatmapCallbacksController_InitData {
    pub fn New(
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
        startFilterTime: f32,
        shouldKeepReplayState: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (beatmapData, startFilterTime, shouldKeepReplayState),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
        startFilterTime: f32,
        shouldKeepReplayState: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapData, startFilterTime, shouldKeepReplayState))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapCallbacksController+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapCallbacksController_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

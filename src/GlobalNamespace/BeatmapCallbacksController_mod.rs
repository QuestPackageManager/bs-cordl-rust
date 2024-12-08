#[cfg(feature = "BeatmapCallbacksController")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCallbacksController {
    __cordl_parent: crate::System::Object,
    pub didProcessAllCallbacksThisFrameEvent: *mut crate::System::Action,
    pub _callbacksInTimes: *mut crate::System::Collections::Generic::Dictionary_2<
        f32,
        *mut CallbacksInTime,
    >,
    pub _beatmapData: *mut IReadonlyBeatmapData,
    pub _callCallbacksBehavior: *mut crate::GlobalNamespace::BeatmapCallbacksController_ICallCallbacksBehavior,
    pub _startFilterTime: f32,
    pub _prevSongTime: f32,
    pub _songTime: f32,
    pub _sendCallbacksOnBeatmapDataChangeChange: bool,
    pub _processingCallbacks: bool,
}
#[cfg(feature = "BeatmapCallbacksController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapCallbacksController => ""
    ."BeatmapCallbacksController"
);
#[cfg(feature = "BeatmapCallbacksController")]
impl std::ops::Deref for BeatmapCallbacksController {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCallbacksController")]
impl std::ops::DerefMut for BeatmapCallbacksController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCallbacksController")]
impl BeatmapCallbacksController {
    #[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehaviorWithLastState")]
    pub type CallCallbacksBehaviorWithLastState = crate::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehaviorWithLastState;
    #[cfg(feature = "BeatmapCallbacksController+InitData")]
    pub type InitData = crate::GlobalNamespace::BeatmapCallbacksController_InitData;
    #[cfg(feature = "BeatmapCallbacksController+ICallCallbacksBehavior")]
    type ICallCallbacksBehavior = crate::GlobalNamespace::BeatmapCallbacksController_ICallCallbacksBehavior;
    #[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehavior")]
    pub type CallCallbacksBehavior = crate::GlobalNamespace::BeatmapCallbacksController_CallCallbacksBehavior;
    pub fn TriggerBeatmapEvent(
        &mut self,
        beatmapEventData: *mut BeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerBeatmapEvent", (beatmapEventData))?;
        Ok(__cordl_ret)
    }
    pub fn add_didProcessAllCallbacksThisFrameEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didProcessAllCallbacksThisFrameEvent", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        initData: *mut crate::GlobalNamespace::BeatmapCallbacksController_InitData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initData))?;
        Ok(__cordl_ret)
    }
    pub fn HandleBeatmapEventDataWillBeRemoved(
        &mut self,
        beatmapEventDataToRemove: *mut BeatmapEventData,
        nodeToRemove: *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut BeatmapDataItem,
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
        Ok(__cordl_ret)
    }
    pub fn HandleBeatmapEventDataWasInserted(
        &mut self,
        beatmapEventData: *mut BeatmapEventData,
        node: *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut BeatmapDataItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapEventDataWasInserted", (beatmapEventData, node))?;
        Ok(__cordl_ret)
    }
    pub fn HandleBeatmapEventDataWasRemoved(
        &mut self,
        beatmapEventData: *mut BeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapEventDataWasRemoved", (beatmapEventData))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn ReplayState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReplayState", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddBeatmapCallback_f32_BeatmapDataCallback_1_0<T>(
        &mut self,
        aheadTime: f32,
        callback: *mut BeatmapDataCallback_1<T>,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapDataCallbackWrapper>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapDataCallbackWrapper = __cordl_object
            .invoke("AddBeatmapCallback", (aheadTime, callback))?;
        Ok(__cordl_ret)
    }
    pub fn AddBeatmapCallback_BeatmapDataCallback_1_1<T>(
        &mut self,
        callback: *mut BeatmapDataCallback_1<T>,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapDataCallbackWrapper>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapDataCallbackWrapper = __cordl_object
            .invoke("AddBeatmapCallback", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn AddBeatmapCallback_BeatmapDataCallback_1_Il2CppArray2<T>(
        &mut self,
        callback: *mut BeatmapDataCallback_1<T>,
        beatmapDataSubtypeIdentifiers: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapDataCallbackWrapper>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapDataCallbackWrapper = __cordl_object
            .invoke("AddBeatmapCallback", (callback, beatmapDataSubtypeIdentifiers))?;
        Ok(__cordl_ret)
    }
    pub fn AddBeatmapCallback_f32_BeatmapDataCallback_1_Il2CppArray3<T>(
        &mut self,
        aheadTime: f32,
        callback: *mut BeatmapDataCallback_1<T>,
        beatmapDataSubtypeIdentifiers: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapDataCallbackWrapper>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapDataCallbackWrapper = __cordl_object
            .invoke(
                "AddBeatmapCallback",
                (aheadTime, callback, beatmapDataSubtypeIdentifiers),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_sendCallbacksOnBeatmapDataChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_sendCallbacksOnBeatmapDataChange", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didProcessAllCallbacksThisFrameEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didProcessAllCallbacksThisFrameEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_songTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveBeatmapCallback(
        &mut self,
        callbackWrapper: *mut BeatmapDataCallbackWrapper,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveBeatmapCallback", (callbackWrapper))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        initData: *mut crate::GlobalNamespace::BeatmapCallbacksController_InitData,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initData))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapCallbacksController")]
impl quest_hook::libil2cpp::ObjectType for BeatmapCallbacksController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehavior")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCallbacksController_CallCallbacksBehavior {
    __cordl_parent: crate::System::Object,
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
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn CallCallbacks(
        &mut self,
        callbacksInTime: *mut CallbacksInTime,
        beatmapDataItem: *mut BeatmapDataItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CallCallbacks", (callbacksInTime, beatmapDataItem))?;
        Ok(__cordl_ret)
    }
    pub fn Replay(
        &mut self,
        callbacksInTimes: *mut crate::System::Collections::Generic::Dictionary_2<
            f32,
            *mut CallbacksInTime,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Replay", (callbacksInTimes))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "BeatmapCallbacksController+CallCallbacksBehaviorWithLastState")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCallbacksController_CallCallbacksBehaviorWithLastState {
    __cordl_parent: crate::System::Object,
    pub _replayState: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::System::ValueTuple_2<*mut crate::System::Type, i32>,
        *mut BeatmapDataItem,
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
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn CallCallbacks(
        &mut self,
        callbacksInTime: *mut CallbacksInTime,
        beatmapDataItem: *mut BeatmapDataItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CallCallbacks", (callbacksInTime, beatmapDataItem))?;
        Ok(__cordl_ret)
    }
    pub fn Replay(
        &mut self,
        callbacksInTimes: *mut crate::System::Collections::Generic::Dictionary_2<
            f32,
            *mut CallbacksInTime,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Replay", (callbacksInTimes))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
        callbacksInTime: *mut CallbacksInTime,
        beatmapDataItem: *mut BeatmapDataItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CallCallbacks", (callbacksInTime, beatmapDataItem))?;
        Ok(__cordl_ret)
    }
    pub fn Replay(
        &mut self,
        callbacksInTimes: *mut crate::System::Collections::Generic::Dictionary_2<
            f32,
            *mut CallbacksInTime,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Replay", (callbacksInTimes))?;
        Ok(__cordl_ret)
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
    __cordl_parent: crate::System::Object,
    pub beatmapData: *mut IReadonlyBeatmapData,
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
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
        beatmapData: *mut IReadonlyBeatmapData,
        startFilterTime: f32,
        shouldKeepReplayState: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapData, startFilterTime, shouldKeepReplayState))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beatmapData: *mut IReadonlyBeatmapData,
        startFilterTime: f32,
        shouldKeepReplayState: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (beatmapData, startFilterTime, shouldKeepReplayState),
            )?;
        Ok(__cordl_object)
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

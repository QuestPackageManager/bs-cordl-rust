#[cfg(feature = "UnityEngine+Timeline+SignalReceiver")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalReceiver {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub m_Events: *mut crate::UnityEngine::Timeline::SignalReceiver_EventKeyValue,
}
#[cfg(feature = "UnityEngine+Timeline+SignalReceiver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::SignalReceiver =>
    "UnityEngine.Timeline"."SignalReceiver"
);
#[cfg(feature = "UnityEngine+Timeline+SignalReceiver")]
impl std::ops::Deref for crate::UnityEngine::Timeline::SignalReceiver {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+SignalReceiver")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::SignalReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+SignalReceiver")]
impl crate::UnityEngine::Timeline::SignalReceiver {
    #[cfg(feature = "UnityEngine+Timeline+SignalReceiver+EventKeyValue")]
    pub type EventKeyValue = crate::UnityEngine::Timeline::SignalReceiver_EventKeyValue;
    pub fn AddEmptyReaction(
        &mut self,
        reaction: quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityEvent>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("AddEmptyReaction", (reaction))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddReaction(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::SignalAsset>,
        reaction: quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddReaction", (asset, reaction))?;
        Ok(__cordl_ret.into())
    }
    pub fn ChangeReactionAtIndex(
        &mut self,
        idx: i32,
        reaction: quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeReactionAtIndex", (idx, reaction))?;
        Ok(__cordl_ret.into())
    }
    pub fn ChangeSignalAtIndex(
        &mut self,
        idx: i32,
        newKey: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::SignalAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeSignalAtIndex", (idx, newKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReaction(
        &mut self,
        key: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::SignalAsset>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityEvent>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent,
        > = __cordl_object.invoke("GetReaction", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReactionAtIndex(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityEvent>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent,
        > = __cordl_object.invoke("GetReactionAtIndex", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRegisteredSignals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::Timeline::SignalAsset,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::Timeline::SignalAsset,
            >,
        > = __cordl_object.invoke("GetRegisteredSignals", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSignalAssetAtIndex(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::SignalAsset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::SignalAsset,
        > = __cordl_object.invoke("GetSignalAssetAtIndex", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnNotify(
        &mut self,
        origin: crate::UnityEngine::Playables::Playable,
        notification: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::INotification,
        >,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNotify", (origin, notification, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::SignalAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (asset))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAtIndex(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAtIndex", (idx))?;
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
#[cfg(feature = "UnityEngine+Timeline+SignalReceiver")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::SignalReceiver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+SignalReceiver+EventKeyValue")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalReceiver_EventKeyValue {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Signals: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Timeline::SignalAsset,
    >,
    pub m_Events: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Events::UnityEvent,
    >,
}
#[cfg(feature = "UnityEngine+Timeline+SignalReceiver+EventKeyValue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::SignalReceiver_EventKeyValue => "UnityEngine.Timeline"
    ."SignalReceiver/EventKeyValue"
);
#[cfg(feature = "UnityEngine+Timeline+SignalReceiver+EventKeyValue")]
impl std::ops::Deref for crate::UnityEngine::Timeline::SignalReceiver_EventKeyValue {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+SignalReceiver+EventKeyValue")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::SignalReceiver_EventKeyValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+SignalReceiver+EventKeyValue")]
impl crate::UnityEngine::Timeline::SignalReceiver_EventKeyValue {
    pub fn Append(
        &mut self,
        key: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::SignalAsset>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Append", (key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Remove_SignalAsset1(
        &mut self,
        key: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::SignalAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove_i32_0(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValue(
        &mut self,
        key: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::SignalAsset>,
        value: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::Events::UnityEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (key, value))?;
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
    pub fn get_events(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::Events::UnityEvent,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::Events::UnityEvent,
            >,
        > = __cordl_object.invoke("get_events", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_signals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::Timeline::SignalAsset,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::Timeline::SignalAsset,
            >,
        > = __cordl_object.invoke("get_signals", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+SignalReceiver+EventKeyValue")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::SignalReceiver_EventKeyValue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

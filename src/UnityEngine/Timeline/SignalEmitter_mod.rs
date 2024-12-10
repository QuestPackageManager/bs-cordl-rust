#[cfg(feature = "UnityEngine+Timeline+SignalEmitter")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalEmitter {
    __cordl_parent: crate::UnityEngine::Timeline::Marker,
    pub m_Retroactive: bool,
    pub m_EmitOnce: bool,
    pub m_Asset: *mut crate::UnityEngine::Timeline::SignalAsset,
}
#[cfg(feature = "UnityEngine+Timeline+SignalEmitter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::SignalEmitter =>
    "UnityEngine.Timeline"."SignalEmitter"
);
#[cfg(feature = "UnityEngine+Timeline+SignalEmitter")]
impl std::ops::Deref for crate::UnityEngine::Timeline::SignalEmitter {
    type Target = crate::UnityEngine::Timeline::Marker;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+SignalEmitter")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::SignalEmitter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+SignalEmitter")]
impl crate::UnityEngine::Timeline::SignalEmitter {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UnityEngine_Playables_INotification_get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PropertyName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::PropertyName = __cordl_object
            .invoke("UnityEngine.Playables.INotification.get_id", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_Timeline_INotificationOptionProvider_get_flags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::NotificationFlags> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::NotificationFlags = __cordl_object
            .invoke("UnityEngine.Timeline.INotificationOptionProvider.get_flags", ())?;
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
    pub fn get_asset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::SignalAsset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::SignalAsset,
        > = __cordl_object.invoke("get_asset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_emitOnce(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_emitOnce", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_retroactive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_retroactive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_asset(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::SignalAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_asset", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_emitOnce(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_emitOnce", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_retroactive(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_retroactive", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+SignalEmitter")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::SignalEmitter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

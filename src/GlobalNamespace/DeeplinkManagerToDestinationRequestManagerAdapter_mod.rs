#[cfg(feature = "DeeplinkManagerToDestinationRequestManagerAdapter")]
#[repr(C)]
#[derive(Debug)]
pub struct DeeplinkManagerToDestinationRequestManagerAdapter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _beatmapLevelsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsModel,
    >,
    pub _beatmapCharacteristicCollection: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCharacteristicCollection,
    >,
    pub didSendMenuDestinationRequestEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut crate::GlobalNamespace::MenuDestination>,
    >,
    pub _currentMenuDestinationRequest: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MenuDestination,
    >,
}
#[cfg(feature = "DeeplinkManagerToDestinationRequestManagerAdapter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DeeplinkManagerToDestinationRequestManagerAdapter => ""
    ."DeeplinkManagerToDestinationRequestManagerAdapter"
);
#[cfg(feature = "DeeplinkManagerToDestinationRequestManagerAdapter")]
impl std::ops::Deref
for crate::GlobalNamespace::DeeplinkManagerToDestinationRequestManagerAdapter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DeeplinkManagerToDestinationRequestManagerAdapter")]
impl std::ops::DerefMut
for crate::GlobalNamespace::DeeplinkManagerToDestinationRequestManagerAdapter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DeeplinkManagerToDestinationRequestManagerAdapter")]
impl crate::GlobalNamespace::DeeplinkManagerToDestinationRequestManagerAdapter {
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleDeeplinkManagerDidReceiveDeeplink(
        &mut self,
        deeplink: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Deeplink>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDeeplinkManagerDidReceiveDeeplink", (deeplink))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        deeplinkManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IDeeplinkManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (deeplinkManager))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn add_didSendMenuDestinationRequestEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::MenuDestination>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSendMenuDestinationRequestEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentMenuDestinationRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuDestination>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MenuDestination,
        > = __cordl_object.invoke("get_currentMenuDestinationRequest", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSendMenuDestinationRequestEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::MenuDestination>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSendMenuDestinationRequestEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_currentMenuDestinationRequest(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuDestination>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_currentMenuDestinationRequest", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DeeplinkManagerToDestinationRequestManagerAdapter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DeeplinkManagerToDestinationRequestManagerAdapter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DeeplinkManagerToDestinationRequestManagerAdapter")]
impl AsRef<crate::GlobalNamespace::IDestinationRequestManager>
for crate::GlobalNamespace::DeeplinkManagerToDestinationRequestManagerAdapter {
    fn as_ref(&self) -> &crate::GlobalNamespace::IDestinationRequestManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DeeplinkManagerToDestinationRequestManagerAdapter")]
impl AsMut<crate::GlobalNamespace::IDestinationRequestManager>
for crate::GlobalNamespace::DeeplinkManagerToDestinationRequestManagerAdapter {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IDestinationRequestManager {
        unsafe { std::mem::transmute(self) }
    }
}

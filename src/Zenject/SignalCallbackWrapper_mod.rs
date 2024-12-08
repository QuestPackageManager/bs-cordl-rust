#[cfg(feature = "Zenject+SignalCallbackWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalCallbackWrapper {
    __cordl_parent: crate::System::Object,
    pub _signalBus: *mut crate::Zenject::SignalBus,
    pub _action: *mut crate::System::Action_1<*mut crate::System::Object>,
    pub _signalType: *mut crate::System::Type,
    pub _identifier: *mut crate::System::Object,
}
#[cfg(feature = "Zenject+SignalCallbackWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SignalCallbackWrapper => "Zenject"
    ."SignalCallbackWrapper"
);
#[cfg(feature = "Zenject+SignalCallbackWrapper")]
impl std::ops::Deref for crate::Zenject::SignalCallbackWrapper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalCallbackWrapper")]
impl std::ops::DerefMut for crate::Zenject::SignalCallbackWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalCallbackWrapper")]
impl crate::Zenject::SignalCallbackWrapper {
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
        bindInfo: *mut crate::Zenject::SignalBindingBindInfo,
        action: *mut crate::System::Action_1<*mut crate::System::Object>,
        signalBus: *mut crate::Zenject::SignalBus,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindInfo, action, signalBus))?;
        Ok(__cordl_ret)
    }
    pub fn OnSignalFired(
        &mut self,
        signal: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSignalFired", (signal))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bindInfo: *mut crate::Zenject::SignalBindingBindInfo,
        action: *mut crate::System::Action_1<*mut crate::System::Object>,
        signalBus: *mut crate::Zenject::SignalBus,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo, action, signalBus))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+SignalCallbackWrapper")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::SignalCallbackWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

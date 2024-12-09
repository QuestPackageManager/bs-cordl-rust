#[cfg(feature = "Zenject+SignalCallbackWithLookupWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalCallbackWithLookupWrapper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _signalBus: *mut crate::Zenject::SignalBus,
    pub _lookupId: crate::System::Guid,
    pub _methodGetter: *mut crate::System::Func_2<
        *mut quest_hook::libil2cpp::Il2CppObject,
        *mut crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
    >,
    pub _objectType: *mut crate::System::Type,
    pub _signalType: *mut crate::System::Type,
    pub _identifier: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+SignalCallbackWithLookupWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SignalCallbackWithLookupWrapper =>
    "Zenject"."SignalCallbackWithLookupWrapper"
);
#[cfg(feature = "Zenject+SignalCallbackWithLookupWrapper")]
impl std::ops::Deref for crate::Zenject::SignalCallbackWithLookupWrapper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalCallbackWithLookupWrapper")]
impl std::ops::DerefMut for crate::Zenject::SignalCallbackWithLookupWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalCallbackWithLookupWrapper")]
impl crate::Zenject::SignalCallbackWithLookupWrapper {
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
    pub fn New(
        signalBindInfo: *mut crate::Zenject::SignalBindingBindInfo,
        objectType: *mut crate::System::Type,
        lookupId: crate::System::Guid,
        methodGetter: *mut crate::System::Func_2<
            *mut quest_hook::libil2cpp::Il2CppObject,
            *mut crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        signalBus: *mut crate::Zenject::SignalBus,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    signalBindInfo,
                    objectType,
                    lookupId,
                    methodGetter,
                    signalBus,
                    container,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn OnSignalFired(
        &mut self,
        signal: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSignalFired", (signal))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        signalBindInfo: *mut crate::Zenject::SignalBindingBindInfo,
        objectType: *mut crate::System::Type,
        lookupId: crate::System::Guid,
        methodGetter: *mut crate::System::Func_2<
            *mut quest_hook::libil2cpp::Il2CppObject,
            *mut crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        signalBus: *mut crate::Zenject::SignalBus,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    signalBindInfo,
                    objectType,
                    lookupId,
                    methodGetter,
                    signalBus,
                    container,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+SignalCallbackWithLookupWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SignalCallbackWithLookupWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

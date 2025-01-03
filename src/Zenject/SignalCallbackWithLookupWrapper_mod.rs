#[cfg(feature = "Zenject+SignalCallbackWithLookupWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalCallbackWithLookupWrapper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub _signalBus: quest_hook::libil2cpp::Gc<crate::Zenject::SignalBus>,
    pub _lookupId: crate::System::Guid,
    pub _methodGetter: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<
            *mut quest_hook::libil2cpp::Il2CppObject,
            *mut crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
    pub _objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        signalBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::SignalBindingBindInfo>,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        lookupId: crate::System::Guid,
        methodGetter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        signalBus: quest_hook::libil2cpp::Gc<crate::Zenject::SignalBus>,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn OnSignalFired(
        &mut self,
        signal: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSignalFired", (signal))?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreate(
        P_0: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("__zenCreate", (P_0))?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreateInjectTypeInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__zenCreateInjectTypeInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        signalBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::SignalBindingBindInfo>,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        lookupId: crate::System::Guid,
        methodGetter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        signalBus: quest_hook::libil2cpp::Gc<crate::Zenject::SignalBus>,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
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
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Zenject+SignalCallbackWithLookupWrapper")]
impl AsRef<crate::System::IDisposable>
for crate::Zenject::SignalCallbackWithLookupWrapper {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+SignalCallbackWithLookupWrapper")]
impl AsMut<crate::System::IDisposable>
for crate::Zenject::SignalCallbackWithLookupWrapper {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}

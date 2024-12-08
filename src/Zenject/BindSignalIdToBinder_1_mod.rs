#[cfg(feature = "Zenject+BindSignalIdToBinder_1")]
#[repr(C)]
#[derive(Debug)]
pub struct BindSignalIdToBinder_1<TSignal: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::Zenject::BindSignalToBinder_1<TSignal>,
    __cordl_phantom_TSignal: std::marker::PhantomData<TSignal>,
}
#[cfg(feature = "Zenject+BindSignalIdToBinder_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::BindSignalIdToBinder_1 < TSignal > =>
    "Zenject"."BindSignalIdToBinder`1" < TSignal >
);
#[cfg(feature = "Zenject+BindSignalIdToBinder_1")]
impl<TSignal: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::BindSignalIdToBinder_1<TSignal> {
    type Target = crate::Zenject::BindSignalToBinder_1<TSignal>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+BindSignalIdToBinder_1")]
impl<TSignal: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::BindSignalIdToBinder_1<TSignal> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+BindSignalIdToBinder_1")]
impl<
    TSignal: quest_hook::libil2cpp::Type,
> crate::Zenject::BindSignalIdToBinder_1<TSignal> {
    pub fn New(
        container: *mut crate::Zenject::DiContainer,
        signalBindInfo: *mut crate::Zenject::SignalBindingBindInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, signalBindInfo))?;
        Ok(__cordl_object)
    }
    pub fn WithId(
        &mut self,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::BindSignalToBinder_1<TSignal>,
    >
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::BindSignalToBinder_1<TSignal> = __cordl_object
            .invoke("WithId", (identifier))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        signalBindInfo: *mut crate::Zenject::SignalBindingBindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (container, signalBindInfo))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+BindSignalIdToBinder_1")]
impl<TSignal: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::BindSignalIdToBinder_1<TSignal> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "Zenject+DeclareSignalRequireHandlerAsyncTickPriorityCopyBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct DeclareSignalRequireHandlerAsyncTickPriorityCopyBinder {
    __cordl_parent: crate::Zenject::DeclareSignalAsyncTickPriorityCopyBinder,
}
#[cfg(feature = "Zenject+DeclareSignalRequireHandlerAsyncTickPriorityCopyBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::DeclareSignalRequireHandlerAsyncTickPriorityCopyBinder => "Zenject"
    ."DeclareSignalRequireHandlerAsyncTickPriorityCopyBinder"
);
#[cfg(feature = "Zenject+DeclareSignalRequireHandlerAsyncTickPriorityCopyBinder")]
impl std::ops::Deref
for crate::Zenject::DeclareSignalRequireHandlerAsyncTickPriorityCopyBinder {
    type Target = crate::Zenject::DeclareSignalAsyncTickPriorityCopyBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DeclareSignalRequireHandlerAsyncTickPriorityCopyBinder")]
impl std::ops::DerefMut
for crate::Zenject::DeclareSignalRequireHandlerAsyncTickPriorityCopyBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DeclareSignalRequireHandlerAsyncTickPriorityCopyBinder")]
impl crate::Zenject::DeclareSignalRequireHandlerAsyncTickPriorityCopyBinder {
    pub fn New(
        signalBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::SignalDeclarationBindInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signalBindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn OptionalSubscriber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::DeclareSignalAsyncTickPriorityCopyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::DeclareSignalAsyncTickPriorityCopyBinder,
        > = __cordl_object.invoke("OptionalSubscriber", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OptionalSubscriberWithWarning(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::DeclareSignalAsyncTickPriorityCopyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::DeclareSignalAsyncTickPriorityCopyBinder,
        > = __cordl_object.invoke("OptionalSubscriberWithWarning", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RequireSubscriber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::DeclareSignalAsyncTickPriorityCopyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::DeclareSignalAsyncTickPriorityCopyBinder,
        > = __cordl_object.invoke("RequireSubscriber", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        signalBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::SignalDeclarationBindInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signalBindInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+DeclareSignalRequireHandlerAsyncTickPriorityCopyBinder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::DeclareSignalRequireHandlerAsyncTickPriorityCopyBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

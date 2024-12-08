#[cfg(feature = "Zenject+DeclareSignalIdRequireHandlerAsyncTickPriorityCopyBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct DeclareSignalIdRequireHandlerAsyncTickPriorityCopyBinder {
    __cordl_parent: crate::Zenject::DeclareSignalRequireHandlerAsyncTickPriorityCopyBinder,
}
#[cfg(feature = "Zenject+DeclareSignalIdRequireHandlerAsyncTickPriorityCopyBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::DeclareSignalIdRequireHandlerAsyncTickPriorityCopyBinder => "Zenject"
    ."DeclareSignalIdRequireHandlerAsyncTickPriorityCopyBinder"
);
#[cfg(feature = "Zenject+DeclareSignalIdRequireHandlerAsyncTickPriorityCopyBinder")]
impl std::ops::Deref
for crate::Zenject::DeclareSignalIdRequireHandlerAsyncTickPriorityCopyBinder {
    type Target = crate::Zenject::DeclareSignalRequireHandlerAsyncTickPriorityCopyBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DeclareSignalIdRequireHandlerAsyncTickPriorityCopyBinder")]
impl std::ops::DerefMut
for crate::Zenject::DeclareSignalIdRequireHandlerAsyncTickPriorityCopyBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DeclareSignalIdRequireHandlerAsyncTickPriorityCopyBinder")]
impl crate::Zenject::DeclareSignalIdRequireHandlerAsyncTickPriorityCopyBinder {
    pub fn New(
        signalBindInfo: *mut crate::Zenject::SignalDeclarationBindInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signalBindInfo))?;
        Ok(__cordl_object)
    }
    pub fn WithId(
        &mut self,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::DeclareSignalRequireHandlerAsyncTickPriorityCopyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DeclareSignalRequireHandlerAsyncTickPriorityCopyBinder = __cordl_object
            .invoke("WithId", (identifier))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        signalBindInfo: *mut crate::Zenject::SignalDeclarationBindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signalBindInfo))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+DeclareSignalIdRequireHandlerAsyncTickPriorityCopyBinder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::DeclareSignalIdRequireHandlerAsyncTickPriorityCopyBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

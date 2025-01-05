#[cfg(feature = "Zenject+SignalExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Zenject+SignalExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SignalExtensions => "Zenject"
    ."SignalExtensions"
);
#[cfg(feature = "Zenject+SignalExtensions")]
impl std::ops::Deref for crate::Zenject::SignalExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalExtensions")]
impl std::ops::DerefMut for crate::Zenject::SignalExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalExtensions")]
impl crate::Zenject::SignalExtensions {
    pub fn BindSignal<TSignal>(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSignal>>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSignal> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BindSignal", (container))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDefaultSignalDeclarationBindInfo(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::SignalDeclarationBindInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::SignalDeclarationBindInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateDefaultSignalDeclarationBindInfo", (container, signalType))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeclareSignal<TSignal>(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::DeclareSignalIdRequireHandlerAsyncTickPriorityCopyBinder,
        >,
    >
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::DeclareSignalIdRequireHandlerAsyncTickPriorityCopyBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeclareSignal", (container))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+SignalExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::SignalExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

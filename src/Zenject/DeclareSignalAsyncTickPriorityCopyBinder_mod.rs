#[cfg(feature = "Zenject+DeclareSignalAsyncTickPriorityCopyBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct DeclareSignalAsyncTickPriorityCopyBinder {
    __cordl_parent: crate::Zenject::SignalTickPriorityCopyBinder,
}
#[cfg(feature = "Zenject+DeclareSignalAsyncTickPriorityCopyBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::DeclareSignalAsyncTickPriorityCopyBinder => "Zenject"
    ."DeclareSignalAsyncTickPriorityCopyBinder"
);
#[cfg(feature = "Zenject+DeclareSignalAsyncTickPriorityCopyBinder")]
impl std::ops::Deref for crate::Zenject::DeclareSignalAsyncTickPriorityCopyBinder {
    type Target = crate::Zenject::SignalTickPriorityCopyBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DeclareSignalAsyncTickPriorityCopyBinder")]
impl std::ops::DerefMut for crate::Zenject::DeclareSignalAsyncTickPriorityCopyBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DeclareSignalAsyncTickPriorityCopyBinder")]
impl crate::Zenject::DeclareSignalAsyncTickPriorityCopyBinder {
    pub fn New(
        signalBindInfo: *mut crate::Zenject::SignalDeclarationBindInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signalBindInfo))?;
        Ok(__cordl_object)
    }
    pub fn RunAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::SignalTickPriorityCopyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::SignalTickPriorityCopyBinder = __cordl_object
            .invoke("RunAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn RunSync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::SignalCopyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::SignalCopyBinder = __cordl_object
            .invoke("RunSync", ())?;
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
#[cfg(feature = "Zenject+DeclareSignalAsyncTickPriorityCopyBinder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::DeclareSignalAsyncTickPriorityCopyBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
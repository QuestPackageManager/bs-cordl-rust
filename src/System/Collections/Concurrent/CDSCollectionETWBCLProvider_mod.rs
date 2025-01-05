#[cfg(feature = "System+Collections+Concurrent+CDSCollectionETWBCLProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct CDSCollectionETWBCLProvider {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Diagnostics::Tracing::EventSource,
    >,
}
#[cfg(feature = "System+Collections+Concurrent+CDSCollectionETWBCLProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Concurrent::CDSCollectionETWBCLProvider =>
    "System.Collections.Concurrent"."CDSCollectionETWBCLProvider"
);
#[cfg(feature = "System+Collections+Concurrent+CDSCollectionETWBCLProvider")]
impl std::ops::Deref
for crate::System::Collections::Concurrent::CDSCollectionETWBCLProvider {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Diagnostics::Tracing::EventSource,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Concurrent+CDSCollectionETWBCLProvider")]
impl std::ops::DerefMut
for crate::System::Collections::Concurrent::CDSCollectionETWBCLProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Concurrent+CDSCollectionETWBCLProvider")]
impl crate::System::Collections::Concurrent::CDSCollectionETWBCLProvider {
    pub fn ConcurrentBag_TryPeekSteals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConcurrentBag_TryPeekSteals", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ConcurrentBag_TryTakeSteals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConcurrentBag_TryTakeSteals", ())?;
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
}
#[cfg(feature = "System+Collections+Concurrent+CDSCollectionETWBCLProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Concurrent::CDSCollectionETWBCLProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

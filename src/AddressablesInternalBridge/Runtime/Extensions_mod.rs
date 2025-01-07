#[cfg(feature = "AddressablesInternalBridge+Runtime+Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "AddressablesInternalBridge+Runtime+Extensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::AddressablesInternalBridge::Runtime::Extensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "AddressablesInternalBridge.Runtime";
    const CLASS_NAME: &'static str = "Extensions";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "AddressablesInternalBridge+Runtime+Extensions")]
impl std::ops::Deref for crate::AddressablesInternalBridge::Runtime::Extensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AddressablesInternalBridge+Runtime+Extensions")]
impl std::ops::DerefMut for crate::AddressablesInternalBridge::Runtime::Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AddressablesInternalBridge+Runtime+Extensions")]
impl crate::AddressablesInternalBridge::Runtime::Extensions {
    pub fn GetCatalogLocationData(
        locatorId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::AddressablesInternalBridge::Runtime::CatalogLocationData,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::AddressablesInternalBridge::Runtime::CatalogLocationData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCatalogLocationData", (locatorId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUpdateableCatalogLocationDatas() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::AddressablesInternalBridge::Runtime::CatalogLocationData,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::AddressablesInternalBridge::Runtime::CatalogLocationData,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUpdateableCatalogLocationDatas", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AddressablesInternalBridge+Runtime+Extensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::AddressablesInternalBridge::Runtime::Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

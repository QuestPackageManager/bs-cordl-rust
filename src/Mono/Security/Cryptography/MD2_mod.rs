#[cfg(feature = "Mono+Security+Cryptography+MD2")]
#[repr(C)]
#[derive(Debug)]
pub struct MD2 {
    __cordl_parent: crate::System::Security::Cryptography::HashAlgorithm,
}
#[cfg(feature = "Mono+Security+Cryptography+MD2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Cryptography::MD2 =>
    "Mono.Security.Cryptography"."MD2"
);
#[cfg(feature = "Mono+Security+Cryptography+MD2")]
impl std::ops::Deref for crate::Mono::Security::Cryptography::MD2 {
    type Target = crate::System::Security::Cryptography::HashAlgorithm;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+MD2")]
impl std::ops::DerefMut for crate::Mono::Security::Cryptography::MD2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+MD2")]
impl crate::Mono::Security::Cryptography::MD2 {
    pub fn Create() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::Cryptography::MD2>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Cryptography::MD2,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Create", ())?;
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
#[cfg(feature = "Mono+Security+Cryptography+MD2")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::Cryptography::MD2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

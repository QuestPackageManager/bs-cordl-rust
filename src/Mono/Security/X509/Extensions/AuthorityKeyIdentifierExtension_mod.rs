#[cfg(feature = "Mono+Security+X509+Extensions+AuthorityKeyIdentifierExtension")]
#[repr(C)]
#[derive(Debug)]
pub struct AuthorityKeyIdentifierExtension {
    __cordl_parent: crate::Mono::Security::X509::X509Extension,
    pub aki: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Mono+Security+X509+Extensions+AuthorityKeyIdentifierExtension")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Security::X509::Extensions::AuthorityKeyIdentifierExtension =>
    "Mono.Security.X509.Extensions"."AuthorityKeyIdentifierExtension"
);
#[cfg(feature = "Mono+Security+X509+Extensions+AuthorityKeyIdentifierExtension")]
impl std::ops::Deref
for crate::Mono::Security::X509::Extensions::AuthorityKeyIdentifierExtension {
    type Target = crate::Mono::Security::X509::X509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+Extensions+AuthorityKeyIdentifierExtension")]
impl std::ops::DerefMut
for crate::Mono::Security::X509::Extensions::AuthorityKeyIdentifierExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+Extensions+AuthorityKeyIdentifierExtension")]
impl crate::Mono::Security::X509::Extensions::AuthorityKeyIdentifierExtension {
    pub fn Decode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Decode", ())?;
        Ok(__cordl_ret)
    }
    pub fn Encode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        extension: *mut crate::Mono::Security::X509::X509Extension,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (extension))?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        extension: *mut crate::Mono::Security::X509::X509Extension,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (extension))?;
        Ok(__cordl_ret)
    }
    pub fn get_Identifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_Identifier", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Security+X509+Extensions+AuthorityKeyIdentifierExtension")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::X509::Extensions::AuthorityKeyIdentifierExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

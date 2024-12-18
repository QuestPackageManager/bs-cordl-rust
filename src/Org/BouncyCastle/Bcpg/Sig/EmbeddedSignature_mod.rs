#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+EmbeddedSignature")]
#[repr(C)]
#[derive(Debug)]
pub struct EmbeddedSignature {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+EmbeddedSignature")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::Sig::EmbeddedSignature
    => "Org.BouncyCastle.Bcpg.Sig"."EmbeddedSignature"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+EmbeddedSignature")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::Sig::EmbeddedSignature {
    type Target = crate::Org::BouncyCastle::Bcpg::SignatureSubpacket;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+EmbeddedSignature")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::Sig::EmbeddedSignature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+EmbeddedSignature")]
impl crate::Org::BouncyCastle::Bcpg::Sig::EmbeddedSignature {
    pub fn New(
        critical: bool,
        isLongLength: bool,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (critical, isLongLength, data))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        critical: bool,
        isLongLength: bool,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (critical, isLongLength, data))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+EmbeddedSignature")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::Sig::EmbeddedSignature {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

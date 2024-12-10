#[cfg(feature = "Org+BouncyCastle+X509+X509KeyUsage")]
#[repr(C)]
#[derive(Debug)]
pub struct X509KeyUsage {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub usage: i32,
}
#[cfg(feature = "Org+BouncyCastle+X509+X509KeyUsage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::X509::X509KeyUsage =>
    "Org.BouncyCastle.X509"."X509KeyUsage"
);
#[cfg(feature = "Org+BouncyCastle+X509+X509KeyUsage")]
impl std::ops::Deref for crate::Org::BouncyCastle::X509::X509KeyUsage {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509KeyUsage")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::X509::X509KeyUsage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509KeyUsage")]
impl crate::Org::BouncyCastle::X509::X509KeyUsage {
    pub const CrlSign: i32 = 2i32;
    pub const DataEncipherment: i32 = 16i32;
    pub const DecipherOnly: i32 = 32768i32;
    pub const DigitalSignature: i32 = 128i32;
    pub const EncipherOnly: i32 = 1i32;
    pub const KeyAgreement: i32 = 8i32;
    pub const KeyCertSign: i32 = 4i32;
    pub const KeyEncipherment: i32 = 32i32;
    pub const NonRepudiation: i32 = 64i32;
    pub fn New(
        usage: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (usage))?;
        Ok(__cordl_object.into())
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = __cordl_object.invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        usage: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (usage))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509KeyUsage")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::X509::X509KeyUsage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

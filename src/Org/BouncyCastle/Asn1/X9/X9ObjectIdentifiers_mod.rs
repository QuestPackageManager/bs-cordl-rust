#[cfg(feature = "Org+BouncyCastle+Asn1+X9+X9ObjectIdentifiers")]
#[repr(C)]
#[derive(Debug)]
pub struct X9ObjectIdentifiers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X9+X9ObjectIdentifiers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X9::X9ObjectIdentifiers
    => "Org.BouncyCastle.Asn1.X9"."X9ObjectIdentifiers"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X9+X9ObjectIdentifiers")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X9::X9ObjectIdentifiers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X9+X9ObjectIdentifiers")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X9::X9ObjectIdentifiers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X9+X9ObjectIdentifiers")]
impl crate::Org::BouncyCastle::Asn1::X9::X9ObjectIdentifiers {
    pub const AnsiX962: &'static str = "1.2.840.10045";
    pub const IdECSigType: &'static str = "1.2.840.10045.4";
    pub const IdPublicKeyType: &'static str = "1.2.840.10045.2";
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X9+X9ObjectIdentifiers")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X9::X9ObjectIdentifiers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

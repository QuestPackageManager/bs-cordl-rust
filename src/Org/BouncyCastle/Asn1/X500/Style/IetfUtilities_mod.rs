#[cfg(feature = "Org+BouncyCastle+Asn1+X500+Style+IetfUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct IetfUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X500+Style+IetfUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::X500::Style::IetfUtilities =>
    "Org.BouncyCastle.Asn1.X500.Style"."IetfUtilities"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X500+Style+IetfUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X500::Style::IetfUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X500+Style+IetfUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X500::Style::IetfUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X500+Style+IetfUtilities")]
impl crate::Org::BouncyCastle::Asn1::X500::Style::IetfUtilities {
    pub fn AtvAreEqual(
        atv1: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X500::AttributeTypeAndValue,
        >,
        atv2: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X500::AttributeTypeAndValue,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AtvAreEqual", (atv1, atv2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CanonicalString(
        value: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Encodable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanonicalString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Canonicalize(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Canonicalize", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecodeObject(
        oValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecodeObject", (oValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RdnAreEqual(
        rdn1: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X500::Rdn>,
        rdn2: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X500::Rdn>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RdnAreEqual", (rdn1, rdn2))?;
        Ok(__cordl_ret.into())
    }
    pub fn StripInternalSpaces(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StripInternalSpaces", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValueToString(
        value: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Encodable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValueToString", (value))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Org+BouncyCastle+Asn1+X500+Style+IetfUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X500::Style::IetfUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "Org+BouncyCastle+Asn1+DerSequenceParser")]
#[repr(C)]
#[derive(Debug)]
pub struct DerSequenceParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _parser: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1StreamParser,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerSequenceParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::DerSequenceParser =>
    "Org.BouncyCastle.Asn1"."DerSequenceParser"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+DerSequenceParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::DerSequenceParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerSequenceParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::DerSequenceParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerSequenceParser")]
impl crate::Org::BouncyCastle::Asn1::DerSequenceParser {
    pub fn New(
        parser: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1StreamParser,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parser))?;
        Ok(__cordl_object.into())
    }
    pub fn ReadObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::IAsn1Convertible>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::IAsn1Convertible,
        > = __cordl_object.invoke("ReadObject", ())?;
        Ok(__cordl_ret.into())
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
        parser: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1StreamParser,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parser))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerSequenceParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::DerSequenceParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerSequenceParser")]
impl AsRef<crate::Org::BouncyCastle::Asn1::Asn1SequenceParser>
for crate::Org::BouncyCastle::Asn1::DerSequenceParser {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Asn1::Asn1SequenceParser {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerSequenceParser")]
impl AsMut<crate::Org::BouncyCastle::Asn1::Asn1SequenceParser>
for crate::Org::BouncyCastle::Asn1::DerSequenceParser {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Asn1::Asn1SequenceParser {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerSequenceParser")]
impl AsRef<crate::Org::BouncyCastle::Asn1::IAsn1Convertible>
for crate::Org::BouncyCastle::Asn1::DerSequenceParser {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Asn1::IAsn1Convertible {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerSequenceParser")]
impl AsMut<crate::Org::BouncyCastle::Asn1::IAsn1Convertible>
for crate::Org::BouncyCastle::Asn1::DerSequenceParser {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Asn1::IAsn1Convertible {
        unsafe { std::mem::transmute(self) }
    }
}

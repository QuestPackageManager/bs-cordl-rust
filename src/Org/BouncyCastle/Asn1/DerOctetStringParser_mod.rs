#[cfg(feature = "Org+BouncyCastle+Asn1+DerOctetStringParser")]
#[repr(C)]
#[derive(Debug)]
pub struct DerOctetStringParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub stream: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DefiniteLengthInputStream,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerOctetStringParser")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::DerOctetStringParser {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1";
    const CLASS_NAME: &'static str = "DerOctetStringParser";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+DerOctetStringParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::DerOctetStringParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerOctetStringParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::DerOctetStringParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerOctetStringParser")]
impl crate::Org::BouncyCastle::Asn1::DerOctetStringParser {
    pub fn GetOctetStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = __cordl_object
            .invoke("GetOctetStream", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        stream: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DefiniteLengthInputStream,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream))?;
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
        stream: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DefiniteLengthInputStream,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerOctetStringParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::DerOctetStringParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerOctetStringParser")]
impl AsRef<crate::Org::BouncyCastle::Asn1::Asn1OctetStringParser>
for crate::Org::BouncyCastle::Asn1::DerOctetStringParser {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Asn1::Asn1OctetStringParser {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerOctetStringParser")]
impl AsMut<crate::Org::BouncyCastle::Asn1::Asn1OctetStringParser>
for crate::Org::BouncyCastle::Asn1::DerOctetStringParser {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Asn1::Asn1OctetStringParser {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerOctetStringParser")]
impl AsRef<crate::Org::BouncyCastle::Asn1::IAsn1Convertible>
for crate::Org::BouncyCastle::Asn1::DerOctetStringParser {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Asn1::IAsn1Convertible {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerOctetStringParser")]
impl AsMut<crate::Org::BouncyCastle::Asn1::IAsn1Convertible>
for crate::Org::BouncyCastle::Asn1::DerOctetStringParser {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Asn1::IAsn1Convertible {
        unsafe { std::mem::transmute(self) }
    }
}

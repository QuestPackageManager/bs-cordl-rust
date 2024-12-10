#[cfg(feature = "Org+BouncyCastle+Asn1+Misc+NetscapeRevocationUrl")]
#[repr(C)]
#[derive(Debug)]
pub struct NetscapeRevocationUrl {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::DerIA5String,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Misc+NetscapeRevocationUrl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Misc::NetscapeRevocationUrl =>
    "Org.BouncyCastle.Asn1.Misc"."NetscapeRevocationUrl"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Misc+NetscapeRevocationUrl")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Misc::NetscapeRevocationUrl {
    type Target = crate::Org::BouncyCastle::Asn1::DerIA5String;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Misc+NetscapeRevocationUrl")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Misc::NetscapeRevocationUrl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Misc+NetscapeRevocationUrl")]
impl crate::Org::BouncyCastle::Asn1::Misc::NetscapeRevocationUrl {
    pub fn New(
        str: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerIA5String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (str))?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        str: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerIA5String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (str))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Misc+NetscapeRevocationUrl")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Misc::NetscapeRevocationUrl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "Org+BouncyCastle+Asn1+Misc+VerisignCzagExtension")]
#[repr(C)]
#[derive(Debug)]
pub struct VerisignCzagExtension {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerIA5String,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Misc+VerisignCzagExtension")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Misc::VerisignCzagExtension =>
    "Org.BouncyCastle.Asn1.Misc"."VerisignCzagExtension"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Misc+VerisignCzagExtension")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Misc::VerisignCzagExtension {
    type Target = quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerIA5String,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Misc+VerisignCzagExtension")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Misc::VerisignCzagExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Misc+VerisignCzagExtension")]
impl crate::Org::BouncyCastle::Asn1::Misc::VerisignCzagExtension {
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Misc+VerisignCzagExtension")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Misc::VerisignCzagExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

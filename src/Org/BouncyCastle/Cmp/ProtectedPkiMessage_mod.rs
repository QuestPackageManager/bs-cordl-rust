#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct ProtectedPkiMessage {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub pkiMessage: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage,
}
#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cmp::ProtectedPkiMessage =>
    "Org.BouncyCastle.Cmp"."ProtectedPkiMessage"
);
#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessage")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessage")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessage")]
impl crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage {
    pub fn GetCertificates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::X509::X509Certificate,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::X509::X509Certificate,
            >,
        > = __cordl_object.invoke("GetCertificates", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_GeneralPkiMessage0(
        pkiMessage: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::GeneralPkiMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pkiMessage))?;
        Ok(__cordl_object.into())
    }
    pub fn New_PkiMessage1(
        pkiMessage: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pkiMessage))?;
        Ok(__cordl_object.into())
    }
    pub fn Process(
        &mut self,
        streamCalculator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IStreamCalculator,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Process", (streamCalculator))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToAsn1Message(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage,
        > = __cordl_object.invoke("ToAsn1Message", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Verify_IVerifierFactory0(
        &mut self,
        verifierFactory: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IVerifierFactory,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Verify", (verifierFactory))?;
        Ok(__cordl_ret.into())
    }
    pub fn Verify_PKMacBuilder_Il2CppArray1(
        &mut self,
        pkMacBuilder: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crmf::PKMacBuilder,
        >,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Verify", (pkMacBuilder, password))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GeneralPkiMessage0(
        &mut self,
        pkiMessage: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::GeneralPkiMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pkiMessage))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PkiMessage1(
        &mut self,
        pkiMessage: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pkiMessage))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Body(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiBody>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiBody,
        > = __cordl_object.invoke("get_Body", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasPasswordBasedMacProtected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_HasPasswordBasedMacProtected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Header(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        > = __cordl_object.invoke("get_Header", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "Org+BouncyCastle+Asn1+Tsp+TstInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct TstInfo {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub version: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub tsaPolicyId: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub messageImprint: *mut crate::Org::BouncyCastle::Asn1::Tsp::MessageImprint,
    pub serialNumber: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub genTime: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    pub accuracy: *mut crate::Org::BouncyCastle::Asn1::Tsp::Accuracy,
    pub ordering: *mut crate::Org::BouncyCastle::Asn1::DerBoolean,
    pub nonce: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub tsa: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    pub extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Tsp+TstInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Tsp::TstInfo =>
    "Org.BouncyCastle.Asn1.Tsp"."TstInfo"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Tsp+TstInfo")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Tsp::TstInfo {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Tsp+TstInfo")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Tsp::TstInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Tsp+TstInfo")]
impl crate::Org::BouncyCastle::Asn1::Tsp::TstInfo {
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_DerObjectIdentifier_MessageImprint_DerInteger_DerGeneralizedTime_Accuracy_DerBoolean_DerInteger_GeneralName_X509Extensions1(
        tsaPolicyId: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        messageImprint: *mut crate::Org::BouncyCastle::Asn1::Tsp::MessageImprint,
        serialNumber: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        genTime: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        accuracy: *mut crate::Org::BouncyCastle::Asn1::Tsp::Accuracy,
        ordering: *mut crate::Org::BouncyCastle::Asn1::DerBoolean,
        nonce: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        tsa: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    tsaPolicyId,
                    messageImprint,
                    serialNumber,
                    genTime,
                    accuracy,
                    ordering,
                    nonce,
                    tsa,
                    extensions,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Sequence0(
        &mut self,
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerObjectIdentifier_MessageImprint_DerInteger_DerGeneralizedTime_Accuracy_DerBoolean_DerInteger_GeneralName_X509Extensions1(
        &mut self,
        tsaPolicyId: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        messageImprint: *mut crate::Org::BouncyCastle::Asn1::Tsp::MessageImprint,
        serialNumber: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        genTime: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        accuracy: *mut crate::Org::BouncyCastle::Asn1::Tsp::Accuracy,
        ordering: *mut crate::Org::BouncyCastle::Asn1::DerBoolean,
        nonce: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        tsa: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    tsaPolicyId,
                    messageImprint,
                    serialNumber,
                    genTime,
                    accuracy,
                    ordering,
                    nonce,
                    tsa,
                    extensions,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_Accuracy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Tsp::Accuracy,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Tsp::Accuracy = __cordl_object
            .invoke("get_Accuracy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Extensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions = __cordl_object
            .invoke("get_Extensions", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GenTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime = __cordl_object
            .invoke("get_GenTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MessageImprint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Tsp::MessageImprint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Tsp::MessageImprint = __cordl_object
            .invoke("get_MessageImprint", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Nonce(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::DerInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerInteger = __cordl_object
            .invoke("get_Nonce", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Ordering(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::DerBoolean> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerBoolean = __cordl_object
            .invoke("get_Ordering", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Policy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier = __cordl_object
            .invoke("get_Policy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SerialNumber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::DerInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerInteger = __cordl_object
            .invoke("get_SerialNumber", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Tsa(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName = __cordl_object
            .invoke("get_Tsa", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::DerInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerInteger = __cordl_object
            .invoke("get_Version", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Tsp+TstInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Asn1::Tsp::TstInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

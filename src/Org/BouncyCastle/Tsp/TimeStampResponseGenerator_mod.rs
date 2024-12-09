#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeStampResponseGenerator {
    __cordl_parent: crate::System::Object,
    pub status: crate::Org::BouncyCastle::Asn1::Cmp::PkiStatus,
    pub statusStrings: *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
    pub failInfo: i32,
    pub tokenGenerator: *mut crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator,
    pub acceptedAlgorithms: *mut crate::System::Collections::IList,
    pub acceptedPolicies: *mut crate::System::Collections::IList,
    pub acceptedExtensions: *mut crate::System::Collections::IList,
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Tsp::TimeStampResponseGenerator => "Org.BouncyCastle.Tsp"
    ."TimeStampResponseGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator")]
impl crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator {
    #[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator+FailInfo")]
    pub type FailInfo = crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator_FailInfo;
    pub fn AddStatusString(
        &mut self,
        statusString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddStatusString", (statusString))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateFailResponse(
        &mut self,
        status: crate::Org::BouncyCastle::Asn1::Cmp::PkiStatus,
        failInfoField: i32,
        statusString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Tsp::TimeStampResponse,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Tsp::TimeStampResponse = __cordl_object
            .invoke("GenerateFailResponse", (status, failInfoField, statusString))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_DateTime0(
        &mut self,
        request: *mut crate::Org::BouncyCastle::Tsp::TimeStampRequest,
        serialNumber: *mut crate::Org::BouncyCastle::Math::BigInteger,
        genTime: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Tsp::TimeStampResponse,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Tsp::TimeStampResponse = __cordl_object
            .invoke("Generate", (request, serialNumber, genTime))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_DateTimeObject1(
        &mut self,
        request: *mut crate::Org::BouncyCastle::Tsp::TimeStampRequest,
        serialNumber: *mut crate::Org::BouncyCastle::Math::BigInteger,
        genTime: *mut crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Tsp::TimeStampResponse,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Tsp::TimeStampResponse = __cordl_object
            .invoke("Generate", (request, serialNumber, genTime))?;
        Ok(__cordl_ret)
    }
    pub fn GetPkiStatusInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo = __cordl_object
            .invoke("GetPkiStatusInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_IList1(
        tokenGenerator: *mut crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator,
        acceptedAlgorithms: *mut crate::System::Collections::IList,
        acceptedPolicy: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tokenGenerator, acceptedAlgorithms, acceptedPolicy))?;
        Ok(__cordl_object)
    }
    pub fn New_IList_IList2(
        tokenGenerator: *mut crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator,
        acceptedAlgorithms: *mut crate::System::Collections::IList,
        acceptedPolicies: *mut crate::System::Collections::IList,
        acceptedExtensions: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    tokenGenerator,
                    acceptedAlgorithms,
                    acceptedPolicies,
                    acceptedExtensions,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_TimeStampTokenGenerator_IList0(
        tokenGenerator: *mut crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator,
        acceptedAlgorithms: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tokenGenerator, acceptedAlgorithms))?;
        Ok(__cordl_object)
    }
    pub fn SetFailInfoField(
        &mut self,
        field: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFailInfoField", (field))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IList1(
        &mut self,
        tokenGenerator: *mut crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator,
        acceptedAlgorithms: *mut crate::System::Collections::IList,
        acceptedPolicy: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tokenGenerator, acceptedAlgorithms, acceptedPolicy))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IList_IList2(
        &mut self,
        tokenGenerator: *mut crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator,
        acceptedAlgorithms: *mut crate::System::Collections::IList,
        acceptedPolicies: *mut crate::System::Collections::IList,
        acceptedExtensions: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    tokenGenerator,
                    acceptedAlgorithms,
                    acceptedPolicies,
                    acceptedExtensions,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TimeStampTokenGenerator_IList0(
        &mut self,
        tokenGenerator: *mut crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator,
        acceptedAlgorithms: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tokenGenerator, acceptedAlgorithms))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator+FailInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeStampResponseGenerator_FailInfo {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::DerBitString,
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator+FailInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Tsp::TimeStampResponseGenerator_FailInfo =>
    "Org.BouncyCastle.Tsp"."TimeStampResponseGenerator/FailInfo"
);
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator+FailInfo")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator_FailInfo {
    type Target = crate::Org::BouncyCastle::Asn1::DerBitString;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator+FailInfo")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator_FailInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator+FailInfo")]
impl crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator_FailInfo {
    pub fn New(failInfoValue: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (failInfoValue))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        failInfoValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (failInfoValue))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampResponseGenerator+FailInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Tsp::TimeStampResponseGenerator_FailInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

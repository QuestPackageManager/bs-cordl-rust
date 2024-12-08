#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PbmParameter")]
#[repr(C)]
#[derive(Debug)]
pub struct PbmParameter {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub salt: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    pub owf: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub iterationCount: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub mac: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PbmParameter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::PbmParameter =>
    "Org.BouncyCastle.Asn1.Cmp"."PbmParameter"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PbmParameter")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::PbmParameter {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PbmParameter")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::PbmParameter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PbmParameter")]
impl crate::Org::BouncyCastle::Asn1::Cmp::PbmParameter {
    pub fn get_Salt(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("get_Salt", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Mac(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_Mac", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Owf(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_Owf", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IterationCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::DerInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerInteger = __cordl_object
            .invoke("get_IterationCount", ())?;
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
    pub fn _ctor_Il2CppArray_AlgorithmIdentifier_i32_AlgorithmIdentifier1(
        &mut self,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        owf: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        iterationCount: i32,
        mac: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (salt, owf, iterationCount, mac))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1OctetString_AlgorithmIdentifier_DerInteger_AlgorithmIdentifier2(
        &mut self,
        salt: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        owf: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        iterationCount: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        mac: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (salt, owf, iterationCount, mac))?;
        Ok(__cordl_ret)
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
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_AlgorithmIdentifier_i32_AlgorithmIdentifier1(
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        owf: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        iterationCount: i32,
        mac: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (salt, owf, iterationCount, mac))?;
        Ok(__cordl_object)
    }
    pub fn New_Asn1OctetString_AlgorithmIdentifier_DerInteger_AlgorithmIdentifier2(
        salt: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        owf: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        iterationCount: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        mac: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (salt, owf, iterationCount, mac))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PbmParameter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::PbmParameter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

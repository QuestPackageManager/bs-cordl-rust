#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+ProfessionInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ProfessionInfo {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub namingAuthority: *mut crate::Org::BouncyCastle::Asn1::IsisMtt::X509::NamingAuthority,
    pub professionItems: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    pub professionOids: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    pub registrationNumber: *mut crate::System::String,
    pub addProfessionInfo: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+ProfessionInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::IsisMtt::X509::ProfessionInfo =>
    "Org.BouncyCastle.Asn1.IsisMtt.X509"."ProfessionInfo"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+ProfessionInfo")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::IsisMtt::X509::ProfessionInfo {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+ProfessionInfo")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::IsisMtt::X509::ProfessionInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+ProfessionInfo")]
impl crate::Org::BouncyCastle::Asn1::IsisMtt::X509::ProfessionInfo {
    pub fn get_NamingAuthority(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::IsisMtt::X509::NamingAuthority,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::IsisMtt::X509::NamingAuthority = __cordl_object
            .invoke("get_NamingAuthority", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetProfessionItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        > = __cordl_object.invoke("GetProfessionItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AddProfessionInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("get_AddProfessionInfo", ())?;
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
    pub fn _ctor_NamingAuthority_Il2CppArray_Il2CppArray_String_Asn1OctetString1(
        &mut self,
        namingAuthority: *mut crate::Org::BouncyCastle::Asn1::IsisMtt::X509::NamingAuthority,
        professionItems: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        >,
        professionOids: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        registrationNumber: *mut crate::System::String,
        addProfessionInfo: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    namingAuthority,
                    professionItems,
                    professionOids,
                    registrationNumber,
                    addProfessionInfo,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetProfessionOids(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        > = __cordl_object.invoke("GetProfessionOids", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RegistrationNumber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_RegistrationNumber", ())?;
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
    pub fn New_NamingAuthority_Il2CppArray_Il2CppArray_String_Asn1OctetString1(
        namingAuthority: *mut crate::Org::BouncyCastle::Asn1::IsisMtt::X509::NamingAuthority,
        professionItems: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        >,
        professionOids: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        registrationNumber: *mut crate::System::String,
        addProfessionInfo: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    namingAuthority,
                    professionItems,
                    professionOids,
                    registrationNumber,
                    addProfessionInfo,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+ProfessionInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::IsisMtt::X509::ProfessionInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

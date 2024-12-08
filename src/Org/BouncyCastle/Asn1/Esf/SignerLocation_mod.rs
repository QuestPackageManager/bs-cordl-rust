#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignerLocation")]
#[repr(C)]
#[derive(Debug)]
pub struct SignerLocation {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub countryName: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
    pub localityName: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
    pub postalAddress: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignerLocation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Esf::SignerLocation =>
    "Org.BouncyCastle.Asn1.Esf"."SignerLocation"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignerLocation")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Esf::SignerLocation {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignerLocation")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Esf::SignerLocation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignerLocation")]
impl crate::Org::BouncyCastle::Asn1::Esf::SignerLocation {
    pub fn GetPostal(
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
        > = __cordl_object.invoke("GetPostal", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_DerUtf8String_DerUtf8String_Asn1Sequence3(
        countryName: *mut crate::Org::BouncyCastle::Asn1::DerUtf8String,
        localityName: *mut crate::Org::BouncyCastle::Asn1::DerUtf8String,
        postalAddress: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (countryName, localityName, postalAddress))?;
        Ok(__cordl_object)
    }
    pub fn New_DirectoryString_DirectoryString_Asn1Sequence1(
        countryName: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        localityName: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        postalAddress: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (countryName, localityName, postalAddress))?;
        Ok(__cordl_object)
    }
    pub fn New_DirectoryString_DirectoryString_Il2CppArray2(
        countryName: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        localityName: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        postalAddress: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (countryName, localityName, postalAddress))?;
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
    pub fn _ctor_DerUtf8String_DerUtf8String_Asn1Sequence3(
        &mut self,
        countryName: *mut crate::Org::BouncyCastle::Asn1::DerUtf8String,
        localityName: *mut crate::Org::BouncyCastle::Asn1::DerUtf8String,
        postalAddress: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (countryName, localityName, postalAddress))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DirectoryString_DirectoryString_Asn1Sequence1(
        &mut self,
        countryName: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        localityName: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        postalAddress: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (countryName, localityName, postalAddress))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DirectoryString_DirectoryString_Il2CppArray2(
        &mut self,
        countryName: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        localityName: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        postalAddress: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (countryName, localityName, postalAddress))?;
        Ok(__cordl_ret)
    }
    pub fn get_Country(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString = __cordl_object
            .invoke("get_Country", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CountryName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerUtf8String,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerUtf8String = __cordl_object
            .invoke("get_CountryName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Locality(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString = __cordl_object
            .invoke("get_Locality", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalityName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerUtf8String,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerUtf8String = __cordl_object
            .invoke("get_LocalityName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PostalAddress(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence = __cordl_object
            .invoke("get_PostalAddress", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignerLocation")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Esf::SignerLocation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
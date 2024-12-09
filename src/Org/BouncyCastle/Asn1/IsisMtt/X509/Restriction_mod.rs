#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+Restriction")]
#[repr(C)]
#[derive(Debug)]
pub struct Restriction {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub restriction: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+Restriction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::IsisMtt::X509::Restriction =>
    "Org.BouncyCastle.Asn1.IsisMtt.X509"."Restriction"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+Restriction")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::IsisMtt::X509::Restriction {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+Restriction")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::IsisMtt::X509::Restriction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+Restriction")]
impl crate::Org::BouncyCastle::Asn1::IsisMtt::X509::Restriction {
    pub fn New_DirectoryString0(
        restriction: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (restriction))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppString1(
        restriction: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (restriction))?;
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
    pub fn _ctor_DirectoryString0(
        &mut self,
        restriction: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (restriction))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        restriction: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (restriction))?;
        Ok(__cordl_ret)
    }
    pub fn get_RestrictionString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString = __cordl_object
            .invoke("get_RestrictionString", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+Restriction")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::IsisMtt::X509::Restriction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

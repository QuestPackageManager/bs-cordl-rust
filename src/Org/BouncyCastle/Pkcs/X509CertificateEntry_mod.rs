#[cfg(feature = "Org+BouncyCastle+Pkcs+X509CertificateEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct X509CertificateEntry {
    __cordl_parent: crate::Org::BouncyCastle::Pkcs::Pkcs12Entry,
    pub cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+X509CertificateEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Pkcs::X509CertificateEntry =>
    "Org.BouncyCastle.Pkcs"."X509CertificateEntry"
);
#[cfg(feature = "Org+BouncyCastle+Pkcs+X509CertificateEntry")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkcs::X509CertificateEntry {
    type Target = crate::Org::BouncyCastle::Pkcs::Pkcs12Entry;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+X509CertificateEntry")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkcs::X509CertificateEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+X509CertificateEntry")]
impl crate::Org::BouncyCastle::Pkcs::X509CertificateEntry {
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_X509Certificate0(
        &mut self,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cert))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Hashtable1(
        &mut self,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        attributes: *mut crate::System::Collections::Hashtable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cert, attributes))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IDictionary2(
        &mut self,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        attributes: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cert, attributes))?;
        Ok(__cordl_ret)
    }
    pub fn get_Certificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509Certificate = __cordl_object
            .invoke("get_Certificate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_X509Certificate0(
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cert))?;
        Ok(__cordl_object)
    }
    pub fn New_Hashtable1(
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        attributes: *mut crate::System::Collections::Hashtable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cert, attributes))?;
        Ok(__cordl_object)
    }
    pub fn New_IDictionary2(
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        attributes: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cert, attributes))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+X509CertificateEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkcs::X509CertificateEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Collection"
)]
#[repr(C)]
#[derive(Debug)]
pub struct X509Certificate2Collection {
    __cordl_parent: crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Collection"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509Certificate2Collection =>
    "System.Security.Cryptography.X509Certificates"."X509Certificate2Collection"
);
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Collection"
)]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection {
    type Target = crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Collection"
)]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Collection"
)]
impl crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection {
    pub fn Add(
        &mut self,
        certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Add", (certificate))?;
        Ok(__cordl_ret)
    }
    pub fn AddRange(
        &mut self,
        certificates: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddRange", (certificates))?;
        Ok(__cordl_ret)
    }
    pub fn Find(
        &mut self,
        findType: crate::System::Security::Cryptography::X509Certificates::X509FindType,
        findValue: *mut crate::System::Object,
        validOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection = __cordl_object
            .invoke("Find", (findType, findValue, validOnly))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Enumerator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Enumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2 = __cordl_object
            .invoke("get_Item", (index))?;
        Ok(__cordl_ret)
    }
    pub fn Contains(
        &mut self,
        certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (certificate))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_X509Certificate2Collection1(
        &mut self,
        certificates: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certificates))?;
        Ok(__cordl_ret)
    }
    pub fn GetKeyIdentifier(
        &mut self,
        x: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetKeyIdentifier", (x))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_X509Certificate2Collection1(
        certificates: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certificates))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Collection"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

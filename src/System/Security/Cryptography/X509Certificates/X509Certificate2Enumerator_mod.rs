#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Enumerator"
)]
#[repr(C)]
#[derive(Debug)]
pub struct X509Certificate2Enumerator {
    __cordl_parent: crate::System::Object,
    pub enumerator: *mut crate::System::Collections::IEnumerator,
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Enumerator"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509Certificate2Enumerator =>
    "System.Security.Cryptography.X509Certificates"."X509Certificate2Enumerator"
);
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Enumerator"
)]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509Certificate2Enumerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Enumerator"
)]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509Certificate2Enumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Enumerator"
)]
impl crate::System::Security::Cryptography::X509Certificates::X509Certificate2Enumerator {
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        collection: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (collection))?;
        Ok(__cordl_object)
    }
    pub fn System_Collections_IEnumerator_MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.IEnumerator.MoveNext", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerator_Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IEnumerator.Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("System.Collections.IEnumerator.get_Current", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        collection: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (collection))?;
        Ok(__cordl_ret)
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2 = __cordl_object
            .invoke("get_Current", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2Enumerator"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509Certificate2Enumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

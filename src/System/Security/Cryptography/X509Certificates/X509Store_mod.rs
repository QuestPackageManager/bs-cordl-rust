#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Store")]
#[repr(C)]
#[derive(Debug)]
pub struct X509Store {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _name: *mut quest_hook::libil2cpp::Il2CppString,
    pub _location: crate::System::Security::Cryptography::X509Certificates::StoreLocation,
    pub list: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    pub _flags: crate::System::Security::Cryptography::X509Certificates::OpenFlags,
    pub store: *mut crate::Mono::Security::X509::X509Store,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Store")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509Store =>
    "System.Security.Cryptography.X509Certificates"."X509Store"
);
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Store")]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509Store {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Store")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509Store {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Store")]
impl crate::System::Security::Cryptography::X509Certificates::X509Store {
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        storeName: crate::System::Security::Cryptography::X509Certificates::StoreName,
        storeLocation: crate::System::Security::Cryptography::X509Certificates::StoreLocation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (storeName, storeLocation))?;
        Ok(__cordl_object.into())
    }
    pub fn Open(
        &mut self,
        flags: crate::System::Security::Cryptography::X509Certificates::OpenFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Open", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        storeName: crate::System::Security::Cryptography::X509Certificates::StoreName,
        storeLocation: crate::System::Security::Cryptography::X509Certificates::StoreLocation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (storeName, storeLocation))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Certificates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        > = __cordl_object.invoke("get_Certificates", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Factory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Stores>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Stores,
        > = __cordl_object.invoke("get_Factory", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Store(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Store>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Store,
        > = __cordl_object.invoke("get_Store", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Store")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509Store {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

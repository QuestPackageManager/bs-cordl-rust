#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainElement")]
#[repr(C)]
#[derive(Debug)]
pub struct X509ChainElement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    pub status: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Security::Cryptography::X509Certificates::X509ChainStatus,
    >,
    pub info: *mut quest_hook::libil2cpp::Il2CppString,
    pub compressed_status_flags: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509ChainElement =>
    "System.Security.Cryptography.X509Certificates"."X509ChainElement"
);
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainElement")]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509ChainElement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainElement")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509ChainElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainElement")]
impl crate::System::Security::Cryptography::X509Certificates::X509ChainElement {
    pub fn Count(
        &mut self,
        flags: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Count", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certificate))?;
        Ok(__cordl_object.into())
    }
    pub fn Set(
        &mut self,
        status: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Security::Cryptography::X509Certificates::X509ChainStatus,
            >,
        >,
        position: quest_hook::libil2cpp::ByRefMut<i32>,
        flags: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
        mask: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (status, position, flags, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn UncompressFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UncompressFlags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Certificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        > = __cordl_object.invoke("get_Certificate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ChainElementStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Security::Cryptography::X509Certificates::X509ChainStatus,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Security::Cryptography::X509Certificates::X509ChainStatus,
            >,
        > = __cordl_object.invoke("get_ChainElementStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StatusFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags = __cordl_object
            .invoke("get_StatusFlags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_StatusFlags(
        &mut self,
        value: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_StatusFlags", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509ChainElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

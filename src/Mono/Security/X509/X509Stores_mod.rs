#[cfg(feature = "Mono+Security+X509+X509Stores")]
#[repr(C)]
#[derive(Debug)]
pub struct X509Stores {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _storePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _newFormat: bool,
    pub _trusted: quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Store>,
}
#[cfg(feature = "Mono+Security+X509+X509Stores")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::X509::X509Stores =>
    "Mono.Security.X509"."X509Stores"
);
#[cfg(feature = "Mono+Security+X509+X509Stores")]
impl std::ops::Deref for crate::Mono::Security::X509::X509Stores {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X509Stores")]
impl std::ops::DerefMut for crate::Mono::Security::X509::X509Stores {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X509Stores")]
impl crate::Mono::Security::X509::X509Stores {
    pub fn New(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newFormat: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path, newFormat))?;
        Ok(__cordl_object.into())
    }
    pub fn Open(
        &mut self,
        storeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        create: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Store>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Store,
        > = __cordl_object.invoke("Open", (storeName, create))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newFormat: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (path, newFormat))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TrustedRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Store>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Store,
        > = __cordl_object.invoke("get_TrustedRoot", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Security+X509+X509Stores")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::X509::X509Stores {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

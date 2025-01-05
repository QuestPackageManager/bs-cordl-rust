#[cfg(feature = "System+Security+Cryptography+OidEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct OidEnumerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _oids: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::OidCollection,
    >,
    pub _current: i32,
}
#[cfg(feature = "System+Security+Cryptography+OidEnumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::OidEnumerator =>
    "System.Security.Cryptography"."OidEnumerator"
);
#[cfg(feature = "System+Security+Cryptography+OidEnumerator")]
impl std::ops::Deref for crate::System::Security::Cryptography::OidEnumerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+OidEnumerator")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::OidEnumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+OidEnumerator")]
impl crate::System::Security::Cryptography::OidEnumerator {
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        oids: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::OidCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oids))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("System.Collections.IEnumerator.get_Current", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        oids: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::OidCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oids))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::Oid>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::Oid,
        > = __cordl_object.invoke("get_Current", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+OidEnumerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::OidEnumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Security+Cryptography+OidEnumerator")]
impl AsRef<crate::System::Collections::IEnumerator>
for crate::System::Security::Cryptography::OidEnumerator {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Cryptography+OidEnumerator")]
impl AsMut<crate::System::Collections::IEnumerator>
for crate::System::Security::Cryptography::OidEnumerator {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}

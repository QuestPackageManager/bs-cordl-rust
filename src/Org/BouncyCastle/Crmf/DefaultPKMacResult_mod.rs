#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacResult")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultPKMacResult {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mac: *mut crate::Org::BouncyCastle::Crypto::IMac,
}
#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crmf::DefaultPKMacResult =>
    "Org.BouncyCastle.Crmf"."DefaultPKMacResult"
);
#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacResult")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crmf::DefaultPKMacResult {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacResult")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crmf::DefaultPKMacResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacResult")]
impl crate::Org::BouncyCastle::Crmf::DefaultPKMacResult {
    pub fn Collect_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("Collect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Collect_Il2CppArray_i32_1(
        &mut self,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Collect", (sig, sigOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        mac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mac))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        mac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (mac))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacResult")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crmf::DefaultPKMacResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacResult")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IBlockResult>
for crate::Org::BouncyCastle::Crmf::DefaultPKMacResult {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IBlockResult {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacResult")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IBlockResult>
for crate::Org::BouncyCastle::Crmf::DefaultPKMacResult {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IBlockResult {
        unsafe { std::mem::transmute(self) }
    }
}

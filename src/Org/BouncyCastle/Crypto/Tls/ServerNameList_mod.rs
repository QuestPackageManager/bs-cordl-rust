#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerNameList")]
#[repr(C)]
#[derive(Debug)]
pub struct ServerNameList {
    __cordl_parent: crate::System::Object,
    pub mServerNameList: *mut crate::System::Collections::IList,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerNameList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::ServerNameList
    => "Org.BouncyCastle.Crypto.Tls"."ServerNameList"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerNameList")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::ServerNameList {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerNameList")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::ServerNameList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerNameList")]
impl crate::Org::BouncyCastle::Crypto::Tls::ServerNameList {
    pub fn Encode(
        &mut self,
        output: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (output))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        serverNameList: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serverNameList))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        serverNameList: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (serverNameList))?;
        Ok(__cordl_ret)
    }
    pub fn get_ServerNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("get_ServerNames", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerNameList")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::ServerNameList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

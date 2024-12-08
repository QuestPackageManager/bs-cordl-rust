#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsSrpGroupVerifier")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultTlsSrpGroupVerifier {
    __cordl_parent: crate::System::Object,
    pub mGroups: *mut crate::System::Collections::IList,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsSrpGroupVerifier")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::DefaultTlsSrpGroupVerifier =>
    "Org.BouncyCastle.Crypto.Tls"."DefaultTlsSrpGroupVerifier"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsSrpGroupVerifier")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsSrpGroupVerifier {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsSrpGroupVerifier")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsSrpGroupVerifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsSrpGroupVerifier")]
impl crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsSrpGroupVerifier {
    pub fn Accept(
        &mut self,
        group: *mut crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Accept", (group))?;
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
    pub fn _ctor_IList1(
        &mut self,
        groups: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (groups))?;
        Ok(__cordl_ret)
    }
    pub fn AreParametersEqual(
        &mut self,
        a: *mut crate::Org::BouncyCastle::Math::BigInteger,
        b: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AreParametersEqual", (a, b))?;
        Ok(__cordl_ret)
    }
    pub fn AreGroupsEqual(
        &mut self,
        a: *mut crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
        b: *mut crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AreGroupsEqual", (a, b))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_IList1(
        groups: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (groups))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsSrpGroupVerifier")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsSrpGroupVerifier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

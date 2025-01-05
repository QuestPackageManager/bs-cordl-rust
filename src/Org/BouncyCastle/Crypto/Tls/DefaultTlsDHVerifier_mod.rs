#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsDHVerifier")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultTlsDHVerifier {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mGroups: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub mMinimumPrimeBits: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsDHVerifier")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::DefaultTlsDHVerifier =>
    "Org.BouncyCastle.Crypto.Tls"."DefaultTlsDHVerifier"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsDHVerifier")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsDHVerifier {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsDHVerifier")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsDHVerifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsDHVerifier")]
impl crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsDHVerifier {
    pub fn Accept(
        &mut self,
        dhParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Accept", (dhParameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddDefaultGroup(
        dhParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddDefaultGroup", (dhParameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn AreGroupsEqual(
        &mut self,
        a: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
        >,
        b: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AreGroupsEqual", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn AreParametersEqual(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        b: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AreParametersEqual", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckGroup(
        &mut self,
        dhParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckGroup", (dhParameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckMinimumPrimeBits(
        &mut self,
        dhParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckMinimumPrimeBits", (dhParameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_IList_i32_2(
        groups: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        minimumPrimeBits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (groups, minimumPrimeBits))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_1(
        minimumPrimeBits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (minimumPrimeBits))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IList_i32_2(
        &mut self,
        groups: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        minimumPrimeBits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (groups, minimumPrimeBits))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        minimumPrimeBits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (minimumPrimeBits))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MinimumPrimeBits(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MinimumPrimeBits", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsDHVerifier")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsDHVerifier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsDHVerifier")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Tls::TlsDHVerifier>
for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsDHVerifier {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Tls::TlsDHVerifier {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsDHVerifier")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Tls::TlsDHVerifier>
for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsDHVerifier {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::Tls::TlsDHVerifier {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierResult")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultVerifierResult {
    __cordl_parent: crate::System::Object,
    pub mSigner: *mut crate::Org::BouncyCastle::Crypto::ISigner,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Operators::DefaultVerifierResult =>
    "Org.BouncyCastle.Crypto.Operators"."DefaultVerifierResult"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierResult")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Operators::DefaultVerifierResult {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierResult")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Operators::DefaultVerifierResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierResult")]
impl crate::Org::BouncyCastle::Crypto::Operators::DefaultVerifierResult {
    pub fn IsVerified_Il2CppArray0(
        &mut self,
        signature: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsVerified", (signature))?;
        Ok(__cordl_ret)
    }
    pub fn IsVerified_i32_i32_1(
        &mut self,
        sig: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        sigOff: i32,
        sigLen: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsVerified", (sig, sigOff, sigLen))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        signer: *mut crate::Org::BouncyCastle::Crypto::ISigner,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signer))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        signer: *mut crate::Org::BouncyCastle::Crypto::ISigner,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signer))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierResult")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::DefaultVerifierResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

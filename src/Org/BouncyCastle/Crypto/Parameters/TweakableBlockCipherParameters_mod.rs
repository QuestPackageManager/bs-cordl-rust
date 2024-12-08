#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+TweakableBlockCipherParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct TweakableBlockCipherParameters {
    __cordl_parent: crate::System::Object,
    pub tweak: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub key: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+TweakableBlockCipherParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::TweakableBlockCipherParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."TweakableBlockCipherParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+TweakableBlockCipherParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::TweakableBlockCipherParameters {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+TweakableBlockCipherParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::TweakableBlockCipherParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+TweakableBlockCipherParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::TweakableBlockCipherParameters {
    pub fn New(
        key: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        tweak: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, tweak))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        key: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        tweak: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key, tweak))?;
        Ok(__cordl_ret)
    }
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter = __cordl_object
            .invoke("get_Key", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Tweak(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_Tweak", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+TweakableBlockCipherParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::TweakableBlockCipherParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
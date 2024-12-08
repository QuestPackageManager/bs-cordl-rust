#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+Iso18033KdfParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct Iso18033KdfParameters {
    __cordl_parent: crate::System::Object,
    pub seed: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+Iso18033KdfParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::Iso18033KdfParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."Iso18033KdfParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+Iso18033KdfParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::Iso18033KdfParameters {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+Iso18033KdfParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::Iso18033KdfParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+Iso18033KdfParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::Iso18033KdfParameters {
    pub fn _ctor(
        &mut self,
        seed: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seed))?;
        Ok(__cordl_ret)
    }
    pub fn GetSeed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetSeed", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        seed: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seed))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+Iso18033KdfParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::Iso18033KdfParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

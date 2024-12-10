#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+VmpcRandomGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct VmpcRandomGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub n: u8,
    pub P: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub s: u8,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+VmpcRandomGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Prng::VmpcRandomGenerator =>
    "Org.BouncyCastle.Crypto.Prng"."VmpcRandomGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+VmpcRandomGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Prng::VmpcRandomGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+VmpcRandomGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Prng::VmpcRandomGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+VmpcRandomGenerator")]
impl crate::Org::BouncyCastle::Crypto::Prng::VmpcRandomGenerator {
    pub fn AddSeedMaterial_Il2CppArray0(
        &mut self,
        seed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSeedMaterial", (seed))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSeedMaterial_i64_1(
        &mut self,
        seed: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSeedMaterial", (seed))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NextBytes_Il2CppArray0(
        &mut self,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NextBytes", (bytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn NextBytes_i32_i32_1(
        &mut self,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        start: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NextBytes", (bytes, start, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+VmpcRandomGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::VmpcRandomGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+VmpcRandomGenerator")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Prng::IRandomGenerator>
for crate::Org::BouncyCastle::Crypto::Prng::VmpcRandomGenerator {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Prng::IRandomGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+VmpcRandomGenerator")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Prng::IRandomGenerator>
for crate::Org::BouncyCastle::Crypto::Prng::VmpcRandomGenerator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::Prng::IRandomGenerator {
        unsafe { std::mem::transmute(self) }
    }
}

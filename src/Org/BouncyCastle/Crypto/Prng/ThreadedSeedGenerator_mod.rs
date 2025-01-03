#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+ThreadedSeedGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreadedSeedGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+ThreadedSeedGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Prng::ThreadedSeedGenerator =>
    "Org.BouncyCastle.Crypto.Prng"."ThreadedSeedGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+ThreadedSeedGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Prng::ThreadedSeedGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+ThreadedSeedGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Prng::ThreadedSeedGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+ThreadedSeedGenerator")]
impl crate::Org::BouncyCastle::Crypto::Prng::ThreadedSeedGenerator {
    #[cfg(feature = "Org+BouncyCastle+Crypto+Prng+ThreadedSeedGenerator+SeedGenerator")]
    pub type SeedGenerator = crate::Org::BouncyCastle::Crypto::Prng::ThreadedSeedGenerator_SeedGenerator;
    pub fn GenerateSeed(
        &mut self,
        numBytes: i32,
        fast: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GenerateSeed", (numBytes, fast))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+ThreadedSeedGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::ThreadedSeedGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+ThreadedSeedGenerator+SeedGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreadedSeedGenerator_SeedGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub counter: i32,
    pub stop: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+ThreadedSeedGenerator+SeedGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Prng::ThreadedSeedGenerator_SeedGenerator =>
    "Org.BouncyCastle.Crypto.Prng"."ThreadedSeedGenerator/SeedGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+ThreadedSeedGenerator+SeedGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Prng::ThreadedSeedGenerator_SeedGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+ThreadedSeedGenerator+SeedGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Prng::ThreadedSeedGenerator_SeedGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+ThreadedSeedGenerator+SeedGenerator")]
impl crate::Org::BouncyCastle::Crypto::Prng::ThreadedSeedGenerator_SeedGenerator {
    pub fn DoGenerateSeed(
        &mut self,
        numBytes: i32,
        fast: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("DoGenerateSeed", (numBytes, fast))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateSeed(
        &mut self,
        numBytes: i32,
        fast: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GenerateSeed", (numBytes, fast))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Run(
        &mut self,
        ignored: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Run", (ignored))?;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+ThreadedSeedGenerator+SeedGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::ThreadedSeedGenerator_SeedGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "BloomFilterUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomFilterUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BloomFilterUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BloomFilterUtil => ""
    ."BloomFilterUtil"
);
#[cfg(feature = "BloomFilterUtil")]
impl std::ops::Deref for crate::GlobalNamespace::BloomFilterUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFilterUtil")]
impl std::ops::DerefMut for crate::GlobalNamespace::BloomFilterUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFilterUtil")]
impl crate::GlobalNamespace::BloomFilterUtil {
    #[cfg(feature = "BloomFilterUtil+__c__DisplayClass1_0_1")]
    pub type __c__DisplayClass1_0_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::BloomFilterUtil___c__DisplayClass1_0_1<
        T,
    >;
    #[cfg(feature = "BloomFilterUtil+__c__DisplayClass2_0_1")]
    pub type __c__DisplayClass2_0_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::BloomFilterUtil___c__DisplayClass2_0_1<
        T,
    >;
    pub fn AddBloomFilterEntry<T>(
        bitMask: T,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hashCount: i32,
        hashBits: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddBloomFilterEntry", (bitMask, value, hashCount, hashBits))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddBloomFilterEntryHash<T>(
        bitMask: T,
        hash: u32,
        hashCount: i32,
        hashBits: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddBloomFilterEntryHash", (bitMask, hash, hashCount, hashBits))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsBloomFilterEntry<T>(
        bitMask: T,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hashCount: i32,
        hashBits: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsBloomFilterEntry", (bitMask, value, hashCount, hashBits))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsBloomFilterEntryHash<T>(
        bitMask: T,
        hash: u32,
        hashCount: i32,
        hashBits: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ContainsBloomFilterEntryHash",
                (bitMask, hash, hashCount, hashBits),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBloomFilter_IEnumerable_1_1<T>(
        strings: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
        hashCount: i32,
        hashBits: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBloomFilter", (strings, hashCount, hashBits))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBloomFilter_IEnumerable_1_2<T>(
        hashes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<u32>,
        >,
        hashCount: i32,
        hashBits: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBloomFilter", (hashes, hashCount, hashBits))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBloomFilter_Il2CppString0<T>(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hashCount: i32,
        hashBits: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBloomFilter", (value, hashCount, hashBits))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BloomFilterUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BloomFilterUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

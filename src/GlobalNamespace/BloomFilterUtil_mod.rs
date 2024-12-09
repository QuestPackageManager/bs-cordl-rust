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

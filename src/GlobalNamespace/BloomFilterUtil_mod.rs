#[cfg(feature = "BloomFilterUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomFilterUtil {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BloomFilterUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BloomFilterUtil => ""."BloomFilterUtil"
);
#[cfg(feature = "BloomFilterUtil")]
impl std::ops::Deref for BloomFilterUtil {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFilterUtil")]
impl std::ops::DerefMut for BloomFilterUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFilterUtil")]
impl BloomFilterUtil {
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
impl quest_hook::libil2cpp::ObjectType for BloomFilterUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

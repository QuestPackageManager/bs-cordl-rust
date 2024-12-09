#[cfg(feature = "Mono+Globalization+Unicode+MSCompatUnicodeTable")]
#[repr(C)]
#[derive(Debug)]
pub struct MSCompatUnicodeTable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Globalization+Unicode+MSCompatUnicodeTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Globalization::Unicode::MSCompatUnicodeTable => "Mono.Globalization.Unicode"
    ."MSCompatUnicodeTable"
);
#[cfg(feature = "Mono+Globalization+Unicode+MSCompatUnicodeTable")]
impl std::ops::Deref for crate::Mono::Globalization::Unicode::MSCompatUnicodeTable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+MSCompatUnicodeTable")]
impl std::ops::DerefMut for crate::Mono::Globalization::Unicode::MSCompatUnicodeTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+MSCompatUnicodeTable")]
impl crate::Mono::Globalization::Unicode::MSCompatUnicodeTable {
    #[cfg(feature = "Mono+Globalization+Unicode+MSCompatUnicodeTable+__c")]
    pub type __c = crate::Mono::Globalization::Unicode::MSCompatUnicodeTable___c;
}
#[cfg(feature = "Mono+Globalization+Unicode+MSCompatUnicodeTable")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Globalization::Unicode::MSCompatUnicodeTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "Mono+Globalization+Unicode+Contraction")]
#[repr(C)]
#[derive(Debug)]
pub struct Contraction {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Index: i32,
    pub Source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    pub Replacement: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub SortKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "Mono+Globalization+Unicode+Contraction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Globalization::Unicode::Contraction =>
    "Mono.Globalization.Unicode"."Contraction"
);
#[cfg(feature = "Mono+Globalization+Unicode+Contraction")]
impl std::ops::Deref for crate::Mono::Globalization::Unicode::Contraction {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+Contraction")]
impl std::ops::DerefMut for crate::Mono::Globalization::Unicode::Contraction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+Contraction")]
impl crate::Mono::Globalization::Unicode::Contraction {
    pub fn New(
        index: i32,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        replacement: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sortkey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (index, source, replacement, sortkey))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        index: i32,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        replacement: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sortkey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (index, source, replacement, sortkey))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+Contraction")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Globalization::Unicode::Contraction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

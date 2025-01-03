#[cfg(feature = "StringExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct StringExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "StringExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StringExtensions => ""
    ."StringExtensions"
);
#[cfg(feature = "StringExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::StringExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StringExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::StringExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StringExtensions")]
impl crate::GlobalNamespace::StringExtensions {
    pub fn AnyContains(
        stringArray: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        stringComparison: crate::System::StringComparison,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AnyContains", (stringArray, value, stringComparison))?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        substring: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        comp: crate::System::StringComparison,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Contains", (source, substring, comp))?;
        Ok(__cordl_ret.into())
    }
    pub fn Truncate(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        length: i32,
        appendEllipsis: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Truncate", (s, length, appendEllipsis))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StringExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::StringExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

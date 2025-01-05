#[cfg(feature = "UnityEngine+TextCore+Text+TextUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TextUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::TextUtilities =>
    "UnityEngine.TextCore.Text"."TextUtilities"
);
#[cfg(feature = "UnityEngine+TextCore+Text+TextUtilities")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::TextUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::TextUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextUtilities")]
impl crate::UnityEngine::TextCore::Text::TextUtilities {
    pub fn GetHashCodeCaseInSensitive(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHashCodeCaseInSensitive", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperFast(c: char) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUpperFast", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn UintToString(
        unicodes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<u32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UintToString", (unicodes))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::TextUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

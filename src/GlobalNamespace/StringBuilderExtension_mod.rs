#[cfg(feature = "StringBuilderExtension")]
#[repr(C)]
#[derive(Debug)]
pub struct StringBuilderExtension {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "StringBuilderExtension")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StringBuilderExtension => ""
    ."StringBuilderExtension"
);
#[cfg(feature = "StringBuilderExtension")]
impl std::ops::Deref for crate::GlobalNamespace::StringBuilderExtension {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StringBuilderExtension")]
impl std::ops::DerefMut for crate::GlobalNamespace::StringBuilderExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StringBuilderExtension")]
impl crate::GlobalNamespace::StringBuilderExtension {
    pub const kCharZero: char = '0';
    pub fn AppendNumber_i32_0(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        number: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendNumber", (sb, number))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendNumber_u32__cordl_bool1(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        uNumber: u32,
        isNegative: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendNumber", (sb, uNumber, isNegative))?;
        Ok(__cordl_ret.into())
    }
    pub fn Swap(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        startIndex: i32,
        endIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Swap", (sb, startIndex, endIndex))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StringBuilderExtension")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StringBuilderExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

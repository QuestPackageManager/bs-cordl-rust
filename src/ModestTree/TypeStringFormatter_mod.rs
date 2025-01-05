#[cfg(feature = "ModestTree+TypeStringFormatter")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeStringFormatter {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "ModestTree+TypeStringFormatter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ModestTree::TypeStringFormatter => "ModestTree"
    ."TypeStringFormatter"
);
#[cfg(feature = "ModestTree+TypeStringFormatter")]
impl std::ops::Deref for crate::ModestTree::TypeStringFormatter {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+TypeStringFormatter")]
impl std::ops::DerefMut for crate::ModestTree::TypeStringFormatter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+TypeStringFormatter")]
impl crate::ModestTree::TypeStringFormatter {
    pub fn GetCSharpTypeName(
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCSharpTypeName", (typeName))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrettyName(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrettyName", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrettyNameInternal(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrettyNameInternal", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ModestTree+TypeStringFormatter")]
impl quest_hook::libil2cpp::ObjectType for crate::ModestTree::TypeStringFormatter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

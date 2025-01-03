#[cfg(feature = "System+TypeNameParser")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeNameParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+TypeNameParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::TypeNameParser => "System"
    ."TypeNameParser"
);
#[cfg(feature = "System+TypeNameParser")]
impl std::ops::Deref for crate::System::TypeNameParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+TypeNameParser")]
impl std::ops::DerefMut for crate::System::TypeNameParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+TypeNameParser")]
impl crate::System::TypeNameParser {
    pub fn GetType(
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assemblyResolver: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut crate::System::Reflection::AssemblyName,
                *mut crate::System::Reflection::Assembly,
            >,
        >,
        typeResolver: quest_hook::libil2cpp::Gc<
            crate::System::Func_4<
                *mut crate::System::Reflection::Assembly,
                *mut quest_hook::libil2cpp::Il2CppString,
                bool,
                *mut crate::System::Type,
            >,
        >,
        throwOnError: bool,
        ignoreCase: bool,
        stackMark: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::StackCrawlMark,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetType",
                (
                    typeName,
                    assemblyResolver,
                    typeResolver,
                    throwOnError,
                    ignoreCase,
                    stackMark,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+TypeNameParser")]
impl quest_hook::libil2cpp::ObjectType for crate::System::TypeNameParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

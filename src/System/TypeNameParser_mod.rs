#[cfg(feature = "System+TypeNameParser")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeNameParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+TypeNameParser")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::TypeNameParser {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "TypeNameParser";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
                quest_hook::libil2cpp::Gc<crate::System::Reflection::AssemblyName>,
                quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
            >,
        >,
        typeResolver: quest_hook::libil2cpp::Gc<
            crate::System::Func_4<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                bool,
                quest_hook::libil2cpp::Gc<crate::System::Type>,
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

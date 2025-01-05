#[cfg(feature = "System+TypeSpec")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeSpec {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub name: quest_hook::libil2cpp::Gc<crate::System::TypeIdentifier>,
    pub assembly_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub nested: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::System::TypeIdentifier>,
    >,
    pub generic_params: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::System::TypeSpec>,
    >,
    pub modifier_spec: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::System::ModifierSpec>,
    >,
    pub is_byref: bool,
    pub display_fullname: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+TypeSpec")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::TypeSpec => "System"."TypeSpec"
);
#[cfg(feature = "System+TypeSpec")]
impl std::ops::Deref for crate::System::TypeSpec {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+TypeSpec")]
impl std::ops::DerefMut for crate::System::TypeSpec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+TypeSpec")]
impl crate::System::TypeSpec {
    #[cfg(feature = "System+TypeSpec+DisplayNameFormat")]
    pub type DisplayNameFormat = crate::System::TypeSpec_DisplayNameFormat;
    pub fn AddModifier(
        &mut self,
        md: quest_hook::libil2cpp::Gc<crate::System::ModifierSpec>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddModifier", (md))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddName(
        &mut self,
        type_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddName", (type_name))?;
        Ok(__cordl_ret.into())
    }
    pub fn BoundCheck(
        idx: i32,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BoundCheck", (idx, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDisplayFullName(
        &mut self,
        flags: crate::System::TypeSpec_DisplayNameFormat,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetDisplayFullName", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetModifierString(
        &mut self,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder> = __cordl_object
            .invoke("GetModifierString", (sb))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Parse_ByRefMut__cordl_bool__cordl_bool1(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        p: quest_hook::libil2cpp::ByRefMut<i32>,
        is_recurse: bool,
        allow_aqn: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::TypeSpec>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::TypeSpec> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (name, p, is_recurse, allow_aqn))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_Gc0(
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::TypeSpec>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::TypeSpec> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (typeName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParsedTypeIdentifier(
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::TypeIdentifier>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::TypeIdentifier> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParsedTypeIdentifier", (displayName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Resolve(
        &mut self,
        assemblyResolver: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Reflection::AssemblyName>,
            quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        >,
        typeResolver: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            bool,
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
        throwOnError: bool,
        ignoreCase: bool,
        stackMark: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::StackCrawlMark,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke(
                "Resolve",
                (assemblyResolver, typeResolver, throwOnError, ignoreCase, stackMark),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SkipSpace(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pos: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SkipSpace", (name, pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnescapeInternalName(
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnescapeInternalName", (displayName))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DisplayFullName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_DisplayFullName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasModifiers(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasModifiers", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+TypeSpec")]
impl quest_hook::libil2cpp::ObjectType for crate::System::TypeSpec {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+TypeSpec+DisplayNameFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypeSpec_DisplayNameFormat {
    #[default]
    Default = 0i32,
    NO_MODIFIERS = 2i32,
    WANT_ASSEMBLY = 1i32,
}
#[cfg(feature = "System+TypeSpec+DisplayNameFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::TypeSpec_DisplayNameFormat => "System"
    ."TypeSpec/DisplayNameFormat"
);

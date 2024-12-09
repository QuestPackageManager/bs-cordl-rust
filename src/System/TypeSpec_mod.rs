#[cfg(feature = "System+TypeSpec")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeSpec {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub name: *mut crate::System::TypeIdentifier,
    pub assembly_name: *mut quest_hook::libil2cpp::Il2CppString,
    pub nested: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::TypeIdentifier,
    >,
    pub generic_params: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::TypeSpec,
    >,
    pub modifier_spec: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::ModifierSpec,
    >,
    pub is_byref: bool,
    pub display_fullname: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+TypeSpec")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::TypeSpec => "System"."TypeSpec"
);
#[cfg(feature = "System+TypeSpec")]
impl std::ops::Deref for crate::System::TypeSpec {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        md: *mut crate::System::ModifierSpec,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddModifier", (md))?;
        Ok(__cordl_ret)
    }
    pub fn AddName(
        &mut self,
        type_name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddName", (type_name))?;
        Ok(__cordl_ret)
    }
    pub fn GetDisplayFullName(
        &mut self,
        flags: crate::System::TypeSpec_DisplayNameFormat,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetDisplayFullName", (flags))?;
        Ok(__cordl_ret)
    }
    pub fn GetModifierString(
        &mut self,
        sb: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("GetModifierString", (sb))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Resolve(
        &mut self,
        assemblyResolver: *mut crate::System::Func_2<
            *mut crate::System::Reflection::AssemblyName,
            *mut crate::System::Reflection::Assembly,
        >,
        typeResolver: *mut crate::System::Func_4<
            *mut crate::System::Reflection::Assembly,
            *mut quest_hook::libil2cpp::Il2CppString,
            bool,
            *mut crate::System::Type,
        >,
        throwOnError: bool,
        ignoreCase: bool,
        stackMark: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::StackCrawlMark,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke(
                "Resolve",
                (assemblyResolver, typeResolver, throwOnError, ignoreCase, stackMark),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DisplayFullName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_DisplayFullName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasModifiers(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasModifiers", ())?;
        Ok(__cordl_ret)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeSpec_DisplayNameFormat {
    Default = 0i32,
    NO_MODIFIERS = 2i32,
    WANT_ASSEMBLY = 1i32,
}
#[cfg(feature = "System+TypeSpec+DisplayNameFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::TypeSpec_DisplayNameFormat => "System"
    ."TypeSpec/DisplayNameFormat"
);

#[cfg(feature = "Newtonsoft+Json+Utilities+TypeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+TypeExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::TypeExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "TypeExtensions";
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
#[cfg(feature = "Newtonsoft+Json+Utilities+TypeExtensions")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::TypeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+TypeExtensions")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::TypeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+TypeExtensions")]
impl crate::Newtonsoft::Json::Utilities::TypeExtensions {
    pub fn Assembly(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Assembly", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssignableToTypeName_ByRefMut0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        fullTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        searchInterfaces: bool,
        _cordl_match: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssignableToTypeName",
                (_cordl_type, fullTypeName, searchInterfaces, _cordl_match),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AssignableToTypeName_Type_Il2CppString__cordl_bool1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        fullTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        searchInterfaces: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssignableToTypeName",
                (_cordl_type, fullTypeName, searchInterfaces),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BaseType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BaseType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsGenericParameters(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsGenericParameters", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplementInterface(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        interfaceType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImplementInterface", (_cordl_type, interfaceType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAbstract(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAbstract", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsClass(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsClass", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEnum(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsEnum", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsGenericType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsGenericType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsGenericTypeDefinition(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsGenericTypeDefinition", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInterface(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInterface", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPrimitive(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPrimitive", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSealed(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSealed", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValueType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValueType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsVisible(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsVisible", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn MemberType(
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::MemberTypes> {
        let __cordl_ret: crate::System::Reflection::MemberTypes = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MemberType", (memberInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn Method(
        d: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Method", (d))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+TypeExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::TypeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

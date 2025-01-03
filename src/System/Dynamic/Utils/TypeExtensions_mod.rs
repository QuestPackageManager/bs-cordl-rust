#[cfg(feature = "System+Dynamic+Utils+TypeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Dynamic+Utils+TypeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::Utils::TypeExtensions =>
    "System.Dynamic.Utils"."TypeExtensions"
);
#[cfg(feature = "System+Dynamic+Utils+TypeExtensions")]
impl std::ops::Deref for crate::System::Dynamic::Utils::TypeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+TypeExtensions")]
impl std::ops::DerefMut for crate::System::Dynamic::Utils::TypeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+TypeExtensions")]
impl crate::System::Dynamic::Utils::TypeExtensions {
    pub fn GetAnyStaticMethodValidated(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        types: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAnyStaticMethodValidated", (_cordl_type, name, types))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParametersCached(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::ParameterInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::ParameterInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParametersCached", (method))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReturnType(
        mi: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetReturnType", (mi))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeCode(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<crate::System::TypeCode> {
        let __cordl_ret: crate::System::TypeCode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTypeCode", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchesArgumentTypes(
        mi: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        argTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchesArgumentTypes", (mi, argTypes))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Dynamic+Utils+TypeExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::Utils::TypeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

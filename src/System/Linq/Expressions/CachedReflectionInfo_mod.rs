#[cfg(feature = "System+Linq+Expressions+CachedReflectionInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct CachedReflectionInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+CachedReflectionInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::CachedReflectionInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions";
    const CLASS_NAME: &'static str = "CachedReflectionInfo";
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
#[cfg(feature = "System+Linq+Expressions+CachedReflectionInfo")]
impl std::ops::Deref for crate::System::Linq::Expressions::CachedReflectionInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+CachedReflectionInfo")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::CachedReflectionInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+CachedReflectionInfo")]
impl crate::System::Linq::Expressions::CachedReflectionInfo {
    pub fn get_CallSiteOps_AddRule() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CallSiteOps_AddRule", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CallSiteOps_Bind() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CallSiteOps_Bind", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CallSiteOps_ClearMatch() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CallSiteOps_ClearMatch", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CallSiteOps_CreateMatchmaker() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CallSiteOps_CreateMatchmaker", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CallSiteOps_GetCachedRules() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CallSiteOps_GetCachedRules", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CallSiteOps_GetMatch() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CallSiteOps_GetMatch", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CallSiteOps_GetRuleCache() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CallSiteOps_GetRuleCache", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CallSiteOps_GetRules() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CallSiteOps_GetRules", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CallSiteOps_MoveRule() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CallSiteOps_MoveRule", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CallSiteOps_SetNotMatched() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CallSiteOps_SetNotMatched", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CallSiteOps_UpdateRules() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CallSiteOps_UpdateRules", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Math_Pow_Double_Double() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Math_Pow_Double_Double", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_String_op_Equality_String_String() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_String_op_Equality_String_String", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+CachedReflectionInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::CachedReflectionInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

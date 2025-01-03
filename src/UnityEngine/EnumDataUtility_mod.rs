#[cfg(feature = "UnityEngine+EnumDataUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumDataUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+EnumDataUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EnumDataUtility => "UnityEngine"
    ."EnumDataUtility"
);
#[cfg(feature = "UnityEngine+EnumDataUtility")]
impl std::ops::Deref for crate::UnityEngine::EnumDataUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EnumDataUtility")]
impl std::ops::DerefMut for crate::UnityEngine::EnumDataUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EnumDataUtility")]
impl crate::UnityEngine::EnumDataUtility {
    #[cfg(feature = "UnityEngine+EnumDataUtility+CachedType")]
    pub type CachedType = crate::UnityEngine::EnumDataUtility_CachedType;
    pub fn CheckObsoleteAddition(
        field: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
        cachedType: crate::UnityEngine::EnumDataUtility_CachedType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckObsoleteAddition", (field, cachedType))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnumNameFromEnumField(
        field: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
        nicifyName: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnumNameFromEnumField", (field, nicifyName))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnumTooltipFromEnumField(
        field: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnumTooltipFromEnumField", (field))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCachedEnumData(
        enumType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        cachedType: crate::UnityEngine::EnumDataUtility_CachedType,
        nicifyName: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::EnumData> {
        let __cordl_ret: crate::UnityEngine::EnumData = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCachedEnumData", (enumType, cachedType, nicifyName))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleInspectorOrderAttribute(
        enumType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        enumData: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::EnumData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HandleInspectorOrderAttribute", (enumType, enumData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _EnumNameFromEnumField_g__NicifyName_8_0(
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "<EnumNameFromEnumField>g__NicifyName|8_0",
                (_cordl_fixed_empty_name_whitespace),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+EnumDataUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::EnumDataUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+EnumDataUtility+CachedType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumDataUtility_CachedType {
    ExcludeObsolete = 0i32,
    IncludeAllObsolete = 2i32,
    IncludeObsoleteExceptErrors = 1i32,
}
#[cfg(feature = "UnityEngine+EnumDataUtility+CachedType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EnumDataUtility_CachedType =>
    "UnityEngine"."EnumDataUtility/CachedType"
);

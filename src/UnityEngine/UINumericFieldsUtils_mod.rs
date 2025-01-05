#[cfg(feature = "UnityEngine+UINumericFieldsUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct UINumericFieldsUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UINumericFieldsUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UINumericFieldsUtils =>
    "UnityEngine"."UINumericFieldsUtils"
);
#[cfg(feature = "UnityEngine+UINumericFieldsUtils")]
impl std::ops::Deref for crate::UnityEngine::UINumericFieldsUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UINumericFieldsUtils")]
impl std::ops::DerefMut for crate::UnityEngine::UINumericFieldsUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UINumericFieldsUtils")]
impl crate::UnityEngine::UINumericFieldsUtils {
    pub fn TryConvertStringToDouble_ByRefMut0(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::ByRefMut<f64>,
        expr: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ExpressionEvaluator_Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryConvertStringToDouble", (str, value, expr))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryConvertStringToDouble_Il2CppString1(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        initialValueAsString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        value: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryConvertStringToDouble", (str, initialValueAsString, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryConvertStringToFloat(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        initialValueAsString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        value: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryConvertStringToFloat", (str, initialValueAsString, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryConvertStringToInt(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        initialValueAsString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryConvertStringToInt", (str, initialValueAsString, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryConvertStringToLong_ByRefMut0(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::ByRefMut<i64>,
        expr: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ExpressionEvaluator_Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryConvertStringToLong", (str, value, expr))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryConvertStringToLong_Il2CppString1(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        initialValueAsString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        value: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryConvertStringToLong", (str, initialValueAsString, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryConvertStringToUInt(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        initialValueAsString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        value: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryConvertStringToUInt", (str, initialValueAsString, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryConvertStringToULong_ByRefMut0(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::ByRefMut<u64>,
        expr: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ExpressionEvaluator_Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryConvertStringToULong", (str, value, expr))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryConvertStringToULong_Il2CppString1(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        initialValueAsString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        value: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryConvertStringToULong", (str, initialValueAsString, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UINumericFieldsUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UINumericFieldsUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

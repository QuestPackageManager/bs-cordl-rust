#[cfg(feature = "UnityEngine+UIElements+ComputedTransitionUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ComputedTransitionUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+ComputedTransitionUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ComputedTransitionUtils
    => "UnityEngine.UIElements"."ComputedTransitionUtils"
);
#[cfg(feature = "UnityEngine+UIElements+ComputedTransitionUtils")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ComputedTransitionUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ComputedTransitionUtils")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ComputedTransitionUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ComputedTransitionUtils")]
impl crate::UnityEngine::UIElements::ComputedTransitionUtils {
    pub fn ComputeTransitionPropertyData(
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
        outData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::ComputedTransitionProperty,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeTransitionPropertyData", (computedStyle, outData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertTransitionFunction(
        mode: crate::UnityEngine::UIElements::EasingMode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertTransitionFunction", (mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertTransitionTime(
        _cordl_time: crate::UnityEngine::UIElements::TimeValue,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertTransitionTime", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrComputeTransitionPropertyData(
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::UIElements::ComputedTransitionProperty,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::UIElements::ComputedTransitionProperty,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOrComputeTransitionPropertyData", (computedStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTransitionHashCode(
        cs: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTransitionHashCode", (cs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTransitionProperty(
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedTransitionProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTransitionProperty", (computedStyle, id, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWrappingTransitionData<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        i: i32,
        defaultValue: T,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWrappingTransitionData", (list, i, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasTransitionProperty(
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasTransitionProperty", (computedStyle, id))?;
        Ok(__cordl_ret.into())
    }
    pub fn SameTransitionProperty_ByRefMut_ByRefMut0(
        x: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
        y: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::ComputedStyle>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SameTransitionProperty", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn SameTransitionProperty_List_1_List_1_1(
        a: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::StylePropertyName,
            >,
        >,
        b: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::StylePropertyName,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SameTransitionProperty", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn SameTransitionProperty_List_1_List_1_2(
        a: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::TimeValue,
            >,
        >,
        b: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::TimeValue,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SameTransitionProperty", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedTransitions(
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateComputedTransitions", (computedStyle))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+ComputedTransitionUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ComputedTransitionUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

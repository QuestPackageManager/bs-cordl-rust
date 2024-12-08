#[cfg(feature = "UnityEngine+UIElements+UQueryBuilder_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UQueryBuilder_1<T: quest_hook::libil2cpp::Type> {
    pub m_StyleSelectors: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::StyleSelector,
    >,
    pub m_Parts: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::StyleSelectorPart,
    >,
    pub m_Element: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_Matchers: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::RuleMatcher,
    >,
    pub m_Relationship: crate::UnityEngine::UIElements::StyleSelectorRelationship,
    pub pseudoStatesMask: i32,
    pub negatedPseudoStatesMask: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+UQueryBuilder_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UQueryBuilder_1 < T >
    => "UnityEngine.UIElements"."UQueryBuilder`1<T>" < T >
);
#[cfg(feature = "UnityEngine+UIElements+UQueryBuilder_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UQueryBuilder_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryBuilder_1")]
impl<T: quest_hook::libil2cpp::Type> crate::UnityEngine::UIElements::UQueryBuilder_1<T> {
    pub fn Name(
        &mut self,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::UQueryBuilder_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::UIElements::UQueryBuilder_1<T> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Name",
            (id),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Class(
        &mut self,
        classname: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::UQueryBuilder_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::UIElements::UQueryBuilder_1<T> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Class",
            (classname),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AddName(
        &mut self,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddName",
            (id),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_styleSelectors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::StyleSelector,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::StyleSelector,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_styleSelectors", ())?;
        Ok(__cordl_ret)
    }
    pub fn SingleBaseType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::UQueryBuilder_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::UIElements::UQueryBuilder_1<T> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SingleBaseType",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AddClass(
        &mut self,
        c: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddClass",
            (c),
        )?;
        Ok(__cordl_ret)
    }
    pub fn FinishSelector(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FinishSelector",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CurrentSelectorEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CurrentSelectorEmpty",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn FinishCurrentSelector(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FinishCurrentSelector",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        visualElement: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (visualElement),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UQueryState_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::UIElements::UQueryState_1<T> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Build",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_parts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::StyleSelectorPart,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::StyleSelectorPart,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_parts", ())?;
        Ok(__cordl_ret)
    }
    pub fn Equals_UQueryBuilder_1_0(
        &mut self,
        other: crate::UnityEngine::UIElements::UQueryBuilder_1<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AddPseudoStatesRuleIfNecessasy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddPseudoStatesRuleIfNecessasy",
            (),
        )?;
        Ok(__cordl_ret)
    }
}

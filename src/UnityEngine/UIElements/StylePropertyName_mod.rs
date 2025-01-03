#[cfg(feature = "UnityEngine+UIElements+StylePropertyName")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StylePropertyName {
    pub _id_k__BackingField: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    pub _name_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyName")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StylePropertyName =>
    "UnityEngine.UIElements"."StylePropertyName"
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyName")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StylePropertyName {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyName")]
impl crate::UnityEngine::UIElements::StylePropertyName {
    pub fn Equals_Il2CppObject0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_StylePropertyName1(
        &mut self,
        other: crate::UnityEngine::UIElements::StylePropertyName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn StylePropertyIdFromString(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StylePropertyIdFromString", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (name),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_StylePropertyId0(
        &mut self,
        stylePropertyId: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (stylePropertyId),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_id",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::UIElements::StylePropertyName,
        rhs: crate::UnityEngine::UIElements::StylePropertyName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StylePropertyName,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::StylePropertyName = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::UnityEngine::UIElements::StylePropertyName,
        rhs: crate::UnityEngine::UIElements::StylePropertyName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyName")]
impl AsRef<
    crate::System::IEquatable_1<crate::UnityEngine::UIElements::StylePropertyName>,
> for crate::UnityEngine::UIElements::StylePropertyName {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::StylePropertyName,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyName")]
impl AsMut<
    crate::System::IEquatable_1<crate::UnityEngine::UIElements::StylePropertyName>,
> for crate::UnityEngine::UIElements::StylePropertyName {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::StylePropertyName,
    > {
        todo!()
    }
}

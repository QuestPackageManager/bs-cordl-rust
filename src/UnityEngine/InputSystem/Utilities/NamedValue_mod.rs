#[cfg(feature = "UnityEngine+InputSystem+Utilities+NamedValue")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct NamedValue {
    pub _name_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _value_k__BackingField: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NamedValue")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::Utilities::NamedValue
    => "UnityEngine.InputSystem.Utilities"."NamedValue"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NamedValue")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::NamedValue {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NamedValue")]
impl crate::UnityEngine::InputSystem::Utilities::NamedValue {
    pub const Separator: &'static str = ",";
    pub fn ApplyAllToObject<TParameterList>(
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        parameters: TParameterList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParameterList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyAllToObject", (instance, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyToObject(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyToObject",
            (instance),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertTo(
        &mut self,
        _cordl_type: crate::System::TypeCode,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::NamedValue,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::NamedValue = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ConvertTo",
            (_cordl_type),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_NamedValue0(
        &mut self,
        other: crate::UnityEngine::InputSystem::Utilities::NamedValue,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn From<TValue>(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::NamedValue,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::NamedValue = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("From", (name, value))?;
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
    pub fn Parse(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::NamedValue,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::NamedValue = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseMultiple(
        parameterString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::InputSystem::Utilities::NamedValue,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::InputSystem::Utilities::NamedValue,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseMultiple", (parameterString))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseParameter(
        parameterString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::NamedValue,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::NamedValue = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseParameter", (parameterString, index))?;
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
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TypeCode> {
        let __cordl_ret: crate::System::TypeCode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_type",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::UnityEngine::InputSystem::Utilities::NamedValue,
        right: crate::UnityEngine::InputSystem::Utilities::NamedValue,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        left: crate::UnityEngine::InputSystem::Utilities::NamedValue,
        right: crate::UnityEngine::InputSystem::Utilities::NamedValue,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_name",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_value(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_value",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NamedValue")]
impl AsRef<
    crate::System::IEquatable_1<crate::UnityEngine::InputSystem::Utilities::NamedValue>,
> for crate::UnityEngine::InputSystem::Utilities::NamedValue {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::Utilities::NamedValue,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NamedValue")]
impl AsMut<
    crate::System::IEquatable_1<crate::UnityEngine::InputSystem::Utilities::NamedValue>,
> for crate::UnityEngine::InputSystem::Utilities::NamedValue {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::Utilities::NamedValue,
    > {
        todo!()
    }
}

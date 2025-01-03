#[cfg(feature = "UnityEngine+InputSystem+Utilities+InternedString")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InternedString {
    pub m_StringOriginalCase: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_StringLowerCase: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+InternedString")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::InternedString =>
    "UnityEngine.InputSystem.Utilities"."InternedString"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+InternedString")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::InternedString {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+InternedString")]
impl crate::UnityEngine::InputSystem::Utilities::InternedString {
    pub fn CompareTo(
        &mut self,
        other: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
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
    pub fn Equals_InternedString1(
        &mut self,
        other: crate::UnityEngine::InputSystem::Utilities::InternedString,
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
    pub fn IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsEmpty",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToLower(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToLower", ())?;
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
    pub fn _ctor(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (text),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_length",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_Il2CppString_InternedString2(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        b: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_InternedString_Il2CppString1(
        a: crate::UnityEngine::InputSystem::Utilities::InternedString,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_InternedString_InternedString0(
        a: crate::UnityEngine::InputSystem::Utilities::InternedString,
        b: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan(
        left: crate::UnityEngine::InputSystem::Utilities::InternedString,
        right: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        str: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("op_Implicit", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_Il2CppString_InternedString2(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        b: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_InternedString_Il2CppString1(
        a: crate::UnityEngine::InputSystem::Utilities::InternedString,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_InternedString_InternedString0(
        a: crate::UnityEngine::InputSystem::Utilities::InternedString,
        b: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan(
        left: crate::UnityEngine::InputSystem::Utilities::InternedString,
        right: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+InternedString")]
impl AsRef<
    crate::System::IComparable_1<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    >,
> for crate::UnityEngine::InputSystem::Utilities::InternedString {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+InternedString")]
impl AsMut<
    crate::System::IComparable_1<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    >,
> for crate::UnityEngine::InputSystem::Utilities::InternedString {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+InternedString")]
impl AsRef<
    crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    >,
> for crate::UnityEngine::InputSystem::Utilities::InternedString {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+InternedString")]
impl AsMut<
    crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    >,
> for crate::UnityEngine::InputSystem::Utilities::InternedString {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        todo!()
    }
}

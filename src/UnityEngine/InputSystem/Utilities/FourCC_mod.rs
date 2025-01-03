#[cfg(feature = "UnityEngine+InputSystem+Utilities+FourCC")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct FourCC {
    pub m_Code: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+FourCC")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::Utilities::FourCC =>
    "UnityEngine.InputSystem.Utilities"."FourCC"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+FourCC")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::FourCC {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+FourCC")]
impl crate::UnityEngine::InputSystem::Utilities::FourCC {
    pub fn Equals_FourCC0(
        &mut self,
        other: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
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
    pub fn FromInt32(
        i: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromInt32", (i))?;
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
    pub fn ToInt32(
        fourCC: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (fourCC))?;
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
    pub fn _ctor_Il2CppString2(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (str),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_char__cordl_char__cordl_char__cordl_char1(
        &mut self,
        a: char,
        b: char,
        c: char,
        d: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_0(
        &mut self,
        code: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (code),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::UnityEngine::InputSystem::Utilities::FourCC,
        right: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_FourCC0(
        fourCC: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (fourCC))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i32_1(
        i: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        left: crate::UnityEngine::InputSystem::Utilities::FourCC,
        right: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+FourCC")]
impl AsRef<
    crate::System::IEquatable_1<crate::UnityEngine::InputSystem::Utilities::FourCC>,
> for crate::UnityEngine::InputSystem::Utilities::FourCC {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+FourCC")]
impl AsMut<
    crate::System::IEquatable_1<crate::UnityEngine::InputSystem::Utilities::FourCC>,
> for crate::UnityEngine::InputSystem::Utilities::FourCC {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        todo!()
    }
}

#[cfg(feature = "UnityEngine+InputSystem+Utilities+Substring")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Substring {
    pub m_String: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Index: i32,
    pub m_Length: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Substring")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::Utilities::Substring
    => "UnityEngine.InputSystem.Utilities"."Substring"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Substring")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::Substring {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Substring")]
impl crate::UnityEngine::InputSystem::Utilities::Substring {
    pub fn CompareTo(
        &mut self,
        other: crate::UnityEngine::InputSystem::Utilities::Substring,
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
    pub fn Equals_Il2CppString1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_InternedString3(
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
    pub fn Equals_Substring2(
        &mut self,
        other: crate::UnityEngine::InputSystem::Utilities::Substring,
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
    pub fn StartsWith(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "StartsWith",
            (str),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Substr(
        &mut self,
        index: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Substr",
            (index, length),
        )?;
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
    pub fn _ctor_Il2CppString0(
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
    pub fn _ctor_i32_2(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (str, index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_1(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (str, index, length),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_index(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_index",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isEmpty",
            (),
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
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Substring")]
impl AsRef<
    crate::System::IComparable_1<crate::UnityEngine::InputSystem::Utilities::Substring>,
> for crate::UnityEngine::InputSystem::Utilities::Substring {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<
        crate::UnityEngine::InputSystem::Utilities::Substring,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Substring")]
impl AsMut<
    crate::System::IComparable_1<crate::UnityEngine::InputSystem::Utilities::Substring>,
> for crate::UnityEngine::InputSystem::Utilities::Substring {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        crate::UnityEngine::InputSystem::Utilities::Substring,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Substring")]
impl AsRef<
    crate::System::IEquatable_1<crate::UnityEngine::InputSystem::Utilities::Substring>,
> for crate::UnityEngine::InputSystem::Utilities::Substring {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::Utilities::Substring,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Substring")]
impl AsMut<
    crate::System::IEquatable_1<crate::UnityEngine::InputSystem::Utilities::Substring>,
> for crate::UnityEngine::InputSystem::Utilities::Substring {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::Utilities::Substring,
    > {
        todo!()
    }
}

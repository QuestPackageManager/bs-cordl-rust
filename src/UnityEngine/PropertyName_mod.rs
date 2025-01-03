#[cfg(feature = "UnityEngine+PropertyName")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PropertyName {
    pub id: i32,
}
#[cfg(feature = "UnityEngine+PropertyName")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PropertyName => "UnityEngine"
    ."PropertyName"
);
#[cfg(feature = "UnityEngine+PropertyName")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::PropertyName {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PropertyName")]
impl crate::UnityEngine::PropertyName {
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
    pub fn Equals_PropertyName1(
        &mut self,
        other: crate::UnityEngine::PropertyName,
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
    pub fn IsNullOrEmpty(
        prop: crate::UnityEngine::PropertyName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNullOrEmpty", (prop))?;
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
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (name),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PropertyName1(
        &mut self,
        other: crate::UnityEngine::PropertyName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::PropertyName,
        rhs: crate::UnityEngine::PropertyName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PropertyName> {
        let __cordl_ret: crate::UnityEngine::PropertyName = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (name))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+PropertyName")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::PropertyName>>
for crate::UnityEngine::PropertyName {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::UnityEngine::PropertyName> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+PropertyName")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::PropertyName>>
for crate::UnityEngine::PropertyName {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::PropertyName> {
        todo!()
    }
}

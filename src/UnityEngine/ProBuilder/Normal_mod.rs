#[cfg(feature = "UnityEngine+ProBuilder+Normal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Normal {
    pub _normal_k__BackingField: crate::UnityEngine::Vector3,
    pub _tangent_k__BackingField: crate::UnityEngine::Vector4,
    pub _bitangent_k__BackingField: crate::UnityEngine::Vector3,
}
#[cfg(feature = "UnityEngine+ProBuilder+Normal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Normal =>
    "UnityEngine.ProBuilder"."Normal"
);
#[cfg(feature = "UnityEngine+ProBuilder+Normal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::Normal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Normal")]
impl crate::UnityEngine::ProBuilder::Normal {
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
    pub fn Equals_Normal1(
        &mut self,
        other: crate::UnityEngine::ProBuilder::Normal,
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
    pub fn get_bitangent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bitangent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_normal",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tangent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_tangent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        a: crate::UnityEngine::ProBuilder::Normal,
        b: crate::UnityEngine::ProBuilder::Normal,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        a: crate::UnityEngine::ProBuilder::Normal,
        b: crate::UnityEngine::ProBuilder::Normal,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bitangent(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_bitangent",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_normal(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_normal",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_tangent(
        &mut self,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_tangent",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Normal")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::Normal>>
for crate::UnityEngine::ProBuilder::Normal {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::Normal> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Normal")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::Normal>>
for crate::UnityEngine::ProBuilder::Normal {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::Normal> {
        todo!()
    }
}

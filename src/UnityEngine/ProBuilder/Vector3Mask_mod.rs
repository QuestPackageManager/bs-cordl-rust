#[cfg(feature = "UnityEngine+ProBuilder+Vector3Mask")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Vector3Mask {
    pub m_Mask: u8,
}
#[cfg(feature = "UnityEngine+ProBuilder+Vector3Mask")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Vector3Mask =>
    "UnityEngine.ProBuilder"."Vector3Mask"
);
#[cfg(feature = "UnityEngine+ProBuilder+Vector3Mask")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::Vector3Mask {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Vector3Mask")]
impl crate::UnityEngine::ProBuilder::Vector3Mask {
    pub const X: u8 = 1u8;
    pub const Y: u8 = 2u8;
    pub const Z: u8 = 4u8;
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
    pub fn Equals_Vector3Mask0(
        &mut self,
        other: crate::UnityEngine::ProBuilder::Vector3Mask,
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
    pub fn _ctor_Vector3_f32_0(
        &mut self,
        v: crate::UnityEngine::Vector3,
        epsilon: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v, epsilon),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u8_1(
        &mut self,
        mask: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (mask),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, i: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (i),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_active(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_active",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_x(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_x",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_y(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_y",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_z(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_z",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        i: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Item",
            (i, value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Vector3Mask")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::Vector3Mask>>
for crate::UnityEngine::ProBuilder::Vector3Mask {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::Vector3Mask> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Vector3Mask")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::Vector3Mask>>
for crate::UnityEngine::ProBuilder::Vector3Mask {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::Vector3Mask> {
        todo!()
    }
}

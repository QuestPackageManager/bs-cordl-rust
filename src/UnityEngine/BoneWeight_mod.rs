#[cfg(feature = "UnityEngine+BoneWeight")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BoneWeight {
    pub m_Weight0: f32,
    pub m_Weight1: f32,
    pub m_Weight2: f32,
    pub m_Weight3: f32,
    pub m_BoneIndex0: i32,
    pub m_BoneIndex1: i32,
    pub m_BoneIndex2: i32,
    pub m_BoneIndex3: i32,
}
#[cfg(feature = "UnityEngine+BoneWeight")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::BoneWeight => "UnityEngine"
    ."BoneWeight"
);
#[cfg(feature = "UnityEngine+BoneWeight")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::BoneWeight {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+BoneWeight")]
impl crate::UnityEngine::BoneWeight {
    pub fn Equals_BoneWeight1(
        &mut self,
        other: crate::UnityEngine::BoneWeight,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        other: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_boneIndex0(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_boneIndex0",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_boneIndex1(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_boneIndex1",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_boneIndex2(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_boneIndex2",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_boneIndex3(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_boneIndex3",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_weight0(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_weight0",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_weight1(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_weight1",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_weight2(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_weight2",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_weight3(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_weight3",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_boneIndex0(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_boneIndex0",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_boneIndex1(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_boneIndex1",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_boneIndex2(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_boneIndex2",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_boneIndex3(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_boneIndex3",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_weight0(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_weight0",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_weight1(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_weight1",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_weight2(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_weight2",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_weight3(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_weight3",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}

#[cfg(feature = "UnityEngine+Pose")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Pose {
    pub position: crate::UnityEngine::Vector3,
    pub rotation: crate::UnityEngine::Quaternion,
}
#[cfg(feature = "UnityEngine+Pose")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Pose => "UnityEngine"."Pose"
);
#[cfg(feature = "UnityEngine+Pose")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Pose {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Pose")]
impl crate::UnityEngine::Pose {
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
    pub fn Equals_Pose1(
        &mut self,
        other: crate::UnityEngine::Pose,
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
    pub fn GetTransformedBy(
        &mut self,
        lhs: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_ret: crate::UnityEngine::Pose = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetTransformedBy",
            (lhs),
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
    pub fn _ctor(
        &mut self,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (position, rotation),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_forward(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_forward",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_identity() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_ret: crate::UnityEngine::Pose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_identity", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Pose")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::Pose>>
for crate::UnityEngine::Pose {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::UnityEngine::Pose> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Pose")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::Pose>>
for crate::UnityEngine::Pose {
    fn as_mut(&mut self) -> &mut crate::System::IEquatable_1<crate::UnityEngine::Pose> {
        todo!()
    }
}

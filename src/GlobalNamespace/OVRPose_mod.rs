#[cfg(feature = "OVRPose")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPose {
    pub position: crate::UnityEngine::Vector3,
    pub orientation: crate::UnityEngine::Quaternion,
}
#[cfg(feature = "OVRPose")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPose => ""."OVRPose"
);
#[cfg(feature = "OVRPose")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::OVRPose {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPose")]
impl crate::GlobalNamespace::OVRPose {
    pub fn Equals(
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
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Inverse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        let __cordl_ret: crate::GlobalNamespace::OVRPose = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Inverse",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Rotate180AlongX(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        let __cordl_ret: crate::GlobalNamespace::OVRPose = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Rotate180AlongX",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToPosef(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Posef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToPosef",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToPosef_Legacy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Posef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToPosef_Legacy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn flipZ(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        let __cordl_ret: crate::GlobalNamespace::OVRPose = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "flipZ",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_identity() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPose,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_identity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        x: crate::GlobalNamespace::OVRPose,
        y: crate::GlobalNamespace::OVRPose,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        x: crate::GlobalNamespace::OVRPose,
        y: crate::GlobalNamespace::OVRPose,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply(
        lhs: crate::GlobalNamespace::OVRPose,
        rhs: crate::GlobalNamespace::OVRPose,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        let __cordl_ret: crate::GlobalNamespace::OVRPose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}

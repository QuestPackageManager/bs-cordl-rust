#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+InstantiationParameters"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InstantiationParameters {
    pub m_Position: crate::UnityEngine::Vector3,
    pub m_Rotation: crate::UnityEngine::Quaternion,
    pub m_Parent: *mut crate::UnityEngine::Transform,
    pub m_InstantiateInWorldPosition: bool,
    pub m_SetPositionRotation: bool,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+InstantiationParameters"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters =>
    "UnityEngine.ResourceManagement.ResourceProviders"."InstantiationParameters"
);
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+InstantiationParameters"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+InstantiationParameters"
)]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters {
    pub fn _ctor_Transform__cordl_bool0(
        &mut self,
        parent: *mut crate::UnityEngine::Transform,
        instantiateInWorldSpace: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (parent, instantiateInWorldSpace),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Vector3_Quaternion_Transform1(
        &mut self,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parent: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (position, rotation, parent),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Parent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_ret: *mut crate::UnityEngine::Transform = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Parent",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Rotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Rotation",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_InstantiateInWorldPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_InstantiateInWorldPosition",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Instantiate<TObject>(
        &mut self,
        source: TObject,
    ) -> quest_hook::libil2cpp::Result<TObject>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Instantiate",
            (source),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Position",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_SetPositionRotation(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_SetPositionRotation",
            (),
        )?;
        Ok(__cordl_ret)
    }
}

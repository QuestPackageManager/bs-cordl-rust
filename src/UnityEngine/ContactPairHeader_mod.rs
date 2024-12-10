#[cfg(feature = "UnityEngine+ContactPairHeader")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ContactPairHeader {
    pub m_BodyID: i32,
    pub m_OtherBodyID: i32,
    pub m_StartPtr: crate::System::IntPtr,
    pub m_NbPairs: u32,
    pub m_Flags: crate::UnityEngine::CollisionPairHeaderFlags,
    pub m_RelativeVelocity: crate::UnityEngine::Vector3,
}
#[cfg(feature = "UnityEngine+ContactPairHeader")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ContactPairHeader => "UnityEngine"
    ."ContactPairHeader"
);
#[cfg(feature = "UnityEngine+ContactPairHeader")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ContactPairHeader {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ContactPairHeader")]
impl crate::UnityEngine::ContactPairHeader {
    pub fn GetContactPair(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactPair>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ContactPair,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetContactPair",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContactPair_Internal(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetContactPair_Internal",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Body(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Body",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BodyInstanceID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_BodyInstanceID",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasRemovedBody(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_HasRemovedBody",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OtherBody(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_OtherBody",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OtherBodyInstanceID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_OtherBodyInstanceID",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PairCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_PairCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}

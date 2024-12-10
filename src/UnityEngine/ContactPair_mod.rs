#[cfg(feature = "UnityEngine+ContactPair")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ContactPair {
    pub m_ColliderID: i32,
    pub m_OtherColliderID: i32,
    pub m_StartPtr: crate::System::IntPtr,
    pub m_NbPoints: u32,
    pub m_Flags: crate::UnityEngine::CollisionPairFlags,
    pub m_Events: crate::UnityEngine::CollisionPairEventFlags,
    pub m_ImpulseSum: crate::UnityEngine::Vector3,
}
#[cfg(feature = "UnityEngine+ContactPair")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ContactPair => "UnityEngine"
    ."ContactPair"
);
#[cfg(feature = "UnityEngine+ContactPair")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::ContactPair {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ContactPair")]
impl crate::UnityEngine::ContactPair {
    pub const c_InvalidFaceIndex: u32 = 33554943u32;
    pub fn CopyToNativeArray(
        &mut self,
        buffer: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::ContactPairPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyToNativeArray",
            (buffer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractContacts(
        &mut self,
        managedContainer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::ContactPoint>,
        >,
        flipped: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ExtractContacts",
            (managedContainer, flipped),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractContactsArray(
        &mut self,
        managedContainer: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ContactPoint>,
        >,
        flipped: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ExtractContactsArray",
            (managedContainer, flipped),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContactPoint(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactPairPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ContactPairPoint,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetContactPoint",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContactPointFaceIndex(
        &mut self,
        contactIndex: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetContactPointFaceIndex",
            (contactIndex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContactPoint_Internal(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetContactPoint_Internal",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Collider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Collider",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ColliderInstanceID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ColliderInstanceID",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ContactCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ContactCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasRemovedCollider(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_HasRemovedCollider",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ImpulseSum(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ImpulseSum",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCollisionEnter(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsCollisionEnter",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCollisionExit(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsCollisionExit",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCollisionStay(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsCollisionStay",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OtherCollider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_OtherCollider",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OtherColliderInstanceID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_OtherColliderInstanceID",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}

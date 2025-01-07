#[cfg(feature = "UnityEngine+ContactPair")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ContactPair {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "ContactPair";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+ContactPair")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::ContactPair {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+ContactPair")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::ContactPair {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+ContactPair")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::ContactPair {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+ContactPair")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::ContactPair {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
    pub fn ExtractContactsArray_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactPair>,
        managedContainer: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ContactPoint>,
        >,
        flipped: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ExtractContactsArray_Injected",
                (_unity_self, managedContainer, flipped),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractContacts_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactPair>,
        managedContainer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::ContactPoint>,
        >,
        flipped: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ExtractContacts_Injected",
                (_unity_self, managedContainer, flipped),
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

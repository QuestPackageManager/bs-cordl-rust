#[cfg(feature = "UnityEngine+Collision2D")]
#[repr(C)]
#[derive(Debug)]
pub struct Collision2D {
    __cordl_parent: crate::System::Object,
    pub m_Collider: i32,
    pub m_OtherCollider: i32,
    pub m_Rigidbody: i32,
    pub m_OtherRigidbody: i32,
    pub m_RelativeVelocity: crate::UnityEngine::Vector2,
    pub m_Enabled: i32,
    pub m_ContactCount: i32,
    pub m_ReusedContacts: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::ContactPoint2D,
    >,
    pub m_LegacyContacts: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::ContactPoint2D,
    >,
}
#[cfg(feature = "UnityEngine+Collision2D")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Collision2D => "UnityEngine"
    ."Collision2D"
);
#[cfg(feature = "UnityEngine+Collision2D")]
impl std::ops::Deref for crate::UnityEngine::Collision2D {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Collision2D")]
impl std::ops::DerefMut for crate::UnityEngine::Collision2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Collision2D")]
impl crate::UnityEngine::Collision2D {}
#[cfg(feature = "UnityEngine+Collision2D")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Collision2D {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

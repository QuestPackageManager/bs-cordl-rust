#[cfg(feature = "UnityEngine+Collision2D")]
#[repr(C)]
#[derive(Debug)]
pub struct Collision2D {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Collider: i32,
    pub m_OtherCollider: i32,
    pub m_Rigidbody: i32,
    pub m_OtherRigidbody: i32,
    pub m_RelativeVelocity: crate::UnityEngine::Vector2,
    pub m_Enabled: i32,
    pub m_ContactCount: i32,
    pub m_ReusedContacts: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ContactPoint2D>,
    >,
    pub m_LegacyContacts: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ContactPoint2D>,
    >,
}
#[cfg(feature = "UnityEngine+Collision2D")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Collision2D {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Collision2D";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+Collision2D")]
impl std::ops::Deref for crate::UnityEngine::Collision2D {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Collision2D")]
impl std::ops::DerefMut for crate::UnityEngine::Collision2D {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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

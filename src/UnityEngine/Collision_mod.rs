#[cfg(feature = "UnityEngine+Collision")]
#[repr(C)]
#[derive(Debug)]
pub struct Collision {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Header: crate::UnityEngine::ContactPairHeader,
    pub m_Pair: crate::UnityEngine::ContactPair,
    pub m_Flipped: bool,
    pub m_LegacyContacts: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::ContactPoint,
    >,
}
#[cfg(feature = "UnityEngine+Collision")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Collision => "UnityEngine"
    ."Collision"
);
#[cfg(feature = "UnityEngine+Collision")]
impl std::ops::Deref for crate::UnityEngine::Collision {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Collision")]
impl std::ops::DerefMut for crate::UnityEngine::Collision {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Collision")]
impl crate::UnityEngine::Collision {
    pub fn GetContact(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ContactPoint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ContactPoint = __cordl_object
            .invoke("GetContact", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetContacts_Il2CppArray0(
        &mut self,
        contacts: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::ContactPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetContacts", (contacts))?;
        Ok(__cordl_ret)
    }
    pub fn GetContacts_List_1_1(
        &mut self,
        contacts: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::ContactPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetContacts", (contacts))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_ByRefMut_ByRefMut__cordl_bool1(
        header: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactPairHeader>,
        pair: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactPair>,
        flipped: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (header, pair, flipped))?;
        Ok(__cordl_object)
    }
    pub fn Reuse(
        &mut self,
        header: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactPairHeader>,
        pair: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactPair>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reuse", (header, pair))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ByRefMut_ByRefMut__cordl_bool1(
        &mut self,
        header: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactPairHeader>,
        pair: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactPair>,
        flipped: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (header, pair, flipped))?;
        Ok(__cordl_ret)
    }
    pub fn get_Flipped(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Flipped", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_articulationBody(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::ArticulationBody> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ArticulationBody = __cordl_object
            .invoke("get_articulationBody", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_body(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("get_body", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_collider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Collider> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Collider = __cordl_object
            .invoke("get_collider", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_contactCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_contactCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_contacts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ContactPoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::ContactPoint,
        > = __cordl_object.invoke("get_contacts", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_frictionForceSum(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_frictionForceSum", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_gameObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_impactForceSum(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_impactForceSum", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_impulse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_impulse", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_other(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("get_other", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_relativeVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_relativeVelocity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rigidbody(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Rigidbody> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Rigidbody = __cordl_object
            .invoke("get_rigidbody", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_transform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_transform", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Flipped(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Flipped", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Collision")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Collision {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

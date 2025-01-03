#[cfg(feature = "UnityEngine+ModifiableContactPair")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ModifiableContactPair {
    pub actor: crate::System::IntPtr,
    pub otherActor: crate::System::IntPtr,
    pub shape: crate::System::IntPtr,
    pub otherShape: crate::System::IntPtr,
    pub rotation: crate::UnityEngine::Quaternion,
    pub position: crate::UnityEngine::Vector3,
    pub otherRotation: crate::UnityEngine::Quaternion,
    pub otherPosition: crate::UnityEngine::Vector3,
    pub numContacts: i32,
    pub contacts: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+ModifiableContactPair")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ModifiableContactPair =>
    "UnityEngine"."ModifiableContactPair"
);
#[cfg(feature = "UnityEngine+ModifiableContactPair")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ModifiableContactPair {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ModifiableContactPair")]
impl crate::UnityEngine::ModifiableContactPair {
    pub fn GetBounciness(&mut self, i: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetBounciness",
            (i),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContact(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetContact", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContactPatch(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetContactPatch", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDynamicFriction(&mut self, i: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetDynamicFriction",
            (i),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFaceIndex(&mut self, i: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetFaceIndex",
            (i),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxImpulse(&mut self, i: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetMaxImpulse",
            (i),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNormal(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetNormal",
            (i),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPoint(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetPoint",
            (i),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSeparation(&mut self, i: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetSeparation",
            (i),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticFriction(&mut self, i: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetStaticFriction",
            (i),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTargetVelocity(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetTargetVelocity",
            (i),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IgnoreContact(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IgnoreContact",
            (i),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBounciness(
        &mut self,
        i: i32,
        bounciness: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetBounciness",
            (i, bounciness),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDynamicFriction(
        &mut self,
        i: i32,
        dynamicFriction: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetDynamicFriction",
            (i, dynamicFriction),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMaxImpulse(
        &mut self,
        i: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetMaxImpulse",
            (i, value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNormal(
        &mut self,
        i: i32,
        normal: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetNormal",
            (i, normal),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPoint(
        &mut self,
        i: i32,
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetPoint",
            (i, v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSeparation(
        &mut self,
        i: i32,
        separation: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetSeparation",
            (i, separation),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticFriction(
        &mut self,
        i: i32,
        staticFriction: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStaticFriction",
            (i, staticFriction),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTargetVelocity(
        &mut self,
        i: i32,
        velocity: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetTargetVelocity",
            (i, velocity),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bodyAngularVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bodyAngularVelocity",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bodyInstanceID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bodyInstanceID",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bodyVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bodyVelocity",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colliderInstanceID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_colliderInstanceID",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_contactCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_contactCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_massProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ModifiableMassProperties> {
        let __cordl_ret: crate::UnityEngine::ModifiableMassProperties = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_massProperties",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_otherBodyAngularVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_otherBodyAngularVelocity",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_otherBodyInstanceID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_otherBodyInstanceID",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_otherBodyVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_otherBodyVelocity",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_otherColliderInstanceID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_otherColliderInstanceID",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_massProperties(
        &mut self,
        value: crate::UnityEngine::ModifiableMassProperties,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_massProperties",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}

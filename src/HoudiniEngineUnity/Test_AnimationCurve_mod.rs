#[cfg(feature = "HoudiniEngineUnity+Test_AnimationCurve")]
#[repr(C)]
#[derive(Debug)]
pub struct Test_AnimationCurve {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cordl_self: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
}
#[cfg(feature = "HoudiniEngineUnity+Test_AnimationCurve")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::Test_AnimationCurve =>
    "HoudiniEngineUnity"."Test_AnimationCurve"
);
#[cfg(feature = "HoudiniEngineUnity+Test_AnimationCurve")]
impl std::ops::Deref for crate::HoudiniEngineUnity::Test_AnimationCurve {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_AnimationCurve")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::Test_AnimationCurve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_AnimationCurve")]
impl crate::HoudiniEngineUnity::Test_AnimationCurve {
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_AnimationCurve>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsNull", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_self: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_self))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_self: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_AnimationCurve")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::Test_AnimationCurve {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_AnimationCurve")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivableWrapperClass_1<
        *mut crate::HoudiniEngineUnity::Test_AnimationCurve,
    >,
> for crate::HoudiniEngineUnity::Test_AnimationCurve {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivableWrapperClass_1<
        *mut crate::HoudiniEngineUnity::Test_AnimationCurve,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_AnimationCurve")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivableWrapperClass_1<
        *mut crate::HoudiniEngineUnity::Test_AnimationCurve,
    >,
> for crate::HoudiniEngineUnity::Test_AnimationCurve {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivableWrapperClass_1<
        *mut crate::HoudiniEngineUnity::Test_AnimationCurve,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_AnimationCurve")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::Test_AnimationCurve,
    >,
> for crate::HoudiniEngineUnity::Test_AnimationCurve {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::Test_AnimationCurve,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_AnimationCurve")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::Test_AnimationCurve,
    >,
> for crate::HoudiniEngineUnity::Test_AnimationCurve {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::Test_AnimationCurve,
    > {
        unsafe { std::mem::transmute(self) }
    }
}

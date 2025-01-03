#[cfg(feature = "HoudiniEngineUnity+Test_MeshRenderer")]
#[repr(C)]
#[derive(Debug)]
pub struct Test_MeshRenderer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cordl_self: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer>,
}
#[cfg(feature = "HoudiniEngineUnity+Test_MeshRenderer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::Test_MeshRenderer =>
    "HoudiniEngineUnity"."Test_MeshRenderer"
);
#[cfg(feature = "HoudiniEngineUnity+Test_MeshRenderer")]
impl std::ops::Deref for crate::HoudiniEngineUnity::Test_MeshRenderer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_MeshRenderer")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::Test_MeshRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_MeshRenderer")]
impl crate::HoudiniEngineUnity::Test_MeshRenderer {
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_MeshRenderer>,
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
        _cordl_self: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_self))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_self: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_MeshRenderer")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::Test_MeshRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_MeshRenderer")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivableWrapperClass_1<
        *mut crate::HoudiniEngineUnity::Test_MeshRenderer,
    >,
> for crate::HoudiniEngineUnity::Test_MeshRenderer {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivableWrapperClass_1<
        *mut crate::HoudiniEngineUnity::Test_MeshRenderer,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_MeshRenderer")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivableWrapperClass_1<
        *mut crate::HoudiniEngineUnity::Test_MeshRenderer,
    >,
> for crate::HoudiniEngineUnity::Test_MeshRenderer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivableWrapperClass_1<
        *mut crate::HoudiniEngineUnity::Test_MeshRenderer,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_MeshRenderer")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::Test_MeshRenderer,
    >,
> for crate::HoudiniEngineUnity::Test_MeshRenderer {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::Test_MeshRenderer,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_MeshRenderer")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::Test_MeshRenderer,
    >,
> for crate::HoudiniEngineUnity::Test_MeshRenderer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::Test_MeshRenderer,
    > {
        unsafe { std::mem::transmute(self) }
    }
}

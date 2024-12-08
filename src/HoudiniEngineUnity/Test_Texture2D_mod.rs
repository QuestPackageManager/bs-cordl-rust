#[cfg(feature = "HoudiniEngineUnity+Test_Texture2D")]
#[repr(C)]
#[derive(Debug)]
pub struct Test_Texture2D {
    __cordl_parent: crate::System::Object,
    pub _cordl_self: *mut crate::UnityEngine::Texture2D,
}
#[cfg(feature = "HoudiniEngineUnity+Test_Texture2D")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::Test_Texture2D =>
    "HoudiniEngineUnity"."Test_Texture2D"
);
#[cfg(feature = "HoudiniEngineUnity+Test_Texture2D")]
impl std::ops::Deref for crate::HoudiniEngineUnity::Test_Texture2D {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_Texture2D")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::Test_Texture2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_Texture2D")]
impl crate::HoudiniEngineUnity::Test_Texture2D {
    pub fn IsEquivalentTo(
        &mut self,
        other: *mut crate::HoudiniEngineUnity::Test_Texture2D,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_self: *mut crate::UnityEngine::Texture2D,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_self))?;
        Ok(__cordl_ret)
    }
    pub fn IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsNull", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_self: *mut crate::UnityEngine::Texture2D,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_self))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_Texture2D")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::Test_Texture2D {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "HoudiniEngineUnity+Test_MeshFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct Test_MeshFilter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cordl_self: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshFilter>,
}
#[cfg(feature = "HoudiniEngineUnity+Test_MeshFilter")]
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::Test_MeshFilter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "Test_MeshFilter";
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
#[cfg(feature = "HoudiniEngineUnity+Test_MeshFilter")]
impl std::ops::Deref for crate::HoudiniEngineUnity::Test_MeshFilter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_MeshFilter")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::Test_MeshFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_MeshFilter")]
impl crate::HoudiniEngineUnity::Test_MeshFilter {
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_MeshFilter>,
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
        _cordl_self: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshFilter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_self))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_self: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshFilter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_MeshFilter")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::Test_MeshFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_MeshFilter")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivableWrapperClass_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_MeshFilter>,
    >,
> for crate::HoudiniEngineUnity::Test_MeshFilter {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivableWrapperClass_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_MeshFilter>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_MeshFilter")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivableWrapperClass_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_MeshFilter>,
    >,
> for crate::HoudiniEngineUnity::Test_MeshFilter {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivableWrapperClass_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_MeshFilter>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_MeshFilter")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_MeshFilter>,
    >,
> for crate::HoudiniEngineUnity::Test_MeshFilter {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_MeshFilter>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_MeshFilter")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_MeshFilter>,
    >,
> for crate::HoudiniEngineUnity::Test_MeshFilter {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_MeshFilter>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}

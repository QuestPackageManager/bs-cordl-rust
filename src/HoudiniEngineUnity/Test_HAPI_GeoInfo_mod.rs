#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_GeoInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct Test_HAPI_GeoInfo {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _cordl_self: crate::HoudiniEngineUnity::HAPI_GeoInfo,
}
#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_GeoInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::Test_HAPI_GeoInfo =>
    "HoudiniEngineUnity"."Test_HAPI_GeoInfo"
);
#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_GeoInfo")]
impl std::ops::Deref for crate::HoudiniEngineUnity::Test_HAPI_GeoInfo {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_GeoInfo")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::Test_HAPI_GeoInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_GeoInfo")]
impl crate::HoudiniEngineUnity::Test_HAPI_GeoInfo {
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_HAPI_GeoInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_self: crate::HoudiniEngineUnity::HAPI_GeoInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_self))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_self: crate::HoudiniEngineUnity::HAPI_GeoInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_GeoInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::Test_HAPI_GeoInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_GeoInfo")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_HAPI_GeoInfo>,
    >,
> for crate::HoudiniEngineUnity::Test_HAPI_GeoInfo {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_HAPI_GeoInfo>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_GeoInfo")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_HAPI_GeoInfo>,
    >,
> for crate::HoudiniEngineUnity::Test_HAPI_GeoInfo {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_HAPI_GeoInfo>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}

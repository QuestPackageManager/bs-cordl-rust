#[cfg(feature = "HEU_ExampleEvergreenQuery")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ExampleEvergreenQuery {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "HEU_ExampleEvergreenQuery")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HEU_ExampleEvergreenQuery => ""
    ."HEU_ExampleEvergreenQuery"
);
#[cfg(feature = "HEU_ExampleEvergreenQuery")]
impl std::ops::Deref for crate::GlobalNamespace::HEU_ExampleEvergreenQuery {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HEU_ExampleEvergreenQuery")]
impl std::ops::DerefMut for crate::GlobalNamespace::HEU_ExampleEvergreenQuery {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HEU_ExampleEvergreenQuery")]
impl crate::GlobalNamespace::HEU_ExampleEvergreenQuery {
    pub fn ChangeParmsAndCook(
        houdiniAsset: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ChangeParmsAndCook", (houdiniAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn CookAsset(
        houdiniAsset: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CookAsset", (houdiniAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn QueryAttribute(
        houdiniAsset: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        >,
        objName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        geoName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        partID: i32,
        attrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "QueryAttribute",
                (houdiniAsset, objName, geoName, partID, attrName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn QueryAttributeByStorageType(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        attrInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
        attrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "QueryAttributeByStorageType",
                (session, geoID, partID, attrInfo, attrName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn QueryGeoParts(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoInfo: quest_hook::libil2cpp::ByRefMut<crate::HoudiniEngineUnity::HAPI_GeoInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QueryGeoParts", (session, geoInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueryHoudiniAsset(
        rootGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QueryHoudiniAsset", (rootGO))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueryObjects(
        houdiniAsset: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QueryObjects", (houdiniAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueryPartAttributeByOwner(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        owner: crate::HoudiniEngineUnity::HAPI_AttributeOwner,
        count: i32,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "QueryPartAttributeByOwner",
                (session, geoID, partID, owner, count, sb),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartQuery() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StartQuery", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HEU_ExampleEvergreenQuery")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HEU_ExampleEvergreenQuery {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

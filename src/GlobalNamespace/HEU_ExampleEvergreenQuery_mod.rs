#[cfg(feature = "HEU_ExampleEvergreenQuery")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ExampleEvergreenQuery {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HEU_ExampleEvergreenQuery")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::HEU_ExampleEvergreenQuery {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "HEU_ExampleEvergreenQuery";
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
#[cfg(feature = "HEU_ExampleEvergreenQuery")]
impl std::ops::Deref for crate::GlobalNamespace::HEU_ExampleEvergreenQuery {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ChangeParmsAndCook")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ChangeParmsAndCook", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (houdiniAsset))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CookAsset(
        houdiniAsset: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CookAsset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CookAsset", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (houdiniAsset))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_HoudiniAsset,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("QueryAttribute")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "QueryAttribute", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked((), (houdiniAsset, objName, geoName, partID, attrName))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_SessionBase,
                    >,
                    i32,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::HoudiniEngineUnity::HAPI_AttributeInfo,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("QueryAttributeByStorageType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "QueryAttributeByStorageType", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (session, geoID, partID, attrInfo, attrName))
        };
        Ok(__cordl_ret.into())
    }
    pub fn QueryGeoParts(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoInfo: quest_hook::libil2cpp::ByRefMut<crate::HoudiniEngineUnity::HAPI_GeoInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_SessionBase,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::HoudiniEngineUnity::HAPI_GeoInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("QueryGeoParts")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "QueryGeoParts", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (session, geoInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn QueryHoudiniAsset(
        rootGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>),
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
                1usize,
            >("QueryHoudiniAsset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "QueryHoudiniAsset", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        > = unsafe { method.invoke_unchecked((), (rootGO)) };
        Ok(__cordl_ret.into())
    }
    pub fn QueryObjects(
        houdiniAsset: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("QueryObjects")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "QueryObjects", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (houdiniAsset))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_SessionBase,
                    >,
                    i32,
                    i32,
                    crate::HoudiniEngineUnity::HAPI_AttributeOwner,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("QueryPartAttributeByOwner")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "QueryPartAttributeByOwner", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (session, geoID, partID, owner, count, sb))
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartQuery() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("StartQuery")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "StartQuery", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
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

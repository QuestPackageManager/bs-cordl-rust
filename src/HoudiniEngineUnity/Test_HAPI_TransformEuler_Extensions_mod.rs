#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_TransformEuler_Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct Test_HAPI_TransformEuler_Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_TransformEuler_Extensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::Test_HAPI_TransformEuler_Extensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "Test_HAPI_TransformEuler_Extensions";
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
#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_TransformEuler_Extensions")]
impl std::ops::Deref for crate::HoudiniEngineUnity::Test_HAPI_TransformEuler_Extensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_TransformEuler_Extensions")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::Test_HAPI_TransformEuler_Extensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_TransformEuler_Extensions")]
impl crate::HoudiniEngineUnity::Test_HAPI_TransformEuler_Extensions {
    pub fn ToTestObject(
        _cordl_self: crate::HoudiniEngineUnity::HAPI_TransformEuler,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_HAPI_TransformEuler>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::HoudiniEngineUnity::HAPI_TransformEuler),
                        quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::Test_HAPI_TransformEuler,
                        >,
                        1usize,
                    >("ToTestObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToTestObject", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::Test_HAPI_TransformEuler,
        > = unsafe { cordl_method_info.invoke_unchecked((), (_cordl_self))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_TransformEuler_Extensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::Test_HAPI_TransformEuler_Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

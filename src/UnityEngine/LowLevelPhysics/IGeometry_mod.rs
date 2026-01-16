#[cfg(feature = "cordl_class_UnityEngine+LowLevelPhysics+IGeometry")]
#[repr(C)]
#[derive(Debug)]
pub struct IGeometry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+LowLevelPhysics+IGeometry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::LowLevelPhysics::IGeometry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.LowLevelPhysics";
    const CLASS_NAME: &'static str = "IGeometry";
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
#[cfg(feature = "UnityEngine+LowLevelPhysics+IGeometry")]
impl std::ops::Deref for crate::UnityEngine::LowLevelPhysics::IGeometry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LowLevelPhysics+IGeometry")]
impl std::ops::DerefMut for crate::UnityEngine::LowLevelPhysics::IGeometry {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LowLevelPhysics+IGeometry")]
impl crate::UnityEngine::LowLevelPhysics::IGeometry {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_GeometryType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::LowLevelPhysics::GeometryType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::LowLevelPhysics::GeometryType,
                        0usize,
                    >("get_GeometryType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_GeometryType", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::LowLevelPhysics::GeometryType = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+LowLevelPhysics+IGeometry")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::LowLevelPhysics::IGeometry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

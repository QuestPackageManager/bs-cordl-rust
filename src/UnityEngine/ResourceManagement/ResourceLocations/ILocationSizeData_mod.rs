#[cfg(feature = "cordl_class_UnityEngine+ResourceManagement+ResourceLocations+ILocationSizeData")]
#[derive(Debug)]
#[repr(C)]
pub struct ILocationSizeData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+ResourceManagement+ResourceLocations+ILocationSizeData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::ResourceManagement::ResourceLocations::ILocationSizeData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.ResourceLocations";
    const CLASS_NAME: &'static str = "ILocationSizeData";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceLocations+ILocationSizeData")]
impl std::ops::Deref
    for crate::UnityEngine::ResourceManagement::ResourceLocations::ILocationSizeData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceLocations+ILocationSizeData")]
impl std::ops::DerefMut
    for crate::UnityEngine::ResourceManagement::ResourceLocations::ILocationSizeData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceLocations+ILocationSizeData")]
impl crate::UnityEngine::ResourceManagement::ResourceLocations::ILocationSizeData {
    pub fn ComputeSize(
        &mut self,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        resourceManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceManager,
                            >,
                        ),
                        i64,
                        2usize,
                    >("ComputeSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeSize", 2usize
                        )
                    })
            });
        let __cordl_ret: i64 =
            unsafe { cordl_method_info.invoke_unchecked(self, (location, resourceManager))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ResourceManagement+ResourceLocations+ILocationSizeData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::ResourceManagement::ResourceLocations::ILocationSizeData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

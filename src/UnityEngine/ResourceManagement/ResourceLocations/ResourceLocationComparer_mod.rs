#[cfg(
    feature = "cordl_class_UnityEngine+ResourceManagement+ResourceLocations+ResourceLocationComparer"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceLocationComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_UnityEngine+ResourceManagement+ResourceLocations+ResourceLocationComparer"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationComparer
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.ResourceLocations";
    const CLASS_NAME: &'static str = "ResourceLocationComparer";
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceLocations+ResourceLocationComparer")]
impl std::ops::Deref
    for crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationComparer
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceLocations+ResourceLocationComparer")]
impl std::ops::DerefMut
    for crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationComparer
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceLocations+ResourceLocationComparer")]
impl crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationComparer {
    pub fn Equals(
        &mut self,
        x: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        y: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
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
                                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                            >,
                        ),
                        bool,
                        2usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                        >),
                        i32,
                        1usize,
                    >("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHashCode", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+ResourceManagement+ResourceLocations+ResourceLocationComparer"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationComparer
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceLocations+ResourceLocationComparer")]
impl
    AsRef<
        crate::System::Collections::Generic::IEqualityComparer_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        >,
    > for crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationComparer
{
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEqualityComparer_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceLocations+ResourceLocationComparer")]
impl
    AsMut<
        crate::System::Collections::Generic::IEqualityComparer_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        >,
    > for crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationComparer
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEqualityComparer_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}

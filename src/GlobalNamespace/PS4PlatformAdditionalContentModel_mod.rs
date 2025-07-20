#[cfg(feature = "PS4PlatformAdditionalContentModel")]
#[repr(C)]
#[derive(Debug)]
pub struct PS4PlatformAdditionalContentModel {
    __cordl_parent: crate::GlobalNamespace::SonyPlatformAdditionalContentModel,
}
#[cfg(feature = "PS4PlatformAdditionalContentModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PS4PlatformAdditionalContentModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PS4PlatformAdditionalContentModel";
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
#[cfg(feature = "PS4PlatformAdditionalContentModel")]
impl std::ops::Deref for crate::GlobalNamespace::PS4PlatformAdditionalContentModel {
    type Target = crate::GlobalNamespace::SonyPlatformAdditionalContentModel;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PS4PlatformAdditionalContentModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::PS4PlatformAdditionalContentModel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PS4PlatformAdditionalContentModel")]
impl crate::GlobalNamespace::PS4PlatformAdditionalContentModel {
    pub fn New(
        sonyCommerceHelper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISonyCommerceHelper,
        >,
        sonyLevelProductCollectionModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyLevelProductCollectionModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (sonyCommerceHelper, sonyLevelProductCollectionModel),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        sonyCommerceHelper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISonyCommerceHelper,
        >,
        sonyLevelProductCollectionModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyLevelProductCollectionModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ISonyCommerceHelper,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::SonyLevelProductCollectionModel,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (sonyCommerceHelper, sonyLevelProductCollectionModel),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PS4PlatformAdditionalContentModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PS4PlatformAdditionalContentModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

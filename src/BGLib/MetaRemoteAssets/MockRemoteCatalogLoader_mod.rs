#[cfg(feature = "BGLib+MetaRemoteAssets+MockRemoteCatalogLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct MockRemoteCatalogLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MockRemoteCatalogLoader")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::MetaRemoteAssets::MockRemoteCatalogLoader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.MetaRemoteAssets";
    const CLASS_NAME: &'static str = "MockRemoteCatalogLoader";
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
#[cfg(feature = "BGLib+MetaRemoteAssets+MockRemoteCatalogLoader")]
impl std::ops::Deref for crate::BGLib::MetaRemoteAssets::MockRemoteCatalogLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MockRemoteCatalogLoader")]
impl std::ops::DerefMut for crate::BGLib::MetaRemoteAssets::MockRemoteCatalogLoader {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MockRemoteCatalogLoader")]
impl crate::BGLib::MetaRemoteAssets::MockRemoteCatalogLoader {
    pub fn LoadRemoteCatalogAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::System::Threading::CancellationToken),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<bool>,
                        >,
                        1usize,
                    >("LoadRemoteCatalogAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadRemoteCatalogAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = unsafe { method.invoke_unchecked(self, (cancellationToken))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MockRemoteCatalogLoader")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::MetaRemoteAssets::MockRemoteCatalogLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MockRemoteCatalogLoader")]
impl AsRef<crate::BGLib::MetaRemoteAssets::IRemoteCatalogLoader>
for crate::BGLib::MetaRemoteAssets::MockRemoteCatalogLoader {
    fn as_ref(&self) -> &crate::BGLib::MetaRemoteAssets::IRemoteCatalogLoader {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MockRemoteCatalogLoader")]
impl AsMut<crate::BGLib::MetaRemoteAssets::IRemoteCatalogLoader>
for crate::BGLib::MetaRemoteAssets::MockRemoteCatalogLoader {
    fn as_mut(&mut self) -> &mut crate::BGLib::MetaRemoteAssets::IRemoteCatalogLoader {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct BundledAssetProvider {
    __cordl_parent: crate::UnityEngine::ResourceManagement::ResourceProviders::ResourceProviderBase,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.ResourceProviders";
    const CLASS_NAME: &'static str = "BundledAssetProvider";
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider {
    type Target = crate::UnityEngine::ResourceManagement::ResourceProviders::ResourceProviderBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider")]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider {
    #[cfg(
        feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider+InternalOp"
    )]
    pub type InternalOp = crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider_InternalOp;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Provide(
        &mut self,
        provideHandle: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Provide")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Provide", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (provideHandle))
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider+InternalOp"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BundledAssetProvider_InternalOp {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_AssetBundle: quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
    pub m_PreloadRequest: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AssetBundleRequest,
    >,
    pub m_RequestOperation: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AssetBundleRequest,
    >,
    pub m_Result: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_ProvideHandle: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
    pub subObjectName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider+InternalOp"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider_InternalOp {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.ResourceProviders";
    const CLASS_NAME: &'static str = "BundledAssetProvider/InternalOp";
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
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider+InternalOp"
)]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider_InternalOp {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider+InternalOp"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider_InternalOp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider+InternalOp"
)]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider_InternalOp {
    pub fn ActionComplete(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ActionComplete")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ActionComplete", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn BeginAssetLoad(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("BeginAssetLoad")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BeginAssetLoad", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompleteOperation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CompleteOperation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompleteOperation", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetArrayResult(
        &mut self,
        allAssets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GetArrayResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetArrayResult", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (allAssets))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetResult(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GetAssetResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetAssetResult", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (asset))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetSubObjectResult(
        &mut self,
        allAssets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GetAssetSubObjectResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetAssetSubObjectResult", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (allAssets))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetListResult(
        &mut self,
        allAssets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GetListResult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetListResult", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (allAssets))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadBundleFromDependecies<T>(
        results: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IList_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                >),
                T,
                1usize,
            >("LoadBundleFromDependecies")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LoadBundleFromDependecies", 1usize
                )
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked((), (results)) };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProgressCallback(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("ProgressCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProgressCallback", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
        provideHandle: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Start")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Start", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (provideHandle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WaitForCompletionHandler(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("WaitForCompletionHandler")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WaitForCompletionHandler", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn _Start_b__7_0(
        &mut self,
        operation: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("<Start>b__7_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "<Start>b__7_0", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (operation))
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
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider+InternalOp"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider_InternalOp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

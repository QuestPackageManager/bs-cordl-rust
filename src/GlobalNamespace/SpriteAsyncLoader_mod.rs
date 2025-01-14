#[cfg(feature = "SpriteAsyncLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct SpriteAsyncLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _lruCache: quest_hook::libil2cpp::Gc<
        crate::BGLib::DotnetExtension::Collections::LRUCache_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _referenceCountingCache: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ReferenceCountingCache_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<
                crate::System::Threading::Tasks::Task_1<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                >,
            >,
        >,
    >,
    pub _loadFunc: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            crate::System::Threading::CancellationToken,
            quest_hook::libil2cpp::Gc<
                crate::System::Threading::Tasks::Task_1<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                >,
            >,
        >,
    >,
    pub _destroyFunc: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>>,
    >,
}
#[cfg(feature = "SpriteAsyncLoader")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::SpriteAsyncLoader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SpriteAsyncLoader";
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
#[cfg(feature = "SpriteAsyncLoader")]
impl std::ops::Deref for crate::GlobalNamespace::SpriteAsyncLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SpriteAsyncLoader")]
impl std::ops::DerefMut for crate::GlobalNamespace::SpriteAsyncLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SpriteAsyncLoader")]
impl crate::GlobalNamespace::SpriteAsyncLoader {
    pub fn ClearCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ClearCache")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ClearCache", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn DestroySprite(
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DestroySprite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DestroySprite", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (sprite))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DestroySpriteTask(
        &mut self,
        spriteTask: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DestroySpriteTask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DestroySpriteTask", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (spriteTask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleItemWillBeRemovedFromCache(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl__: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleItemWillBeRemovedFromCache")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleItemWillBeRemovedFromCache", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (path, _cordl__))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSpriteAsync(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                    >,
                >,
                1usize,
            >("LoadSpriteAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LoadSpriteAsync", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
            >,
        > = unsafe { method.invoke_unchecked(self, (path)) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        loadFunc: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                crate::System::Threading::CancellationToken,
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                    >,
                >,
            >,
        >,
        destroyFunc: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
            >,
        >,
        cacheSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (loadFunc, destroyFunc, cacheSize))?;
        Ok(__cordl_object.into())
    }
    pub fn UnloadSprite_IReferenceCountingCache_2_Il2CppString1(
        &mut self,
        cache: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReferenceCountingCache_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                    >,
                >,
            >,
        >,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IReferenceCountingCache_2<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task_1<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                                >,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("UnloadSprite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnloadSprite", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (cache, path))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSprite_Il2CppString0(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UnloadSprite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnloadSprite", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (path))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        loadFunc: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                crate::System::Threading::CancellationToken,
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                    >,
                >,
            >,
        >,
        destroyFunc: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
            >,
        >,
        cacheSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_3<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Threading::CancellationToken,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task_1<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                                >,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        >,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (loadFunc, destroyFunc, cacheSize))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SpriteAsyncLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SpriteAsyncLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

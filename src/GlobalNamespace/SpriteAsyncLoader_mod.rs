#[cfg(feature = "SpriteAsyncLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct SpriteAsyncLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _lruCache: quest_hook::libil2cpp::Gc<
        crate::BGLib::DotnetExtension::Collections::LRUCache_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub _referenceCountingCache: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ReferenceCountingCache_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::Sprite>,
        >,
    >,
    pub _loadFunc: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<
            *mut quest_hook::libil2cpp::Il2CppString,
            crate::System::Threading::CancellationToken,
            *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::Sprite>,
        >,
    >,
    pub _destroyFunc: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut crate::UnityEngine::Sprite>,
    >,
}
#[cfg(feature = "SpriteAsyncLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SpriteAsyncLoader => ""
    ."SpriteAsyncLoader"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearCache", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroySprite(
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroySprite", (sprite))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroySpriteTask(
        &mut self,
        spriteTask: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::Sprite>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroySpriteTask", (spriteTask))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleItemWillBeRemovedFromCache(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl__: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleItemWillBeRemovedFromCache", (path, _cordl__))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSpriteAsync(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::Sprite>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::Sprite>,
        > = __cordl_object.invoke("LoadSpriteAsync", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        loadFunc: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                *mut quest_hook::libil2cpp::Il2CppString,
                crate::System::Threading::CancellationToken,
                *mut crate::System::Threading::Tasks::Task_1<
                    *mut crate::UnityEngine::Sprite,
                >,
            >,
        >,
        destroyFunc: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::Sprite>,
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
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::System::Threading::Tasks::Task_1<
                    *mut crate::UnityEngine::Sprite,
                >,
            >,
        >,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnloadSprite", (cache, path))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSprite_Il2CppString0(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnloadSprite", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        loadFunc: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                *mut quest_hook::libil2cpp::Il2CppString,
                crate::System::Threading::CancellationToken,
                *mut crate::System::Threading::Tasks::Task_1<
                    *mut crate::UnityEngine::Sprite,
                >,
            >,
        >,
        destroyFunc: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::Sprite>,
        >,
        cacheSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (loadFunc, destroyFunc, cacheSize))?;
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

#[cfg(feature = "SpriteAsyncLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct SpriteAsyncLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cache: *mut crate::BGLib::DotnetExtension::Collections::LRUCache_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::UnityEngine::Sprite,
    >,
    pub _loadingTasks: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::Sprite>,
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
    #[cfg(feature = "SpriteAsyncLoader+_LoadSpriteAsyncInternal_d__4")]
    pub type _LoadSpriteAsyncInternal_d__4 = crate::GlobalNamespace::SpriteAsyncLoader__LoadSpriteAsyncInternal_d__4;
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
    pub fn LoadSpriteAsync(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
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
        > = __cordl_object.invoke("LoadSpriteAsync", (path, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSpriteAsyncInternal(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
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
        > = __cordl_object.invoke("LoadSpriteAsyncInternal", (path, cancellationToken))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
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

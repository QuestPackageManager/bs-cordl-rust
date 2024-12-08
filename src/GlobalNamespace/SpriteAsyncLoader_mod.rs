#[cfg(feature = "SpriteAsyncLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct SpriteAsyncLoader {
    __cordl_parent: crate::System::Object,
    pub _cache: *mut crate::BGLib::DotnetExtension::Collections::LRUCache_2<
        *mut crate::System::String,
        *mut crate::UnityEngine::Sprite,
    >,
    pub _loadingTasks: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::Sprite>,
    >,
}
#[cfg(feature = "SpriteAsyncLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SpriteAsyncLoader => ""."SpriteAsyncLoader"
);
#[cfg(feature = "SpriteAsyncLoader")]
impl std::ops::Deref for SpriteAsyncLoader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SpriteAsyncLoader")]
impl std::ops::DerefMut for SpriteAsyncLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SpriteAsyncLoader")]
impl SpriteAsyncLoader {
    #[cfg(feature = "SpriteAsyncLoader+_LoadSpriteAsyncInternal_d__4")]
    pub type _LoadSpriteAsyncInternal_d__4 = crate::GlobalNamespace::SpriteAsyncLoader__LoadSpriteAsyncInternal_d__4;
    pub fn LoadSpriteAsyncInternal(
        &mut self,
        path: *mut crate::System::String,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::Sprite>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::UnityEngine::Sprite,
        > = __cordl_object.invoke("LoadSpriteAsyncInternal", (path, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn LoadSpriteAsync(
        &mut self,
        path: *mut crate::System::String,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::Sprite>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::UnityEngine::Sprite,
        > = __cordl_object.invoke("LoadSpriteAsync", (path, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn ClearCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearCache", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "SpriteAsyncLoader")]
impl quest_hook::libil2cpp::ObjectType for SpriteAsyncLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

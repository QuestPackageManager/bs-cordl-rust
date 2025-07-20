#[cfg(feature = "UnityEngine+U2D+SpriteAtlasManager")]
#[repr(C)]
#[derive(Debug)]
pub struct SpriteAtlasManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+U2D+SpriteAtlasManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::U2D::SpriteAtlasManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.U2D";
    const CLASS_NAME: &'static str = "SpriteAtlasManager";
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
#[cfg(feature = "UnityEngine+U2D+SpriteAtlasManager")]
impl std::ops::Deref for crate::UnityEngine::U2D::SpriteAtlasManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+U2D+SpriteAtlasManager")]
impl std::ops::DerefMut for crate::UnityEngine::U2D::SpriteAtlasManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+U2D+SpriteAtlasManager")]
impl crate::UnityEngine::U2D::SpriteAtlasManager {
    pub fn PostRegisteredAtlas(
        spriteAtlas: quest_hook::libil2cpp::Gc<crate::UnityEngine::U2D::SpriteAtlas>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::U2D::SpriteAtlasManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::U2D::SpriteAtlas>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("PostRegisteredAtlas")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::U2D::SpriteAtlasManager as
                    quest_hook::libil2cpp::Type > ::class(), "PostRegisteredAtlas",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (spriteAtlas))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Register(
        spriteAtlas: quest_hook::libil2cpp::Gc<crate::UnityEngine::U2D::SpriteAtlas>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::U2D::SpriteAtlasManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::U2D::SpriteAtlas>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Register")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::U2D::SpriteAtlasManager as
                    quest_hook::libil2cpp::Type > ::class(), "Register", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (spriteAtlas))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAtlas(
        tag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::U2D::SpriteAtlasManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("RequestAtlas")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::U2D::SpriteAtlasManager as
                    quest_hook::libil2cpp::Type > ::class(), "RequestAtlas", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (tag))? };
        Ok(__cordl_ret.into())
    }
    pub fn add_atlasRegistered(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::U2D::SpriteAtlas>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::U2D::SpriteAtlasManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::U2D::SpriteAtlas>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_atlasRegistered")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::U2D::SpriteAtlasManager as
                    quest_hook::libil2cpp::Type > ::class(), "add_atlasRegistered",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_atlasRegistered(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::U2D::SpriteAtlas>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::U2D::SpriteAtlasManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::U2D::SpriteAtlas>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_atlasRegistered")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::U2D::SpriteAtlasManager as
                    quest_hook::libil2cpp::Type > ::class(), "remove_atlasRegistered",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+U2D+SpriteAtlasManager")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::U2D::SpriteAtlasManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "UnityEngine+UIElements+TextureRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct TextureRegistry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Textures: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::TextureRegistry_TextureInfo,
        >,
    >,
    pub m_TextureToId: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
            crate::UnityEngine::UIElements::TextureId,
        >,
    >,
    pub m_FreeIds: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Stack_1<
            crate::UnityEngine::UIElements::TextureId,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::TextureRegistry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "TextureRegistry";
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
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TextureRegistry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::TextureRegistry {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry")]
impl crate::UnityEngine::UIElements::TextureRegistry {
    pub const maxTextures: i32 = 2048i32;
    #[cfg(feature = "UnityEngine+UIElements+TextureRegistry+TextureInfo")]
    pub type TextureInfo = crate::UnityEngine::UIElements::TextureRegistry_TextureInfo;
    pub fn Acquire(
        &mut self,
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TextureId> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>),
                        crate::UnityEngine::UIElements::TextureId,
                        1usize,
                    >("Acquire")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Acquire", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::TextureId = unsafe {
            method.invoke_unchecked(self, (tex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocAndAcquire(
        &mut self,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dynamic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TextureId> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>, bool),
                        crate::UnityEngine::UIElements::TextureId,
                        2usize,
                    >("AllocAndAcquire")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AllocAndAcquire", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::TextureId = unsafe {
            method.invoke_unchecked(self, (texture, dynamic))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocAndAcquireDynamic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TextureId> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::TextureId,
                        0usize,
                    >("AllocAndAcquireDynamic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AllocAndAcquireDynamic", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::TextureId = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTexture(
        &mut self,
        id: crate::UnityEngine::UIElements::TextureId,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::TextureId),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        1usize,
                    >("GetTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetTexture", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = unsafe {
            method.invoke_unchecked(self, (id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
        id: crate::UnityEngine::UIElements::TextureId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::TextureId),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Release")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Release", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateDynamic(
        &mut self,
        id: crate::UnityEngine::UIElements::TextureId,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::TextureId,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("UpdateDynamic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "UpdateDynamic", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (id, texture))?
        };
        Ok(__cordl_ret.into())
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
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextureRegistry>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::TextureRegistry,
                        >,
                        0usize,
                    >("get_instance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_instance", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TextureRegistry,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TextureRegistry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry+TextureInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextureRegistry_TextureInfo {
    pub texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub dynamic: bool,
    pub refCount: i32,
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry+TextureInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::TextureRegistry_TextureInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "TextureRegistry/TextureInfo";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry+TextureInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::TextureRegistry_TextureInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry+TextureInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::TextureRegistry_TextureInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry+TextureInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::TextureRegistry_TextureInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry+TextureInfo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::TextureRegistry_TextureInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry+TextureInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::TextureRegistry_TextureInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry+TextureInfo")]
impl crate::UnityEngine::UIElements::TextureRegistry_TextureInfo {}

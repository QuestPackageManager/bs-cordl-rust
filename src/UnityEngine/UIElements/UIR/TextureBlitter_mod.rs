#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter")]
#[repr(C)]
#[derive(Debug)]
pub struct TextureBlitter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_SingleBlit: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo,
        >,
    >,
    pub m_BlitMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub m_Properties: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::MaterialPropertyBlock,
    >,
    pub m_Viewport: crate::UnityEngine::RectInt,
    pub m_PrevRT: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    pub m_PendingBlits: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo,
        >,
    >,
    pub _disposed_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::TextureBlitter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "TextureBlitter";
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
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::TextureBlitter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::TextureBlitter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter")]
impl crate::UnityEngine::UIElements::UIR::TextureBlitter {
    #[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter+BlitInfo")]
    pub type BlitInfo = crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo;
    pub fn BeginBlit(
        &mut self,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("BeginBlit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BeginBlit", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dst))
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitOneNow(
        &mut self,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        srcRect: crate::UnityEngine::RectInt,
        dstPos: crate::UnityEngine::Vector2Int,
        addBorder: bool,
        tint: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                    crate::UnityEngine::RectInt,
                    crate::UnityEngine::Vector2Int,
                    bool,
                    crate::UnityEngine::Color,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("BlitOneNow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BlitOneNow", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dst, src, srcRect, dstPos, addBorder, tint))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Commit(
        &mut self,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Commit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Commit", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dst))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (disposing))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoBlit(
        &mut self,
        blitInfos: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo,
            >,
        >,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<
                            crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo,
                        >,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("DoBlit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DoBlit", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (blitInfos, startIndex))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndBlit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("EndBlit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EndBlit", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (capacity))?;
        Ok(__cordl_object.into())
    }
    pub fn QueueBlit(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        srcRect: crate::UnityEngine::RectInt,
        dstPos: crate::UnityEngine::Vector2Int,
        addBorder: bool,
        tint: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                    crate::UnityEngine::RectInt,
                    crate::UnityEngine::Vector2Int,
                    bool,
                    crate::UnityEngine::Color,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("QueueBlit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "QueueBlit", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (src, srcRect, dstPos, addBorder, tint))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (capacity))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_disposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_disposed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_disposed", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_disposed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_disposed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_disposed", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::TextureBlitter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UIR::TextureBlitter {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UIR::TextureBlitter {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter+BlitInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextureBlitter_BlitInfo {
    pub src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub srcRect: crate::UnityEngine::RectInt,
    pub dstPos: crate::UnityEngine::Vector2Int,
    pub border: i32,
    pub tint: crate::UnityEngine::Color,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter+BlitInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "TextureBlitter/BlitInfo";
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
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter+BlitInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter+BlitInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo {
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
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter+BlitInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo {
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
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter+BlitInfo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo {
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
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter+BlitInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter+BlitInfo")]
impl crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo {}

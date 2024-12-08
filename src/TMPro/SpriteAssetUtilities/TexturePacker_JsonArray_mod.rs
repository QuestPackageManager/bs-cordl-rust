#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Frame")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TexturePacker_JsonArray_Frame {
    pub filename: *mut crate::System::String,
    pub frame: crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteFrame,
    pub rotated: bool,
    pub trimmed: bool,
    pub spriteSourceSize: crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteFrame,
    pub sourceSize: crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteSize,
    pub pivot: crate::UnityEngine::Vector2,
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Frame")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Frame =>
    "TMPro.SpriteAssetUtilities"."TexturePacker_JsonArray/Frame"
);
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Frame")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Frame {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Frame")]
impl crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Frame {}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Meta")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TexturePacker_JsonArray_Meta {
    pub app: *mut crate::System::String,
    pub version: *mut crate::System::String,
    pub image: *mut crate::System::String,
    pub format: *mut crate::System::String,
    pub _cordl_size: crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteSize,
    pub scale: f32,
    pub smartupdate: *mut crate::System::String,
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Meta")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Meta =>
    "TMPro.SpriteAssetUtilities"."TexturePacker_JsonArray/Meta"
);
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Meta")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Meta {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Meta")]
impl crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Meta {}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteDataObject")]
#[repr(C)]
#[derive(Debug)]
pub struct TexturePacker_JsonArray_SpriteDataObject {
    __cordl_parent: crate::System::Object,
    pub frames: *mut crate::System::Collections::Generic::List_1<
        crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Frame,
    >,
    pub meta: crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Meta,
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteDataObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteDataObject =>
    "TMPro.SpriteAssetUtilities"."TexturePacker_JsonArray/SpriteDataObject"
);
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteDataObject")]
impl std::ops::Deref
for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteDataObject {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteDataObject")]
impl std::ops::DerefMut
for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteDataObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteDataObject")]
impl crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteDataObject {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteDataObject")]
impl quest_hook::libil2cpp::ObjectType
for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteDataObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteFrame")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TexturePacker_JsonArray_SpriteFrame {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteFrame")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteFrame =>
    "TMPro.SpriteAssetUtilities"."TexturePacker_JsonArray/SpriteFrame"
);
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteFrame")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteFrame {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteFrame")]
impl crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteFrame {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteSize")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TexturePacker_JsonArray_SpriteSize {
    pub w: f32,
    pub h: f32,
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteSize")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteSize =>
    "TMPro.SpriteAssetUtilities"."TexturePacker_JsonArray/SpriteSize"
);
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteSize")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteSize {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteSize")]
impl crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteSize {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray")]
#[repr(C)]
#[derive(Debug)]
pub struct TexturePacker_JsonArray {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray =>
    "TMPro.SpriteAssetUtilities"."TexturePacker_JsonArray"
);
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray")]
impl std::ops::Deref for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray")]
impl std::ops::DerefMut for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray")]
impl crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray {
    #[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Frame")]
    pub type Frame = crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Frame;
    #[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Meta")]
    pub type Meta = crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Meta;
    #[cfg(
        feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteDataObject"
    )]
    pub type SpriteDataObject = crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteDataObject;
    #[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteFrame")]
    pub type SpriteFrame = crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteFrame;
    #[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteSize")]
    pub type SpriteSize = crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteSize;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray")]
impl quest_hook::libil2cpp::ObjectType
for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

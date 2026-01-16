#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray")]
#[repr(C)]
#[derive(Debug)]
pub struct TexturePacker_JsonArray {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro.SpriteAssetUtilities";
    const CLASS_NAME: &'static str = "TexturePacker_JsonArray";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray")]
impl std::ops::Deref for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray")]
impl std::ops::DerefMut for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray")]
impl crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray {
    #[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Frame")]
    pub type Frame = crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Frame;
    #[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Meta")]
    pub type Meta = crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Meta;
    #[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteDataObject")]
    pub type SpriteDataObject =
        crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteDataObject;
    #[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteFrame")]
    pub type SpriteFrame = crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteFrame;
    #[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteSize")]
    pub type SpriteSize = crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteSize;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray")]
impl quest_hook::libil2cpp::ObjectType
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Frame")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct TexturePacker_JsonArray_Frame {
    pub filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub frame: crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteFrame,
    pub rotated: bool,
    pub trimmed: bool,
    pub spriteSourceSize: crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteFrame,
    pub sourceSize: crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteSize,
    pub pivot: crate::UnityEngine::Vector2,
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Frame")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Frame
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro.SpriteAssetUtilities";
    const CLASS_NAME: &'static str = "TexturePacker_JsonArray/Frame";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Frame")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Frame
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Frame")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Frame
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Frame")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Frame
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Frame")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Frame
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Frame")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Frame
{
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
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Meta")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct TexturePacker_JsonArray_Meta {
    pub app: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub image: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _cordl_size: crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteSize,
    pub scale: f32,
    pub smartupdate: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Meta")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Meta
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro.SpriteAssetUtilities";
    const CLASS_NAME: &'static str = "TexturePacker_JsonArray/Meta";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Meta")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Meta
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Meta")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Meta
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Meta")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Meta
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Meta")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Meta
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+Meta")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Meta
{
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
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteDataObject")]
#[repr(C)]
#[derive(Debug)]
pub struct TexturePacker_JsonArray_SpriteDataObject {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub frames: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Frame,
        >,
    >,
    pub meta: crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_Meta,
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteDataObject")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteDataObject
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro.SpriteAssetUtilities";
    const CLASS_NAME: &'static str = "TexturePacker_JsonArray/SpriteDataObject";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteDataObject")]
impl std::ops::Deref
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteDataObject
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteDataObject")]
impl std::ops::DerefMut
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteDataObject
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteDataObject")]
impl crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteDataObject {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteDataObject")]
impl quest_hook::libil2cpp::ObjectType
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteDataObject
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteFrame")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct TexturePacker_JsonArray_SpriteFrame {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteFrame")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteFrame
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro.SpriteAssetUtilities";
    const CLASS_NAME: &'static str = "TexturePacker_JsonArray/SpriteFrame";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteFrame")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteFrame
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteFrame")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteFrame
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteFrame")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteFrame
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteFrame")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteFrame
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteFrame")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteFrame
{
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteSize")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct TexturePacker_JsonArray_SpriteSize {
    pub w: f32,
    pub h: f32,
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteSize")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteSize
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro.SpriteAssetUtilities";
    const CLASS_NAME: &'static str = "TexturePacker_JsonArray/SpriteSize";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteSize")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteSize
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteSize")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteSize
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteSize")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteSize
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteSize")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteSize
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_TMPro+SpriteAssetUtilities+TexturePacker_JsonArray+SpriteSize")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::TMPro::SpriteAssetUtilities::TexturePacker_JsonArray_SpriteSize
{
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}

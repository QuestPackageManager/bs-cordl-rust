#[cfg(feature = "cordl_class_NoteCutter")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _colliders: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>>,
    >,
    pub _cuttableBySaberSortParams: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParams>,
        >,
    >,
    pub _comparer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer,
    >,
}
#[cfg(feature = "cordl_class_NoteCutter")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::NoteCutter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteCutter";
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
#[cfg(feature = "NoteCutter")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutter")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutter")]
impl crate::GlobalNamespace::NoteCutter {
    pub const kMaxNumberOfColliders: i32 = 16i32;
    #[cfg(feature = "NoteCutter+CuttableBySaberSortParams")]
    pub type CuttableBySaberSortParams =
        crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParams;
    #[cfg(feature = "NoteCutter+CuttableBySaberSortParamsComparer")]
    pub type CuttableBySaberSortParamsComparer =
        crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer;
    pub fn Cut(
        &mut self,
        saber: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Cut")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Cut",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (saber))? };
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "cordl_class_NoteCutter")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NoteCutter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_NoteCutter+CuttableBySaberSortParams")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutter_CuttableBySaberSortParams {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub cuttableBySaber: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::CuttableBySaber>,
    pub distance: f32,
    pub pos: crate::UnityEngine::Vector3,
}
#[cfg(feature = "cordl_class_NoteCutter+CuttableBySaberSortParams")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParams
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteCutter/CuttableBySaberSortParams";
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
#[cfg(feature = "NoteCutter+CuttableBySaberSortParams")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParams {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParams")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParams {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParams")]
impl crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParams {
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
#[cfg(feature = "cordl_class_NoteCutter+CuttableBySaberSortParams")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParams
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_NoteCutter+CuttableBySaberSortParamsComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutter_CuttableBySaberSortParamsComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_NoteCutter+CuttableBySaberSortParamsComparer")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteCutter/CuttableBySaberSortParamsComparer";
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
#[cfg(feature = "NoteCutter+CuttableBySaberSortParamsComparer")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParamsComparer")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParamsComparer")]
impl crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer {
    pub fn Compare(
        &mut self,
        p0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        p1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), i32, 2usize>("Compare")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Compare",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (p0, p1))? };
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "cordl_class_NoteCutter+CuttableBySaberSortParamsComparer")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParamsComparer")]
impl AsRef<crate::System::Collections::IComparer>
    for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer
{
    fn as_ref(&self) -> &crate::System::Collections::IComparer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParamsComparer")]
impl AsMut<crate::System::Collections::IComparer>
    for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer
{
    fn as_mut(&mut self) -> &mut crate::System::Collections::IComparer {
        unsafe { std::mem::transmute(self) }
    }
}

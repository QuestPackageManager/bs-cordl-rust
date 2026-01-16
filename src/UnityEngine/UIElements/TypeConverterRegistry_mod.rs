#[cfg(feature = "cordl_class_UnityEngine+UIElements+TypeConverterRegistry")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct TypeConverterRegistry {
    pub m_Converters: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey,
            quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TypeConverterRegistry")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::TypeConverterRegistry {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "TypeConverterRegistry";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TypeConverterRegistry")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::TypeConverterRegistry
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TypeConverterRegistry")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::TypeConverterRegistry
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TypeConverterRegistry")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::TypeConverterRegistry
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TypeConverterRegistry")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::TypeConverterRegistry
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TypeConverterRegistry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::TypeConverterRegistry
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TypeConverterRegistry")]
impl crate::UnityEngine::UIElements::TypeConverterRegistry {
    #[cfg(feature = "UnityEngine+UIElements+TypeConverterRegistry+ConverterKey")]
    pub type ConverterKey = crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey;
    #[cfg(feature = "UnityEngine+UIElements+TypeConverterRegistry+ConverterKeyComparer")]
    pub type ConverterKeyComparer =
        crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKeyComparer;
    pub fn Apply(
        &mut self,
        registry: crate::UnityEngine::UIElements::TypeConverterRegistry,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::TypeConverterRegistry),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Apply")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Apply",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (registry))? };
        Ok(__cordl_ret.into())
    }
    pub fn Create(
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TypeConverterRegistry> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::UIElements::TypeConverterRegistry,
                        0usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::TypeConverterRegistry =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        x: crate::UnityEngine::UIElements::TypeConverterRegistry,
        y: crate::UnityEngine::UIElements::TypeConverterRegistry,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::TypeConverterRegistry,
                        crate::UnityEngine::UIElements::TypeConverterRegistry,
                    ), bool, 2usize>("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetConverter(
        &mut self,
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        destination: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Delegate>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    ), quest_hook::libil2cpp::Gc<crate::System::Delegate>, 2usize>(
                        "GetConverter"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetConverter",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, destination))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        &mut self,
        obj: crate::UnityEngine::UIElements::TypeConverterRegistry,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::TypeConverterRegistry),
                        i32,
                        1usize,
                    >("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHashCode", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn Register(
        &mut self,
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        destination: quest_hook::libil2cpp::Gc<crate::System::Type>,
        converter: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
                    ), quest_hook::libil2cpp::Void, 3usize>("Register")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Register",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, destination, converter))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetConverter(
        &mut self,
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        destination: quest_hook::libil2cpp::Gc<crate::System::Type>,
        converter: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::System::Delegate>,
                        >,
                    ), bool, 3usize>("TryGetConverter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetConverter",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, destination, converter))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        storage: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey,
                quest_hook::libil2cpp::Gc<crate::System::Delegate>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::Dictionary_2<
                            crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey,
                            quest_hook::libil2cpp::Gc<crate::System::Delegate>,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (storage))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+TypeConverterRegistry")]
impl
    AsRef<
        crate::System::Collections::Generic::IEqualityComparer_1<
            crate::UnityEngine::UIElements::TypeConverterRegistry,
        >,
    > for crate::UnityEngine::UIElements::TypeConverterRegistry
{
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEqualityComparer_1<
        crate::UnityEngine::UIElements::TypeConverterRegistry,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+TypeConverterRegistry")]
impl
    AsMut<
        crate::System::Collections::Generic::IEqualityComparer_1<
            crate::UnityEngine::UIElements::TypeConverterRegistry,
        >,
    > for crate::UnityEngine::UIElements::TypeConverterRegistry
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEqualityComparer_1<
        crate::UnityEngine::UIElements::TypeConverterRegistry,
    > {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TypeConverterRegistry+ConverterKey")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct TypeConverterRegistry_ConverterKey {
    pub SourceType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub DestinationType: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TypeConverterRegistry+ConverterKey")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "TypeConverterRegistry/ConverterKey";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TypeConverterRegistry+ConverterKey")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TypeConverterRegistry+ConverterKey")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TypeConverterRegistry+ConverterKey")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TypeConverterRegistry+ConverterKey")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TypeConverterRegistry+ConverterKey")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TypeConverterRegistry+ConverterKey")]
impl crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey {
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        destination: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, destination))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TypeConverterRegistry+ConverterKeyComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeConverterRegistry_ConverterKeyComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TypeConverterRegistry+ConverterKeyComparer")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKeyComparer
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "TypeConverterRegistry/ConverterKeyComparer";
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
#[cfg(feature = "UnityEngine+UIElements+TypeConverterRegistry+ConverterKeyComparer")]
impl std::ops::Deref
    for crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKeyComparer
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TypeConverterRegistry+ConverterKeyComparer")]
impl std::ops::DerefMut
    for crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKeyComparer
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TypeConverterRegistry+ConverterKeyComparer")]
impl crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKeyComparer {
    pub fn Equals(
        &mut self,
        x: crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey,
        y: crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey,
                        crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey,
                    ), bool, 2usize>("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        &mut self,
        obj: crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey),
                        i32,
                        1usize,
                    >("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHashCode", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (obj))? };
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TypeConverterRegistry+ConverterKeyComparer")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKeyComparer
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+TypeConverterRegistry+ConverterKeyComparer")]
impl
    AsRef<
        crate::System::Collections::Generic::IEqualityComparer_1<
            crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey,
        >,
    > for crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKeyComparer
{
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEqualityComparer_1<
        crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TypeConverterRegistry+ConverterKeyComparer")]
impl
    AsMut<
        crate::System::Collections::Generic::IEqualityComparer_1<
            crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey,
        >,
    > for crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKeyComparer
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEqualityComparer_1<
        crate::UnityEngine::UIElements::TypeConverterRegistry_ConverterKey,
    > {
        unsafe { std::mem::transmute(self) }
    }
}

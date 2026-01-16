#[cfg(feature = "cordl_class_Unity+Properties+ConversionRegistry")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConversionRegistry {
    pub m_Converters: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::Unity::Properties::ConversionRegistry_ConverterKey,
            quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        >,
    >,
}
#[cfg(feature = "cordl_class_Unity+Properties+ConversionRegistry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Properties::ConversionRegistry {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "ConversionRegistry";
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
#[cfg(feature = "cordl_class_Unity+Properties+ConversionRegistry")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Properties::ConversionRegistry {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+ConversionRegistry")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Properties::ConversionRegistry {
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
#[cfg(feature = "cordl_class_Unity+Properties+ConversionRegistry")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Properties::ConversionRegistry {
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
#[cfg(feature = "cordl_class_Unity+Properties+ConversionRegistry")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Properties::ConversionRegistry {
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
#[cfg(feature = "cordl_class_Unity+Properties+ConversionRegistry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Properties::ConversionRegistry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Properties+ConversionRegistry")]
impl crate::Unity::Properties::ConversionRegistry {
    #[cfg(feature = "Unity+Properties+ConversionRegistry+ConverterKey")]
    pub type ConverterKey = crate::Unity::Properties::ConversionRegistry_ConverterKey;
    #[cfg(feature = "Unity+Properties+ConversionRegistry+ConverterKeyComparer")]
    pub type ConverterKeyComparer = crate::Unity::Properties::ConversionRegistry_ConverterKeyComparer;
    pub fn Create() -> quest_hook::libil2cpp::Result<
        crate::Unity::Properties::ConversionRegistry,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::Unity::Properties::ConversionRegistry,
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
        let __cordl_ret: crate::Unity::Properties::ConversionRegistry = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        x: crate::Unity::Properties::ConversionRegistry,
        y: crate::Unity::Properties::ConversionRegistry,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Properties::ConversionRegistry,
                            crate::Unity::Properties::ConversionRegistry,
                        ),
                        bool,
                        2usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (x, y))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetConverter(
        &mut self,
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        destination: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
                        2usize,
                    >("GetConverter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetConverter", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = unsafe {
            cordl_method_info.invoke_unchecked(self, (source, destination))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        &mut self,
        obj: crate::Unity::Properties::ConversionRegistry,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Properties::ConversionRegistry),
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
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Register(
        &mut self,
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        destination: quest_hook::libil2cpp::Gc<crate::System::Type>,
        converter: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::System::Delegate>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Register")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Register", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (source, destination, converter))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<crate::System::Delegate>,
                            >,
                        ),
                        bool,
                        3usize,
                    >("TryGetConverter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetConverter", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (source, destination, converter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        storage: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::Unity::Properties::ConversionRegistry_ConverterKey,
                quest_hook::libil2cpp::Gc<crate::System::Delegate>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                crate::Unity::Properties::ConversionRegistry_ConverterKey,
                                quest_hook::libil2cpp::Gc<crate::System::Delegate>,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (storage))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+ConversionRegistry")]
impl AsRef<
    crate::System::Collections::Generic::IEqualityComparer_1<
        crate::Unity::Properties::ConversionRegistry,
    >,
> for crate::Unity::Properties::ConversionRegistry {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEqualityComparer_1<
        crate::Unity::Properties::ConversionRegistry,
    > {
        todo!()
    }
}
#[cfg(feature = "Unity+Properties+ConversionRegistry")]
impl AsMut<
    crate::System::Collections::Generic::IEqualityComparer_1<
        crate::Unity::Properties::ConversionRegistry,
    >,
> for crate::Unity::Properties::ConversionRegistry {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEqualityComparer_1<
        crate::Unity::Properties::ConversionRegistry,
    > {
        todo!()
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+ConversionRegistry+ConverterKey")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConversionRegistry_ConverterKey {
    pub SourceType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub DestinationType: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "cordl_class_Unity+Properties+ConversionRegistry+ConverterKey")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Properties::ConversionRegistry_ConverterKey {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "ConversionRegistry/ConverterKey";
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
#[cfg(feature = "cordl_class_Unity+Properties+ConversionRegistry+ConverterKey")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Properties::ConversionRegistry_ConverterKey {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+ConversionRegistry+ConverterKey")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Properties::ConversionRegistry_ConverterKey {
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
#[cfg(feature = "cordl_class_Unity+Properties+ConversionRegistry+ConverterKey")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Properties::ConversionRegistry_ConverterKey {
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
#[cfg(feature = "cordl_class_Unity+Properties+ConversionRegistry+ConverterKey")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Properties::ConversionRegistry_ConverterKey {
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
#[cfg(feature = "cordl_class_Unity+Properties+ConversionRegistry+ConverterKey")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Properties::ConversionRegistry_ConverterKey {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Properties+ConversionRegistry+ConverterKey")]
impl crate::Unity::Properties::ConversionRegistry_ConverterKey {
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        destination: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (source, destination))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+ConversionRegistry+ConverterKeyComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct ConversionRegistry_ConverterKeyComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Properties+ConversionRegistry+ConverterKeyComparer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Properties::ConversionRegistry_ConverterKeyComparer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "ConversionRegistry/ConverterKeyComparer";
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
#[cfg(feature = "Unity+Properties+ConversionRegistry+ConverterKeyComparer")]
impl std::ops::Deref
for crate::Unity::Properties::ConversionRegistry_ConverterKeyComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ConversionRegistry+ConverterKeyComparer")]
impl std::ops::DerefMut
for crate::Unity::Properties::ConversionRegistry_ConverterKeyComparer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ConversionRegistry+ConverterKeyComparer")]
impl crate::Unity::Properties::ConversionRegistry_ConverterKeyComparer {
    pub fn Equals(
        &mut self,
        x: crate::Unity::Properties::ConversionRegistry_ConverterKey,
        y: crate::Unity::Properties::ConversionRegistry_ConverterKey,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Properties::ConversionRegistry_ConverterKey,
                            crate::Unity::Properties::ConversionRegistry_ConverterKey,
                        ),
                        bool,
                        2usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (x, y))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        &mut self,
        obj: crate::Unity::Properties::ConversionRegistry_ConverterKey,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Properties::ConversionRegistry_ConverterKey),
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
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (obj))?
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
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+ConversionRegistry+ConverterKeyComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::ConversionRegistry_ConverterKeyComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+ConversionRegistry+ConverterKeyComparer")]
impl AsRef<
    crate::System::Collections::Generic::IEqualityComparer_1<
        crate::Unity::Properties::ConversionRegistry_ConverterKey,
    >,
> for crate::Unity::Properties::ConversionRegistry_ConverterKeyComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEqualityComparer_1<
        crate::Unity::Properties::ConversionRegistry_ConverterKey,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+ConversionRegistry+ConverterKeyComparer")]
impl AsMut<
    crate::System::Collections::Generic::IEqualityComparer_1<
        crate::Unity::Properties::ConversionRegistry_ConverterKey,
    >,
> for crate::Unity::Properties::ConversionRegistry_ConverterKeyComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEqualityComparer_1<
        crate::Unity::Properties::ConversionRegistry_ConverterKey,
    > {
        unsafe { std::mem::transmute(self) }
    }
}

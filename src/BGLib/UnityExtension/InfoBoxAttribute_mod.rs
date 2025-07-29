#[cfg(feature = "cordl_class_BGLib+UnityExtension+InfoBoxAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct InfoBoxAttribute {
    __cordl_parent: crate::UnityEngine::PropertyAttribute,
    pub info: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub messageType: crate::BGLib::UnityExtension::InfoBoxAttribute_Type,
}
#[cfg(feature = "cordl_class_BGLib+UnityExtension+InfoBoxAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::UnityExtension::InfoBoxAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.UnityExtension";
    const CLASS_NAME: &'static str = "InfoBoxAttribute";
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
#[cfg(feature = "cordl_class_BGLib+UnityExtension+InfoBoxAttribute")]
impl std::ops::Deref for crate::BGLib::UnityExtension::InfoBoxAttribute {
    type Target = crate::UnityEngine::PropertyAttribute;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_BGLib+UnityExtension+InfoBoxAttribute")]
impl std::ops::DerefMut for crate::BGLib::UnityExtension::InfoBoxAttribute {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+InfoBoxAttribute")]
impl crate::BGLib::UnityExtension::InfoBoxAttribute {
    #[cfg(feature = "BGLib+UnityExtension+InfoBoxAttribute+Type")]
    pub type Type = crate::BGLib::UnityExtension::InfoBoxAttribute_Type;
    pub fn New(
        info: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        messageType: crate::BGLib::UnityExtension::InfoBoxAttribute_Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, messageType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        info: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        messageType: crate::BGLib::UnityExtension::InfoBoxAttribute_Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::BGLib::UnityExtension::InfoBoxAttribute_Type,
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
            cordl_method_info.invoke_unchecked(self, (info, messageType))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BGLib+UnityExtension+InfoBoxAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::InfoBoxAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_BGLib+UnityExtension+InfoBoxAttribute+Type")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InfoBoxAttribute_Type {
    #[default]
    Error = 3i32,
    Info = 1i32,
    None = 0i32,
    Warning = 2i32,
}
#[cfg(feature = "cordl_class_BGLib+UnityExtension+InfoBoxAttribute+Type")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::UnityExtension::InfoBoxAttribute_Type {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BGLib.UnityExtension";
    const CLASS_NAME: &'static str = "InfoBoxAttribute/Type";
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
#[cfg(feature = "cordl_class_BGLib+UnityExtension+InfoBoxAttribute+Type")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BGLib::UnityExtension::InfoBoxAttribute_Type {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_BGLib+UnityExtension+InfoBoxAttribute+Type")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BGLib::UnityExtension::InfoBoxAttribute_Type {
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
#[cfg(feature = "cordl_class_BGLib+UnityExtension+InfoBoxAttribute+Type")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BGLib::UnityExtension::InfoBoxAttribute_Type {
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
#[cfg(feature = "cordl_class_BGLib+UnityExtension+InfoBoxAttribute+Type")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BGLib::UnityExtension::InfoBoxAttribute_Type {
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

#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderPropertyAttributeFilter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub propType: crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType,
    pub nameFilter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.UnityExtension";
    const CLASS_NAME: &'static str = "ShaderPropertyAttributeFilter";
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
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter")]
impl std::ops::Deref for crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter")]
impl std::ops::DerefMut for crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter")]
impl crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter {
    #[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter+PropType")]
    pub type PropType = crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType;
    pub fn New(
        nameFilter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        propType: crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nameFilter, propType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        nameFilter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        propType: crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType,
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
                            crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType,
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
            cordl_method_info.invoke_unchecked(self, (nameFilter, propType))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter+PropType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShaderPropertyAttributeFilter_PropType {
    #[default]
    Any = 0i32,
    Color = 1i32,
    Float = 3i32,
    Int = 6i32,
    Range = 4i32,
    Texture = 5i32,
    Vector = 2i32,
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter+PropType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BGLib.UnityExtension";
    const CLASS_NAME: &'static str = "ShaderPropertyAttributeFilter/PropType";
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
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter+PropType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter+PropType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType {
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
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter+PropType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType {
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
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter+PropType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType {
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

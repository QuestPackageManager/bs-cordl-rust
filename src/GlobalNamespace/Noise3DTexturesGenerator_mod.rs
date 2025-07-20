#[cfg(feature = "Noise3DTexturesGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct Noise3DTexturesGenerator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _data: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialTextureParamsCouple,
        >,
    >,
}
#[cfg(feature = "Noise3DTexturesGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::Noise3DTexturesGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "Noise3DTexturesGenerator";
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
#[cfg(feature = "Noise3DTexturesGenerator")]
impl std::ops::Deref for crate::GlobalNamespace::Noise3DTexturesGenerator {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Noise3DTexturesGenerator")]
impl std::ops::DerefMut for crate::GlobalNamespace::Noise3DTexturesGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Noise3DTexturesGenerator")]
impl crate::GlobalNamespace::Noise3DTexturesGenerator {
    #[cfg(feature = "Noise3DTexturesGenerator+MaterialPropertyNameCouple")]
    pub type MaterialPropertyNameCouple = crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialPropertyNameCouple;
    #[cfg(feature = "Noise3DTexturesGenerator+MaterialTextureParamsCouple")]
    pub type MaterialTextureParamsCouple = crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialTextureParamsCouple;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::Noise3DTexturesGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Awake")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::Noise3DTexturesGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "Awake", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateNoisePixels(
        width: i32,
        height: i32,
        depth: i32,
        scale: f32,
        repeat: i32,
        contrast: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::Noise3DTexturesGenerator as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32, i32, f32, i32, f32),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
                >,
                6usize,
            >("CreateNoisePixels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::Noise3DTexturesGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "CreateNoisePixels", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        > = unsafe {
            method.invoke_unchecked((), (width, height, depth, scale, repeat, contrast))?
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::Noise3DTexturesGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::Noise3DTexturesGenerator as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Noise3DTexturesGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::Noise3DTexturesGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Noise3DTexturesGenerator+MaterialPropertyNameCouple")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Noise3DTexturesGenerator_MaterialPropertyNameCouple {
    pub texturePropertyName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
}
#[cfg(feature = "Noise3DTexturesGenerator+MaterialPropertyNameCouple")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialPropertyNameCouple {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "Noise3DTexturesGenerator/MaterialPropertyNameCouple";
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
#[cfg(feature = "Noise3DTexturesGenerator+MaterialPropertyNameCouple")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialPropertyNameCouple {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Noise3DTexturesGenerator+MaterialPropertyNameCouple")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialPropertyNameCouple {
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
#[cfg(feature = "Noise3DTexturesGenerator+MaterialPropertyNameCouple")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialPropertyNameCouple {
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
#[cfg(feature = "Noise3DTexturesGenerator+MaterialPropertyNameCouple")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialPropertyNameCouple {
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
#[cfg(feature = "Noise3DTexturesGenerator+MaterialPropertyNameCouple")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialPropertyNameCouple {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Noise3DTexturesGenerator+MaterialPropertyNameCouple")]
impl crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialPropertyNameCouple {}
#[cfg(feature = "Noise3DTexturesGenerator+MaterialTextureParamsCouple")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Noise3DTexturesGenerator_MaterialTextureParamsCouple {
    pub globalPropertyName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub materialPropertyNameCouples: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialPropertyNameCouple,
        >,
    >,
}
#[cfg(feature = "Noise3DTexturesGenerator+MaterialTextureParamsCouple")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialTextureParamsCouple {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "Noise3DTexturesGenerator/MaterialTextureParamsCouple";
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
#[cfg(feature = "Noise3DTexturesGenerator+MaterialTextureParamsCouple")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialTextureParamsCouple {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Noise3DTexturesGenerator+MaterialTextureParamsCouple")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialTextureParamsCouple {
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
#[cfg(feature = "Noise3DTexturesGenerator+MaterialTextureParamsCouple")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialTextureParamsCouple {
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
#[cfg(feature = "Noise3DTexturesGenerator+MaterialTextureParamsCouple")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialTextureParamsCouple {
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
#[cfg(feature = "Noise3DTexturesGenerator+MaterialTextureParamsCouple")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialTextureParamsCouple {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Noise3DTexturesGenerator+MaterialTextureParamsCouple")]
impl crate::GlobalNamespace::Noise3DTexturesGenerator_MaterialTextureParamsCouple {}

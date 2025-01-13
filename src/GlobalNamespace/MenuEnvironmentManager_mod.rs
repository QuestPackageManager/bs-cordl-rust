#[cfg(feature = "MenuEnvironmentManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuEnvironmentManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _data: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentObjects,
            >,
        >,
    >,
    pub _prevMenuEnvironmentType: crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType,
}
#[cfg(feature = "MenuEnvironmentManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MenuEnvironmentManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MenuEnvironmentManager";
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
#[cfg(feature = "MenuEnvironmentManager")]
impl std::ops::Deref for crate::GlobalNamespace::MenuEnvironmentManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuEnvironmentManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuEnvironmentManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuEnvironmentManager")]
impl crate::GlobalNamespace::MenuEnvironmentManager {
    #[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentObjects")]
    pub type MenuEnvironmentObjects = crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentObjects;
    #[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentType")]
    pub type MenuEnvironmentType = crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ShowEnvironmentType(
        &mut self,
        menuEnvironmentType: crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowEnvironmentType", (menuEnvironmentType))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MenuEnvironmentManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuEnvironmentManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentObjects")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuEnvironmentManager_MenuEnvironmentObjects {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _menuEnvironmentType: crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType,
    pub _wrapper: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
}
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentObjects")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentObjects {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MenuEnvironmentManager/MenuEnvironmentObjects";
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
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentObjects")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentObjects {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentObjects")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentObjects {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentObjects")]
impl crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentObjects {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_menuEnvironmentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType = __cordl_object
            .invoke("get_menuEnvironmentType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wrapper(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("get_wrapper", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentObjects")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentObjects {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MenuEnvironmentManager_MenuEnvironmentType {
    #[default]
    Default = 1i32,
    Lobby = 2i32,
    None = 0i32,
}
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MenuEnvironmentManager/MenuEnvironmentType";
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
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType {
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
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType {
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
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType {
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

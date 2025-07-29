#[cfg(feature = "cordl_class_SelectSubMenuDestination")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectSubMenuDestination {
    __cordl_parent: crate::GlobalNamespace::MenuDestination,
    pub menuDestination: crate::GlobalNamespace::SelectSubMenuDestination_Destination,
}
#[cfg(feature = "cordl_class_SelectSubMenuDestination")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SelectSubMenuDestination {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SelectSubMenuDestination";
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
#[cfg(feature = "cordl_class_SelectSubMenuDestination")]
impl std::ops::Deref for crate::GlobalNamespace::SelectSubMenuDestination {
    type Target = crate::GlobalNamespace::MenuDestination;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_SelectSubMenuDestination")]
impl std::ops::DerefMut for crate::GlobalNamespace::SelectSubMenuDestination {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SelectSubMenuDestination")]
impl crate::GlobalNamespace::SelectSubMenuDestination {
    #[cfg(feature = "SelectSubMenuDestination+Destination")]
    pub type Destination = crate::GlobalNamespace::SelectSubMenuDestination_Destination;
    pub fn New(
        menuDestination: crate::GlobalNamespace::SelectSubMenuDestination_Destination,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (menuDestination))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        menuDestination: crate::GlobalNamespace::SelectSubMenuDestination_Destination,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::SelectSubMenuDestination_Destination),
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
            cordl_method_info.invoke_unchecked(self, (menuDestination))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_SelectSubMenuDestination")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SelectSubMenuDestination {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_SelectSubMenuDestination+Destination")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectSubMenuDestination_Destination {
    #[default]
    Campaign = 1i32,
    MainMenu = 0i32,
    Multiplayer = 6i32,
    PartyFreePlay = 3i32,
    Settings = 4i32,
    SoloFreePlay = 2i32,
    Tutorial = 5i32,
}
#[cfg(feature = "cordl_class_SelectSubMenuDestination+Destination")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SelectSubMenuDestination_Destination {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SelectSubMenuDestination/Destination";
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
#[cfg(feature = "cordl_class_SelectSubMenuDestination+Destination")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::SelectSubMenuDestination_Destination {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_SelectSubMenuDestination+Destination")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::SelectSubMenuDestination_Destination {
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
#[cfg(feature = "cordl_class_SelectSubMenuDestination+Destination")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::SelectSubMenuDestination_Destination {
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
#[cfg(feature = "cordl_class_SelectSubMenuDestination+Destination")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::SelectSubMenuDestination_Destination {
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

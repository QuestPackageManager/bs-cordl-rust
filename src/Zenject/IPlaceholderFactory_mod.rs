#[cfg(feature = "cordl_class_Zenject+IPlaceholderFactory")]
#[derive(Debug)]
#[repr(C)]
pub struct IPlaceholderFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Zenject+IPlaceholderFactory")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::IPlaceholderFactory {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "IPlaceholderFactory";
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
#[cfg(feature = "Zenject+IPlaceholderFactory")]
impl std::ops::Deref for crate::Zenject::IPlaceholderFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IPlaceholderFactory")]
impl std::ops::DerefMut for crate::Zenject::IPlaceholderFactory {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IPlaceholderFactory")]
impl crate::Zenject::IPlaceholderFactory {
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_Zenject+IPlaceholderFactory")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::IPlaceholderFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+IPlaceholderFactory")]
impl AsRef<crate::Zenject::IValidatable> for crate::Zenject::IPlaceholderFactory {
    fn as_ref(&self) -> &crate::Zenject::IValidatable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+IPlaceholderFactory")]
impl AsMut<crate::Zenject::IValidatable> for crate::Zenject::IPlaceholderFactory {
    fn as_mut(&mut self) -> &mut crate::Zenject::IValidatable {
        unsafe { std::mem::transmute(self) }
    }
}

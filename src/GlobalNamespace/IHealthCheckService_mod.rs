#[cfg(feature = "IHealthCheckService")]
#[repr(C)]
#[derive(Debug)]
pub struct IHealthCheckService {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IHealthCheckService")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::IHealthCheckService {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IHealthCheckService";
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
#[cfg(feature = "IHealthCheckService")]
impl std::ops::Deref for crate::GlobalNamespace::IHealthCheckService {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IHealthCheckService")]
impl std::ops::DerefMut for crate::GlobalNamespace::IHealthCheckService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IHealthCheckService")]
impl crate::GlobalNamespace::IHealthCheckService {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IHealthCheckService")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IHealthCheckService {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IHealthCheckService")]
impl AsRef<crate::GlobalNamespace::IPollable>
for crate::GlobalNamespace::IHealthCheckService {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPollable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IHealthCheckService")]
impl AsMut<crate::GlobalNamespace::IPollable>
for crate::GlobalNamespace::IHealthCheckService {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPollable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IHealthCheckService")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::IHealthCheckService {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IHealthCheckService")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::IHealthCheckService {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}

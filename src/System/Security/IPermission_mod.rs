#[cfg(feature = "System+Security+IPermission")]
#[repr(C)]
#[derive(Debug)]
pub struct IPermission {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Security+IPermission")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Security::IPermission {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security";
    const CLASS_NAME: &'static str = "IPermission";
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
#[cfg(feature = "System+Security+IPermission")]
impl std::ops::Deref for crate::System::Security::IPermission {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+IPermission")]
impl std::ops::DerefMut for crate::System::Security::IPermission {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+IPermission")]
impl crate::System::Security::IPermission {
    pub fn Demand(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Demand")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Demand", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsSubsetOf(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Security::IPermission>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Security::IPermission>),
                bool,
                1usize,
            >("IsSubsetOf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsSubsetOf", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (target)) };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Security+IPermission")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Security::IPermission {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Security+IPermission")]
impl AsRef<crate::System::Security::ISecurityEncodable>
for crate::System::Security::IPermission {
    fn as_ref(&self) -> &crate::System::Security::ISecurityEncodable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+IPermission")]
impl AsMut<crate::System::Security::ISecurityEncodable>
for crate::System::Security::IPermission {
    fn as_mut(&mut self) -> &mut crate::System::Security::ISecurityEncodable {
        unsafe { std::mem::transmute(self) }
    }
}

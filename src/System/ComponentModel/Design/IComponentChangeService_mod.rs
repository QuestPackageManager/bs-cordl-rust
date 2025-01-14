#[cfg(feature = "System+ComponentModel+Design+IComponentChangeService")]
#[repr(C)]
#[derive(Debug)]
pub struct IComponentChangeService {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+Design+IComponentChangeService")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::ComponentModel::Design::IComponentChangeService {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.ComponentModel.Design";
    const CLASS_NAME: &'static str = "IComponentChangeService";
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
#[cfg(feature = "System+ComponentModel+Design+IComponentChangeService")]
impl std::ops::Deref for crate::System::ComponentModel::Design::IComponentChangeService {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+IComponentChangeService")]
impl std::ops::DerefMut
for crate::System::ComponentModel::Design::IComponentChangeService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+IComponentChangeService")]
impl crate::System::ComponentModel::Design::IComponentChangeService {
    pub fn OnComponentChanged(
        &mut self,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        member: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::MemberDescriptor,
        >,
        oldValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        newValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::ComponentModel::MemberDescriptor,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("OnComponentChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnComponentChanged", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (component, member, oldValue, newValue))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnComponentChanging(
        &mut self,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        member: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::MemberDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::ComponentModel::MemberDescriptor,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("OnComponentChanging")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnComponentChanging", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (component, member))
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+ComponentModel+Design+IComponentChangeService")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::Design::IComponentChangeService {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

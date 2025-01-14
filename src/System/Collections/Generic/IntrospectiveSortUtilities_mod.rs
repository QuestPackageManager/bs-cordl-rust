#[cfg(feature = "System+Collections+Generic+IntrospectiveSortUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct IntrospectiveSortUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Collections+Generic+IntrospectiveSortUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Collections::Generic::IntrospectiveSortUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Collections.Generic";
    const CLASS_NAME: &'static str = "IntrospectiveSortUtilities";
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
#[cfg(feature = "System+Collections+Generic+IntrospectiveSortUtilities")]
impl std::ops::Deref
for crate::System::Collections::Generic::IntrospectiveSortUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+IntrospectiveSortUtilities")]
impl std::ops::DerefMut
for crate::System::Collections::Generic::IntrospectiveSortUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+IntrospectiveSortUtilities")]
impl crate::System::Collections::Generic::IntrospectiveSortUtilities {
    pub fn FloorLog2PlusOne(n: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i32, 1usize>("FloorLog2PlusOne")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FloorLog2PlusOne", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (n)) };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowOrIgnoreBadComparer(
        comparer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ThrowOrIgnoreBadComparer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ThrowOrIgnoreBadComparer", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (comparer))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+Generic+IntrospectiveSortUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Generic::IntrospectiveSortUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

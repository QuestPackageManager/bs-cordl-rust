#[cfg(feature = "System+Collections+Generic+IntrospectiveSortUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct IntrospectiveSortUtilities {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Collections+Generic+IntrospectiveSortUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Generic::IntrospectiveSortUtilities =>
    "System.Collections.Generic"."IntrospectiveSortUtilities"
);
#[cfg(feature = "System+Collections+Generic+IntrospectiveSortUtilities")]
impl std::ops::Deref
for crate::System::Collections::Generic::IntrospectiveSortUtilities {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FloorLog2PlusOne", (n))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowOrIgnoreBadComparer(
        comparer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowOrIgnoreBadComparer", (comparer))?;
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

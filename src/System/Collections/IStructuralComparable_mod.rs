#[cfg(feature = "System+Collections+IStructuralComparable")]
#[repr(C)]
#[derive(Debug)]
pub struct IStructuralComparable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Collections+IStructuralComparable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::IStructuralComparable =>
    "System.Collections"."IStructuralComparable"
);
#[cfg(feature = "System+Collections+IStructuralComparable")]
impl std::ops::Deref for crate::System::Collections::IStructuralComparable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+IStructuralComparable")]
impl std::ops::DerefMut for crate::System::Collections::IStructuralComparable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+IStructuralComparable")]
impl crate::System::Collections::IStructuralComparable {
    pub fn CompareTo(
        &mut self,
        other: *mut quest_hook::libil2cpp::Il2CppObject,
        comparer: *mut crate::System::Collections::IComparer,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (other, comparer))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Collections+IStructuralComparable")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::IStructuralComparable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "System+Collections+IStructuralEquatable")]
#[repr(C)]
#[derive(Debug)]
pub struct IStructuralEquatable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Collections+IStructuralEquatable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::IStructuralEquatable =>
    "System.Collections"."IStructuralEquatable"
);
#[cfg(feature = "System+Collections+IStructuralEquatable")]
impl std::ops::Deref for crate::System::Collections::IStructuralEquatable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+IStructuralEquatable")]
impl std::ops::DerefMut for crate::System::Collections::IStructuralEquatable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+IStructuralEquatable")]
impl crate::System::Collections::IStructuralEquatable {
    pub fn Equals(
        &mut self,
        other: *mut quest_hook::libil2cpp::Il2CppObject,
        comparer: *mut crate::System::Collections::IEqualityComparer,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other, comparer))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(
        &mut self,
        comparer: *mut crate::System::Collections::IEqualityComparer,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (comparer))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Collections+IStructuralEquatable")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::IStructuralEquatable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

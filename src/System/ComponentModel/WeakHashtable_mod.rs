#[cfg(feature = "System+ComponentModel+WeakHashtable")]
#[repr(C)]
#[derive(Debug)]
pub struct WeakHashtable {
    __cordl_parent: crate::System::Collections::Hashtable,
}
#[cfg(feature = "System+ComponentModel+WeakHashtable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::WeakHashtable =>
    "System.ComponentModel"."WeakHashtable"
);
#[cfg(feature = "System+ComponentModel+WeakHashtable")]
impl std::ops::Deref for crate::System::ComponentModel::WeakHashtable {
    type Target = crate::System::Collections::Hashtable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+WeakHashtable")]
impl std::ops::DerefMut for crate::System::ComponentModel::WeakHashtable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+WeakHashtable")]
impl crate::System::ComponentModel::WeakHashtable {
    #[cfg(feature = "System+ComponentModel+WeakHashtable+WeakKeyComparer")]
    pub type WeakKeyComparer = crate::System::ComponentModel::WeakHashtable_WeakKeyComparer;
    pub fn Remove(
        &mut self,
        key: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (key))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+ComponentModel+WeakHashtable")]
impl quest_hook::libil2cpp::ObjectType for crate::System::ComponentModel::WeakHashtable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+WeakHashtable+WeakKeyComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct WeakHashtable_WeakKeyComparer {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+ComponentModel+WeakHashtable+WeakKeyComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::WeakHashtable_WeakKeyComparer => "System.ComponentModel"
    ."WeakHashtable/WeakKeyComparer"
);
#[cfg(feature = "System+ComponentModel+WeakHashtable+WeakKeyComparer")]
impl std::ops::Deref for crate::System::ComponentModel::WeakHashtable_WeakKeyComparer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+WeakHashtable+WeakKeyComparer")]
impl std::ops::DerefMut
for crate::System::ComponentModel::WeakHashtable_WeakKeyComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+WeakHashtable+WeakKeyComparer")]
impl crate::System::ComponentModel::WeakHashtable_WeakKeyComparer {
    pub fn System_Collections_IEqualityComparer_Equals(
        &mut self,
        x: *mut crate::System::Object,
        y: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.IEqualityComparer.Equals", (x, y))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEqualityComparer_GetHashCode(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Collections.IEqualityComparer.GetHashCode", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+ComponentModel+WeakHashtable+WeakKeyComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::WeakHashtable_WeakKeyComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

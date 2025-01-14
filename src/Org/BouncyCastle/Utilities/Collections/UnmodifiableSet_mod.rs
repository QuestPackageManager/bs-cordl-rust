#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+UnmodifiableSet")]
#[repr(C)]
#[derive(Debug)]
pub struct UnmodifiableSet {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+UnmodifiableSet")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Utilities::Collections::UnmodifiableSet {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Utilities.Collections";
    const CLASS_NAME: &'static str = "UnmodifiableSet";
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
#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+UnmodifiableSet")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Utilities::Collections::UnmodifiableSet {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+UnmodifiableSet")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Utilities::Collections::UnmodifiableSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+UnmodifiableSet")]
impl crate::Org::BouncyCastle::Utilities::Collections::UnmodifiableSet {
    pub fn Add(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Add")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Add", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (o))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddAll(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddAll")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddAll", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (e))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Clear", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Contains(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                bool,
                1usize,
            >("Contains")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Contains", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (o)) };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo(
        &mut self,
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Array>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CopyTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CopyTo", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (array, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                0usize,
            >("GetEnumerator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetEnumerator", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Remove(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Remove")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Remove", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (o))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAll(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RemoveAll")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RemoveAll", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (e))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Count")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Count", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsEmpty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsEmpty", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsFixedSize(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsFixedSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsFixedSize", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsReadOnly")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsReadOnly", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSynchronized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsSynchronized")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsSynchronized", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_SyncRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("get_SyncRoot")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_SyncRoot", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+UnmodifiableSet")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::Collections::UnmodifiableSet {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+UnmodifiableSet")]
impl AsRef<crate::Org::BouncyCastle::Utilities::Collections::ISet>
for crate::Org::BouncyCastle::Utilities::Collections::UnmodifiableSet {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Utilities::Collections::ISet {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+UnmodifiableSet")]
impl AsMut<crate::Org::BouncyCastle::Utilities::Collections::ISet>
for crate::Org::BouncyCastle::Utilities::Collections::UnmodifiableSet {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Utilities::Collections::ISet {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+UnmodifiableSet")]
impl AsRef<crate::System::Collections::ICollection>
for crate::Org::BouncyCastle::Utilities::Collections::UnmodifiableSet {
    fn as_ref(&self) -> &crate::System::Collections::ICollection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+UnmodifiableSet")]
impl AsMut<crate::System::Collections::ICollection>
for crate::Org::BouncyCastle::Utilities::Collections::UnmodifiableSet {
    fn as_mut(&mut self) -> &mut crate::System::Collections::ICollection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+UnmodifiableSet")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::Org::BouncyCastle::Utilities::Collections::UnmodifiableSet {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+UnmodifiableSet")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::Org::BouncyCastle::Utilities::Collections::UnmodifiableSet {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(feature = "System+Runtime+InteropServices+Marshal")]
#[repr(C)]
#[derive(Debug)]
pub struct Marshal {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::InteropServices::Marshal =>
    "System.Runtime.InteropServices"."Marshal"
);
#[cfg(feature = "System+Runtime+InteropServices+Marshal")]
impl std::ops::Deref for crate::System::Runtime::InteropServices::Marshal {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal")]
impl std::ops::DerefMut for crate::System::Runtime::InteropServices::Marshal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal")]
impl crate::System::Runtime::InteropServices::Marshal {
    #[cfg(feature = "System+Runtime+InteropServices+Marshal+__c")]
    pub type __c = crate::System::Runtime::InteropServices::Marshal___c;
    #[cfg(
        feature = "System+Runtime+InteropServices+Marshal+MarshalerInstanceKeyComparer"
    )]
    pub type MarshalerInstanceKeyComparer = crate::System::Runtime::InteropServices::Marshal_MarshalerInstanceKeyComparer;
    #[cfg(feature = "System+Runtime+InteropServices+Marshal+SecureStringAllocator")]
    pub type SecureStringAllocator = crate::System::Runtime::InteropServices::Marshal_SecureStringAllocator;
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::Marshal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+MarshalerInstanceKeyComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct Marshal_MarshalerInstanceKeyComparer {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+MarshalerInstanceKeyComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::InteropServices::Marshal_MarshalerInstanceKeyComparer =>
    "System.Runtime.InteropServices"."Marshal/MarshalerInstanceKeyComparer"
);
#[cfg(feature = "System+Runtime+InteropServices+Marshal+MarshalerInstanceKeyComparer")]
impl std::ops::Deref
for crate::System::Runtime::InteropServices::Marshal_MarshalerInstanceKeyComparer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+MarshalerInstanceKeyComparer")]
impl std::ops::DerefMut
for crate::System::Runtime::InteropServices::Marshal_MarshalerInstanceKeyComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+MarshalerInstanceKeyComparer")]
impl crate::System::Runtime::InteropServices::Marshal_MarshalerInstanceKeyComparer {
    pub fn GetHashCode(
        &mut self,
        key: crate::System::ValueTuple_2<
            *mut crate::System::Type,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (key))?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        lhs: crate::System::ValueTuple_2<
            *mut crate::System::Type,
            *mut crate::System::String,
        >,
        rhs: crate::System::ValueTuple_2<
            *mut crate::System::Type,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (lhs, rhs))?;
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
#[cfg(feature = "System+Runtime+InteropServices+Marshal+MarshalerInstanceKeyComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::Marshal_MarshalerInstanceKeyComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+SecureStringAllocator")]
#[repr(C)]
#[derive(Debug)]
pub struct Marshal_SecureStringAllocator {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+SecureStringAllocator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::InteropServices::Marshal_SecureStringAllocator =>
    "System.Runtime.InteropServices"."Marshal/SecureStringAllocator"
);
#[cfg(feature = "System+Runtime+InteropServices+Marshal+SecureStringAllocator")]
impl std::ops::Deref
for crate::System::Runtime::InteropServices::Marshal_SecureStringAllocator {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+SecureStringAllocator")]
impl std::ops::DerefMut
for crate::System::Runtime::InteropServices::Marshal_SecureStringAllocator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+SecureStringAllocator")]
impl crate::System::Runtime::InteropServices::Marshal_SecureStringAllocator {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object.invoke("Invoke", (len))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+SecureStringAllocator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::Marshal_SecureStringAllocator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

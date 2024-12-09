#[cfg(feature = "System+Xml+SecureStringHasher")]
#[repr(C)]
#[derive(Debug)]
pub struct SecureStringHasher {
    __cordl_parent: crate::System::Object,
    pub hashCodeRandomizer: i32,
}
#[cfg(feature = "System+Xml+SecureStringHasher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::SecureStringHasher => "System.Xml"
    ."SecureStringHasher"
);
#[cfg(feature = "System+Xml+SecureStringHasher")]
impl std::ops::Deref for crate::System::Xml::SecureStringHasher {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+SecureStringHasher")]
impl std::ops::DerefMut for crate::System::Xml::SecureStringHasher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+SecureStringHasher")]
impl crate::System::Xml::SecureStringHasher {
    #[cfg(feature = "System+Xml+SecureStringHasher+HashCodeOfStringDelegate")]
    pub type HashCodeOfStringDelegate = crate::System::Xml::SecureStringHasher_HashCodeOfStringDelegate;
    pub fn Equals(
        &mut self,
        x: *mut crate::System::String,
        y: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (x, y))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(
        &mut self,
        key: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (key))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "System+Xml+SecureStringHasher")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::SecureStringHasher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+SecureStringHasher+HashCodeOfStringDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct SecureStringHasher_HashCodeOfStringDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Xml+SecureStringHasher+HashCodeOfStringDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::SecureStringHasher_HashCodeOfStringDelegate => "System.Xml"
    ."SecureStringHasher/HashCodeOfStringDelegate"
);
#[cfg(feature = "System+Xml+SecureStringHasher+HashCodeOfStringDelegate")]
impl std::ops::Deref
for crate::System::Xml::SecureStringHasher_HashCodeOfStringDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+SecureStringHasher+HashCodeOfStringDelegate")]
impl std::ops::DerefMut
for crate::System::Xml::SecureStringHasher_HashCodeOfStringDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+SecureStringHasher+HashCodeOfStringDelegate")]
impl crate::System::Xml::SecureStringHasher_HashCodeOfStringDelegate {
    pub fn Invoke(
        &mut self,
        s: *mut crate::System::String,
        sLen: i32,
        additionalEntropy: i64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Invoke", (s, sLen, additionalEntropy))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
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
}
#[cfg(feature = "System+Xml+SecureStringHasher+HashCodeOfStringDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::SecureStringHasher_HashCodeOfStringDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

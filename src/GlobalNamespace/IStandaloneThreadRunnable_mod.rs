#[cfg(feature = "IStandaloneThreadRunnable")]
#[repr(C)]
#[derive(Debug)]
pub struct IStandaloneThreadRunnable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IStandaloneThreadRunnable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IStandaloneThreadRunnable => ""
    ."IStandaloneThreadRunnable"
);
#[cfg(feature = "IStandaloneThreadRunnable")]
impl std::ops::Deref for crate::GlobalNamespace::IStandaloneThreadRunnable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IStandaloneThreadRunnable")]
impl std::ops::DerefMut for crate::GlobalNamespace::IStandaloneThreadRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IStandaloneThreadRunnable")]
impl crate::GlobalNamespace::IStandaloneThreadRunnable {
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IStandaloneThreadRunnable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IStandaloneThreadRunnable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IStandaloneThreadRunnable")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::IStandaloneThreadRunnable {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IStandaloneThreadRunnable")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::IStandaloneThreadRunnable {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}

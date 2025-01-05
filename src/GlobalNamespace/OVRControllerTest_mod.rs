#[cfg(feature = "OVRControllerTest+BoolMonitor+BoolGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct BoolMonitor_OVRControllerTest_BoolGenerator {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVRControllerTest+BoolMonitor+BoolGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BoolMonitor_OVRControllerTest_BoolGenerator => ""
    ."OVRControllerTest/BoolMonitor/BoolGenerator"
);
#[cfg(feature = "OVRControllerTest+BoolMonitor+BoolGenerator")]
impl std::ops::Deref
for crate::GlobalNamespace::BoolMonitor_OVRControllerTest_BoolGenerator {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRControllerTest+BoolMonitor+BoolGenerator")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BoolMonitor_OVRControllerTest_BoolGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRControllerTest+BoolMonitor+BoolGenerator")]
impl crate::GlobalNamespace::BoolMonitor_OVRControllerTest_BoolGenerator {
    pub fn BeginInvoke(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRControllerTest+BoolMonitor+BoolGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BoolMonitor_OVRControllerTest_BoolGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRControllerTest")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRControllerTest {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub uiText: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
    pub monitors: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::OVRControllerTest_BoolMonitor,
            >,
        >,
    >,
    pub data: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
}
#[cfg(feature = "OVRControllerTest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRControllerTest => ""
    ."OVRControllerTest"
);
#[cfg(feature = "OVRControllerTest")]
impl std::ops::Deref for crate::GlobalNamespace::OVRControllerTest {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRControllerTest")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRControllerTest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRControllerTest")]
impl crate::GlobalNamespace::OVRControllerTest {
    #[cfg(feature = "OVRControllerTest+BoolMonitor")]
    pub type BoolMonitor = crate::GlobalNamespace::OVRControllerTest_BoolMonitor;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRControllerTest")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRControllerTest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRControllerTest+BoolMonitor")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRControllerTest_BoolMonitor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_generator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BoolMonitor_OVRControllerTest_BoolGenerator,
    >,
    pub m_prevValue: bool,
    pub m_currentValue: bool,
    pub m_currentValueRecentlyChanged: bool,
    pub m_displayTimeout: f32,
    pub m_displayTimer: f32,
}
#[cfg(feature = "OVRControllerTest+BoolMonitor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRControllerTest_BoolMonitor
    => ""."OVRControllerTest/BoolMonitor"
);
#[cfg(feature = "OVRControllerTest+BoolMonitor")]
impl std::ops::Deref for crate::GlobalNamespace::OVRControllerTest_BoolMonitor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRControllerTest+BoolMonitor")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRControllerTest_BoolMonitor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRControllerTest+BoolMonitor")]
impl crate::GlobalNamespace::OVRControllerTest_BoolMonitor {
    #[cfg(feature = "OVRControllerTest+BoolMonitor+BoolGenerator")]
    pub type BoolGenerator = crate::GlobalNamespace::BoolMonitor_OVRControllerTest_BoolGenerator;
    pub fn AppendToStringBuilder(
        &mut self,
        sb: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendToStringBuilder", (sb))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        generator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BoolMonitor_OVRControllerTest_BoolGenerator,
        >,
        displayTimeout: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, generator, displayTimeout))?;
        Ok(__cordl_object.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        generator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BoolMonitor_OVRControllerTest_BoolGenerator,
        >,
        displayTimeout: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, generator, displayTimeout))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRControllerTest+BoolMonitor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRControllerTest_BoolMonitor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

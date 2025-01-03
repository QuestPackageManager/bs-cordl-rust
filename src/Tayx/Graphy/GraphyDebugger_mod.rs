#[cfg(feature = "Tayx+Graphy+GraphyDebugger")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphyDebugger {
    __cordl_parent: crate::Tayx::Graphy::Utils::G_Singleton_1<
        *mut crate::Tayx::Graphy::GraphyDebugger,
    >,
    pub m_debugPackets: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Tayx::Graphy::GraphyDebugger_DebugPacket,
    >,
    pub m_fpsMonitor: *mut crate::Tayx::Graphy::Fps::G_FpsMonitor,
    pub m_ramMonitor: *mut crate::Tayx::Graphy::Ram::G_RamMonitor,
    pub m_audioMonitor: *mut crate::Tayx::Graphy::Audio::G_AudioMonitor,
}
#[cfg(feature = "Tayx+Graphy+GraphyDebugger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::GraphyDebugger => "Tayx.Graphy"
    ."GraphyDebugger"
);
#[cfg(feature = "Tayx+Graphy+GraphyDebugger")]
impl std::ops::Deref for crate::Tayx::Graphy::GraphyDebugger {
    type Target = crate::Tayx::Graphy::Utils::G_Singleton_1<
        *mut crate::Tayx::Graphy::GraphyDebugger,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+GraphyDebugger")]
impl std::ops::DerefMut for crate::Tayx::Graphy::GraphyDebugger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+GraphyDebugger")]
impl crate::Tayx::Graphy::GraphyDebugger {
    #[cfg(feature = "Tayx+Graphy+GraphyDebugger+ConditionEvaluation")]
    pub type ConditionEvaluation = crate::Tayx::Graphy::GraphyDebugger_ConditionEvaluation;
    #[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugComparer")]
    pub type DebugComparer = crate::Tayx::Graphy::GraphyDebugger_DebugComparer;
    #[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugCondition")]
    pub type DebugCondition = crate::Tayx::Graphy::GraphyDebugger_DebugCondition;
    #[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugPacket")]
    pub type DebugPacket = crate::Tayx::Graphy::GraphyDebugger_DebugPacket;
    #[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugVariable")]
    pub type DebugVariable = crate::Tayx::Graphy::GraphyDebugger_DebugVariable;
    #[cfg(feature = "Tayx+Graphy+GraphyDebugger+MessageType")]
    pub type MessageType = crate::Tayx::Graphy::GraphyDebugger_MessageType;
    pub fn AddCallbackToAllDebugPacketWithId(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCallbackToAllDebugPacketWithId", (callback, id))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddCallbackToFirstDebugPacketWithId(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCallbackToFirstDebugPacketWithId", (callback, id))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddNewDebugPacket_GraphyDebugger_DebugPacket0(
        &mut self,
        newDebugPacket: quest_hook::libil2cpp::Gc<
            crate::Tayx::Graphy::GraphyDebugger_DebugPacket,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNewDebugPacket", (newDebugPacket))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddNewDebugPacket_i32_GraphyDebugger_DebugCondition_GraphyDebugger_MessageType_Il2CppString__cordl_bool_Action1(
        &mut self,
        newId: i32,
        newDebugCondition: crate::Tayx::Graphy::GraphyDebugger_DebugCondition,
        newMessageType: crate::Tayx::Graphy::GraphyDebugger_MessageType,
        newMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newDebugBreak: bool,
        newCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddNewDebugPacket",
                (
                    newId,
                    newDebugCondition,
                    newMessageType,
                    newMessage,
                    newDebugBreak,
                    newCallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddNewDebugPacket_i32_GraphyDebugger_DebugCondition_GraphyDebugger_MessageType_Il2CppString__cordl_bool_List_1_3(
        &mut self,
        newId: i32,
        newDebugCondition: crate::Tayx::Graphy::GraphyDebugger_DebugCondition,
        newMessageType: crate::Tayx::Graphy::GraphyDebugger_MessageType,
        newMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newDebugBreak: bool,
        newCallbacks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<*mut crate::System::Action>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddNewDebugPacket",
                (
                    newId,
                    newDebugCondition,
                    newMessageType,
                    newMessage,
                    newDebugBreak,
                    newCallbacks,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddNewDebugPacket_i32_List_1_GraphyDebugger_MessageType_Il2CppString__cordl_bool_Action2(
        &mut self,
        newId: i32,
        newDebugConditions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::Tayx::Graphy::GraphyDebugger_DebugCondition,
            >,
        >,
        newMessageType: crate::Tayx::Graphy::GraphyDebugger_MessageType,
        newMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newDebugBreak: bool,
        newCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddNewDebugPacket",
                (
                    newId,
                    newDebugConditions,
                    newMessageType,
                    newMessage,
                    newDebugBreak,
                    newCallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddNewDebugPacket_i32_List_1_GraphyDebugger_MessageType_Il2CppString__cordl_bool_List_1_4(
        &mut self,
        newId: i32,
        newDebugConditions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::Tayx::Graphy::GraphyDebugger_DebugCondition,
            >,
        >,
        newMessageType: crate::Tayx::Graphy::GraphyDebugger_MessageType,
        newMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newDebugBreak: bool,
        newCallbacks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<*mut crate::System::Action>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddNewDebugPacket",
                (
                    newId,
                    newDebugConditions,
                    newMessageType,
                    newMessage,
                    newDebugBreak,
                    newCallbacks,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckDebugPackets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckDebugPackets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckIfConditionIsMet(
        &mut self,
        debugCondition: crate::Tayx::Graphy::GraphyDebugger_DebugCondition,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckIfConditionIsMet", (debugCondition))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteOperationsInDebugPacket(
        &mut self,
        debugPacket: quest_hook::libil2cpp::Gc<
            crate::Tayx::Graphy::GraphyDebugger_DebugPacket,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteOperationsInDebugPacket", (debugPacket))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllDebugPacketsWithId(
        &mut self,
        packetId: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Tayx::Graphy::GraphyDebugger_DebugPacket,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Tayx::Graphy::GraphyDebugger_DebugPacket,
            >,
        > = __cordl_object.invoke("GetAllDebugPacketsWithId", (packetId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFirstDebugPacketWithId(
        &mut self,
        packetId: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Tayx::Graphy::GraphyDebugger_DebugPacket>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Tayx::Graphy::GraphyDebugger_DebugPacket,
        > = __cordl_object.invoke("GetFirstDebugPacketWithId", (packetId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRequestedValueFromDebugVariable(
        &mut self,
        debugVariable: crate::Tayx::Graphy::GraphyDebugger_DebugVariable,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetRequestedValueFromDebugVariable", (debugVariable))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveAllDebugPacketsWithId(
        &mut self,
        packetId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAllDebugPacketsWithId", (packetId))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFirstDebugPacketWithId(
        &mut self,
        packetId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveFirstDebugPacketWithId", (packetId))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Tayx+Graphy+GraphyDebugger")]
impl quest_hook::libil2cpp::ObjectType for crate::Tayx::Graphy::GraphyDebugger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+ConditionEvaluation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphyDebugger_ConditionEvaluation {
    All_conditions_must_be_met = 0i32,
    Only_one_condition_has_to_be_met = 1i32,
}
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+ConditionEvaluation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::GraphyDebugger_ConditionEvaluation
    => "Tayx.Graphy"."GraphyDebugger/ConditionEvaluation"
);
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugComparer")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphyDebugger_DebugComparer {
    Equals = 2i32,
    Equals_or_greater_than = 3i32,
    Equals_or_less_than = 1i32,
    Greater_than = 4i32,
    Less_than = 0i32,
}
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugComparer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::GraphyDebugger_DebugComparer =>
    "Tayx.Graphy"."GraphyDebugger/DebugComparer"
);
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugCondition")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GraphyDebugger_DebugCondition {
    pub Variable: crate::Tayx::Graphy::GraphyDebugger_DebugVariable,
    pub Comparer: crate::Tayx::Graphy::GraphyDebugger_DebugComparer,
    pub Value: f32,
}
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugCondition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::GraphyDebugger_DebugCondition =>
    "Tayx.Graphy"."GraphyDebugger/DebugCondition"
);
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugCondition")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Tayx::Graphy::GraphyDebugger_DebugCondition {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugCondition")]
impl crate::Tayx::Graphy::GraphyDebugger_DebugCondition {}
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphyDebugger_DebugPacket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Active: bool,
    pub Id: i32,
    pub ExecuteOnce: bool,
    pub InitSleepTime: f32,
    pub ExecuteSleepTime: f32,
    pub ConditionEvaluation: crate::Tayx::Graphy::GraphyDebugger_ConditionEvaluation,
    pub DebugConditions: *mut crate::System::Collections::Generic::List_1<
        crate::Tayx::Graphy::GraphyDebugger_DebugCondition,
    >,
    pub MessageType: crate::Tayx::Graphy::GraphyDebugger_MessageType,
    pub Message: *mut quest_hook::libil2cpp::Il2CppString,
    pub TakeScreenshot: bool,
    pub ScreenshotFileName: *mut quest_hook::libil2cpp::Il2CppString,
    pub DebugBreak: bool,
    pub UnityEvents: *mut crate::UnityEngine::Events::UnityEvent,
    pub Callbacks: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Action,
    >,
    pub canBeChecked: bool,
    pub executed: bool,
    pub timePassed: f32,
}
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::GraphyDebugger_DebugPacket =>
    "Tayx.Graphy"."GraphyDebugger/DebugPacket"
);
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugPacket")]
impl std::ops::Deref for crate::Tayx::Graphy::GraphyDebugger_DebugPacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugPacket")]
impl std::ops::DerefMut for crate::Tayx::Graphy::GraphyDebugger_DebugPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugPacket")]
impl crate::Tayx::Graphy::GraphyDebugger_DebugPacket {
    pub fn Executed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Executed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Check(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Check", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugPacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::Tayx::Graphy::GraphyDebugger_DebugPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugVariable")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphyDebugger_DebugVariable {
    Audio_DB = 7i32,
    Fps = 0i32,
    Fps_Avg = 3i32,
    Fps_Max = 2i32,
    Fps_Min = 1i32,
    Ram_Allocated = 4i32,
    Ram_Mono = 6i32,
    Ram_Reserved = 5i32,
}
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+DebugVariable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::GraphyDebugger_DebugVariable =>
    "Tayx.Graphy"."GraphyDebugger/DebugVariable"
);
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+MessageType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphyDebugger_MessageType {
    Error = 2i32,
    Log = 0i32,
    Warning = 1i32,
}
#[cfg(feature = "Tayx+Graphy+GraphyDebugger+MessageType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::GraphyDebugger_MessageType =>
    "Tayx.Graphy"."GraphyDebugger/MessageType"
);

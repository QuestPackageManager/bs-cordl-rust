#[cfg(feature = "System+Security+AccessControl+ControlFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ControlFlags {
    #[default]
    DiscretionaryAclAutoInheritRequired = 256i32,
    DiscretionaryAclAutoInherited = 1024i32,
    DiscretionaryAclDefaulted = 8i32,
    DiscretionaryAclPresent = 4i32,
    DiscretionaryAclProtected = 4096i32,
    DiscretionaryAclUntrusted = 64i32,
    GroupDefaulted = 2i32,
    None = 0i32,
    OwnerDefaulted = 1i32,
    RMControlValid = 16384i32,
    SelfRelative = 32768i32,
    ServerSecurity = 128i32,
    SystemAclAutoInheritRequired = 512i32,
    SystemAclAutoInherited = 2048i32,
    SystemAclDefaulted = 32i32,
    SystemAclPresent = 16i32,
    SystemAclProtected = 8192i32,
}
#[cfg(feature = "System+Security+AccessControl+ControlFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::AccessControl::ControlFlags =>
    "System.Security.AccessControl"."ControlFlags"
);

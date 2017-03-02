//! USB structures and types

/// CCID USB descriptor
#[allow(non_snake_case)]
#[repr(C, packed)]
pub struct Descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bcdCCID: u16,
    pub bMaxSlotIndex: u8,
    pub bVoltageSupport: u8,
    pub dwProtocols: u32,
    pub dwDefaultClock: u32,
    pub dwMaximumClock: u32,
    pub bNumClockSupported: u8,
    pub dwDataRate: u32,
    pub dwMaxDataRate: u32,
    pub bNumDataRatesSupported: u8,
    pub dwMaxIFSD: u32,
    pub dwSynchProtocols: u32,
    pub dwMechanical: u32,
    pub dwFeatures: u32,
    pub dwMaxCCIDMessageLength: u32,
    pub bClassGetResponse: u8,
    pub bClassEnvelope: u8,
    pub wLcdLayout: u16,
    pub bPINSupport: u8,
    pub bMaxCCIDBusySlots: u8,
}

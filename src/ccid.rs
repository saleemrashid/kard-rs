//! CCID structures and types
#![allow(non_camel_case_types, non_snake_case)]

/// Generic CCID message header
#[repr(C, packed)]
pub struct Header {
    /// The message type (`bMessageType`) identifies the message.
    pub bMessageType: MessageType,

    /// The length field (`dwLength`) is the length of the message not
    /// including the 10-byte header.
    pub dwLength: u32,

    /// Identifies the slot number for this command
    ///
    /// The slot number (`bSlot`) identifies which ICC slot is being addressed
    /// by the message, if the CCID supports multiple slots. The slot number is
    /// zero-relative, and is in the range of zero to FFh.
    pub bSlot: u8,

    /// Sequence number for command
    ///
    /// The sequence number (`bSeq`) is a monotonically increasing by one
    /// counter of bulk messages sent to the CCID. Because the response to a
    /// command always uses the exact same sequence number contained in the
    /// command, the host can use the sequence number in a response message to
    /// verify that a particular response is the one expected in reply to a
    /// particular command.  This sequence number is not related to any
    /// interaction between the CCID and the ICC itself, but simply tracks the
    /// USB bulk message exchanges between the host and the CCID. The initial
    /// value of the sequence number is not important, but typically starts at
    /// zero.
    pub bSeq: u8,

    /// Reserved for Future Use
    pub abRFU: [u8; 3],
}

/// CCID message identifiers
#[repr(u8)]
pub enum MessageType {
    PC_to_RDR_IccPowerOn = 0x62,
    PC_to_RDR_IccPowerOff = 0x63,
    PC_to_RDR_GetSlotStatus = 0x65,
    PC_to_RDR_XfrBlock = 0x6F,
    PC_to_RDR_GetParameters = 0x6C,
    PC_to_RDR_ResetParameters = 0x6D,
    PC_to_RDR_SetParameters = 0x61,
    PC_to_RDR_Escape = 0x6B,
    PC_to_RDR_IccClock = 0x6E,
    PC_to_RDR_T0APDU = 0x6A,
    PC_to_RDR_Secure = 0x69,
    PC_to_RDR_Mechanical = 0x71,
    PC_to_RDR_Abort = 0x72,
    PC_to_RDR_SetDataRateAndClockFrequency = 0x73,

    RDR_to_PC_DataBlock = 0x80,
    RDR_to_PC_SlotStatus = 0x81,
    RDR_to_PC_Parameters = 0x82,
    RDR_to_PC_Escape = 0x83,
    RDR_to_PC_DataRateAndClockFrequency = 0x84,
}

use std::fmt::Debug;
use xcb_rust_protocol::con::{SocketIo, XcbBuffers, XcbState};
use xcb_rust_protocol::connection::xproto::get_input_focus;
use xcb_rust_protocol::proto::xproto::{Setup, VisualClassEnum};
use xcb_rust_protocol::util::VariableLengthFromBytes;
use xcb_rust_protocol::{Error, XcbConnection};

pub(crate) struct BasicCon {
    pub(crate) offset: usize,
    pub(crate) seq: u16,
}

impl BasicCon {
    pub fn new() -> Self {
        Self { offset: 0, seq: 0 }
    }
}

impl XcbConnection for BasicCon {
    fn apply_offset<'a>(&mut self, buffer: &'a mut [u8]) -> &'a mut [u8] {
        &mut buffer[self.offset..]
    }

    fn max_request_size(&self) -> usize {
        usize::MAX
    }

    fn next_seq(&mut self) -> u16 {
        self.seq += 1;
        self.seq
    }

    fn keep_and_return_next_seq(&mut self) -> u16 {
        self.seq += 1;
        self.seq
    }

    fn forget(&mut self, _seq: u16) {}

    fn advance_reader(&mut self, _step: usize) {}

    fn advance_writer(&mut self, step: usize) {
        self.offset += step;
    }

    fn generate_id(&mut self, _buffers: &mut XcbBuffers) -> Result<u32, Error> {
        todo!()
    }

    fn block_for_reply(&mut self, _buffers: &mut XcbBuffers, _seq: u16) -> Result<Vec<u8>, Error> {
        todo!()
    }

    fn block_check_for_err(&mut self, _buffers: &mut XcbBuffers, _seq: u16) -> Result<(), Error> {
        Ok(())
    }

    fn major_opcode(&self, _extension_name: &'static str) -> Option<u8> {
        None
    }

    fn setup(&self) -> &Setup {
        todo!()
    }
}

#[derive(Debug)]
struct DummyIo {
    inner: Vec<u8>,
    offset: usize,
}
impl SocketIo for DummyIo {
    fn block_for_more_data(&mut self) -> Result<(), &'static str> {
        todo!()
    }

    fn use_read_buffer<E, F: FnOnce(&[u8]) -> Result<usize, E>>(
        &mut self,
        read_op: F,
    ) -> Result<(), E>
    where
        E: Debug,
    {
        todo!()
    }

    fn use_write_buffer<E, F: FnOnce(&mut [u8]) -> Result<usize, E>>(
        &mut self,
        write_op: F,
    ) -> Result<(), E>
    where
        E: Debug,
    {
        let val = (write_op)(&mut self.inner).unwrap();
        self.offset += val;
        Ok(())
    }
}

struct DummyState;

impl XcbState for DummyState {
    fn major_opcode(&self, extension_name: &'static str) -> Option<u8> {
        todo!()
    }

    fn next_seq(&mut self) -> u16 {
        1
    }

    fn keep_and_return_next_seq(&mut self) -> u16 {
        1
    }

    fn max_request_size(&self) -> usize {
        todo!()
    }

    fn setup(&self) -> &Setup {
        todo!()
    }

    fn generate_id<IO: SocketIo>(&mut self, io: &mut IO) -> Result<u32, Error> {
        todo!()
    }

    fn block_for_reply<IO: SocketIo>(&mut self, io: &mut IO, seq: u16) -> Result<Vec<u8>, Error> {
        todo!()
    }

    fn block_check_err<IO: SocketIo>(&mut self, io: &mut IO, seq: u16) -> Result<(), Error> {
        todo!()
    }

    fn forget(&mut self, seq: u16) {
        todo!()
    }
}

#[test]
fn test_one() {
    let mut buf = vec![0u8; u32::MAX as usize];
    let mut dummy = DummyIo {
        inner: buf,
        offset: 0,
    };
    let mut cookie = get_input_focus(&mut dummy, &mut DummyState, false).unwrap();
    assert_eq!(4, dummy.offset);
    assert_eq!(1, cookie.seq);
}

fn get_setup_data() -> Vec<u8> {
    let mut s = Vec::new();

    let vendor_len: u16 = 2;
    let num_pixmap_formats: u8 = 1;
    let roots_len: u8 = 18;
    let header: u16 = 10;
    let length: u16 =
        header + vendor_len + 2 * u16::from(num_pixmap_formats) + u16::from(roots_len);

    s.extend(&[1, 0]); // Status "success" and padding
    s.extend(&11u16.to_ne_bytes()); // major version
    s.extend(&0u16.to_ne_bytes()); // minor version
    s.extend(&length.to_ne_bytes()); // length
    s.extend(&0x1234_5678u32.to_ne_bytes()); // release number
    s.extend(&0x1000_0000u32.to_ne_bytes()); // resource id base
    s.extend(&0x0000_00ffu32.to_ne_bytes()); // resource id mask
    s.extend(&0u32.to_ne_bytes()); // motion buffer size
    s.extend(&6u16.to_ne_bytes()); // vendor length
    s.extend(&0x100u16.to_ne_bytes()); // maximum request length
    s.push(1); // roots length
    s.push(num_pixmap_formats); // pixmap formats length
    s.push(1); // image byte order: MSB first
    s.push(1); // bitmap format bit order: MSB first
    s.push(0); // scanline unit
    s.push(0); // scanline pad
    s.push(0); // min keycode
    s.push(0xff); // max keycode
    s.extend(&[0, 0, 0, 0]); // padding
    assert_eq!(s.len(), usize::from(header) * 4);

    s.extend("Vendor  ".bytes()); // vendor + padding
    assert_eq!(s.len(), usize::from(header + vendor_len) * 4);

    // Pixmap formats, we said above there is one entry
    s.push(15); // depth
    s.push(42); // bits per pixel
    s.push(21); // scanline pad
    s.extend(&[0, 0, 0, 0, 0]); // padding
    assert_eq!(
        s.len(),
        4 * usize::from(header + vendor_len + 2 * u16::from(num_pixmap_formats))
    );

    // Screens, we said above there is one entry
    s.extend(&1u32.to_ne_bytes()); // root window
    s.extend(&2u32.to_ne_bytes()); // default colormap
    s.extend(&3u32.to_ne_bytes()); // white pixel
    s.extend(&4u32.to_ne_bytes()); // black pixel
    s.extend(&0u32.to_ne_bytes()); // current input masks
    s.extend(&0u16.to_ne_bytes()); // width in pixels
    s.extend(&0u16.to_ne_bytes()); // height in pixels
    s.extend(&0u16.to_ne_bytes()); // width in mm
    s.extend(&0u16.to_ne_bytes()); // height in mm
    s.extend(&0u16.to_ne_bytes()); // min installed maps
    s.extend(&0u16.to_ne_bytes()); // max installed maps
    s.extend(&0u32.to_ne_bytes()); // root visual
    s.extend(&[0, 0, 0, 1]); // backing stores, save unders, root depths, allowed depths len

    // one depth entry
    s.extend(&[99, 0]); // depth and padding
    s.extend(&1u16.to_ne_bytes()); // width visuals len
    s.extend(&[0, 0, 0, 0]); // padding

    // one visualtype entry
    s.extend(&80u32.to_ne_bytes()); // visualid
    s.extend(&[2, 4]); // class and bits per rgb value
    s.extend(&81u16.to_ne_bytes()); // colormap entries
    s.extend(&82u32.to_ne_bytes()); // red mask
    s.extend(&83u32.to_ne_bytes()); // green mask
    s.extend(&84u32.to_ne_bytes()); // blue mask
    s.extend(&[0, 0, 0, 0]); // padding

    assert_eq!(s.len(), usize::from(length) * 4);

    s
}

#[test]
fn parse_setup() {
    let setup = get_setup_data();
    let (setup, _offset) = Setup::from_bytes(&*setup).unwrap();

    //assert_eq!(offset, setup.length);

    assert_eq!(
        (1, 11, 0),
        (
            setup.status,
            setup.protocol_major_version,
            setup.protocol_minor_version
        )
    );
    assert_eq!(0x1234_5678, setup.release_number);
    assert_eq!((0, 0xff), (setup.min_keycode, setup.max_keycode));
    assert_eq!(b"Vendor", &setup.vendor[..]);

    assert_eq!(1, setup.pixmap_formats.len());
    let format = &setup.pixmap_formats[0];
    assert_eq!(15, format.depth);
    assert_eq!(42, format.bits_per_pixel);
    assert_eq!(21, format.scanline_pad);

    assert_eq!(1, setup.roots.len());
    let root = &setup.roots[0];
    assert_eq!(
        (1, 2, 3, 4),
        (
            root.root,
            root.default_colormap,
            root.white_pixel,
            root.black_pixel
        )
    );

    assert_eq!(1, root.allowed_depths.len());
    let depth = &root.allowed_depths[0];
    assert_eq!(99, depth.depth);

    assert_eq!(1, depth.visuals.len());
    let visual = &depth.visuals[0];
    assert_eq!(80, visual.visual_id);
    assert_eq!(VisualClassEnum::STATIC_COLOR.0, visual.class.0);
    assert_eq!(4, visual.bits_per_rgb_value);
    assert_eq!(81, visual.colormap_entries);
    assert_eq!(82, visual.red_mask);
    assert_eq!(83, visual.green_mask);
    assert_eq!(84, visual.blue_mask);
}

/// A canary for the cases where we transmute stuff between byte arrays
#[test]
#[allow(unsafe_code, clippy::drop_copy)]
fn ensure_safe_transmute_u32_to_u8_arr_len_known() {
    let orig: [u32; 3] = [8179234, 559, 58928];
    let want: [u8; 12] = [34, 206, 124, 0, 47, 2, 0, 0, 48, 230, 0, 0];
    unsafe {
        let u8_res: [u8; 12] = core::mem::transmute(orig);
        assert_eq!(u8_res, want);
    }
}

#[test]
#[allow(unsafe_code, clippy::drop_copy)]
fn ensure_safe_transmute_u8_to_u32_len_known() {
    let orig: [u8; 12] = [34, 206, 124, 0, 47, 2, 0, 0, 48, 230, 0, 0];
    let want: [u32; 3] = [8179234, 559, 58928];
    unsafe {
        let u32_res: [u32; 3] = core::mem::transmute(orig);
        assert_eq!(u32_res, want);
    }
}

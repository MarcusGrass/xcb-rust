#[allow(unused_imports)]
use crate::cookie::Cookie;
#[allow(unused_imports)]
use crate::cookie::FixedCookie;
#[allow(unused_imports)]
use crate::cookie::VoidCookie;
#[allow(unused_imports)]
use crate::util::FixedLengthSerialize;
#[allow(unused_imports)]
use crate::util::VariableLengthSerialize;
pub trait DamageConnection {
    fn query_version(
        &mut self,
        client_major_version: u32,
        client_minor_version: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::damage::QueryVersionReply, 32>>;

    fn create(
        &mut self,
        damage: crate::proto::damage::Damage,
        drawable: crate::proto::xproto::Drawable,
        level: crate::proto::damage::ReportLevelEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn destroy(
        &mut self,
        damage: crate::proto::damage::Damage,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn subtract(
        &mut self,
        damage: crate::proto::damage::Damage,
        repair: crate::proto::xfixes::RegionEnum,
        parts: crate::proto::xfixes::RegionEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;

    fn add(
        &mut self,
        drawable: crate::proto::xproto::Drawable,
        region: crate::proto::xfixes::Region,
        forget: bool,
    ) -> crate::error::Result<VoidCookie>;
}
impl<C> DamageConnection for C
where
    C: crate::con::XcbConnection,
{
    fn query_version(
        &mut self,
        client_major_version: u32,
        client_minor_version: u32,
        forget: bool,
    ) -> crate::error::Result<FixedCookie<crate::proto::damage::QueryVersionReply, 32>> {
        let major_opcode = self
            .major_opcode(crate::proto::damage::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::damage::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let client_major_version_bytes = client_major_version.serialize_fixed();
        let client_minor_version_bytes = client_minor_version.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                0,
                length[0],
                length[1],
                client_major_version_bytes[0],
                client_major_version_bytes[1],
                client_major_version_bytes[2],
                client_major_version_bytes[3],
                client_minor_version_bytes[0],
                client_minor_version_bytes[1],
                client_minor_version_bytes[2],
                client_minor_version_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(FixedCookie::new(seq))
    }

    fn create(
        &mut self,
        damage: crate::proto::damage::Damage,
        drawable: crate::proto::xproto::Drawable,
        level: crate::proto::damage::ReportLevelEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::damage::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::damage::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let damage_bytes = damage.serialize_fixed();
        let drawable_bytes = drawable.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                1,
                length[0],
                length[1],
                damage_bytes[0],
                damage_bytes[1],
                damage_bytes[2],
                damage_bytes[3],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                level.0 as u8,
                0,
                0,
                0,
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn destroy(
        &mut self,
        damage: crate::proto::damage::Damage,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::damage::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::damage::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (2u16).to_ne_bytes();
        let damage_bytes = damage.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..8)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                2,
                length[0],
                length[1],
                damage_bytes[0],
                damage_bytes[1],
                damage_bytes[2],
                damage_bytes[3],
            ]);
        self.advance_writer(8);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn subtract(
        &mut self,
        damage: crate::proto::damage::Damage,
        repair: crate::proto::xfixes::RegionEnum,
        parts: crate::proto::xfixes::RegionEnum,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::damage::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::damage::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (4u16).to_ne_bytes();
        let damage_bytes = damage.serialize_fixed();
        let repair_bytes = (repair.0 as u32).serialize_fixed();
        let parts_bytes = (parts.0 as u32).serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..16)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                3,
                length[0],
                length[1],
                damage_bytes[0],
                damage_bytes[1],
                damage_bytes[2],
                damage_bytes[3],
                repair_bytes[0],
                repair_bytes[1],
                repair_bytes[2],
                repair_bytes[3],
                parts_bytes[0],
                parts_bytes[1],
                parts_bytes[2],
                parts_bytes[3],
            ]);
        self.advance_writer(16);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }

    fn add(
        &mut self,
        drawable: crate::proto::xproto::Drawable,
        region: crate::proto::xfixes::Region,
        forget: bool,
    ) -> crate::error::Result<VoidCookie> {
        let major_opcode = self
            .major_opcode(crate::proto::damage::EXTENSION_NAME)
            .ok_or(crate::error::Error::MissingExtension(
                crate::proto::damage::EXTENSION_NAME,
            ))?;
        let length: [u8; 2] = (3u16).to_ne_bytes();
        let drawable_bytes = drawable.serialize_fixed();
        let region_bytes = region.serialize_fixed();
        let buf = self.write_buf();
        buf.get_mut(..12)
            .ok_or(crate::error::Error::Serialize)?
            .copy_from_slice(&[
                major_opcode,
                4,
                length[0],
                length[1],
                drawable_bytes[0],
                drawable_bytes[1],
                drawable_bytes[2],
                drawable_bytes[3],
                region_bytes[0],
                region_bytes[1],
                region_bytes[2],
                region_bytes[3],
            ]);
        self.advance_writer(12);
        let seq = if forget {
            self.next_seq()
        } else {
            self.keep_and_return_next_seq()
        };
        Ok(VoidCookie::new(seq))
    }
}

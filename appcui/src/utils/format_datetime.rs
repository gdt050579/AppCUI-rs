use chrono::{Datelike, NaiveDateTime, Timelike};

static SHORT_MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];


pub struct FormatDateTime {

}
impl FormatDateTime {
    pub fn normal<'a>(dt: &NaiveDateTime, buf: &'a mut [u8]) -> Option<&'a str> {
        if buf.len() < 20 {
            return None;
        }
        let year = dt.year();
        buf[0] = (year / 1000) as u8 + 48;
        buf[1] = ((year % 1000) / 100) as u8 + 48;
        buf[2] = ((year % 100) / 10) as u8 + 48;
        buf[3] = (year % 10) as u8 + 48;
        buf[4] = b'-';
        let month = SHORT_MONTHS[dt.month().saturating_sub(1) as usize].as_bytes();
        buf[5] = month[0];
        buf[6] = month[1];
        buf[7] = month[2];
        buf[8] = b'-';
        let day = dt.day();
        buf[9] = ((day / 10) as u8) + 48;  
        buf[10] = ((day % 10) as u8) + 48;
        buf[11] = b' ';
        let hour = dt.hour();
        if hour < 10 {
            buf[12] = b' ';
        } else {
            buf[12] = ((hour / 10) as u8) + 48;
        }
        buf[13] = ((hour % 10) as u8) + 48;
        buf[14] = b':';
        let minute = dt.minute();
        buf[15] = ((minute / 10) as u8) + 48;
        buf[16] = ((minute % 10) as u8) + 48;
        buf[17] = b':';
        let second = dt.second();
        buf[18] = ((second / 10) as u8) + 48;
        buf[19] = ((second % 10) as u8) + 48;

        Some(unsafe { core::str::from_utf8_unchecked(&buf[..20]) })
    }
    pub fn full<'a>(dt: &NaiveDateTime, buf: &'a mut [u8]) -> Option<&'a str> {
        if buf.len() < 20 {
            return None;
        }
        let year = dt.year();
        buf[0] = (year / 1000) as u8 + 48;
        buf[1] = ((year % 1000) / 100) as u8 + 48;
        buf[2] = ((year % 100) / 10) as u8 + 48;
        buf[3] = (year % 10) as u8 + 48;
        buf[4] = b'-';
        let month = SHORT_MONTHS[dt.month().saturating_sub(1) as usize].as_bytes();
        buf[5] = month[0];
        buf[6] = month[1];
        buf[7] = month[2];
        buf[8] = b'-';
        let day = dt.day();
        buf[9] = ((day / 10) as u8) + 48;  
        buf[10] = ((day % 10) as u8) + 48;
        buf[11] = b' ';
        let hour = dt.hour();
        if hour < 10 {
            buf[12] = b' ';
        } else {
            buf[12] = ((hour / 10) as u8) + 48;
        }
        buf[13] = ((hour % 10) as u8) + 48;
        buf[14] = b':';
        let minute = dt.minute();
        buf[15] = ((minute / 10) as u8) + 48;
        buf[16] = ((minute % 10) as u8) + 48;
        buf[17] = b':';
        let second = dt.second();
        buf[18] = ((second / 10) as u8) + 48;
        buf[19] = ((second % 10) as u8) + 48;

        Some(unsafe { core::str::from_utf8_unchecked(&buf[..20]) })
    }
    pub fn short<'a>(dt: &NaiveDateTime, buf: &'a mut [u8]) -> Option<&'a str> {
        if buf.len() < 20 {
            return None;
        }
        let year = dt.year();
        buf[0] = (year / 1000) as u8 + 48;
        buf[1] = ((year % 1000) / 100) as u8 + 48;
        buf[2] = ((year % 100) / 10) as u8 + 48;
        buf[3] = (year % 10) as u8 + 48;
        buf[4] = b'-';
        let month = SHORT_MONTHS[dt.month().saturating_sub(1) as usize].as_bytes();
        buf[5] = month[0];
        buf[6] = month[1];
        buf[7] = month[2];
        buf[8] = b'-';
        let day = dt.day();
        buf[9] = ((day / 10) as u8) + 48;  
        buf[10] = ((day % 10) as u8) + 48;
        buf[11] = b' ';
        let hour = dt.hour();
        if hour < 10 {
            buf[12] = b' ';
        } else {
            buf[12] = ((hour / 10) as u8) + 48;
        }
        buf[13] = ((hour % 10) as u8) + 48;
        buf[14] = b':';
        let minute = dt.minute();
        buf[15] = ((minute / 10) as u8) + 48;
        buf[16] = ((minute % 10) as u8) + 48;
        buf[17] = b':';
        let second = dt.second();
        buf[18] = ((second / 10) as u8) + 48;
        buf[19] = ((second % 10) as u8) + 48;

        Some(unsafe { core::str::from_utf8_unchecked(&buf[..20]) })
    }
}